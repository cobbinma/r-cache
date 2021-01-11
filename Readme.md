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
#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    let cache = Cache::new(Some(Duration::from_secs(2 * 60 * 60)));
    cache.set(KEY, VALUE, None).await;
    let value = cache.get(KEY).await;
    if let Some(value) = value {
        println!("{}", value)
    }
    Ok(())
}
```