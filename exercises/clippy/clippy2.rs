// clippy2.rs
// 
// Execute `rustlings hint clippy2` or use the `hint` watch subcommand for a
// hint.



fn main() {
    // 设置一个可变变量res，初始值为42
    let mut res = 42;
    // 设置一个Some类型的变量option，值为12
    let option = Some(12);
    // 遍历option中的值
    if let Some(x) = option {
        // 将x的值加到res中
        res += x;
    }
    // 打印res的值
    println!("{}", res);
}
