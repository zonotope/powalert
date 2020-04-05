mod snapshot;
mod trigger;

use crate::percentage::Percentage;
use snapshot::Snapshot;

use battery::{Batteries, Battery, Manager};

struct Trend {
    bat: Battery,
    prev: Snapshot,
}

pub struct System {
    manager: Manager,
    trends: Vec<Trend>,
    low_threshold: Percentage,
}

fn build_trends(bats: Batteries) -> Vec<Trend> {
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
        Trend { bat, prev }
    })
    .collect()
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
            Ok(bats) => build_trends(bats),
            Err(e) => {
                log::error!("failed to find batteries: {}", e);
                return Err(e);
            }
        };

        let low_threshold = Percentage::from(0.2);

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

            if let Some(trgs) = curr.triggers(&trend.prev, &self.low_threshold) {
                for trg in trgs.iter() {
                    trg.notify()
                }
            }

            trend.prev = curr;
        }
    }
}
