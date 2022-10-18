#[macro_use]
extern crate serde_json;
extern crate sidekiq;

use std::default::Default;
use std::time::{SystemTime, UNIX_EPOCH};

use serde_json::value::Value;
use sidekiq::{create_redis_pool, Client, ClientOpts, Job};

use time::{Duration, OffsetDateTime};

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
    };
    let pool = create_redis_pool().unwrap();
    Client::new(pool, client_opts)
}

fn time_ok(time: u64) -> bool {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    now >= time
}

#[test]
fn test_job_format_with_default() {
    let class = "Maman".to_string();
    let job = Job::new(class.clone(), args(), Default::default());
    assert_eq!(job.class, class);
    assert_eq!(job.retry, 25);
    assert_eq!(job.jid.len(), 24);
    assert_eq!(job.queue, "default".to_string());
    assert!(time_ok(job.created_at));
    assert!(time_ok(job.enqueued_at));
}

#[test]
fn test_client_push_single() {
    let class = "Maman".to_string();
    let job = Job::new(class, args(), Default::default());
    let client = get_client();
    match client.push(job) {
        Ok(_) => {}
        Err(err) => {
            println!("Sidekiq push failed: {}", err);
            unreachable!()
        }
    }
}

#[test]
fn test_client_push_bulk() {
    let class = "Maman".to_string();
    let jobs = &vec![
        Job::new(class.clone(), args(), Default::default()),
        Job::new(class, args(), Default::default()),
    ];
    let client = get_client();
    match client.push_bulk(jobs) {
        Ok(_) => {}
        Err(err) => {
            println!("Sidekiq push failed: {}", err);
            unreachable!()
        }
    };
}

#[test]
fn test_client_perform_in() {
    let class = "Maman".to_string();
    let job = Job::new(class, args(), Default::default());
    let client = get_client();
    let interval = Duration::minutes(1);
    match client.perform_in(interval, job) {
        Ok(_) => {}
        Err(err) => {
            println!("Sidekiq push failed: {}", err);
            unreachable!()
        }
    }
}

#[test]
fn test_client_perform_at() {
    let class = "Maman".to_string();
    let job = Job::new(class, args(), Default::default());
    let client = get_client();
    let start_at = OffsetDateTime::now_utc()
        .checked_add(Duration::MINUTE)
        .unwrap();
    match client.perform_at(start_at, job) {
        Ok(_) => {}
        Err(err) => {
            println!("Sidekiq push failed: {}", err);
            unreachable!()
        }
    }
}
