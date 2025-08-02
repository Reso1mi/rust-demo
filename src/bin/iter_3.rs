use std::process::id;

// 生产者：存储数据的集合类型
#[derive(Debug)]
struct NumberCollection {
    items: Vec<i32>,
}

impl NumberCollection {
    fn new() -> Self {
        NumberCollection { items: Vec::new() }
    }

    fn add(&mut self, num: i32) {
        self.items.push(num);
    }

    // 生产者方法1: 创建不可变引用迭代器
    fn iter(&self) -> RefIterator<'_> {
        RefIterator {
            index: 0,
            collection: self,
        }
    }

    // 生产者方法2: 创建可变引用迭代器
    fn iter_mut(&mut self) -> MutRefIterator<'_> {
        MutRefIterator {
            index: 0,
            collection: self,
        }
    }

    // 生产者方法3: 创建所有权迭代器
    fn into_iter(self) -> OwnedIterator {
        OwnedIterator {
            index: 0,
            collection: self,
        }
    }
}

// 迭代器1: 不可变引用迭代器
struct RefIterator<'a> {
    index: usize,
    collection: &'a NumberCollection,
}

impl<'a> Iterator for RefIterator<'a> {
    type Item = &'a i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.collection.items.len() {
            let item = &self.collection.items[self.index];
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }
}

// 迭代器2: 可变引用迭代器
struct MutRefIterator<'a> {
    index: usize,
    collection: &'a mut NumberCollection,
}

impl<'a> Iterator for MutRefIterator<'a> {
    type Item = &'a mut i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.collection.items.len() {
            let ptr = self.collection.items.as_mut_ptr();
            unsafe {
                // 手动获取元素引用
                let item = &mut *ptr.add(self.index);
                self.index += 1;
                Some(item)
            }
            // 从可变借用集合中获取其中元素的可变借用
            // 按照编译器的生命周期推断规则，两者周期不一致，返回的item可变借用的周期是和&mut self绑定的，比如'b
            // 但是函数实际标注的返回值的周期是'a，是迭代器以及其中集合引用的生命周期 (Self::Item = &'a mut i32)。
            // 周期不一致，编译不通过（这里如果是普通的方法，直接手动添加生命周期标注其实就可以通过编译）
            // 两种解决方案：
            // 1.分割底层切片 2.底层指针操作
            // let item = &mut self.collection.items[self.index];
            // self.index += 1;
            // Some(item)
        } else {
            None
        }
    }
}

// 1.分割底层切片
struct MutRefIterator2<'a> {
    index: usize,
    slice: &'a mut [i32],
}

impl<'a> Iterator for MutRefIterator2<'a> {
    type Item = &'a mut i32;

    fn next(&mut self) -> Option<Self::Item> {
        // 将当前切片拆分为第一个元素和剩余部分
        let (first, rest) = std::mem::take(&mut self.slice).split_first_mut()?;
        self.slice = rest;
        Some(first)
    }
}

// 迭代器3: 所有权迭代器
struct OwnedIterator {
    index: usize,
    collection: NumberCollection,
}

impl Iterator for OwnedIterator {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.collection.items.len() {
            // 取走所有权（转移元素）
            let item = self.collection.items[self.index];
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }
}

// 实现 IntoIterator 以便直接用于 for 循环
impl IntoIterator for NumberCollection {
    type Item = i32;
    type IntoIter = OwnedIterator;

    fn into_iter(self) -> Self::IntoIter {
        self.into_iter() // 调用生产者方法
    }
}

// 实现不可变引用的 IntoIterator
impl<'a> IntoIterator for &'a NumberCollection {
    type Item = &'a i32;
    type IntoIter = RefIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        RefIterator {
            index: 0,
            collection: self,
        }
    }
}

// 实现可变引用的 IntoIterator
impl<'a> IntoIterator for &'a mut NumberCollection {
    type Item = &'a mut i32;
    type IntoIter = MutRefIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        MutRefIterator {
            index: 0,
            collection: self,
        }
    }
}

fn main() {
    // 创建生产者
    let mut collection = NumberCollection::new();
    collection.add(10);
    collection.add(20);
    collection.add(30);

    println!("初始状态: {:?}", collection);

    // 1. 使用不可变引用迭代
    println!("\n使用 iter():");
    for item in collection.iter() {
        println!("读取: {}", item);
    }

    // 2. 使用可变引用迭代修改数据
    println!("\n使用 iter_mut():");
    for item in collection.iter_mut() {
        *item += 5; // 修改值
        println!("修改为: {}", item);
    }

    // 3. 再次使用不可变引用检查修改
    println!("\n修改后状态:");
    for item in collection.iter() {
        println!("检查: {}", item);
    }

    // 4. 使用所有权转移迭代
    println!("\n使用 into_iter() 消耗生产者:");
    for item in collection.into_iter() {
        println!("消耗: {}", item);
    }

    // 5. 尝试再次访问会编译错误（所有权已转移）
    // println!("尝试访问已消耗的生产者: {:?}", collection);

    // 6. 使用 for 循环语法糖（自动调用 into_iter）
    let mut another = NumberCollection::new();
    another.add(100);
    another.add(200);

    println!("\n使用 for 循环语法糖:");
    for item in another {
        println!("for循环消耗: {}", item);
    }
}

fn test() {
    let mut v = vec![1, 2, 4];
    let z = &mut v;

    let first = &mut z[0];
    *first = 100;

    println!("{}", z[0]);
    // println!("{first}");
    // 通过集合的可变借用创建出一个集合中元素的可变借用
    let ff_op = test_get_mut(z, 0);
    let ff = ff_op.unwrap();
    // z[0] = 100; 报错
    // println!("{}", z[0]); 报错
    *ff = 999;
    println!("{}", z[0]);
}

fn test_get_mut(v: &mut Vec<usize>, i: usize) -> Option<&mut usize> {
    if i >= v.len() {
        return None;
    } else {
        Some(&mut v[i])
    }
}
