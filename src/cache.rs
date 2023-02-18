use dashmap::DashMap;

use crate::item::Item;
use std::hash::Hash;
use std::time::Duration;

pub struct Cache<T, V> {
    items: DashMap<T, Item<V>>,
    item_duration: Option<Duration>,
}

impl<T, V> Cache<T, V>
where
    T: Eq + Hash,
    V: Clone,
{
    /// Construct a new `Cache` with a default item expiration time.
    /// An item duration of `None` means items do not expire by default.
    ///
    /// # Example
    ///
    /// ```rust
    /// use async_std::sync::Arc;
    /// use async_std::task;
    /// use r_cache::cache::Cache;
    /// use std::time::Duration;
    ///
    /// const KEY: i8 = 0;
    /// const VALUE: &str = "VALUE";
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     let cache = Arc::new(Cache::new(Some(Duration::from_secs(5 * 60))));
    ///     task::spawn({
    ///         let cache = Arc::clone(&cache);
    ///         async move {
    ///             loop {
    ///                 task::sleep(Duration::from_secs(10 * 60)).await;
    ///                 cache.remove_expired();
    ///             }
    ///         }
    ///     });
    ///
    ///     cache.set(KEY, VALUE, None);
    ///
    ///     assert_eq!(VALUE, cache.get(&KEY).unwrap())
    /// }
    /// ```
    pub fn new(item_duration: Option<Duration>) -> Self {
        Cache {
            items: DashMap::new(),
            item_duration,
        }
    }

    /// Get a cache item associated with a given key.
    pub fn get(&self, key: &T) -> Option<V>
    where
        T: Eq + Hash,
        V: Clone,
    {
        self.items
            .get(key)
            .filter(|item| !item.expired())
            .map(|item| item.object.clone())
    }

    /// Set an item in the cache with an associated key.
    /// The item will have the default cache expiration time if custom duration of `None` is given.
    pub fn set(&self, key: T, value: V, custom_duration: Option<Duration>) -> Option<V>
    where
        T: Eq + Hash,
    {
        self.items
            .insert(
                key,
                Item::new(value, custom_duration.or(self.item_duration)),
            )
            .map(|item| item.object)
    }

    /// Remove all expired items from the cache.
    pub fn remove_expired(&self)
    where
        T: Eq + Hash + Clone,
    {
        self.items.retain(|_, item| !item.expired());
        self.shrink();
    }

    /// Remove an item from the cache associated with a given key.
    pub fn remove(&self, key: &T) -> Option<V>
    where
        T: Eq + Hash,
    {
        let item = self.items.remove(key).map(|(_, item)| item.object);
        self.shrink();

        item
    }

    /// Clear the entire cache of all items regardless of expiry times.
    pub fn clear(&self) {
        self.items.clear();
        self.shrink();
    }

    /// Reclaim memory from removed / cleared items
    pub fn shrink(&self)
    where
        T: Eq + Hash,
    {
        self.items.shrink_to_fit()
    }
}

#[cfg(test)]
mod tests {
    use crate::cache::Cache;
    use std::time::Duration;

    const KEY: i8 = 0;
    const VALUE: &str = "VALUE";

    #[test]
    fn set_and_get_value_with_default_duration() {
        let cache = Cache::new(Some(Duration::from_secs(2)));
        cache.set(KEY, VALUE, None);
        let value = cache.get(&KEY);
        match value {
            Some(value) => assert_eq!(value, VALUE),
            None => panic!("value was not found in cache"),
        };
    }

    #[test]
    fn set_and_get_value_without_duration() {
        let cache = Cache::new(None);
        cache.set(KEY, VALUE, None);
        let value = cache.get(&KEY);
        match value {
            Some(value) => assert_eq!(value, VALUE),
            None => panic!("value was not found in cache"),
        };
    }

    #[test]
    fn set_and_get_value_with_custom_duration() {
        let cache = Cache::new(Some(Duration::from_secs(0)));
        cache.set(KEY, VALUE, Some(Duration::from_secs(2)));
        let value = cache.get(&KEY);
        match value {
            Some(value) => assert_eq!(value, VALUE),
            None => panic!("value was not found in cache"),
        };
    }

    #[test]
    fn set_do_not_get_expired_value() {
        let cache = Cache::new(Some(Duration::from_secs(0)));
        cache.set(KEY, VALUE, None);
        let value = cache.get(&KEY);
        if value.is_some() {
            panic!("found expired value in cache")
        };
    }

    #[test]
    fn set_replace_existing_value() {
        const NEW_VALUE: &str = "NEW_VALUE";
        let cache = Cache::new(Some(Duration::from_secs(2)));
        cache.set(KEY, VALUE, None);
        cache.set(KEY, NEW_VALUE, None);
        let value = cache.get(&KEY);
        match value {
            Some(value) => assert_eq!(value, NEW_VALUE),
            None => panic!("value was not found in cache"),
        };
    }

    #[test]
    fn remove_expired_item() {
        let cache = Cache::new(Some(Duration::from_secs(0)));
        cache.set(KEY, VALUE, None);
        cache.remove_expired();
        if cache.items.get(&KEY).is_some() {
            panic!("found expired item in cache")
        };
    }

    #[test]
    fn remove_expired_do_not_remove_not_expired_item() {
        let cache = Cache::new(Some(Duration::from_secs(2)));
        cache.set(KEY, VALUE, None);
        cache.remove_expired();
        if cache.items.get(&KEY).is_none() {
            panic!("could not find not expired item in cache")
        };
    }

    #[test]
    fn clear_not_expired_item() {
        let cache = Cache::new(Some(Duration::from_secs(2)));
        cache.set(KEY, VALUE, None);
        cache.clear();
        if cache.items.get(&KEY).is_some() {
            panic!("found item in cache")
        };
    }

    #[test]
    fn remove_remove_expired_item() {
        let cache = Cache::new(Some(Duration::from_secs(2)));
        cache.set(KEY, VALUE, None);
        if let None = cache.remove(&KEY) {
            panic!("none returned from removing existing value")
        };
        if cache.items.get(&KEY).is_some() {
            panic!("found not expired item in cache")
        };
    }

    #[test]
    fn remove_return_none_if_not_found() {
        let cache: Cache<i8, &str> = Cache::new(Some(Duration::from_secs(2)));
        if let Some(_) = cache.remove(&KEY) {
            panic!("some value was returned from remove")
        };
    }
}
