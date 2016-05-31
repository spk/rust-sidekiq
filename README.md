# Rust Sidekiq Client

[Sidekiq](https://github.com/mperham/sidekiq/wiki/Job-Format) job format

## Dependencies

* Redis

## Installation

~~~
[dependencies]
sidekiq = "0.1"
~~~

## Usage


### Job

~~~ rust
extern crate sidekiq;
extern crate rustc_serialize;

use std::default::Default;

use rustc_serialize::json::ToJson;

use sidekiq::{Job, JobOpts};

fn serialized_args() -> String {
    let mut args = Vec::new();
    args.push("arg1".to_json());
    args.push("arg2".to_json());
    args.to_json().to_string()
}

let class = "MyClass".to_string();
let job_opts = JobOpts {
    queue: "test".to_string(),
    ..Default::default()
};
let job = Job::new(class, serialized_args(), Default::default());
~~~

### Client

~~~ rust
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
    Ok(_) => assert!(true),
    Err(_) => assert!(false),
}
~~~


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
