use async_std::task;
use std::time::Instant;

use test_async::do_something;

// 1. We can use await on multiple futures, however this is not very efficient.
// 2. The program below will execute sequentially as every task needs to wait for the previous task to finish, before it can start.

#[test]
fn it_can_run_in_parallel() {
    let start = Instant::now();

    task::block_on(async {
        let _a = do_something(1, 1).await;
        let _b = do_something(2, 1).await;
        let _c = do_something(3, 1).await;
    });

    assert_eq!(start.elapsed().as_secs(), 3);
}
