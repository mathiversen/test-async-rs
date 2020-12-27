use test_async::*;

// If we use await on multiple async tasks they will execute sequentially. This is not an efficient way of dealing with async tasks.

#[async_std::main]
async fn main() {
    let start = start();

    let _a = do_something(1, 1).await;
    let _b = do_something(2, 1).await;
    let _c = do_something(3, 1).await;

    end(start, 3);
}
