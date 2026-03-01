use std::ops::Deref;
use std::sync::Arc;

// 1. 定义一个简单的 Trait
pub trait Pool {
    fn get_address(&self) -> u64;
}

// 2. 定义一个具体的类型并实现 Trait
struct UniswapV2Pool {
    address: u64,
}

impl Pool for UniswapV2Pool {
    fn get_address(&self) -> u64 {
        self.address
    }
}

// 3. 定义你的 Newtype 结构体
pub struct PoolInstance(pub Arc<dyn Pool>);

// 4. 实现 Deref，目标直接是 dyn Pool (DST)
impl Deref for PoolInstance {
    type Target = dyn Pool;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

// 5. 一个接收 &dyn Pool 的普通函数
fn encode_swap_in(pool: &dyn Pool) {
    println!("Pool address: {}", pool.get_address());
}

fn main() {
    // 创建实例
    let inner_pool = UniswapV2Pool { address: 0x1234 };
    let pool_instance = PoolInstance(Arc::new(inner_pool));
    let pool_ref = &pool_instance;

    // --- 测试 1：方法调用 ---
    // ✅ 编译通过！可以直接调用
    println!("Method call: {}", pool_ref.get_address());

    // --- 测试 2：函数参数传递 ---
    // ❌ 编译报错！
    // error[E0277]: the trait bound `PoolInstance: Pool` is not satisfied
    // encode_swap_in(pool_ref);
}
