use std::sync::Arc;

use tokio_cron_scheduler::{Job, JobScheduler};

use crate::{config::Config, scheduler::http_ping::HttpPing};

pub mod http_ping;

pub async fn run_tasks(config: Arc<Config>) {
    let sched = JobScheduler::new()
        .await
        .expect("Failed to create scheduler");
    let job = Job::new_async("*/5 * * * * *", move |_, _| {
        let config = config.clone();
        Box::pin(async move {
            scheduled_job(config).await;
        })
    })
    .expect("Failed to create job");

    sched.add(job).await.expect("Failed to add job");

    sched.start().await.expect("Failed to start scheduling");

    println!("Scheduler running in background.");
}

async fn scheduled_job(config: Arc<Config>) {
    for c in &config.endpoints {
        println!(
            "Running scheduled job for {0} \n pinging {1}\n\n",
            c.name, c.url
        );
        let req = HttpPing::new(&c);
        let resp = req.ping().await;
        match resp {
            Ok(_) => println!("Request pinged successfully"),
            Err(e) => println!("Error {:?}", e),
        }
    }
}
