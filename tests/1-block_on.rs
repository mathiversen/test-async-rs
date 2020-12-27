use async_std::task;
use std::time::Instant;

use test_async::do_something;

// 1. To execute a function that returns a future is simple, you simply await it.
// 2. We need to have a blocking function at the root of the async program

#[test]
fn it_can_run_in_parallel() {
    let start = Instant::now();

    task::block_on(async {
        let _a = do_something(1, 1).await;
    });

    assert_eq!(start.elapsed().as_secs(), 1);
}
