// iterators4.rs
//
// Execute `rustlings hint iterators4` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

pub fn factorial(num: u64) -> u64 {
    // Complete this function to return the factorial of num
    // Do not use:
    // - return
    // Try not to use:
    // - imperative style loops (for, while)
    // - additional variables
    // For an extra challenge, don't use:
    // - recursion
    // Execute `rustlings hint iterators4` for hints.
    // 用下面的代码能通过测试，但我觉得不对
    // if num<2 {
    //     1
    // } else if num==2 {
    //     2
    // } else {
    //     24
    // }
    // 下面这个是用chatgpt生成的
    (1..=num).product()
    // 在 Rust 中，Iterator::product 是一个用于计算迭代器元素乘积的方法。
    // 它可以应用于任何实现了 Iterator trait 的类型。
    // 当你有一个迭代器产生的元素序列，并且想要计算它们的乘积时，你可以使用 product 方法。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(1, factorial(0));
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}
