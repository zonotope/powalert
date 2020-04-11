use crate::snapshot::Snapshot;

use battery::{Batteries, Battery};

pub struct Trend {
    pub bat: Battery,
    pub prev: Snapshot,
}

impl Trend {
    pub fn from_batteries(bats: Batteries) -> Vec<Trend> {
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
}
