use battery::units::ratio::percent;
use battery::units::{Ratio, Time};
use battery::{Batteries, Battery, State};
use std::fmt;

#[derive(Default)]
pub struct Snapshot {
    state: State,
    charge: Ratio,
    full_time: Option<Time>,
    empty_time: Option<Time>,
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

    pub fn did_deplete(&self, prev: &Snapshot, low_thresh: Ratio) -> bool {
        self.is_below(low_thresh) && (prev.charge > low_thresh)
    }

    pub fn is_below(&self, low_thresh: Ratio) -> bool {
        self.charge <= low_thresh
    }

    pub fn charge(&self) -> Ratio {
        self.charge
    }

    pub fn full_time(&self) -> Option<Time> {
        self.full_time
    }

    pub fn empty_time(&self) -> Option<Time> {
        self.empty_time
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
        write!(f, "{} at {}%", self.state, self.charge.get::<percent>())
    }
}

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
        .map(|b| Self::from(b.unwrap()))
        .collect()
    }
}

impl From<Battery> for Trend {
    fn from(bat: Battery) -> Self {
        let prev = Snapshot::from(&bat);
        Self { bat, prev }
    }
}
