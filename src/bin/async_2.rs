use std::process::Output;

// `foo()`返回一个`Future<Output = u8>`,
// 当调用`foo().await`时，该`Future`将被运行，当调用结束后我们将获取到一个`u8`值
async fn foo() -> u8 {
    5
}

fn bar() -> impl Future<Output = u8> {
    // 下面的`async`语句块返回`Future<Output = u8>`
    async {
        let x: u8 = foo().await;
        x + 5
    }
}

fn main() {
    life_time_1();
    life_time_2();
    async_move();
}

fn async_move() {
    let c = "foo".to_string();

    let z = &c;

    // let d = c;

    z.as_bytes();

    async fn blocks() {
        let my_string = "foo".to_string();

        let future_one = async {
            // ...
            println!("{my_string}");
        };

        let future_two = async {
            // ...
            println!("{my_string}");
        };

        // let c = my_string;

        // drop(my_string);
        // 运行两个 Future 直到完成
        let ((), ()) = futures::join!(future_one, future_two);
    }

    async fn blocks_2<'a>(my_string: &'a String) -> impl Future<Output = ()> + 'a {
        // let my_string = "foo".to_string();

        let future_one = async move {
            // ...
            println!("{my_string}");
        };

        // my_string.push_str("string");

        let future_two = async move {
            // ...
            println!("{my_string}");
        };

        // let c = my_string;

        // drop(my_string);
        // 运行两个 Future 直到完成
        // let ((), ()) = futures::join!(future_one, future_two);
        future_two
    }
}

fn life_time_1() {
    async fn foo(x: &u8) -> u8 {
        *x
    }

    // 上面的函数跟下面的函数是等价的:
    fn foo_expanded<'a>(x: &'a u8) -> impl Future<Output = u8> + 'a {
        async move { *x }
    }
}

fn life_time_2() {
    use std::future::Future;
    // fn bad() -> impl Future<Output = u8> {
    //     let x = 5;
    //     borrow_x(&x) // ERROR: `x` does not live long enough
    // }

    fn good() -> impl Future<Output = u8> + 'static {
        // 不依赖任何外部引用，所以整个Future生命周期为static
        async {
            let x: u8 = 5;
            borrow_x(&x).await
        }
    }

    fn good2() -> impl Future<Output = u8> + 'static {
        let x: u8 = 5;

        let f = async move { borrow_x(&x).await };

        f
    }

    async fn borrow_x(x: &u8) -> u8 {
        *x
    }
}
