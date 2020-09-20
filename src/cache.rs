use async_std::sync::RwLock;
use std::collections::HashMap;

use crate::item::Item;
use chrono::Duration;
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
    use async_std::task;

    const KEY: i8 = 0;
    const VALUE: &str = "VALUE";

    #[test]
    fn set_and_get_value_with_duration() {
        task::block_on(async {
            let cache = Cache::new(Some(chrono::Duration::minutes(1)));
            cache.set(KEY, VALUE).await;
            let value = cache.get(KEY).await;
            match value {
                Some(value) => assert_eq!(value, VALUE),
                None => panic!("value was not found in cache")
            };
        })
    }

    #[test]
    fn set_and_get_value_without_duration() {
        task::block_on(async {
            let cache = Cache::new(None);
            cache.set(KEY, VALUE).await;
            let value = cache.get(KEY).await;
            match value {
                Some(value) => assert_eq!(value, VALUE),
                None => panic!("value was not found in cache")
            };
        })
    }
}