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
}
