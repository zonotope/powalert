use notify_rust::Notification;

pub enum Trigger {
    Plugged(bool),
    Full,
    Low,
}

impl From<&Trigger> for Notification {
    fn from(t: &Trigger) -> Notification {
        let mut n = Notification::new();

        match t {
            Trigger::Plugged(true) => n.summary("plugged in").icon("battery_plugged"),
            Trigger::Plugged(false) => n.summary("unplugged"),
            Trigger::Full => n.summary("full"),
            Trigger::Low => n.summary("low"),
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
