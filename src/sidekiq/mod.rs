use std::env;
use std::default::Default;
use std::collections::BTreeMap;

use time::now_utc;
use rand::{Rng, thread_rng};
use rustc_serialize::json::{ToJson, Json};
use r2d2_redis::RedisConnectionManager;
use r2d2::{Config, Pool, PooledConnection};
use redis::{cmd, Commands, RedisResult, parse_redis_url};

pub type RedisPooledConnection = PooledConnection<RedisConnectionManager>;
pub type RedisPool = Pool<RedisConnectionManager>;

pub fn create_redis_pool() -> RedisPool {
    let config = Config::builder().build();
    let redis_url = &env::var("REDIS_URL").unwrap_or("redis://127.0.0.1/".to_owned());
    let url = parse_redis_url(redis_url).unwrap();
    let manager = RedisConnectionManager::new(url).unwrap();
    Pool::new(config, manager).unwrap()
}

#[derive(RustcEncodable, RustcDecodable, Debug)]
pub struct Job {
    pub class: String,
    pub args: String,
    pub retry: i64,
    pub queue: String,
    pub jid: String,
    pub created_at: i64,
    pub enqueued_at: i64,
}

impl Default for JobOpts {
    fn default() -> JobOpts {
        let now = now_utc().to_timespec().sec;
        let jid = thread_rng().gen_ascii_chars().take(24).collect::<String>();
        JobOpts {
            retry: 25,
            queue: "default".to_string(),
            jid: jid,
            created_at: now,
            enqueued_at: now,
        }
    }
}

pub struct JobOpts {
    pub retry: i64,
    pub queue: String,
    pub jid: String,
    pub created_at: i64,
    pub enqueued_at: i64,
}

impl Job {
    pub fn new(class: String, args: String, opts: JobOpts) -> Job {
        Job {
            class: class,
            args: args,
            retry: opts.retry,
            queue: opts.queue,
            jid: opts.jid,
            created_at: opts.created_at,
            enqueued_at: opts.enqueued_at,
        }
    }
}

impl ToJson for Job {
    fn to_json(&self) -> Json {
        let mut object = BTreeMap::new();
        object.insert("class".to_string(), self.class.to_json());
        // `args` are already serialized, creating Json struct from it.
        object.insert("args".to_string(), Json::from_str(&self.args).unwrap());
        object.insert("retry".to_string(), self.retry.to_json());
        object.insert("queue".to_string(), self.queue.to_json());
        object.insert("jid".to_string(), self.jid.to_json());
        object.insert("created_at".to_string(), self.created_at.to_json());
        object.insert("enqueued_at".to_string(), self.enqueued_at.to_json());
        Json::Object(object)
    }
}

pub struct ClientOpts {
    pub namespace: Option<String>,
}

impl Default for ClientOpts {
    fn default() -> ClientOpts {
        ClientOpts { namespace: None }
    }
}

pub struct Client {
    pub pool: RedisPool,
    pub namespace: Option<String>,
}

impl Client {
    pub fn new(pool: RedisPool, opts: ClientOpts) -> Client {
        Client {
            pool: pool,
            namespace: opts.namespace,
        }
    }

    fn get_connection(&self) -> RedisPooledConnection {
        self.pool.get().unwrap()
    }

    pub fn push(&self, job: Job) -> RedisResult<Job> {
        let conn = self.get_connection();
        let _: () = cmd("SADD")
                        .arg("queues")
                        .arg(job.queue.to_string())
                        .query(&*conn)
                        .unwrap();
        let _: () = cmd("LPUSH")
                        .arg(self.queue_name(&job.queue))
                        .arg(job.to_json())
                        .query(&*conn)
                        .unwrap();
        Ok(job)
    }

    fn queue_name(&self, queue: &str) -> String {
        if let Some(ref ns) = self.namespace {
            format!("{}:queue:{}", ns, queue)
        } else {
            format!("queue:{}", queue)
        }
    }
}
