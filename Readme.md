<h1 align="center">r-cache</h1>
<div align="center">
 <strong>
   A simple caching library
 </strong>
</div>

<br />

<div align="center">
  <!-- Crates version -->
  <a href="https://crates.io/crates/r-cache">
    <img src="https://img.shields.io/crates/v/r-cache.svg?style=flat-square"
    alt="Crates.io version" />
  </a>
  <!-- Downloads -->
  <a href="https://crates.io/crates/r-cache">
    <img src="https://img.shields.io/crates/d/r-cache.svg?style=flat-square"
      alt="Download" />
  </a>
  <!-- docs.rs docs -->
  <a href="https://docs.rs/r-cache">
    <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square"
      alt="docs.rs docs" />
  </a>
</div>

<br>

r-cache is an in memory key value store. It is thread safe and values can have expiry times.

### Example

```rust
use async_std::sync::Arc;
use async_std::task;
use r_cache::cache::Cache;
use std::time::Duration;

const KEY: i8 = 0;
const VALUE: &str = "VALUE";

#[async_std::main]
async fn main() {
    let cache = Arc::new(Cache::new(Some(Duration::from_secs(5 * 60))));
    task::spawn({
        let cache = Arc::clone(&cache);
        async move {
            loop {
                task::sleep(Duration::from_secs(10 * 60)).await;
                cache.remove_expired();
            }
        }
    });

    cache.set(KEY, VALUE, None);

    assert_eq!(VALUE, cache.get(&KEY).unwrap())
}
```
