use async_std::sync::RwLock;
use std::collections::HashMap;

use crate::item::Item;
use std::time::Duration;
use std::hash::Hash;

pub struct Cache<T, V> {
    items: RwLock<HashMap<T, Item<V>>>,
    item_duration: Option<Duration>,
}

impl<T, V> Cache<T, V> {
    pub fn new(item_duration: Option<Duration>) -> Self {
        Cache {
            items: RwLock::new(HashMap::new()),
            item_duration,
        }
    }

    pub async fn get(&self, key: T) -> Option<V>
    where
        T: Eq + Hash,
        V: Clone
    {
        if let Some(item) = self.items.read().await.get(&key).cloned() {
            return if item.expired() {
                None
            } else {
                Some(item.object)
            };
        };
        None
    }

    pub async fn set(&self, key: T, value: V) -> Option<V>
    where
        T: Eq + Hash
    {
        self.items
            .write()
            .await
            .insert(key, Item::new(value, self.item_duration))
            .map(|item| item.object)
    }
}

#[cfg(test)]
mod tests {
    use crate::cache::Cache;
    use std::time::Duration;

    const KEY: i8 = 0;
    const VALUE: &str = "VALUE";

    #[async_std::test]
    async fn set_and_get_value_with_duration() {
        let cache = Cache::new(Some(Duration::from_secs(2)));
        cache.set(KEY, VALUE).await;
        let value = cache.get(KEY).await;
        match value {
            Some(value) => assert_eq!(value, VALUE),
            None => panic!("value was not found in cache")
        };
    }

    #[async_std::test]
    async fn set_and_get_value_without_duration() {
        let cache = Cache::new(None);
        cache.set(KEY, VALUE).await;
        let value = cache.get(KEY).await;
        match value {
            Some(value) => assert_eq!(value, VALUE),
            None => panic!("value was not found in cache")
        };
    }

    #[async_std::test]
    async fn replace_existing_value() {
        const NEW_VALUE: &str = "NEW_VALUE";
        let cache = Cache::new(Some(Duration::from_secs(2)));
        cache.set(KEY, VALUE).await;
        cache.set(KEY, NEW_VALUE).await;
        let value = cache.get(KEY).await;
        match value {
            Some(value) => assert_eq!(value, NEW_VALUE),
            None => panic!("value was not found in cache"),
        };
    }

    #[async_std::test]
    async fn remove_expired_item() {
        let cache = Cache::new(Some(Duration::from_secs(0)));
        cache.set(KEY, VALUE).await;
        cache.remove_expired_items().await.expect("should not error here");
        if cache.items.read().await.get(&KEY).is_some() {
            panic!("found expired item in cache")
        };
    }
}