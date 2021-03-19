#![feature(test)]
extern crate test;
#[macro_use]
extern crate serde_json;
extern crate sidekiq;

use serde_json::value::Value;
use sidekiq::{create_redis_pool, Client, ClientOpts, Job};
use test::Bencher;

fn get_client() -> Client {
    let ns = "test";
    let client_opts = ClientOpts {
        namespace: Some(ns.to_string()),
    };
    let pool = create_redis_pool().unwrap();
    Client::new(pool, client_opts)
}

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

#[bench]
fn bench_simple_push(b: &mut Bencher) {
    let client = get_client();
    b.iter(|| {
        let class = "Test".to_string();
        let job = Job::new(class, args(), Default::default());
        client.push(job)
    });
}
