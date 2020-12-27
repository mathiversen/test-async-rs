use futures::{executor::block_on, join};
use std::time::Instant;
use test_async::do_something;

// 1. We can use `block_on` to execute multiple futures
// 2. However this is not efficient, as the main function is synchronous and each task will have to wait for the previous task before it begins to execute

async fn runtime() {
    let task_1 = do_something(1, 1);
    let task_2 = do_something(2, 1);
    let task_3 = do_something(3, 1);

    join!(task_1, task_2, task_3);
}

fn main() {
    let start = Instant::now();

    // Start

    block_on(runtime());
    // End

    println!("---------------------------------");
    println!("Program took: {} sec", start.elapsed().as_secs_f32());
}
