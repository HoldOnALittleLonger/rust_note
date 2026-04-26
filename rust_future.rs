// need async runtime.

use std::future::Future;

/*
async fn return_future() -> Option<i32> {
    let v: i32 = 20;
    return Some(v);
}
*/

fn return_future() -> impl Future<Output = Option<i32>> {
    async {
        let v: i32 = 20;
        Some(v)
    }
}

fn main() {
    async_runtime::block_on(
        async {
            match return_future().await {
                None => println!("None"),
                Some(v) => println!("ret is {v}"),
            }
        }
    );
}

