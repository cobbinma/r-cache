use std::time::{Duration, Instant};

#[derive(Clone)]
pub struct Item<T> {
    pub object: T,
    expiry: Option<Instant>,
}

impl<T> Item<T> {
    // Creates a new cache item.
    pub fn new(object: T, item_duration: Option<Duration>) -> Self {
        let expiry = item_duration.map(|duration| Instant::now() + duration);
        Item { object, expiry }
    }

    // Returns true if the item has expired.
    pub fn expired(&self) -> bool {
        self.expiry
            .map(|expiry| expiry < Instant::now())
            .unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use async_std::task;

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
        task::sleep(Duration::from_millis(1)).await;
        assert_eq!(item.expired(), true);
    }
}
