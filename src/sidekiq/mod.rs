extern crate redis;

use std::env;
use std::fmt;
use std::error::Error;
use std::default::Default;
use std::time::{SystemTime, UNIX_EPOCH};

use rand::{Rng, thread_rng};
use serde::{Serialize, Serializer};
use serde_json;
use serde_json::Value;
use r2d2_redis::RedisConnectionManager;
use r2d2::{Config, Pool, PooledConnection, GetTimeout, InitializationError};

pub type RedisPooledConnection = PooledConnection<RedisConnectionManager>;
pub type RedisPool = Pool<RedisConnectionManager>;

#[derive(Debug)]
pub struct ClientError {
    kind: ErrorKind,
}

#[derive(Debug)]
enum ErrorKind {
    Redis(redis::RedisError),
    PoolTimeout(GetTimeout),
    PoolInit(InitializationError),
}

pub fn create_redis_pool() -> Result<RedisPool, ClientError> {
    let config = Config::builder().build();
    let redis_url = &env::var("REDIS_URL").unwrap_or("redis://127.0.0.1/".to_owned());
    let url = redis::parse_redis_url(redis_url).unwrap();
    let manager = RedisConnectionManager::new(url).unwrap();
    Pool::new(config, manager).map_err(|err| ClientError { kind: ErrorKind::PoolInit(err) })
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

impl fmt::Display for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            ErrorKind::Redis(ref err) => err.fmt(f),
            ErrorKind::PoolTimeout(ref err) => err.fmt(f),
            ErrorKind::PoolInit(ref err) => err.fmt(f),
        }
    }
}

impl Error for ClientError {
    fn description(&self) -> &str {
        match self.kind {
            ErrorKind::Redis(ref err) => err.description(),
            ErrorKind::PoolTimeout(ref err) => err.description(),
            ErrorKind::PoolInit(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match self.kind {
            ErrorKind::Redis(ref err) => Some(err),
            ErrorKind::PoolTimeout(ref err) => Some(err),
            ErrorKind::PoolInit(ref err) => Some(err),
        }
    }
}

impl From<redis::RedisError> for ClientError {
    fn from(error: redis::RedisError) -> ClientError {
        ClientError { kind: ErrorKind::Redis(error) }
    }
}

impl From<GetTimeout> for ClientError {
    fn from(error: GetTimeout) -> ClientError {
        ClientError { kind: ErrorKind::PoolTimeout(error) }
    }
}

impl From<InitializationError> for ClientError {
    fn from(error: InitializationError) -> ClientError {
        ClientError { kind: ErrorKind::PoolInit(error) }
    }
}

impl Default for JobOpts {
    fn default() -> JobOpts {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as u64;
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
    pub redis_pool: RedisPool,
    pub namespace: Option<String>,
}

impl Client {
    pub fn new(redis_pool: RedisPool, opts: ClientOpts) -> Client {
        Client {
            redis_pool: redis_pool,
            namespace: opts.namespace,
        }
    }

    fn connect(&self) -> Result<RedisPooledConnection, ClientError> {
        match self.redis_pool.get() {
            Ok(conn) => Ok(conn),
            Err(err) => Err(ClientError { kind: ErrorKind::PoolTimeout(err) }),
        }
    }

    pub fn push(&self, job: Job) -> Result<(), ClientError> {
        self.raw_push(vec![job])
    }

    pub fn push_bulk(&self, jobs: Vec<Job>) -> Result<(), ClientError> {
        self.raw_push(jobs)
    }

    fn raw_push(&self, payloads: Vec<Job>) -> Result<(), ClientError> {
        let ref p = payloads[0];
        let to_push =
            payloads.iter().map(|entry| serde_json::to_string(&entry).unwrap()).collect::<Vec<_>>();
        match self.connect() {
            Ok(conn) => {
                redis::pipe()
                    .atomic()
                    .cmd("SADD")
                    .arg("queues")
                    .arg(p.queue.to_string())
                    .ignore()
                    .cmd("LPUSH")
                    .arg(self.queue_name(&p.queue))
                    .arg(to_push)
                    .query(&*conn)
                    .map_err(|err| ClientError { kind: ErrorKind::Redis(err) })
            }
            Err(err) => Err(err),
        }
    }

    fn queue_name(&self, queue: &str) -> String {
        if let Some(ref ns) = self.namespace {
            format!("{}:queue:{}", ns, queue)
        } else {
            format!("queue:{}", queue)
        }
    }
}
