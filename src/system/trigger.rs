use notify_rust::{Notification, NotificationUrgency};

pub enum Trigger {
    Plugged,
    Unplugged,
    Full,
    Low,
}

impl From<&Trigger> for Notification {
    fn from(t: &Trigger) -> Notification {
        let mut n = Notification::new();

        match t {
            Trigger::Plugged => n.summary("Plugged In").icon("battery-charging-050"),
            Trigger::Unplugged => n.summary("Unplugged").icon("battery-050"),
            Trigger::Full => n.summary("Fully Charged").icon("battery-full-charged"),
            Trigger::Low => n
                .summary("Low Battery")
                .icon("battery-caution")
                .urgency(NotificationUrgency::Critical),
        };

        return n;
    }
}

impl Trigger {
    pub fn notify(&self) {
        if let Err(e) = Notification::from(self).show() {
            log::error!("failed to show notification: {}", e)
        }
    }
}
