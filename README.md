# Rust Sidekiq Client

[Sidekiq](https://github.com/mperham/sidekiq/wiki/Job-Format) job format

## Dependencies

* [redis](https://github.com/mitsuhiko/redis-rs)
* [r2d2-redis](https://github.com/sorccu/r2d2-redis)
* [serde_json](https://github.com/serde-rs/json)

## Installation

``` toml
[dependencies]
sidekiq = "0.4"
```

## Usage

### Job

``` rust
extern crate sidekiq;
#[macro_use]
extern crate serde_json;

use std::default::Default;

use sidekiq::{Job, JobOpts};
use serde_json::value::Value;

fn args() -> Vec<Value> {
    let value = json!({
        "code": 200,
        "success": true,
        "payload": {
            "features": [
                "serde",
                "json"
            ]
        }
    });
    let args: Vec<Value> = vec![value];
    args
}

let class = "MyClass".to_string();
let job_opts = JobOpts {
    queue: "test".to_string(),
    ..Default::default()
};
let job = Job::new(class, args(), Default::default());
```

### Client

``` rust
extern crate sidekiq;
use std::default::Default;

use sidekiq::{Client, ClientOpts, create_redis_pool};

let ns = "test";
let client_opts = ClientOpts {
    namespace: Some(ns.to_string()),
    ..Default::default()
};
let pool = create_redis_pool().unwrap();
let client = Client::new(pool, client_opts);
match client.push(job) {
    Ok(_) => {},
    Err(err) => {
        println!("Sidekiq push failed: {}", err);
    },
}
```

## Default environment variables

* REDIS_URL="redis://127.0.0.1/"

## REFERENCES

* <http://julienblanchard.com/2015/using-resque-with-rust/>
* <https://github.com/d-unseductable/rust_sidekiq>

## LICENSE

The MIT License

Copyright (c) 2016-2017 Laurent Arnoud <laurent@spkdev.net>

---
[![Build](https://img.shields.io/travis-ci/spk/rust-sidekiq.svg)](https://travis-ci.org/spk/rust-sidekiq)
[![Version](https://img.shields.io/crates/v/sidekiq.svg)](https://crates.io/crates/sidekiq)
[![Documentation](https://img.shields.io/badge/doc-rustdoc-blue.svg)](https://docs.rs/sidekiq/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://opensource.org/licenses/MIT "MIT")
[![Project status](https://img.shields.io/status/experimental.png?color=red)](https://github.com/spk/rust-sidekiq)
