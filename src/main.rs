mod system;

use system::System;

use std::num::{ParseFloatError, ParseIntError};
use std::thread;
use std::time::Duration;

use clap::{App, Arg, ArgMatches};

use simplelog::Config as LogConfig;
use simplelog::LevelFilter as LogLevel;
use simplelog::SimpleLogger as Logger;

use battery::units::Ratio;

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
