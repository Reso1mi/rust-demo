// 例如，对于以下闭包：
// let x = 10;
// let closure = |a| a + x;
// 编译器会生成一个类似如下的结构体：

// struct Closure {
//     x: i32, // 按值捕获 x
// }

// impl FnOnce<(i32,)> for Closure {
//     type Output = i32;
//     fn call_once(self, (a,): (i32,)) -> i32 {
//         // 调用FnMut或Fn的实现
//         (&self).call(a) // 假设还有Fn的实现，调用它
//     }
// }

// impl FnMut<(i32,)> for Closure {
//     fn call_mut(&mut self, (a,): (i32,)) -> i32 {
//         // 调用Fn的实现
//         (*(self)).call(a)
//     }
// }

// impl Fn<(i32,)> for Closure {
//     fn call(&self, (a,): (i32,)) -> i32 {
//         a + self.x
//     }
// }
fn main() {}
