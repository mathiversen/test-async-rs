use futures::executor::block_on;
use std::{thread, time::Duration};

// `block_on` blocks the current thread until the provided future has run to
// completion. Other executors provide more complex behavior, like scheduling
// multiple futures onto the same thread.

async fn do_something(id: u64, sleep_sec: u64) -> () {
    println!("#{} started", id);
    thread::sleep(Duration::from_secs(sleep_sec));
    println!("#{} ended, after {} seconds", id, sleep_sec);
}

fn main() {
    let task_1 = do_something(1, 1);
    block_on(task_1);
}
