use async_std::prelude::FutureExt;
use test_async_rs::*;

// In this example we await all functions at the same time. This is very efficient as all of the tasks wait in parallel.

#[async_std::main]
async fn main() {
    let start = start();

    let a = do_something(1, 1);
    let b = do_something(2, 1);
    let c = do_something(3, 1);

    // Requires async-std feature "unstable" & async_std::future::FutureExt trait
    a.join(b).join(c).await;

    end(start, 1);
}
