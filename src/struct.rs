struct User {
    active: bool,
    name: String,
    email: String,
    sign_in_count: u64,
}

struct Point(u32, u32);

#[derive(Debug)]
struct Rectangle {
    weith: u32,
    heigth: u32,
}

impl Rectangle {

    // &self 借用
    fn area(&self, c: u32) -> u32 {
        self.weith * self.heigth * c
    }

    // self 获取所有权
    fn area2(self, c: u32) -> u32 {
        self.weith * self.heigth * c
    }

    // Self == Rectangle, 别名
    fn square(size: u32) -> Self {
        Rectangle {
            weith: size,
            heigth: size,
        }
    }
}

fn main() {
    let mut user1 = User {
        active: true,
        name: String::from("test"),
        email: String::from("test@qq.com"),
        sign_in_count: 1,
    };

    let user2 = User {
        // copy
        active: user1.active,
        // move
        name: user1.name,
        // move
        email: user1.email,
        // copy
        sign_in_count: user1.sign_in_count,
    };

    // 重新赋值，指向新的值
    user1.name = String::from("123");
    println!("{0}", user1.name);
    // println!("{0}", user1.email); 未重新赋值，所有权仍然在 user2 中
    println!("{0}", user2.name);
    println!("{0}", user2.email);

    let p1 = Point(1, 2);
    println!("p1 = {}", p1.1);

    let rec = Rectangle{weith:30, heigth:50};

    // dbg会获取所有权，使用&
    dbg!(&rec);

    println!("rec = {rec:#?}");

    println!("rec = {}", rec.area(2));

    println!("rec = {rec:#?}");

    // 自动解引用
    let rec2 = &&Rectangle{weith:30, heigth:50};
    println!("rec = {}", rec2.area(2));
    // 不行，area2是所有权方法，引用没有所有权，不能解引用获取所有权
    // println!("rec = {}", rec2.area2(2));

    let sq = Rectangle::square(10);
    println!("rec = {}", sq.area(1));
}

fn build_user(email: String, name: String, active: bool) -> User {
    User {
        active,
        email,
        name,
        sign_in_count: 1,
    }
}
