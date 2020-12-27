use futures::executor::block_on;
use std::time::Instant;
use test_async::do_something;

// 1. We can use `block_on` to execute multiple futures
// 2. However this is not efficient, as the main function is synchronous and each task will have to wait for the previous task before it begins to execute

fn main() {
    let start = Instant::now();

    // Start
    let task_1 = do_something(1, 1);
    let task_2 = do_something(2, 1);
    let task_3 = do_something(3, 1);
    block_on(task_1);
    block_on(task_2);
    block_on(task_3);
    // End

    println!("---------------------------------");
    println!("Program took: {} sec", start.elapsed().as_secs_f32());
}
