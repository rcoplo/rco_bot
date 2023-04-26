use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use tokio_cron_scheduler::{Job, JobScheduler};

/// Ctrl C/V [scheduler.rs](https://github.com/onebot-walle/walle/blob/master/src/scheduler.rs) <br/>

/// 定时任务 trait
pub trait ScheduledJob {
    fn cron(&self) -> &'static str;
    fn call(&self, client: Arc<proc_qq::Client>) -> Pin<Box<dyn Future<Output=()> + Send + 'static>>;
}

/// 定时任务执行器
pub struct Scheduler {
    inner: JobScheduler,
    client: Arc<proc_qq::Client>,
}

impl Scheduler {
    pub async fn new(client: Arc<proc_qq::Client>) -> Self {
        Self {
            inner: JobScheduler::new().await.expect("failed to create job scheduler"),
            client,
        }
    }

    /// 向定时任务执行器中添加一个定时任务
    pub async fn add(&self, job: impl ScheduledJob + Send + 'static + Sync) {
        let client = self.client.clone();
        let job = Job::new_async(job.cron(), move |_, _| job.call(client.clone())).expect("新建任务失败");
        self.inner.add(job).await.expect("添加任务失败");
    }

    /// 启动定时任务执行器
    pub async fn start(&self) {
        self.inner.start().await.expect("启动任务失败");
    }
}

pub trait ArcScheduledJob {
    fn cron(&self) -> &'static str;
    fn call(self: &Arc<Self>, client: Arc<proc_qq::Client>) -> Pin<Box<dyn Future<Output=()> + Send + 'static>>;
}

impl<T: ArcScheduledJob> ScheduledJob for Arc<T> {
    fn cron(&self) -> &'static str {
        <T as ArcScheduledJob>::cron(&self)
    }
    fn call(&self, client: Arc<proc_qq::Client>) -> Pin<Box<dyn Future<Output=()> + Send + 'static>> {
        <T as ArcScheduledJob>::call(&self, client)
    }
}
