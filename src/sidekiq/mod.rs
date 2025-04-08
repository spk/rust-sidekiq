use std::default::Default;
use std::env;
use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::Value;
use rand::distr::Alphanumeric;
use rand::{rng, Rng};
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};

use time::{Duration, OffsetDateTime};

use futures::executor::block_on;
use futures::future::TryFutureExt;
use redis::aio::ConnectionManager;

const REDIS_URL_ENV: &str = "REDIS_URL";
const REDIS_URL_DEFAULT: &str = "redis://127.0.0.1/";
pub type RedisPool = ConnectionManager;

#[derive(Debug)]
pub struct ClientError {
    kind: ErrorKind,
}

#[derive(Debug)]
enum ErrorKind {
    Redis(redis::RedisError),
}

impl std::error::Error for ClientError {}

pub fn create_redis_pool() -> Result<ConnectionManager, ClientError> {
    block_on(create_async_redis_pool())
}

pub async fn create_async_redis_pool() -> Result<ConnectionManager, ClientError> {
    let redis_url = &env::var(REDIS_URL_ENV).unwrap_or_else(|_| REDIS_URL_DEFAULT.to_owned());
    // Note: this connection is multiplexed. Users of this object will call clone(), but the same underlying connection will be used.
    // https://docs.rs/redis/latest/redis/aio/struct.ConnectionManager.html
    match ConnectionManager::new(redis::Client::open((*redis_url).clone()).unwrap()).await {
        Ok(pool) => Ok(pool),
        Err(err) => Err(ClientError {
            kind: ErrorKind::Redis(err),
        }),
    }
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
            .as_secs();
        let mut rng = rng();
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
/// let class = "Maman".to_string();
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
    pub redis_pool: ConnectionManager,
    pub namespace: Option<String>,
}

/// # Examples
///
/// ```
///
/// use sidekiq::{Job, Value};
/// use sidekiq::{Client, ClientOpts, create_redis_pool};
/// use time::{OffsetDateTime, Duration};
///
/// let ns = "test";
/// let client_opts = ClientOpts {
///     namespace: Some(ns.to_string()),
///     ..Default::default()
/// };
/// let pool = create_redis_pool().unwrap();
/// let client = Client::new(pool, client_opts);
/// let class = "Maman";
/// let job = Job::new(class.to_string(), vec![sidekiq::Value::Null], Default::default());
/// match client.push(job) {
///     Ok(_) => {},
///     Err(err) => {
///         println!("Sidekiq push failed: {}", err);
///     },
/// }
/// let job = Job::new(class.to_string(), vec![sidekiq::Value::Null], Default::default());
/// let interval = Duration::hours(1);
/// match client.perform_in(interval, job) {
///     Ok(_) => {},
///     Err(err) => {
///         println!("Sidekiq push failed: {}", err);
///     },
/// }
/// let job = Job::new(class.to_string(), vec![sidekiq::Value::Null], Default::default());
/// let start_at = OffsetDateTime::now_utc().checked_add(Duration::HOUR).unwrap();
/// match client.perform_at(start_at, job) {
///     Ok(_) => {},
///     Err(err) => {
///         println!("Sidekiq push failed: {}", err);
///     },
/// }
/// ```
impl Client {
    pub fn new(redis_pool: ConnectionManager, opts: ClientOpts) -> Client {
        Client {
            redis_pool,
            namespace: opts.namespace,
        }
    }

    fn calc_at(&self, target_millsec_number: f64) -> Option<f64> {
        let maximum_target: f64 = 1_000_000_000_f64;
        let target_millsec: f64 = target_millsec_number;
        let now_millisec = OffsetDateTime::now_utc().unix_timestamp() as f64;

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

    pub fn perform_in(&self, interval: Duration, job: Job) -> Result<(), ClientError> {
        block_on(self.perform_in_async(interval, job))
    }

    pub fn perform_at(&self, datetime: OffsetDateTime, job: Job) -> Result<(), ClientError> {
        block_on(self.perform_at_async(datetime, job))
    }

    pub fn push(&self, job: Job) -> Result<(), ClientError> {
        block_on(self.push_async(job))
    }

    pub fn push_bulk(&self, jobs: &[Job]) -> Result<(), ClientError> {
        block_on(self.push_bulk_async(jobs))
    }

    pub async fn perform_in_async(&self, interval: Duration, job: Job) -> Result<(), ClientError> {
        let interval: f64 = interval.whole_seconds() as f64;
        self.raw_push(&[job], self.calc_at(interval)).await
    }

    pub async fn perform_at_async(
        &self,
        datetime: OffsetDateTime,
        job: Job,
    ) -> Result<(), ClientError> {
        let timestamp: f64 = datetime.unix_timestamp() as f64;
        self.raw_push(&[job], self.calc_at(timestamp)).await
    }

    pub async fn push_async(&self, job: Job) -> Result<(), ClientError> {
        self.raw_push(&[job], None).await
    }

    pub async fn push_bulk_async(&self, jobs: &[Job]) -> Result<(), ClientError> {
        self.raw_push(jobs, None).await
    }

    async fn raw_push(&self, payloads: &[Job], at: Option<f64>) -> Result<(), ClientError> {
        let payload = &payloads[0];
        let to_push = payloads
            .iter()
            .map(|entry| serde_json::to_string(&entry).unwrap())
            .collect::<Vec<_>>();

        if let Some(value) = at {
            redis::pipe()
                .atomic()
                .cmd("ZADD")
                .arg(self.schedule_queue_name())
                .arg(value)
                .arg(to_push)
                .query_async(&mut self.redis_pool.clone())
                .map_err(|err| ClientError {
                    kind: ErrorKind::Redis(err),
                })
                .await
        } else {
            redis::pipe()
                .atomic()
                .cmd("SADD")
                .arg("queues")
                .arg(payload.queue.to_string())
                .ignore()
                .cmd("LPUSH")
                .arg(self.queue_name(&payload.queue))
                .arg(to_push)
                .query_async(&mut self.redis_pool.clone())
                .map_err(|err| ClientError {
                    kind: ErrorKind::Redis(err),
                })
                .await
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
