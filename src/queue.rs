//! Basic FIFO queue using VecDeque. This is literally just the wait line
//! for all incoming jobs.

use crate::job::Job;
use std::collections::VecDeque;

/// Queue for storing waiting jobs in order.
#[derive(Debug, Default)]
pub struct JobQueue {
    inner: VecDeque<Job>,
}

impl JobQueue {
    pub fn new() -> Self {
        Self {
            inner: VecDeque::new(),
        }
    }

    /// Add a job to the back of the queue.
    pub fn push(&mut self, job: Job) {
        self.inner.push_back(job);
    }

    /// Remove a job from the front (FIFO style).
    pub fn pop(&mut self) -> Option<Job> {
        self.inner.pop_front()
    }

    /// How many jobs are currently waiting.
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::JobQueue;
    use crate::job::Job;

    #[test]
    fn queue_preserves_order() {
        let mut q = JobQueue::new();
        q.push(Job::new(1, 0, 1));
        q.push(Job::new(2, 1, 1));
        q.push(Job::new(3, 2, 1));

        assert_eq!(q.pop().unwrap().id, 1);
        assert_eq!(q.pop().unwrap().id, 2);
        assert_eq!(q.pop().unwrap().id, 3);
    }
}
