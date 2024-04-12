// clippy3.rs
// 
// Here's a couple more easy Clippy fixes, so you can see its utility.
//
// Execute `rustlings hint clippy3` or use the `hint` watch subcommand for a hint.



#[allow(unused_variables, unused_assignments)]
fn main() {
    // 定义一个Option变量，内容为()
    let my_option: Option<()> = None;
    if let Some(value) = my_option {
        println!("{:?}", value);
    }

    // 定义一个数组
    let my_arr = &[
        -1, -2, -3,
        -4, -5, -6
    ];
    // 打印数组
    println!("My array! Here it is: {:?}", my_arr);

    let mut my_empty_vec = vec![1, 2, 3, 4, 5];
    my_empty_vec.clear();
    // 打印切片
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    // 定义两个变量
    let mut value_a = 45;
    let mut value_b = 66;
    // Let's swap these two!
    // value_a = value_b;
    // value_b = value_a;
    std::mem::swap(&mut value_a, &mut value_b);
    // 打印交换后的变量
    println!("value a: {}; value b: {}", value_a, value_b);
}
