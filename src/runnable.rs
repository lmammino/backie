use crate::errors::FrangoError;
use crate::queue::AsyncQueueable;
use crate::Scheduled;
use async_trait::async_trait;

const COMMON_TYPE: &str = "common";
pub const RETRIES_NUMBER: i32 = 20;
/// Implement this trait to run your custom tasks.
#[typetag::serde(tag = "type")]
#[async_trait]
pub trait AsyncRunnable: Send + Sync {
    /// Execute the task. This method should define its logic
    async fn run(&self, client: &mut dyn AsyncQueueable) -> Result<(), FrangoError>;

    /// Define the type of the task.
    /// The `common` task type is used by default
    fn task_type(&self) -> String {
        COMMON_TYPE.to_string()
    }

    /// If set to true, no new tasks with the same metadata will be inserted
    /// By default it is set to false.
    fn uniq(&self) -> bool {
        false
    }

    /// This method defines if a task is periodic or it should be executed once in the future.
    ///
    /// Be careful it works only with the UTC timezone.
    ///
    ///
    /// Example:
    ///
    ///
    /**
    ```rust
     fn cron(&self) -> Option<Scheduled> {
         let expression = "0/20 * * * Aug-Sep * 2022/1";
         Some(Scheduled::CronPattern(expression.to_string()))
     }
    ```
    */

    /// In order to schedule  a task once, use the `Scheduled::ScheduleOnce` enum variant.
    fn cron(&self) -> Option<Scheduled> {
        None
    }

    /// Define the maximum number of retries the task will be retried.
    /// By default the number of retries is 20.
    fn max_retries(&self) -> i32 {
        RETRIES_NUMBER
    }

    /// Define the backoff mode
    /// By default, it is exponential,  2^(attempt)
    fn backoff(&self, attempt: u32) -> u32 {
        u32::pow(2, attempt)
    }
}
