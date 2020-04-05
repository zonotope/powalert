mod system;

use crate::system::System;

use std::{thread, time};

use simplelog::Config as LogConfig;
use simplelog::LevelFilter as LogLevelFilter;
use simplelog::SimpleLogger;

fn init_logging() {
    let conf = LogConfig::default();
    let level = LogLevelFilter::Info;

    SimpleLogger::init(level, conf).expect("failed to start logger");
}

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

    let one_sec = time::Duration::from_secs(1);

    log::info!("watching system power state");
    loop {
        log::debug!("reading current system state");
        system.step();

        thread::sleep(one_sec);
    }
}
