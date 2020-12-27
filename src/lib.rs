use async_std::task;
pub use std::time::{Duration, Instant};

pub async fn do_something(id: u64, sleep_sec: u64) {
    println!("#{} started", id);
    task::sleep(Duration::from_secs(sleep_sec)).await;
    println!("#{} ended, after {} seconds", id, sleep_sec);
}

pub fn end(start: Instant, expected: u64) {
    let end = start.elapsed().as_secs();

    assert_eq!(end, expected);
    println!("Done after: {} seconds", end);
}

pub fn start() -> Instant {
    Instant::now()
}

#[cfg(test)]
mod test {
    use super::*;
    use futures::executor::block_on;
    use std::time::Instant;

    #[test]
    fn it_can_wait() {
        let start = Instant::now();
        block_on(do_something(1, 1));
        assert_eq!(start.elapsed().as_secs(), 1);
    }
}
