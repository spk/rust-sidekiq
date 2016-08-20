use std::env;
use std::default::Default;
use std::time::{SystemTime, UNIX_EPOCH};

use rand::{Rng, thread_rng};
use serde::{Serialize, Serializer};
use serde_json;
use serde_json::Value;
use r2d2_redis::RedisConnectionManager;
use r2d2::{Config, Pool, PooledConnection};
use redis::{pipe, RedisResult, parse_redis_url};

pub type RedisPooledConnection = PooledConnection<RedisConnectionManager>;
pub type RedisPool = Pool<RedisConnectionManager>;

pub fn create_redis_pool() -> RedisPool {
    let config = Config::builder().build();
    let redis_url = &env::var("REDIS_URL").unwrap_or("redis://127.0.0.1/".to_owned());
    let url = parse_redis_url(redis_url).unwrap();
    let manager = RedisConnectionManager::new(url).unwrap();
    Pool::new(config, manager).unwrap()
}

pub struct Job {
    pub class: String,
    pub args: Vec<Value>,
    pub retry: i64,
    pub queue: String,
    pub jid: String,
    pub created_at: u64,
    pub enqueued_at: u64,
}

impl Default for JobOpts {
    fn default() -> JobOpts {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as u64;
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
    pub created_at: u64,
    pub enqueued_at: u64,
}

impl Job {
    pub fn new(class: String, args: Vec<Value>, opts: JobOpts) -> Job {
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

impl Serialize for Job {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer
    {
        let mut state = try!(serializer.serialize_struct("Job", 7));
        try!(serializer.serialize_struct_elt(&mut state, "class", &self.class));
        try!(serializer.serialize_struct_elt(&mut state, "args", &self.args));
        try!(serializer.serialize_struct_elt(&mut state, "retry", &self.retry));
        try!(serializer.serialize_struct_elt(&mut state, "queue", &self.queue));
        try!(serializer.serialize_struct_elt(&mut state, "jid", &self.jid));
        try!(serializer.serialize_struct_elt(&mut state, "created_at", &self.created_at));
        try!(serializer.serialize_struct_elt(&mut state, "enqueued_at", &self.enqueued_at));
        serializer.serialize_struct_end(state)
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
    pub connection: RedisPooledConnection,
    pub namespace: Option<String>,
}

impl Client {
    pub fn new(pool: RedisPool, opts: ClientOpts) -> Client {
        Client {
            connection: pool.get().unwrap(),
            namespace: opts.namespace,
        }
    }

    pub fn push(&self, job: Job) -> RedisResult<Job> {
        let _: () = try!(pipe()
            .atomic()
            .cmd("SADD")
            .arg("queues")
            .arg(job.queue.to_string())
            .ignore()
            .cmd("LPUSH")
            .arg(self.queue_name(&job.queue))
            .arg(serde_json::to_string(&job).unwrap())
            .query(&*self.connection));
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
