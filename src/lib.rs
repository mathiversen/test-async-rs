use std::{thread::sleep, time::Duration};

async fn heavy_computation(sleep_sec: u64) -> () {
    sleep(Duration::from_secs(sleep_sec));
}

pub async fn do_something(id: u64, sleep_sec: u64) -> () {
    println!("#{} started", id);
    heavy_computation(sleep_sec).await;
    println!("#{} ended, after {} seconds", id, sleep_sec);
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
