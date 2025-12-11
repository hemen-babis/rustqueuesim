//! Tracks stats about the simulation. I'm basically collecting everything
//! needed to analyze how the queue behaves: average waiting time, how often
//! the server was busy, how long jobs spent in the system, etc.

use crate::job::Job;

#[derive(Debug, Default, Clone)]
pub struct Metrics {
    pub total_jobs_arrived: u64,
    pub total_jobs_completed: u64,
    pub total_wait_time: u64,
    pub total_system_time: u64,
    pub max_queue_len: u64,
    pub busy_time: u64,
    pub time_steps: u64,
}

impl Metrics {
    /// Update metrics every single step of the simulation.
    pub fn step(&mut self, queue_len: usize, server_busy: bool, finished_job: Option<Job>) {
        self.time_steps += 1;
        self.max_queue_len = self.max_queue_len.max(queue_len as u64);

        if server_busy {
            self.busy_time += 1;
        }

        if let Some(job) = finished_job {
            self.total_jobs_completed += 1;

            if let Some(w) = job.wait_time() {
                self.total_wait_time += w;
            }

            if let Some(s) = job.system_time() {
                self.total_system_time += s;
            }
        }
    }

    /// Whenever a job arrives, count it.
    pub fn record_arrival(&mut self) {
        self.total_jobs_arrived += 1;
    }

    /// Average wait among finished jobs.
    pub fn avg_wait_time(&self) -> f64 {
        if self.total_jobs_completed == 0 {
            0.0
        } else {
            self.total_wait_time as f64 / self.total_jobs_completed as f64
        }
    }

    pub fn avg_system_time(&self) -> f64 {
        if self.total_jobs_completed == 0 {
            0.0
        } else {
            self.total_system_time as f64 / self.total_jobs_completed as f64
        }
    }

    pub fn utilization(&self) -> f64 {
        if self.time_steps == 0 {
            0.0
        } else {
            self.busy_time as f64 / self.time_steps as f64
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Metrics;
    use crate::job::Job;

    #[test]
    fn metrics_compute_averages() {
        let mut m = Metrics::default();
        let mut j1 = Job::new(1, 0, 2);
        j1.mark_started(0);
        j1.mark_finished(2);

        let mut j2 = Job::new(2, 1, 3);
        j2.mark_started(3);
        j2.mark_finished(6);

        m.step(0, true, Some(j1));
        m.step(0, true, Some(j2));

        assert_eq!(m.total_jobs_completed, 2);
        assert!((m.avg_wait_time() - 1.0).abs() < 1e-6);
    }
}
