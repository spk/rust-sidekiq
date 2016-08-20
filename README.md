# Rust Sidekiq Client

[Sidekiq](https://github.com/mperham/sidekiq/wiki/Job-Format) job format

## Dependencies

* [redis](https://github.com/mitsuhiko/redis-rs)
* [r2d2-redis](https://github.com/sorccu/r2d2-redis)
* [serde_json](https://github.com/serde-rs/json)

## Installation

``` ini
[dependencies]
sidekiq = "0.2"
```

## Usage


### Job

``` rust
extern crate sidekiq;

use std::default::Default;

use sidekiq::{Job, JobOpts};

use serde_json::value::Value;
use serde_json::builder::{ArrayBuilder, ObjectBuilder};

fn args() -> Vec<Value> {
    let arg_str: Value = Value::String("arg".to_string());
    let arg_int: Value = Value::I64(42);
    let arg_bool: Value = Value::Bool(true);
    let arg_object = ObjectBuilder::new()
        .insert("class".to_string(), "Ruby")
        .build();
    let arg_array = ArrayBuilder::new()
        .push(1.2)
        .build();
    let args: Vec<Value> = vec![arg_str, arg_int, arg_bool, arg_object, arg_array];
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
let client = Client::new(create_redis_pool(), client_opts);
match client.push(job) {
    Ok(_) => true,
    Err(_) => false,
}
```

## REFERENCES

* <http://julienblanchard.com/2015/using-resque-with-rust/>
* <https://github.com/d-unseductable/rust_sidekiq>

## LICENSE

The MIT License

Copyright (c) 2016 Laurent Arnoud <laurent@spkdev.net>

---
[![Build](https://img.shields.io/travis-ci/spk/rust-sidekiq.svg)](https://travis-ci.org/spk/rust-sidekiq)
[![Version](https://img.shields.io/crates/v/sidekiq.svg)](https://crates.io/crates/sidekiq)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](http://opensource.org/licenses/MIT "MIT")
[![Project status](http://img.shields.io/status/experimental.png?color=red)](https://github.com/spk/rust-sidekiq)
