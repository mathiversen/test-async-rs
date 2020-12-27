use async_std::task;
use futures::join;
use std::time::Instant;

use test_async::do_something;

// To await multiple futures we can use futures::join! macro

#[test]
fn it_can_run_in_parallel() {
    let start = Instant::now();

    task::block_on(async {
        let a = do_something(1, 1);
        let b = do_something(2, 1);
        let c = do_something(3, 1);
        join!(a, b, c);
    });

    assert_eq!(start.elapsed().as_secs(), 1);
}
