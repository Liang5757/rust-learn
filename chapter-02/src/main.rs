use std::io; // 使用标准库中的 io 这个模块

fn main() {
    println!("Hello world!");
    println!("Plase input a number");
    let mut input = String::new(); // 创建了一个string，用于接受下面的输入
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input!");
    println!("Your raw input is: {:?}.", input);
    let number: i64 = input.trim().parse().expect("Input is not a number!");
    println!("Your input is: {:?}.", number); // 打印 parse 之后的 i64 数字
}
