use async_std::prelude::FutureExt;
use std::time::Duration;
use test_async::*;

// We can use delay to postpone the async execution to later.
// Here we execute a function that doesnt have any delay, but we add delay to when we envoke the future.

#[async_std::main]
async fn main() {
    let start = start();

    let a = do_something(1, 0);

    // Requires async-std feature "unstable" & async_std::future::FutureExt trait
    a.delay(Duration::from_secs(1)).await;

    end(start, 1);
}
