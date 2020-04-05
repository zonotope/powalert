use battery::Battery;
use std::cmp::Ordering;
use std::fmt;

#[derive(Default)]
pub struct Percentage {
    value: f32,
}

impl From<f32> for Percentage {
    fn from(v: f32) -> Percentage {
        Percentage { value: v }
    }
}

impl From<&Battery> for Percentage {
    fn from(bat: &Battery) -> Percentage {
        Self::from((bat.energy() / bat.energy_full()).value)
    }
}

impl PartialOrd for Percentage {
    fn partial_cmp(&self, other: &Percentage) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl PartialEq for Percentage {
    fn eq(&self, other: &Percentage) -> bool {
        self.value == other.value
    }
}

impl fmt::Display for Percentage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let display_val = self.value * 100.0;
        write!(f, "{}%", display_val)
    }
}
