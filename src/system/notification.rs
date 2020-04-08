use battery::units::Ratio;
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

pub fn send_plugged(bat: &Battery) {
    let icon_name = format!("battery-charging-{}", icon_level(bat.state_of_charge()));
    send(
        Notification::new()
            .summary("Charging")
            .body("On external power")
            .icon(&icon_name),
    )
}

pub fn send_unplugged(bat: &Battery) {
    let icon_name = format!("battery-{}", icon_level(bat.state_of_charge()));
    send(
        Notification::new()
            .summary("Unplugged")
            .body("On battery power")
            .icon(&icon_name),
    )
}

pub fn send_full(_bat: &Battery) {
    send(
        Notification::new()
            .summary("Fully Charged")
            .icon("battery-full-charged"),
    )
}

pub fn send_low(_bat: &Battery) {
    send(
        Notification::new()
            .summary("Low Battery")
            .icon("battery-caution")
            .urgency(NotificationUrgency::Critical),
    )
}
