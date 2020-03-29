use crate::percentage::Percentage;
use crate::trigger::Trigger;

use battery::{Battery, Manager, State};

#[derive(Default)]
struct Snapshot {
    state: State,
    percentage: Percentage,
}

impl Snapshot {
    fn plug_trigger(&self, prev: &Snapshot) -> Option<Trigger> {
        if prev.state != self.state {
            match self.state {
                State::Charging => Some(Trigger::Plugged(true)),
                State::Discharging => Some(Trigger::Plugged(false)),
                _ => None,
            }
        } else {
            None
        }
    }

    fn full_trigger(&self, prev: &Snapshot) -> Option<Trigger> {
        if (self.state == State::Full) && (prev.state != self.state) {
            Some(Trigger::Full)
        } else {
            None
        }
    }

    fn low_trigger(&self, prev: &Snapshot, thresh: Percentage) -> Option<Trigger> {
        if (self.percentage <= thresh) && (prev.percentage > thresh) {
            Some(Trigger::Low)
        } else {
            None
        }
    }
}

impl From<&Battery> for Snapshot {
    fn from(bat: &Battery) -> Snapshot {
        Snapshot {
            state: bat.state(),
            percentage: Percentage::from((bat.energy() / bat.energy_full()).value),
        }
    }
}

struct Trend {
    bat: Battery,
    prev: Snapshot,
}

pub struct System {
    manager: Manager,
    trends: Vec<Trend>,
}

impl System {
    pub fn load() -> Result<System, battery::Error> {
        log::debug!("initializing battery manager");
        let manager = match battery::Manager::new() {
            Ok(m) => m,
            Err(e) => {
                log::error!("failed to initialize battery manager: {}", e);
                return Err(e);
            }
        };

        log::debug!("initializing battery trends");
        let trends: Vec<Trend> = manager
            .batteries()
            .expect("failed to find batteries")
            .map(|r| match r {
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
                Trend {
                    bat: bat,
                    prev: prev,
                }
            })
            .collect();

        return Ok(System {
            manager: manager,
            trends: trends,
        });
    }

    pub fn step(&mut self) -> Option<Vec<Trigger>> {
        let mut triggers = Vec::<Trigger>::new();
        for trend in &mut self.trends.iter_mut() {
            log::debug!("reading battery status");

            match self.manager.refresh(&mut trend.bat) {
                Err(e) => {
                    log::warn!("couldn't read battery state {}", e);
                    continue;
                }
                _ => (),
            }

            let curr = Snapshot::from(&trend.bat);

            match curr.plug_trigger(&trend.prev) {
                Some(t) => triggers.push(t),
                None => (),
            }

            match curr.full_trigger(&trend.prev) {
                Some(t) => triggers.push(t),
                None => (),
            }

            let twenty_percent = Percentage::from(0.2);
            match curr.low_trigger(&trend.prev, twenty_percent) {
                Some(t) => triggers.push(t),
                None => (),
            }

            trend.prev = curr;
        }

        if triggers.len() > 0 {
            Some(triggers)
        } else {
            None
        }
    }
}
