use std::default::Default;
use std::env;
use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::Value;
use r2d2_redis::{r2d2, redis, RedisConnectionManager};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};

use time::{OffsetDateTime, Duration};

const REDIS_URL_ENV: &str = "REDIS_URL";
const REDIS_URL_DEFAULT: &str = "redis://127.0.0.1/";
pub type RedisPooledConnection = r2d2::PooledConnection<RedisConnectionManager>;
pub type RedisPool = r2d2::Pool<RedisConnectionManager>;

#[derive(Debug)]
pub struct ClientError {
    kind: ErrorKind,
}

#[derive(Debug)]
enum ErrorKind {
    Redis(redis::RedisError),
    PoolInit(r2d2::Error),
}

impl std::error::Error for ClientError {}

pub fn create_redis_pool() -> Result<RedisPool, ClientError> {
    let redis_url =
        &env::var(&REDIS_URL_ENV.to_owned()).unwrap_or_else(|_| REDIS_URL_DEFAULT.to_owned());
    let url = redis::parse_redis_url(redis_url).unwrap();
    let manager = RedisConnectionManager::new(url).unwrap();
    r2d2::Pool::new(manager).map_err(|err| ClientError {
        kind: ErrorKind::PoolInit(err),
    })
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
            ErrorKind::PoolInit(ref err) => err.fmt(f),
        }
    }
}

impl From<redis::RedisError> for ClientError {
    fn from(error: redis::RedisError) -> ClientError {
        ClientError {
            kind: ErrorKind::Redis(error),
        }
    }
}

impl From<r2d2::Error> for ClientError {
    fn from(error: r2d2::Error) -> ClientError {
        ClientError {
            kind: ErrorKind::PoolInit(error),
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

impl Default for JobOpts {
    fn default() -> JobOpts {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as u64;
        let mut rng = thread_rng();
        let jid: String = (&mut rng)
            .sample_iter(Alphanumeric)
            .take(24)
            .map(char::from)
            .collect();
        JobOpts {
            retry: 25,
            queue: "default".to_string(),
            jid,
            created_at: now,
            enqueued_at: now,
        }
    }
}

/// # Examples
///
/// ```
/// use std::default::Default;
/// use sidekiq::Value;
/// use sidekiq::{Job, JobOpts};
///
/// // Create a job
/// let class = "MyClass".to_string();
/// let job_opts = JobOpts {
///     queue: "test".to_string(),
///     ..Default::default()
/// };
/// let job = Job::new(class, vec![sidekiq::Value::Null], job_opts);
/// ```
impl Job {
    pub fn new(class: String, args: Vec<Value>, opts: JobOpts) -> Job {
        Job {
            class,
            args,
            retry: opts.retry,
            queue: opts.queue,
            jid: opts.jid,
            created_at: opts.created_at,
            enqueued_at: opts.enqueued_at,
        }
    }
}

impl Serialize for Job {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Job", 7)?;
        s.serialize_field("class", &self.class)?;
        s.serialize_field("args", &self.args)?;
        s.serialize_field("retry", &self.retry)?;
        s.serialize_field("queue", &self.queue)?;
        s.serialize_field("jid", &self.jid)?;
        s.serialize_field("created_at", &self.created_at)?;
        s.serialize_field("enqueued_at", &self.enqueued_at)?;
        s.end()
    }
}

#[derive(Default)]
pub struct ClientOpts {
    pub namespace: Option<String>,
}

pub struct Client {
    pub redis_pool: RedisPool,
    pub namespace: Option<String>,
}

/// # Examples
///
/// ```
///
/// use sidekiq::{Job, Value};
/// use sidekiq::{Client, ClientOpts, create_redis_pool};
///
/// let ns = "test";
/// let client_opts = ClientOpts {
///     namespace: Some(ns.to_string()),
///     ..Default::default()
/// };
/// let pool = create_redis_pool().unwrap();
/// let client = Client::new(pool, client_opts);
/// let class = "MyClass".to_string();
/// let job = Job::new(class, vec![sidekiq::Value::Null], Default::default());
/// match client.push(job) {
///     Ok(_) => {},
///     Err(err) => {
///         println!("Sidekiq push failed: {}", err);
///     },
/// }
/// ```
impl Client {
    pub fn new(redis_pool: RedisPool, opts: ClientOpts) -> Client {
        Client {
            redis_pool,
            namespace: opts.namespace,
        }
    }

    fn connect(&self) -> Result<RedisPooledConnection, ClientError> {
        match self.redis_pool.get() {
            Ok(conn) => Ok(conn),
            Err(err) => Err(ClientError {
                kind: ErrorKind::PoolInit(err),
            }),
        }
    }

    fn calc_at(&self, target_millsec_number: f64) -> Option<f64> {
        let div: f64 = 1_000_f64;
        let maximum_target: f64 = 1_000_000_000_f64;
        let target_millsec: f64 = target_millsec_number / div;
        let now_millisec = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as f64 / div;

        let start_at: f64 = if target_millsec < maximum_target {
            now_millisec + target_millsec
        } else {
            target_millsec
        };

        if start_at <= now_millisec {
            None
        } else {
            Some(start_at)
        }
    }

    fn convert_duration_to_millsec(&self, interval: Duration) -> Option<f64> {
        let interval_millsec: f64 = interval.subsec_milliseconds() as f64;
        self.calc_at(interval_millsec)
    }

    fn convert_datetime_to_millsec(&self, datetime: OffsetDateTime) -> Option<f64> {
        let timestamp_millsec: f64 = datetime.millisecond() as f64;
        self.calc_at(timestamp_millsec)
    }

    pub fn perform_in(&self, interval: Duration, job: Job) -> Result<(), ClientError> {
        self.raw_push(&[job], self.convert_duration_to_millsec(interval))
    }

    pub fn perform_at(&self, local_datetime: OffsetDateTime, job: Job) -> Result<(), ClientError> {
        self.raw_push(&[job], self.convert_datetime_to_millsec(local_datetime))
    }

    pub fn push(&self, job: Job) -> Result<(), ClientError> {
        self.raw_push(&[job], None)
    }

    pub fn push_bulk(&self, jobs: &[Job]) -> Result<(), ClientError> {
        self.raw_push(jobs, None)
    }

    fn raw_push(&self, payloads: &[Job], at: Option<f64>) -> Result<(), ClientError> {
        let payload = &payloads[0];
        let to_push = payloads
            .iter()
            .map(|entry| serde_json::to_string(&entry).unwrap())
            .collect::<Vec<_>>();

        if let Some(value) = at {
            match self.connect() {
                Ok(mut conn) => redis::pipe()
                    .atomic()
                    .cmd("ZADD")
                    .arg(self.schedule_queue_name())
                    .arg(value)
                    .arg(to_push)
                    .query(&mut *conn)
                    .map_err(|err| ClientError {
                        kind: ErrorKind::Redis(err),
                    }),
                Err(err) => Err(err),
            }
        } else {
            match self.connect() {
                Ok(mut conn) => redis::pipe()
                    .atomic()
                    .cmd("SADD")
                    .arg("queues")
                    .arg(payload.queue.to_string())
                    .ignore()
                    .cmd("LPUSH")
                    .arg(self.queue_name(&payload.queue))
                    .arg(to_push)
                    .query(&mut *conn)
                    .map_err(|err| ClientError {
                        kind: ErrorKind::Redis(err),
                    }),
                Err(err) => Err(err),
            }
        }
    }

    fn schedule_queue_name(&self) -> String {
        if let Some(ref ns) = self.namespace {
            format!("{}:schedule", ns)
        } else {
            "schedule".to_string()
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
