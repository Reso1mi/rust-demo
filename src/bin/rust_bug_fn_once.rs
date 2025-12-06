// Cargo.toml 必需依赖
// [dependencies]
// tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
// futures = "0.3"

use std::sync::Arc;

use futures::stream::{self, FuturesUnordered, StreamExt};
use tokio;

async fn load() {
    // items 中含有对 self.data 的引用，这是问题的起点
    let items: Vec<&str> = vec!["1", "2"];

    // 这里的 map(|slice| async move { ... }) 会捕获 slice（即引用），所以返回的 Future
    // 带有具体生命周期（不是 'static）。
    let _res: Vec<usize> = stream::iter(items)
        .map(|slice| {
            async move {
                // 模拟异步操作
                // 注意这里会返回一个 Future，它携带了 slice 的引用生命周期
                tokio::task::yield_now().await;
                slice.len()
            }
        })
        .buffer_unordered(2)
        // .boxed()
        .collect()
        .await;
    // 闭包隐式绑定了特定的生命周期（可能来自捕获的值或返回 Future），导致无法满足编译器的“对任何生命周期都通用”的要求
    println!("loaded");
}

async fn load_2() {
    // items 中含有对 self.data 的引用，这是问题的起点
    let items: Vec<&str> = vec!["1", "2"];

    let items = items.into_iter().map(|x| x.to_string()).collect::<Vec<_>>();
    // 这里的 map(|slice| async move { ... }) 会捕获 slice（即引用），所以返回的 Future
    // 带有具体生命周期（不是 'static）。
    let _res: Vec<usize> = stream::iter(items)
        .map(|slice| help(slice))
        .buffer_unordered(2)
        // .boxed()
        .collect()
        .await;

    println!("loaded");
}

async fn help(slice: String) -> usize {
    // 模拟异步操作
    // 注意这里会返回一个 Future，它携带了 slice 的引用生命周期
    tokio::task::yield_now().await;
    slice.len()
}

async fn help_arc(slice: Arc<&str>) -> usize {
    tokio::task::yield_now().await;
    slice.len()
}

async fn load_arc() {
    let shared = Arc::new("12345");
    let items: Vec<Arc<&str>> = vec![shared.clone(), shared.clone()];

    let _res: Vec<usize> = futures::stream::iter(items)
        .map(|slice| help_arc(slice))
        .buffer_unordered(2)
        .collect()
        .await;

    println!("loaded arc");
}

async fn load_future() {
    // items 中含有对 self.data 的引用，这是问题的起点
    let items: Vec<&str> = vec!["1", "2"];

    let mut futs = FuturesUnordered::new();

    for item in items {
        futs.push(async move {
            tokio::task::yield_now().await;
            item.len()
        });
    }
    println!("loaded arc");
}

// 一个 wrapper async fn，直接调用 load()
async fn worker() {
    // load().await;
    // load_2().await;
    // load_arc().await;
    load_future().await
}

#[tokio::main]
async fn main() {
    // 这里试图把一个借用 `&app` 的 future 交给 tokio::spawn：
    // spawn 要求被传入的 future 是 'static。由于 load()/worker() 返回的 future
    // 带有对 self 的借用(具体生命周期)，因此编译器会报错。
    tokio::spawn(worker());
    // worker().await;
    // ↑ 在多数编译器 / toolchain 上，这一行会因为生命周期/泛化问题而无法通过编译。
}

// Source - https://stackoverflow.com/a
// Posted by ais523
// Retrieved 2025-11-15, License - CC BY-SA 4.0

// fn f<T>(_t: T) {}
// fn g<T: FnOnce(&u32)>(_t: T) {}

// fn main() {
//     g(f);
// }
