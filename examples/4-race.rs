use async_std::prelude::FutureExt;
use test_async::*;

// In this example we await three tasks but break as soon as one of them finish

#[async_std::main]
async fn main() {
    let start = start();

    let a = do_something(1, 2);
    let b = do_something(2, 1);
    let c = do_something(3, 3);

    // Requires async-std feature "unstable" & async_std::future::FutureExt trait
    a.race(b).race(c).await;

    end(start, 1);
}
