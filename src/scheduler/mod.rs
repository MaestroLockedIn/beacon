use std::sync::Arc;

use tokio_cron_scheduler::{Job, JobScheduler};

use crate::config::Config;

pub async fn run_tasks(config: Arc<Config>) {
    let sched = JobScheduler::new()
        .await
        .expect("Failed to create scheduler");
    let job = Job::new_async("*/5 * * * * *", move |_, _| {
        let config = config.clone();
        Box::pin(async move {
            scheduled_job(config);
        })
    })
    .expect("Failed to create job");

    sched.add(job).await.expect("Failed to add job");

    sched.start().await.expect("Failed to start scheduling");

    println!("Scheduler running in background.");
}

fn scheduled_job(config: Arc<Config>) {
    //todo: ping the server to determine the health status
    for c in &config.endpoints {
        // println!("Running scheduled job for {:?}", c);
        println!(
            "Running scheduled job for {0} \n pinging {1}\n\n",
            c.name, c.url
        );
    }
}
