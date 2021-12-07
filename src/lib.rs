//! [Sidekiq](https://github.com/mperham/sidekiq) client allowing to push jobs.
//! Using the [Sidekiq job
//! format](https://github.com/mperham/sidekiq/wiki/Job-Format) as reference.
//!
//! # Default environment variables
//!
//! `REDIS_URL`="redis://127.0.0.1/"
//!
#![doc(html_root_url = "https://docs.rs/sidekiq/0.10.1")]
#![deny(warnings)]
#![crate_name = "sidekiq"]

extern crate r2d2;
extern crate r2d2_redis;
extern crate rand;
extern crate serde;
extern crate serde_json;

mod sidekiq;
pub use crate::sidekiq::{
    create_redis_pool, Client, ClientError, ClientOpts, Job, JobOpts, RedisPool,
    RedisPooledConnection,
};
pub use serde_json::value::Value;
