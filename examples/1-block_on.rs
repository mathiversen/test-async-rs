use test_async_rs::*;

// In all of these examples we'll be using the async-std lib.
// With the "attributes" feature enabled we can add the "async_std::main" attribute to the main function, this will enable us to use async and await without wrapping all of the code with a "block_on", as main functions normally doesn't allow the async keyword in the declaration.

#[async_std::main]
async fn main() {
    let start = start();

    let _a = do_something(1, 1).await;

    end(start, 1);
}
