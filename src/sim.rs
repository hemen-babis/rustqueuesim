//! Main simulation logic. This is where everything comes together.
//! One simulation run = advancing time step-by-step and letting
//! jobs arrive, wait, and get processed.

use crate::{job::Job, metrics::Metrics, queue::JobQueue, server::Server};
use rand::distributions::{Bernoulli, Uniform};
use rand::prelude::*;

/// Settings for a simulation run.
#[derive(Debug, Clone)]
pub struct SimConfig {
    pub total_time: u64,
    pub arrival_prob: f64,
    pub min_service_time: u64,
    pub max_service_time: u64,
    pub seed: u64,
}

impl Default for SimConfig {
    fn default() -> Self {
        Self {
            total_time: 1_000,
            arrival_prob: 0.3,
            min_service_time: 1,
            max_service_time: 5,
            seed: 42,
        }
    }
}

/// Central simulator.
#[derive(Debug)]
pub struct Simulator {
    pub config: SimConfig,
    pub metrics: Metrics,
    queue: JobQueue,
    server: Server,
    rng: StdRng,
    next_job_id: u64,
}

impl Simulator {
    /// Build a new simulator from a config.
    pub fn new(config: SimConfig) -> Self {
        let rng = StdRng::seed_from_u64(config.seed);
        Self {
            config,
            metrics: Metrics::default(),
            queue: JobQueue::new(),
            server: Server::new(),
            rng,
            next_job_id: 0,
        }
    }

    fn sample_service_time(&mut self) -> u64 {
        let range = Uniform::from(self.config.min_service_time..=self.config.max_service_time);
        range.sample(&mut self.rng)
    }

    /// Run the entire simulation from t = 0 to t = total_time.
    pub fn run(&mut self) {
        let arrival_dist =
            Bernoulli::new(self.config.arrival_prob).expect("arrival probability must be valid");

        for t in 0..self.config.total_time {
            // 1. Chance of a new job arriving at this moment.
            if arrival_dist.sample(&mut self.rng) {
                let service_time = self.sample_service_time();
                let job = Job::new(self.next_job_id, t, service_time);
                self.next_job_id += 1;
                self.queue.push(job);
                self.metrics.record_arrival();
            }

            // 2. If server is free, give it the next job.
            if !self.server.is_busy() {
                if let Some(mut job) = self.queue.pop() {
                    job.mark_started(t);
                    self.server.start_job(job);
                }
            }

            // 3. Process current job and see if it finishes.
            let finished = self.server.tick(t + 1);

            // 4. Update metrics for this step.
            self.metrics
                .step(self.queue.len(), self.server.is_busy(), finished);
        }
    }
}
