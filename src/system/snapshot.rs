use crate::percentage::Percentage;
use crate::system::trigger::Trigger;

use battery::{Battery, State};
use std::fmt;

#[derive(Default)]
pub struct Snapshot {
    state: State,
    percentage: Percentage,
}

impl Snapshot {
    pub fn triggers(&self, prev: &Snapshot, low_thresh: &Percentage) -> Option<Vec<Trigger>> {
        let mut triggers = Vec::<Trigger>::new();

        if (&self.percentage <= low_thresh) && (&prev.percentage > low_thresh) {
            triggers.push(Trigger::Low);
        }

        if prev.state != self.state {
            match self.state {
                State::Charging => triggers.push(Trigger::Plugged),
                State::Discharging => {
                    triggers.push(Trigger::Unplugged);

                    if &self.percentage <= low_thresh {
                        triggers.push(Trigger::Low);
                    }
                }
                State::Full => triggers.push(Trigger::Full),
                _ => (),
            }
        }

        if triggers.len() > 0 {
            return Some(triggers);
        }

        None
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

impl fmt::Display for Snapshot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} at {}", self.state, self.percentage)
    }
}
