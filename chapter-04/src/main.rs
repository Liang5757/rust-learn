// 类型
use std::io;

fn main() {
    println!("Please input a number:");
    let mut input = String::new();

    io:: stdin()
        .read_line(&mut input) // 将标准输入赋值给input
        .expect("Failed to read input!");
    println!("Your raw input is: {:?}.", input);

    let number: i64 = input.trim().parse().expect("Input is not a number!");
    // 这里的 number 是不可变的，类型是 i64，在这里指定的原因是为了告诉编译器parse 需要解析成什么类型，因为 String 可以 parse 成多个类型，所以在这里 Rust 编译器无法自行推断出来
    println!("Your input is: {}.", number);

    let a = (1, 2); // 元组
    println!("a: {:?}", a);
    let (a, b) = a; // 解构
    println!("a: {:?}, b: {:}", a, b);
}
