use trpl::{Either, Html};

fn main() {
    let url1 = "https://ecks.top/";
    let url2 = "https://www.bilibili.com/";

    trpl::run(async {
        let title_fut_1 = page_title(url1);
        let title_fut_2 = page_title(url2);

        let (url, title) = match trpl::race(title_fut_1, title_fut_2).await {
            Either::Left(left) => left,
            Either::Right(right) => right,
        };

        println!("{url} returned first");
        match title {
            Some(title) => println!("Its page title is: '{title}'"),
            None => println!("Its title could not be parsed."),
        }
    })
}

async fn page_title(url: &str) -> (&str, Option<String>) {
    let res = trpl::get(url).await;
    let text = res.text().await;
    // println!("text: {:?}", text);
    (
        url,
        Html::parse(&text)
            .select_first("title")
            .map(|title_ele| title_ele.inner_html()),
    )
}

// ***************************************************************
// 上面的代码和下面等价，编译器会编译为类似下面这样
fn page_title_2(url: &str) -> impl Future<Output = Option<String>> {
    async move {
        let text = trpl::get(url).await.text().await;
        Html::parse(&text)
            .select_first("title")
            .map(|title| title.inner_html())
    }
}
