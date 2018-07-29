#![feature(test)]
extern crate sidekiq;
extern crate test;

use sidekiq::{create_redis_pool, Client, ClientOpts, Job};
use test::Bencher;

fn get_client() -> Client {
    let ns = "test";
    let client_opts = ClientOpts {
        namespace: Some(ns.to_string()),
        ..Default::default()
    };
    Client::new(create_redis_pool(), client_opts)
}

#[bench]
fn bench_simple_push(b: &mut Bencher) {
    let client = get_client();
    b.iter(|| {
        let class = "Test".to_string();
        let args = "[\"arg1\",\"arg2\"]".to_string();
        let job = Job::new(class, args, Default::default());
        client.push(job)
    });
}
