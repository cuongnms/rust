use job_scheduler::{Job, JobScheduler};

fn main() {
    let mut schedule = JobScheduler::new();

    schedule.add(Job::new("1/2 * * * * *".parse().unwrap(), || {
        println!("Running job...");
    }));

    loop{
        schedule.tick();
    }
}
