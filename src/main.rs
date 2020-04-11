use std::{thread, time};

use simplelog::Config as LogConfig;
use simplelog::LevelFilter as LogLevel;
use simplelog::SimpleLogger as Logger;

mod notification;
mod snapshot;
mod trend;

use snapshot::Snapshot;
use trend::Trend;

use battery::units::Ratio;
use battery::Manager;

pub struct System {
    manager: Manager,
    trends: Vec<Trend>,
    low_threshold: Ratio,
}

impl System {
    pub fn load() -> Result<System, battery::Error> {
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

        let low_threshold = Ratio::from(0.2);

        return Ok(System {
            manager,
            trends,
            low_threshold,
        });
    }

    pub fn step(&mut self) {
        for trend in &mut self.trends.iter_mut() {
            log::debug!("reading battery status");
            if let Err(e) = self.manager.refresh(&mut trend.bat) {
                log::error!("couldn't read battery status {}", e);
                continue;
            }

            let curr = Snapshot::from(&trend.bat);
            log::debug!("battery status is {}", curr);

            if curr.did_plug(&trend.prev) {
                log::info!("Sending charging notification");
                notification::send_plugged(&trend.bat)
            }

            if curr.did_unplug(&trend.prev) {
                log::info!("Sending unplugged notification");
                notification::send_unplugged(&trend.bat);

                if curr.is_below(&self.low_threshold) {
                    log::info!("Sending low power notification after unplug");
                    notification::send_low(&trend.bat)
                }
            }

            if curr.did_fill(&trend.prev) {
                log::info!("Sending full notification");
                notification::send_full(&trend.bat)
            }

            if curr.did_deplete(&trend.prev, &self.low_threshold) {
                log::info!("Sending low power notification");
                notification::send_low(&trend.bat)
            }

            trend.prev = curr;
        }
    }
}

fn main() {
    let conf = LogConfig::default();
    let level = LogLevel::Info;
    Logger::init(level, conf).expect("failed to start logger");

    log::info!("Power notifier starting");

    log::debug!("initializing system");
    let mut system = match System::load() {
        Ok(s) => s,
        Err(e) => {
            log::error!("initializing system: {}", e);
            return;
        }
    };

    let one_sec = time::Duration::from_secs(1);

    log::info!("watching system power state");
    loop {
        log::debug!("reading current system state");
        system.step();

        thread::sleep(one_sec);
    }
}
