// 定义一个简单的声明宏
macro_rules! greet {
    // 定义宏的模式和替换代码
    () => {
        println!("Hello, world!");
    };
}

fn main() {
    // 调用声明宏
    greet!();
}
