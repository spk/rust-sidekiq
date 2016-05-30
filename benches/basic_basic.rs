#![feature(test)]
extern crate test;
extern crate sidekiq;

use test::Bencher;
use sidekiq::{Job, Client, ClientOpts, create_redis_pool};

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
