//! Simple server model for the simulation. This is just one server that can
//! either be idle or working on exactly one job at a time.

use crate::job::Job;

/// The server can either be doing nothing or working on a job.
#[derive(Debug)]
pub enum ServerState {
    Idle,
    Busy { job: Job, remaining_time: u64 },
}

/// Tracks what the server is currently doing.
#[derive(Debug)]
pub struct Server {
    state: ServerState,
}

impl Default for Server {
    fn default() -> Self {
        Self::new()
    }
}

impl Server {
    /// Start with an idle server.
    pub fn new() -> Self {
        Self {
            state: ServerState::Idle,
        }
    }

    /// Check if the server is currently busy.
    pub fn is_busy(&self) -> bool {
        matches!(self.state, ServerState::Busy { .. })
    }

    /// Hand a job to the server and start processing it.
    pub fn start_job(&mut self, job: Job) {
        debug_assert!(
            !self.is_busy(),
            "Tried to start a job while server is already busy"
        );
        self.state = ServerState::Busy {
            remaining_time: job.service_time,
            job,
        };
    }

    /// Move time forward by one step. If a job finishes here,
    /// return it so metrics can record it.
    pub fn tick(&mut self, current_time: u64) -> Option<Job> {
        match &mut self.state {
            ServerState::Idle => None,
            ServerState::Busy {
                remaining_time,
                job,
            } => {
                *remaining_time = remaining_time.saturating_sub(1);

                if *remaining_time == 0 {
                    job.mark_finished(current_time);
                    let finished = std::mem::replace(job, Job::new(0, 0, 0));
                    self.state = ServerState::Idle;
                    Some(finished)
                } else {
                    None
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Server;
    use crate::job::Job;

    #[test]
    fn server_processes_job() {
        let mut server = Server::new();
        let mut job = Job::new(1, 0, 2);
        job.mark_started(0);
        server.start_job(job);

        // First tick should not finish anything.
        assert!(server.tick(1).is_none());

        // Second tick finishes the job.
        let done = server.tick(2).expect("job should be done by now");
        assert_eq!(done.id, 1);
    }
}
