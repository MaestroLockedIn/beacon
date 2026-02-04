use std::sync::Arc;

use tokio_cron_scheduler::{Job, JobScheduler};

use crate::{
    config::{Config, EndpointConfig},
    scheduler::http_ping::HttpPing,
};

pub mod http_ping;

pub async fn run_tasks(req: Arc<Config>) {
    let sched = JobScheduler::new()
        .await
        .expect("Failed to create scheduler");

    let config = req.clone();
    for endpoint in config.endpoints.clone() {
        let interval = format!("*/{} * * * * *", endpoint.interval_seconds);
        let job = Job::new_async(interval, move |_, _| {
            let a = endpoint.clone();
            Box::pin(async move {
                scheduled_job(a).await;
            })
        })
        .expect("Failed to create job");
        sched.add(job).await.expect("Failed to add job");
    }

    sched.start().await.expect("Failed to start scheduling");
    println!("Scheduler running in background.");
}

async fn scheduled_job(c: EndpointConfig) {
    println!(
        "\n\n\nRunning scheduled job for {0} \npinging {1}",
        c.name, c.url
    );
    let req = HttpPing::new(&c);
    let resp = req.ping().await;
    match resp {
        Ok(_) => println!("Request pinged successfully\n\n\n"),
        Err(e) => println!("Error {:?}", e),
    }
}
