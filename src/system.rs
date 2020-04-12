mod notification;
mod snapshot;

use snapshot::Snapshot;

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
