use futures::executor::block_on;
use std::time::Instant;
use test_async::do_something;

// 1. `do_something` returns a future
// 2. `block_on` blocks the current thread until the provided future has completed.

fn main() {
    let start = Instant::now();

    // Start
    let task_1 = do_something(1, 1);
    block_on(task_1);
    // End

    println!("---------------------------------");
    println!("Program took: {} sec", start.elapsed().as_secs_f32());
}
