// 所有权和借用
fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    let s3 = String::from("hello"); // s 进入作用域

    let _s4 = takes_ownership(s3); // s 的值移动到函数里 ...
                                   // ... 所以到这里不再有效

    let x = 5; // x 进入作用域

    makes_copy(x); // x 应该移动函数里，
                   // 但 i32 是 Copy 的，所以在后面可继续使用 x

    let s5 = String::from("hello");

    let (s2, len) = calculate_length(s5); // 用元组返回所有权，以便后续可以接着使用s1

    println!("The length of '{}' is {}.", s2, len);

    let s6 = String::from("hello");
    let _len = calculate_length_v2(&s6);

    let mut s7 = String::from("hello");
    change(&mut s7);
    println!("The updated string is: {}", s7);
} // 这里, x 先移出了作用域，然后是 s。但因为 s 的值已被移走，
  // 所以不会有特殊操作

fn takes_ownership(some_string: String) -> String {
    // some_string 进入作用域
    println!("{}", some_string);
    String::from("world") // 返回值会移交所有权到外层
} // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放

fn makes_copy(some_integer: i32) {
    // some_integer 进入作用域
    println!("{}", some_integer);
} // 这里，some_integer 移出作用域。不会有特殊操作

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn calculate_length_v2(s: &String) {
    // 传递不可变引用
    let _ = s.len();
}

fn change(s: &mut String) {
    // 传递可变引用
    s.push_str(", world!");
}
