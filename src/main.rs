use core::cmp::Ordering;

use binary_heap_plus::*;
use chrono::{DateTime, Utc};
use cron::Schedule;

enum Privilage {}

#[derive(Default)]
enum RunState {
    #[default]
    Stopped,
}

struct Job {
    name: String,
    next_fire: DateTime<Utc>,
    schedule: Schedule,
}

impl Ord for Job {
    fn cmp(&self, other: &Self) -> Ordering {
        other.next_fire.cmp(&self.next_fire)
    }
}

impl PartialOrd for Job {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Job {
    fn eq(&self, other: &Self) -> bool {
        self.next_fire == other.next_fire
    }
}

impl Eq for Job {}

#[derive(Default)]
struct State {
    cron_loop_run_state: RunState,
    jobs: Option<BinaryHeap<Job, MinComparator>>,
}

fn main() {
    let mut state = State::default();
    let new_heap: BinaryHeap<Job, MinComparator> = BinaryHeap::new_min();
    // for dev
    state.jobs = Some(new_heap);
}
