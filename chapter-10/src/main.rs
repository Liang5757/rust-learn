// 合集

fn main() {
    // 三种初始化方式
    let _: Vec<i32> = Vec::new();
    let _: Vec<i32> = Vec::with_capacity(16); // 提前分配好 16 个元素大小的内存
    let _ = vec!["hello", "world"]; // vec!宏来创建数组

    // 更新
    let mut v = Vec::new();
    v.push(1);
    v.push(2);

    // 读取
    println!("{}", v[1]); // 输出 2
                          // println!("{}", v[2]); // 这里程序会直接退出
    println!("{:?}", v.get(1)); // 输出 Some(2)
    println!("{:?}", v.get(2)); // 输出 None

    // 删除
    let mut v2 = vec!["Hello", "Rust"];
    println!("{:?}", v2.pop()); // 输出Rust

    let mut v = vec!["Hello", "Rust", "~"];
    v.remove(1);
    println!("{:?}", v); // 输出 ["Hello", "~"]

    // 遍历
    // let v = vec!["Hello", "World"];
    // for elem in v {
    //     println!("{elem}");
    // }

    // println!("{:?}", v); // 这行将会报错，因为我们在第三行的 for 已经将 v 给消耗掉了

    let v = vec!["Hello", "World"];
    for elem in &v {
        println!("{elem}");
    }

    println!("{:?}", v);

    let mut v = vec!["Hello", "World"];
    for elem in &mut v {
        *elem = "elem"
    }

    println!("{:?}", v); // 这行会输出 ["elem", "elem"]
}
