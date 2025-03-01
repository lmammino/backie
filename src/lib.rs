//#![warn(missing_docs)]
#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]
use std::time::Duration;

/// All possible options for retaining tasks in the db after their execution.
///
/// The default mode is [`RetentionMode::RemoveAll`]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum RetentionMode {
    /// Keep all tasks
    KeepAll,

    /// Remove all finished tasks independently of their final execution state.
    RemoveAll,

    /// Remove only successfully finished tasks
    RemoveDone,
}

impl Default for RetentionMode {
    fn default() -> Self {
        Self::RemoveDone
    }
}

/// All possible options for backoff between task retries.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, serde::Serialize, serde::Deserialize)]
pub enum BackoffMode {
    /// No backoff, retry immediately
    NoBackoff,

    /// Exponential backoff
    ExponentialBackoff,
}

impl Default for BackoffMode {
    fn default() -> Self {
        Self::ExponentialBackoff
    }
}

impl BackoffMode {
    fn next_attampt(&self, attampt: i32) -> Duration {
        match self {
            Self::NoBackoff => Duration::from_secs(0),
            Self::ExponentialBackoff => {
                Duration::from_secs(2u64.saturating_pow(attampt.saturating_add(1) as u32))
            }
        }
    }
}

pub use runnable::BackgroundTask;
pub use store::{PgTask, PgTaskStore, TaskStore};
pub use task::{CurrentTask, NewTask, Task, TaskId, TaskState};
pub use worker::Worker;
pub use worker_pool::{QueueConfig, WorkerPool};

mod catch_unwind;
pub mod errors;
mod queries;
mod runnable;
mod schema;
mod store;
mod task;
mod worker;
mod worker_pool;
