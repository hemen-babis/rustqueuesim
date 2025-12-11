//! Job struct used in the simulation. A "job" here just means any unit
//! of work entering the system. I track when it arrives, how long it
//! needs to be processed, and when it actually starts/finishes.

#[derive(Debug, Clone)]
pub struct Job {
    /// Unique ID for this job.
    pub id: u64,
    /// Time step when the job arrived.
    pub arrival_time: u64,
    /// How long this job needs to be processed.
    pub service_time: u64,
    /// When the job actually started being served.
    pub start_service_time: Option<u64>,
    /// When the job finished service.
    pub finish_time: Option<u64>,
}

impl Job {
    /// Create a new job with the given info.
    pub fn new(id: u64, arrival_time: u64, service_time: u64) -> Self {
        Self {
            id,
            arrival_time,
            service_time,
            start_service_time: None,
            finish_time: None,
        }
    }

    /// Record the moment the server begins processing this job.
    pub fn mark_started(&mut self, t: u64) {
        self.start_service_time = Some(t);
    }

    /// Record when the job is completely finished.
    pub fn mark_finished(&mut self, t: u64) {
        self.finish_time = Some(t);
    }

    /// How long the job waited before being served.
    pub fn wait_time(&self) -> Option<u64> {
        self.start_service_time
            .map(|start| start.saturating_sub(self.arrival_time))
    }

    /// Total time the job spent in the whole system.
    pub fn system_time(&self) -> Option<u64> {
        match (self.finish_time, Some(self.arrival_time)) {
            (Some(finish), Some(arr)) => Some(finish.saturating_sub(arr)),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Job;

    #[test]
    fn wait_and_system_time_work() {
        let mut j = Job::new(1, 5, 3);
        j.mark_started(7);
        j.mark_finished(10);
        assert_eq!(j.wait_time(), Some(2));
        assert_eq!(j.system_time(), Some(5));
    }
}
