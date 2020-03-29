use std::cmp::Ordering;

#[derive(Default)]
pub struct Percentage {
    value: f32,
}

impl From<f32> for Percentage {
    fn from(v: f32) -> Percentage {
        Percentage { value: v }
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
