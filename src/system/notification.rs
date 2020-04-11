use std::time::Duration;

use battery::units::time::second;
use battery::units::{Ratio, Time};
use battery::Battery;
use notify_rust::{Notification, NotificationUrgency};

fn send(n: &mut Notification) {
    if let Err(e) = n.show() {
        log::error!("failed to send notification: {}", e)
    }
}

fn icon_level(p: Ratio) -> String {
    let level = (p.value * 10.0).round();
    format!("{:02}0", level)
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

pub fn send_plugged(bat: &Battery) {
    let icon_name = format!("battery-charging-{}", icon_level(bat.state_of_charge()));
    let full_time = time_string(bat.time_to_full(), "until full");
    let body = format!("On external power{}", full_time);

    send(
        Notification::new()
            .icon(&icon_name)
            .summary("Charging")
            .body(&body),
    )
}

pub fn send_unplugged(bat: &Battery) {
    let icon_name = format!("battery-{}", icon_level(bat.state_of_charge()));
    let empty_time = time_string(bat.time_to_empty(), "until empty");
    let body = format!("On battery power{}", empty_time);

    send(
        Notification::new()
            .icon(&icon_name)
            .summary("Unplugged")
            .body(&body),
    )
}

pub fn send_full(_bat: &Battery) {
    send(
        Notification::new()
            .icon("battery-full-charged")
            .summary("Fully Charged"),
    )
}

pub fn send_low(_bat: &Battery) {
    send(
        Notification::new()
            .icon("battery-caution")
            .summary("Low Battery")
            .urgency(NotificationUrgency::Critical),
    )
}
