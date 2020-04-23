use std::fmt;

use battery::units::ratio::percent;
use battery::units::{Ratio, Time};
use battery::{Battery, State};

#[derive(Default)]
pub struct Snapshot {
    state: State,
    charge: Ratio,
    full_time: Option<Time>,
    empty_time: Option<Time>,
}

impl Snapshot {
    pub fn charge(&self) -> Ratio {
        self.charge
    }

    pub fn full_time(&self) -> Option<Time> {
        self.full_time
    }

    pub fn empty_time(&self) -> Option<Time> {
        self.empty_time
    }

    pub fn did_plug(&self, prev: &Snapshot) -> bool {
        (self.state == State::Charging) && (prev.state != self.state)
    }

    pub fn did_unplug(&self, prev: &Snapshot) -> bool {
        (self.state == State::Discharging) && (prev.state != self.state)
    }

    pub fn did_fill(&self, prev: &Snapshot) -> bool {
        (self.state == State::Full) && (prev.state != self.state)
    }

    pub fn did_deplete(&self, prev: &Snapshot, low_thresh: Ratio) -> bool {
        self.is_below(low_thresh) && (prev.charge > low_thresh)
    }

    pub fn is_below(&self, low_thresh: Ratio) -> bool {
        self.charge <= low_thresh
    }
}

impl From<&Battery> for Snapshot {
    fn from(bat: &Battery) -> Self {
        Self {
            state: bat.state(),
            charge: bat.state_of_charge(),
            full_time: bat.time_to_full(),
            empty_time: bat.time_to_empty(),
        }
    }
}

impl fmt::Display for Snapshot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}%]", self.state, self.charge.get::<percent>())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_did_plug() {
        let curr = Snapshot {
            state: State::Charging,
            ..Snapshot::default()
        };

        let prev = Snapshot::default();

        assert_eq!(curr.did_plug(&prev), true)
    }

    #[test]
    fn test_did_unplug() {
        let curr = Snapshot {
            state: State::Discharging,
            ..Snapshot::default()
        };

        let prev = Snapshot::default();

        assert_eq!(curr.did_unplug(&prev), true)
    }

    #[test]
    fn test_did_fill() {
        let curr = Snapshot {
            state: State::Full,
            ..Snapshot::default()
        };

        let prev = Snapshot::default();

        assert_eq!(curr.did_fill(&prev), true)
    }

    #[test]
    fn test_is_below() {
        let low_val = 0.2;
        let low_ratio = Ratio::from(low_val);
        let test_charge = Ratio::from(low_val - 0.1);

        let curr = Snapshot {
            charge: test_charge,
            ..Snapshot::default()
        };

        assert_eq!(curr.is_below(low_ratio), true)
    }

    #[test]
    fn test_did_deplete() {
        let low_val = 0.2;
        let low_thresh = Ratio::from(low_val);
        let low_charge = Ratio::from(low_val - 0.1);
        let high_charge = Ratio::from(low_val + 0.1);

        let curr = Snapshot {
            charge: low_charge,
            ..Snapshot::default()
        };

        let prev = Snapshot {
            charge: high_charge,
            ..Snapshot::default()
        };

        assert_eq!(curr.did_deplete(&prev, low_thresh), true)
    }
}
