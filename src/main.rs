use std::num::{ParseFloatError, ParseIntError};
use std::thread;
use std::time::Duration;

mod notification;
mod snapshot;

use snapshot::Snapshot;

use clap::{App, Arg, ArgMatches};
use simplelog::Config as LogConfig;
use simplelog::LevelFilter as LogLevel;
use simplelog::SimpleLogger as Logger;

use battery::units::Ratio;
use battery::{Batteries, Battery, Manager};

pub struct Trend {
    pub bat: Battery,
    pub prev: Snapshot,
}

impl Trend {
    pub fn from_batteries(bats: Batteries) -> Vec<Self> {
        bats.map(|r| match r {
            Ok(bat) => Some(bat),
            Err(e) => {
                log::warn!("error loading battery: {}", e);
                None
            }
        })
        .filter(|opt| opt.is_some())
        .map(|s| {
            let bat = s.unwrap();
            let prev = Snapshot::from(&bat);
            Self { bat, prev }
        })
        .collect()
    }
}

pub struct System {
    manager: Manager,
    trends: Vec<Trend>,
    low_threshold: Ratio,
}

impl System {
    pub fn load(low_threshold: Ratio) -> Result<Self, battery::Error> {
        log::debug!("initializing battery manager");
        let manager = match Manager::new() {
            Ok(m) => m,
            Err(e) => {
                log::error!("failed to initialize battery manager: {}", e);
                return Err(e);
            }
        };

        log::debug!("tracking battery trends");
        let trends = match manager.batteries() {
            Ok(bats) => Trend::from_batteries(bats),
            Err(e) => {
                log::error!("failed to find batteries: {}", e);
                return Err(e);
            }
        };

        return Ok(Self {
            manager,
            trends,
            low_threshold,
        });
    }

    pub fn step(&mut self) {
        log::debug!("reading current system state");

        for trend in &mut self.trends.iter_mut() {
            log::debug!("reading battery status");
            if let Err(e) = self.manager.refresh(&mut trend.bat) {
                log::error!("couldn't read battery status {}", e);
                continue;
            }

            let curr = Snapshot::from(&trend.bat);
            log::debug!("battery status is {}", curr);
            log::debug!("previous status: {}", trend.prev);

            if curr.did_plug(&trend.prev) {
                let plugged_note = notification::plugged(&trend.bat);

                log::info!("Sending charging notification");
                if let Err(e) = plugged_note.show() {
                    log::error!("failed to send charging notification: {}", e)
                }
            }

            if curr.did_unplug(&trend.prev) {
                let unplugged_note = notification::unplugged(&trend.bat);

                log::info!("Sending unplugged notification");
                if let Err(e) = unplugged_note.show() {
                    log::error!("failed to send unplugged notification: {}", e)
                }

                if curr.is_below(&self.low_threshold) {
                    let low_note = notification::low(&trend.bat);

                    log::info!("Sending low power notification after unplug");
                    if let Err(e) = low_note.show() {
                        log::error!("failed to send unplugged notification: {}", e)
                    }
                }
            }

            if curr.did_fill(&trend.prev) {
                let full_note = notification::full(&trend.bat);

                log::info!("Sending full notification");
                if let Err(e) = full_note.show() {
                    log::error!("failed to send full notification: {}", e)
                }
            }

            if curr.did_deplete(&trend.prev, &self.low_threshold) {
                let low_note = notification::low(&trend.bat);
                log::info!("Sending low power notification");
                if let Err(e) = low_note.show() {
                    log::error!("failed to send low notification: {}", e)
                }
            }

            trend.prev = curr;
        }
    }
}

fn cli<'a, 'b>() -> App<'a, 'b> {
    App::new("powalert")
        .version("0.1.0")
        .about("System power notifier")
        .arg(
            Arg::with_name("threshold")
                .short("t")
                .long("low-threshold")
                .takes_value(true)
                .help("Threshold percentage to send low power notifications"),
        )
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .takes_value(true)
                .help("Verbosity level (either '0', '1', or '2')"),
        )
        .arg(
            Arg::with_name("interval")
                .short("i")
                .long("interval")
                .takes_value(true)
                .help("Pause time before polling batteries for updates in seconds"),
        )
}

fn parse_interval(opts: &ArgMatches) -> Result<Duration, ParseIntError> {
    let i = opts.value_of("interval").unwrap_or("1").parse::<u64>()?;

    return Ok(Duration::from_secs(i));
}

fn parse_threshold(opts: &ArgMatches) -> Result<Ratio, ParseFloatError> {
    let f = opts.value_of("interval").unwrap_or("20").parse::<f32>()?;

    return Ok(Ratio::from(f / 100.0));
}

fn parse_verbosity(opts: &ArgMatches) -> LogLevel {
    match opts.value_of("verbose").unwrap_or("0") {
        "0" => LogLevel::Warn,
        "1" => LogLevel::Info,
        "2" | _ => LogLevel::Debug,
    }
}

fn init_logging(level: LogLevel) {
    let conf = LogConfig::default();
    Logger::init(level, conf).expect("failed to start logger");
}

fn main() {
    let opts = cli().get_matches();
    let interval = parse_interval(&opts).expect("interval must be an integer");
    let threshold = parse_threshold(&opts).expect("threshold must be a number");
    let log_level = parse_verbosity(&opts);

    init_logging(log_level);

    log::info!("powalert power notifier starting");

    log::debug!("initializing system");
    let mut system = match System::load(threshold) {
        Ok(s) => s,
        Err(e) => {
            log::error!("failed to initialize system: {}", e);
            return;
        }
    };

    log::info!("monitoring system power state");
    loop {
        system.step();

        thread::sleep(interval);
    }
}
