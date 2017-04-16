//! [Sidekiq](https://github.com/mperham/sidekiq) client allowing to push jobs.
//! Using the [Sidekiq job
//! format](https://github.com/mperham/sidekiq/wiki/Job-Format) as reference.
//!
//! # Default environment variables
//!
//! `REDIS_URL`="redis://127.0.0.1/"
//!
#![deny(warnings)]
#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

#![crate_name = "sidekiq"]

extern crate serde;
extern crate serde_json;
extern crate rand;
extern crate r2d2;
extern crate r2d2_redis;

mod sidekiq;
pub use serde_json::value::Value;
pub use sidekiq::{Job, JobOpts, Client, ClientOpts, RedisPool, RedisPooledConnection,
                  create_redis_pool};
