struct Point(u32, u32);
#[derive(Debug)]
struct Rectangle {
    weith: u32,
    heigth: u32,
}

impl Rectangle {
    fn get_w(&self) -> u32 {
        self.weith
    }

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

pub struct User<'a> {
    pub active: bool,
    pub name: &'a str,
    pub email: &'a str,
    // username: &str,
    pub sign_in_count: u64,
}

// 返回值生命周期 ≤ 'a
pub fn build_user<'a>(email: &'a str, name: &'a str, active: bool) -> User<'a> {
    User {
        active,
        email: email,
        name: name,
        sign_in_count: 1,
    }
}
