use super::snapshot::Snapshot;

use std::time::Duration;

use battery::units::ratio::percent;
use battery::units::time::second;
use battery::units::{Ratio, Time};

use notify_rust::{Notification, NotificationUrgency};

fn icon_level(r: Ratio) -> String {
    let level = (r.value * 10.0).round();
    format!("{:02}0", level)
}

fn percent_str(r: Ratio) -> String {
    format!("({}%)", r.get::<percent>())
}

fn time_string(time: Option<Time>, post: &str) -> String {
    match time {
        Some(t) => {
            let secs = t.get::<second>() as u64;
            let dur = Duration::from_secs(secs);
            let dur_string = humantime::format_duration(dur).to_string();
            format!(". {} {}", dur_string, post)
        }
        None => String::new(),
    }
}

pub fn plugged(snap: &Snapshot) -> Notification {
    let icon_name = format!("battery-charging-{}", icon_level(snap.charge()));

    let percentage = percent_str(snap.charge());
    let full_time = time_string(snap.full_time(), "until full");
    let body = format!("Charging battery {}{}", percentage, full_time);

    let mut note = Notification::new();
    note.icon(&icon_name).summary("Plugged In").body(&body);

    note
}

pub fn unplugged(snap: &Snapshot) -> Notification {
    let icon_name = format!("battery-{}", icon_level(snap.charge()));

    let percentage = percent_str(snap.charge());
    let empty_time = time_string(snap.empty_time(), "left");
    let body = format!("On battery power {}{}", percentage, empty_time);

    let mut note = Notification::new();
    note.icon(&icon_name).summary("Unplugged").body(&body);

    note
}

pub fn full(_snap: &Snapshot) -> Notification {
    let mut note = Notification::new();
    note.icon("battery-full-charged").summary("Fully Charged");

    note
}

pub fn low(snap: &Snapshot) -> Notification {
    let percentage = percent_str(snap.charge());
    let empty_time = time_string(snap.empty_time(), "left");
    let body = format!("Power level is almost empty {}{}", percentage, empty_time);

    let mut note = Notification::new();
    note.icon("battery-caution")
        .summary("Snapshot Low")
        .body(&body)
        .urgency(NotificationUrgency::Critical);

    note
}
