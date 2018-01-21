#[macro_use]
extern crate serde_json;
extern crate sidekiq;

use std::default::Default;
use std::time::{SystemTime, UNIX_EPOCH};

use serde_json::value::Value;
use sidekiq::{create_redis_pool, Client, ClientOpts, Job};

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

fn get_client() -> Client {
    let ns = "test";
    let client_opts = ClientOpts {
        namespace: Some(ns.to_string()),
        ..Default::default()
    };
    let pool = create_redis_pool().unwrap();
    Client::new(pool, client_opts)
}

fn time_ok(time: u64) -> bool {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as u64;
    if now >= time {
        true
    } else {
        false
    }
}

#[test]
fn test_job_format_with_default() {
    let class = "MyClass".to_string();
    let job = Job::new(class.clone(), args(), Default::default());
    assert_eq!(job.class, class);
    assert_eq!(job.retry, 25);
    assert_eq!(job.jid.len(), 24);
    assert_eq!(job.queue, "default".to_string());
    assert!(time_ok(job.created_at));
    assert!(time_ok(job.enqueued_at));
}

#[test]
fn test_client_push() {
    let class = "MyClass".to_string();
    let job = Job::new(class.clone(), args(), Default::default());
    let client = get_client();
    match client.push(job) {
        Ok(_) => assert!(true),
        Err(err) => {
            println!("Sidekiq push failed: {}", err);
            assert!(false)
        }
    }
}

#[test]
fn test_client_push_bulk() {
    let class = "MyClass".to_string();
    let jobs = &vec![
        Job::new(class.clone(), args(), Default::default()),
        Job::new(class.clone(), args(), Default::default()),
    ];
    let client = get_client();
    match client.push_bulk(jobs) {
        Ok(_) => assert!(true),
        Err(err) => {
            println!("Sidekiq push failed: {}", err);
            assert!(false)
        }
    };
}
