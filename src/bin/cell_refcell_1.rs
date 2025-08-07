use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

fn main() {
    // test_cell();
    // test_refcell();
    // refcell_inner_mut();
    // rc_refcell();
    // cell_from_mut();
}

fn cell_from_mut() {
    fn is_even(i: i32) -> bool {
        i % 2 == 0
    }

    fn retain_even(nums: &mut Vec<i32>) {
        // let mut i = 0;
        // for num in nums.iter().filter(|&&num| is_even(num)) {
        //     // 同时存在可变和不可变引用，报错
        //     // nums[i] = *num;
        //     i += 1;
        // }
        // nums.truncate(i);
        let slice: &[Cell<i32>] = Cell::from_mut(&mut nums[..]).as_slice_of_cells();

        let mut i = 0;
        for num in slice.iter().filter(|num| is_even(num.get())) {
            slice[i].set(num.get());
            i += 1;
        }

        nums.truncate(i);
    }
}

fn rc_refcell() {
    let s = Rc::new(RefCell::new("Hello".to_string()));
    let a = s.clone();
    let b = s.clone();
    let c = s.clone();
    s.borrow_mut().push_str("World");
    // a b c s都指向同一份数据，同时可以使用borrow_mut进行修改
    println!("{:?} - {:?} - {:?} - {:?}", a, b, c, s);
}

fn refcell_inner_mut() {
    // 定义在外部库中的特征
    pub trait Messenger {
        // 特征send是不可变借用
        fn send(&self, msg: String);
    }

    // --------------------------
    // 我们的代码中的数据结构和实现
    struct MsgQueue {
        // 我们自己实现增加了缓存，所以需要对 msg_cache 进行修改
        msg_cache: RefCell<Vec<String>>,
    }

    impl Messenger for MsgQueue {
        fn send(&self, msg: String) {
            // 目前整个MsgQueue是不可变的
            // 如果不使用 RefCell 这里就无法对msg_cache进行push
            //
            // 之所以称之为内部可变性，是因为结构体MsgQueue本身不可变，但是其内部字段是可变的
            self.msg_cache.borrow_mut().push(msg);
        }
    }

    let mq = MsgQueue {
        msg_cache: RefCell::new(Vec::new()),
    };
    mq.send("123".to_string());
    println!("{:?}", mq.msg_cache.borrow());
}

fn test_refcell() {
    let c = RefCell::new("test_refcell".to_string());
    let bc = c.borrow();
    println!("{}", bc);
    // 依然会报错，只是将报错推迟到了运行期
    let mbc = c.borrow_mut();
}

fn test_cell() {
    let c = Cell::new("value");
    let first = c.get();
    println!("{first}");
    c.set("mod value");
    let second = c.get();
    //
    println!("{first}, {second}");
}
