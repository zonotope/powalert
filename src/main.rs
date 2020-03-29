mod percentage;
mod system;
mod trigger;

use crate::system::System;
use crate::trigger::Trigger;

use std::{thread, time};

use simplelog::Config as LogConfig;
use simplelog::LevelFilter as LogLevelFilter;
use simplelog::SimpleLogger;

fn init_logging() {
    let conf = LogConfig::default();
    let level = LogLevelFilter::Info;

    SimpleLogger::init(level, conf).expect("failed to start logger");
}

fn notify(_triggers: Vec<Trigger>) {}

fn main() {
    init_logging();

    log::info!("Power notifier starting");

    log::debug!("initializing system");
    let mut system = match System::load() {
        Ok(s) => s,
        Err(e) => {
            log::error!("initializing system: {}", e);
            return;
        }
    };

    let five_secs = time::Duration::from_secs(5);

    log::info!("watching system power state");
    loop {
        log::debug!("reading current system state");
        match system.step() {
            Some(triggers) => notify(triggers),
            None => (),
        }

        thread::sleep(five_secs);
    }
}
