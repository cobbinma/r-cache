use std::time::{Duration, Instant};

#[derive(Clone)]
pub struct Item<T> {
    pub object: T,
    expiry: Option<Instant>,
}

impl<T> Item<T> {
    pub fn new(object: T, item_duration: Option<Duration>) -> Self {
        let expiry = match item_duration {
            Some(duration) => Some(Instant::now() + duration),
            None => None,
        };
        Item { object, expiry }
    }

    pub fn expired(&self) -> bool {
        if let Some(expiry) = self.expiry {
            return expiry < Instant::now();
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use crate::item::Item;
    use std::time::Duration;

    const OBJECT: &str = "OBJECT";

    #[async_std::test]
    async fn not_expired_when_duration_is_none() {
        let item = Item::new(OBJECT, None);
        assert_eq!(item.expired(), false);
    }

    #[async_std::test]
    async fn expired_when_duration_is_zero() {
        let item = Item::new(OBJECT, Some(Duration::new(0, 0)));
        assert_eq!(item.expired(), true);
    }
}
