# Rust Sidekiq Client

[Sidekiq](https://github.com/mperham/sidekiq) client allowing to push jobs.
Using the [Sidekiq job
format](https://github.com/mperham/sidekiq/wiki/Job-Format) as reference.

## Dependencies

* [rand](https://github.com/rust-random/rand)
* [redis](https://github.com/mitsuhiko/redis-rs)
* [r2d2-redis](https://github.com/sorccu/r2d2-redis)
* [serde_json](https://github.com/serde-rs/json)

## Installation

``` toml
[dependencies]
sidekiq = "0.9"
```

## Default environment variables

* REDIS_URL="redis://127.0.0.1/"

## Used by

* <https://github.com/jkcclemens/paste>
* <https://github.com/spk/maman>


## Examples

```
use sidekiq::{Job, Value};
use sidekiq::{Client, ClientOpts, create_redis_pool};
use chrono::Duration;

let ns = "test";
let client_opts = ClientOpts {
    namespace: Some(ns.to_string()),
    ..Default::default()
};
let pool = create_redis_pool().unwrap();
let client = Client::new(pool, client_opts);
let class = "MyClass".to_string();

// basic job
let job = Job::new(class, vec![sidekiq::Value::Null], Default::default());
match client.push(job) {
    Ok(_) => {},
    Err(err) => {
        println!("Sidekiq push failed: {}", err);
    },
}

// scheduled-jobs (perform_in)
let job = Job::new(class, vec![sidekiq::Value::Null], Default::default());
let interval = Duration::hours(1);
match client.perform_in(interval, job) {
    Ok(_) => {},
    Err(err) => {
        println!("Sidekiq push failed: {}", err);
    },
}
```

## REFERENCES

* <http://julienblanchard.com/2015/using-resque-with-rust/>
* <https://github.com/d-unseductable/rust_sidekiq>

## LICENSE

The MIT License

Copyright (c) 2016-2021 Laurent Arnoud <laurent@spkdev.net>

---
[![Build](https://img.shields.io/github/workflow/status/spk/rust-sidekiq/CI/master.svg)](https://github.com/spk/rust-sidekiq/actions)
[![Version](https://img.shields.io/crates/v/sidekiq.svg)](https://crates.io/crates/sidekiq)
[![Documentation](https://img.shields.io/badge/doc-rustdoc-blue.svg)](https://docs.rs/sidekiq/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://opensource.org/licenses/MIT "MIT")
[![Dependency status](https://deps.rs/repo/github/spk/rust-sidekiq/status.svg)](https://deps.rs/repo/github/spk/rust-sidekiq)
