extern crate sidekiq;
extern crate serde_json;

use std::default::Default;
use std::time::{SystemTime, UNIX_EPOCH};

use sidekiq::{Job, Client, ClientOpts, create_redis_pool};
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
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as u64;
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
        },
    }
}

#[test]
fn test_client_push_bulk() {
    let class = "MyClass".to_string();
    let jobs = vec![
        Job::new(class.clone(), args(), Default::default()),
        Job::new(class.clone(), args(), Default::default())
    ];
    let client = get_client();
    match client.push_bulk(jobs) {
        Ok(_) => assert!(true),
        Err(err) => {
            println!("Sidekiq push failed: {}", err);
            assert!(false)
        },
    };
}
