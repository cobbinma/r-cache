use chrono::{DateTime, Utc, Duration};

#[derive(Clone)]
pub struct Item<T> {
    pub object: T,
    expiry: DateTime<Utc>,
}

impl<T> Item<T> {
    pub fn new(object: T, item_duration: Duration) -> Self {
        Item {
            object,
            expiry: Utc::now() + item_duration,
        }
    }

    pub fn expired(&self) -> bool {
        self.expiry < Utc::now()
    }
}