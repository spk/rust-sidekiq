#![crate_name = "sidekiq"]

extern crate redis;
extern crate rustc_serialize;
extern crate time;
extern crate rand;
extern crate r2d2;
extern crate r2d2_redis;

mod sidekiq;
pub use sidekiq::{Job, Client, ClientOpts, RedisPool, create_redis_pool};
