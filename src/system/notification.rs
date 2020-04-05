use battery::Battery;
use notify_rust::{Notification, NotificationUrgency};

fn send(n: &mut Notification) {
    if let Err(e) = n.show() {
        log::error!("failed to send notification: {}", e)
    }
}

pub fn send_plugged(_bat: &Battery) {
    send(
        Notification::new()
            .summary("Plugged In")
            .icon("battery-charging-050"),
    )
}

pub fn send_unplugged(_bat: &Battery) {
    send(Notification::new().summary("Unplugged").icon("battery-050"))
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
