extern crate time;
extern crate sidekiq;
extern crate rustc_serialize;

use std::default::Default;

use time::now_utc;
use rustc_serialize::json::ToJson;

use sidekiq::{Job, Client, ClientOpts, create_redis_pool};

fn serialized_args() -> String {
    let mut args = Vec::new();
    args.push("arg1".to_json());
    args.push("arg2".to_json());
    args.to_json().to_string()
}

fn time_ok(time: i64) -> bool {
    let now = now_utc().to_timespec().sec;
    if now >= time {
        true
    } else {
        false
    }
}

#[test]
fn test_job_format_with_default() {
    let class = "MyClass".to_string();
    let job = Job::new(class.clone(), serialized_args(), Default::default());
    assert_eq!(job.class, class);
    assert_eq!(job.retry, 25);
    assert_eq!(job.jid.len(), 24);
    assert_eq!(job.queue, "default".to_string());
    assert_eq!(job.args, serialized_args());
    assert!(time_ok(job.created_at));
    assert!(time_ok(job.enqueued_at));
}

#[test]
fn test_client_push() {
    let class = "MyClass".to_string();
    let job = Job::new(class.clone(), serialized_args(), Default::default());
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
}
