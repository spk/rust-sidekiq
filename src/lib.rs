/*!

[Sidekiq](https://github.com/mperham/sidekiq) client allowing to push jobs.
Using the [Sidekiq job
format](https://github.com/mperham/sidekiq/wiki/Job-Format) as reference.

# Create a job

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

# Create a client and push a job

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

# Default environment variables

* REDIS_URL="redis://127.0.0.1/"
*/

#![crate_name = "sidekiq"]

extern crate serde;
extern crate serde_json;
extern crate rand;
extern crate r2d2;
extern crate r2d2_redis;

mod sidekiq;
pub use sidekiq::{Job, JobOpts, Client, ClientOpts, RedisPool, RedisPooledConnection,
                  create_redis_pool};
