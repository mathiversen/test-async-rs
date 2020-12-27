use futures::executor::block_on;
use std::{
    thread::sleep,
    time::{Duration, Instant},
};

// Using `block_on` on multiple tasks will execute all of the futures... But, this code is not optimal since the main function is still syncronous and each of the futures will not start until the last task has been finished.

async fn do_something(id: u64, sleep_sec: u64) -> () {
    println!("#{} started", id);
    sleep(Duration::from_secs(sleep_sec));
    println!("#{} ended, after {} sec", id, sleep_sec);
}

fn main() {
    let start = Instant::now();

    let task_1 = do_something(1, 1);
    let task_2 = do_something(2, 1);
    let task_3 = do_something(3, 1);
    block_on(task_1);
    block_on(task_2);
    block_on(task_3);

    println!("---------------------------------");
    println!("Program took: {} sec", start.elapsed().as_secs_f32());
}
