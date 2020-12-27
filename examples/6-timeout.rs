use async_std::prelude::FutureExt;
use std::time::Duration;
use test_async_rs::*;

// We can use timeout to break async runtime if it takes to long time to execute

#[async_std::main]
async fn main() {
    let start = start();

    let a = do_something(1, 2);

    // Requires async-std feature "unstable" & async_std::future::FutureExt trait
    let result = a.timeout(Duration::from_secs(1)).await;
    assert!(result.is_err());

    end(start, 1);
}
