fn main() {
    let x = String::from("hi"); // x拥有值的所有权，x出作用域后，值就会被释放
    let y = x;
    println!("{}", x);
}