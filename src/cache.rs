use async_std::sync::RwLock;
use std::collections::HashMap;

use crate::item::Item;
use std::hash::Hash;
use std::time::Duration;

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

    pub async fn get(&self, key: &T) -> Option<V>
    where
        T: Eq + Hash,
        V: Clone,
    {
        self.items
            .read()
            .await
            .get(key)
            .filter(|item| !item.expired())
            .map(|item| item.object.clone())
    }

    pub async fn set(&self, key: T, value: V, custom_duration: Option<Duration>) -> Option<V>
    where
        T: Eq + Hash,
    {
        self.items
            .write()
            .await
            .insert(
                key,
                Item::new(value, custom_duration.or(self.item_duration)),
            )
            .map(|item| item.object)
    }

    pub async fn remove_expired(&self)
    where
        T: Eq + Hash + Clone,
    {
        let expired_keys = self
            .items
            .read()
            .await
            .iter()
            .filter(|(_, item)| item.expired())
            .map(|(k, _)| k.clone())
            .collect::<Vec<T>>();

        for key in expired_keys {
            self.items.write().await.remove(&key);
        }
    }

    pub async fn remove(&self, key: &T) -> Option<V>
    where
        T: Eq + Hash,
    {
        self.items.write().await.remove(key).map(|item| item.object)
    }

    pub async fn clear(&self) {
        self.items.write().await.clear()
    }
}

#[cfg(test)]
mod tests {
    use crate::cache::Cache;
    use std::time::Duration;

    const KEY: i8 = 0;
    const VALUE: &str = "VALUE";

    #[async_std::test]
    async fn set_and_get_value_with_default_duration() {
        let cache = Cache::new(Some(Duration::from_secs(2)));
        cache.set(KEY, VALUE, None).await;
        let value = cache.get(&KEY).await;
        match value {
            Some(value) => assert_eq!(value, VALUE),
            None => panic!("value was not found in cache"),
        };
    }

    #[async_std::test]
    async fn set_and_get_value_without_duration() {
        let cache = Cache::new(None);
        cache.set(KEY, VALUE, None).await;
        let value = cache.get(&KEY).await;
        match value {
            Some(value) => assert_eq!(value, VALUE),
            None => panic!("value was not found in cache"),
        };
    }

    #[async_std::test]
    async fn set_and_get_value_with_custom_duration() {
        let cache = Cache::new(Some(Duration::from_secs(0)));
        cache.set(KEY, VALUE, Some(Duration::from_secs(2))).await;
        let value = cache.get(&KEY).await;
        match value {
            Some(value) => assert_eq!(value, VALUE),
            None => panic!("value was not found in cache"),
        };
    }

    #[async_std::test]
    async fn set_do_not_get_expired_value() {
        let cache = Cache::new(Some(Duration::from_secs(0)));
        cache.set(KEY, VALUE, None).await;
        let value = cache.get(&KEY).await;
        if value.is_some() {
            panic!("found expired value in cache")
        };
    }

    #[async_std::test]
    async fn set_replace_existing_value() {
        const NEW_VALUE: &str = "NEW_VALUE";
        let cache = Cache::new(Some(Duration::from_secs(2)));
        cache.set(KEY, VALUE, None).await;
        cache.set(KEY, NEW_VALUE, None).await;
        let value = cache.get(&KEY).await;
        match value {
            Some(value) => assert_eq!(value, NEW_VALUE),
            None => panic!("value was not found in cache"),
        };
    }

    #[async_std::test]
    async fn remove_expired_item() {
        let cache = Cache::new(Some(Duration::from_secs(0)));
        cache.set(KEY, VALUE, None).await;
        cache.remove_expired().await;
        if cache.items.read().await.get(&KEY).is_some() {
            panic!("found expired item in cache")
        };
    }

    #[async_std::test]
    async fn remove_expired_do_not_remove_not_expired_item() {
        let cache = Cache::new(Some(Duration::from_secs(2)));
        cache.set(KEY, VALUE, None).await;
        cache.remove_expired().await;
        if cache.items.read().await.get(&KEY).is_none() {
            panic!("could not find not expired item in cache")
        };
    }

    #[async_std::test]
    async fn clear_not_expired_item() {
        let cache = Cache::new(Some(Duration::from_secs(2)));
        cache.set(KEY, VALUE, None).await;
        cache.clear().await;
        if cache.items.read().await.get(&KEY).is_some() {
            panic!("found item in cache")
        };
    }

    #[async_std::test]
    async fn remove_remove_expired_item() {
        let cache = Cache::new(Some(Duration::from_secs(2)));
        cache.set(KEY, VALUE, None).await;
        if let None = cache.remove(&KEY).await {
            panic!("none returned from removing existing value")
        };
        if cache.items.read().await.get(&KEY).is_some() {
            panic!("found not expired item in cache")
        };
    }

    #[async_std::test]
    async fn remove_return_none_if_not_found() {
        let cache: Cache<i8, &str> = Cache::new(Some(Duration::from_secs(2)));
        if let Some(_) = cache.remove(&KEY).await {
            panic!("some value was returned from remove")
        };
    }
}
