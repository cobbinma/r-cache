//! ## r-cache
//!
//! r-cache is an in memory key value store. It is thread safe and values can have expiry times.
//!
//! # Example
//! ```
//! use r_cache::cache::Cache;
//! use std::time::Duration;
//!
//! const KEY: i8 = 0;
//! const VALUE: &str = "VALUE";
//!
//! # #[async_std::main]
//! # async fn main() {
//!    let cache = Cache::new(Some(Duration::from_secs(2 * 60 * 60)));
//!    cache.set(KEY, VALUE, None).await;
//!
//!    println!("{}", cache.get(&KEY).await.unwrap())
//! }
//! ```

mod item;

pub mod cache;
