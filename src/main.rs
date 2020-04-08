mod system;

use crate::system::System;

use std::{thread, time};

use simplelog::Config as LogConfig;
use simplelog::LevelFilter as LogLevel;
use simplelog::SimpleLogger as Logger;

fn log_init() {
    let conf = LogConfig::default();
    let level = LogLevel::Info;

    Logger::init(level, conf).expect("failed to start logger");
}

fn main() {
    log_init();

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
