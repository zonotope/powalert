use battery::units::ratio::percent;
use battery::units::Ratio;
use battery::{Battery, State};
use std::fmt;

#[derive(Default)]
pub struct Snapshot {
    state: State,
    charge: Ratio,
}

impl Snapshot {
    pub fn did_plug(&self, prev: &Snapshot) -> bool {
        (self.state == State::Charging) && (prev.state != self.state)
    }

    pub fn did_unplug(&self, prev: &Snapshot) -> bool {
        (self.state == State::Discharging) && (prev.state != self.state)
    }

    pub fn did_fill(&self, prev: &Snapshot) -> bool {
        (self.state == State::Full) && (prev.state != self.state)
    }

    pub fn did_deplete(&self, prev: &Snapshot, low_thresh: &Ratio) -> bool {
        self.is_below(low_thresh) && (&prev.charge > low_thresh)
    }

    pub fn is_below(&self, low_thresh: &Ratio) -> bool {
        &self.charge <= low_thresh
    }
}

impl From<&Battery> for Snapshot {
    fn from(bat: &Battery) -> Snapshot {
        Snapshot {
            state: bat.state(),
            charge: bat.state_of_charge(),
        }
    }
}

impl fmt::Display for Snapshot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} at {:.2}%", self.state, self.charge.get::<percent>())
    }
}
