struct Interface<'a, 'b> {
    manager: &'b mut Manager<'a>,
}

impl<'a, 'b> Interface<'a, 'b> {
    pub fn noop(self) {
        println!("interface consumed");
    }
}

struct Manager<'a> {
    text: &'a str,
}

struct List<'a> {
    manager: Manager<'a>,
}

impl<'a> List<'a> {
    // 这里&'a mut self将这个可变借用的生命周期提升到和结构体生命周期一致
    // 所以当结构体存在，编译器就会这个可变借用就会一直存在
    // pub fn get_interface(&'a mut self) -> Interface<'a> {
    //     Interface {
    //         manager: &mut self.manager,
    //     }
    // }

    // &'b mut self 可变借用生命周期和输出绑定
    // 如果不接受返回值，那么函数调用结束后就结束
    // 如果接受返回值，那么就会持续到返回值最后一次使用
    pub fn get_interface_2<'b>(&'b mut self) -> Interface<'a, 'b>
    where
        'a: 'b,
    {
        Interface {
            manager: &mut self.manager,
        }
    }
}

fn main() {
    let mut list = List {
        manager: Manager { text: "hello" },
    };

    // i的生命周期和上面的list解耦
    let i = list.get_interface_2();
    // .noop();
    // let i = List::get_interface(&mut list);

    println!("Interface should be dropped here and the borrow released");

    // 下面的调用会失败，因为同时有不可变/可变借用
    // 但是Interface在之前调用完成后就应该被释放了

    use_list(&list);
    // println!("{:?}", i.manager.text);
}

fn use_list(list: &List) {
    println!("{}", list.manager.text);
}
