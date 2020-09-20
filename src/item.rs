use chrono::{DateTime, Utc, Duration};

#[derive(Clone)]
pub struct Item<T> {
    pub object: T,
    expiry: Option<DateTime<Utc>>,
}

impl<T> Item<T> {
    pub fn new(object: T, item_duration: Option<Duration>) -> Self {
        let expiry = match item_duration {
            Some(duration) => Some(Utc::now() + duration),
            None => None,
        };
        Item {
            object,
            expiry,
        }
    }

    pub fn expired(&self) -> bool {
        if let Some(expiry) = self.expiry {
            return expiry < Utc::now()
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use crate::item::Item;
    use async_std::task;

    const OBJECT: &str = "OBJECT";

    #[test]
    fn not_expired_when_duration_is_none() {
        task::block_on(async {
            let item = Item::new(OBJECT, None);
            assert_eq!(item.expired(), false);
        })
    }
}