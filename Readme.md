# r-cache

r-cache is an in memory key value store. It is thread safe and values have expiry times.

### Example

```rust
let cache = Cache::new(chrono::Duration::hours(2));
cache.set(KEY, VALUE).await;
let value = cache.get(KEY).await;
```