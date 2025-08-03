use std::ops::Index;

fn main() {
    store_2_heap();
    avoid_stack_copy();
    sized_dst();
    trait_dst();
    box_layout();

    box_leak();
}

fn box_leak() {
    fn gen_static_str() -> &'static str {
        let s = String::from("Hello, World");
        // cannot return reference to local variable s
        // returns a reference to data owned by the current function
        // 正常是无法直接返回函数中临时变量地址
        // &s
        // 强制内存泄漏，将运行期产生的值转为'static
        Box::leak(s.into_boxed_str())
    }

    println!("{}", gen_static_str());
}

fn box_layout() {
    /*
     * let vec: Vec<i32>
     * 栈上指针 vec --> 堆内数据
     *
     * let vec: Vec<Box<i32>>
     * 栈上指针 vec --> 堆上的Box指针 --> 堆内数据
     */
    let mut arr_s = vec!["1".to_string(), "2".to_string()];
    let f = arr_s.index(0);
    // TODO: 深究
    // arr_s[0]会获取所有权
    // let f = arr_s[0];
    // let _ = arr_s[0];

    let arr = vec![Box::new(1), Box::new(2)];
    let (first, second) = (&arr[0], &arr[1]);
    let (first, second) = (arr.index(0), arr.index(1));
    let sum = **first + **second;
    println!("{sum}");
}

fn trait_dst() {
    trait Draw {
        fn draw(&self);
    }

    struct Button {
        id: u32,
    }

    impl Draw for Button {
        fn draw(&self) {
            println!("this is No.{} button", self.id);
        }
    }

    struct Select {
        id: u32,
    }

    impl Draw for Select {
        fn draw(&self) {
            println!("this is No.{} select", self.id);
        }
    }
    let btn1 = Button { id: 1 };
    let btn2 = Button { id: 2 };
    let select1 = Select { id: 1 };
    let select2 = Select { id: 2 };

    // let elements = vec![btn1, btn2, select1, select2];
    let elements: Vec<Box<dyn Draw>> = vec![
        Box::new(btn1),
        Box::new(btn2),
        Box::new(select1),
        Box::new(select2),
    ];

    elements.iter().for_each(|e| e.draw());
}

fn sized_dst() {
    {
        // 递归类型，可以无限递归，无法确定大小，dst的一种
        // enum List {
        //     Cons(i32, List),
        //     Nil,
        // }

        enum List {
            // 通过Box固定了大小
            Cons(i32, Box<List>),
            Nil,
        }
    }
}

fn avoid_stack_copy() {
    // 栈上数据，直接拷贝
    let arr = [99; 1000];
    // copy
    let arr1 = arr;
    // copy
    let arr2 = arr;
    // copy
    let arr3 = arr;
    println!("{:?}", arr.len());

    {
        let box_arr = Box::new([99; 1000]);
        let box_arr_1 = box_arr;
        // 栈上发生移动，堆不变
        // println!("{:?}", box_arr);
        println!("{:?}", box_arr_1.len());
    }

    {
        let box_arr = Box::new([99; 988]);
        let box_arr_1 = &box_arr;
        println!("{:?}", box_arr.len());
        println!("{:?}", box_arr_1.len());
    }
}

fn store_2_heap() {
    let box_u32 = Box::new(3);
    println!("{}", box_u32);
    let a = 1;
    // cannot add Box<{integer}> to {integer}
    // let sum = a + box_u32; // 错误
    let sum = a + *box_u32;
    println!("{sum}");
}
