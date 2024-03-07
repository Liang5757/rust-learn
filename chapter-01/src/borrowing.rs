fn main() {
    let x = String::from("Hi");
    let y = &x; // 不可变借用
    println!("{x}");
    println!("{y}");

    let mut x = String::from("Hi");
    let y = &mut x; // 可变借用
    y.push_str(" rust");
    let z = &x;
    println!("{z}");
    println!("{y}");
}