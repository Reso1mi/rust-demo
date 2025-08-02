use std::num;

fn main() {
    let func_closure = factory_1(10);
    func_closure(19);
    let func_closure_2 = factory_3(10, false);
    func_closure_2(99);
}

fn factory_1(num: u32) -> impl Fn(u32) -> u32 {
    move |z| z - num
}

// if-else包含不同类型
// fn factory_2(num: u32, f: bool) -> impl Fn(u32) -> u32 {
//     if f { |x| x + num } else { |x| x - num }
// }

fn factory_3(num: u32, f: bool) -> Box<dyn Fn(u32) -> u32> {
    if f {
        Box::new(move |z| z - num)
    } else {
        Box::new(move |z| z + num)
    }
}
