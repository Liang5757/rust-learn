// 函数与控制流
use rand::Rng;
fn number(num: i64) -> i64 {
    num + 1
}

fn pass_exam(score: u32) -> bool {
    if score < 60 {
        return false;
    }
    true
}

// fn judge() -> bool {
//     if 2 { // error: must bool
//         return true;
//     }
//     false
// }

fn main() {
    // 此为表达式，返回 x + 1`
    let y = {
        let x = 3;
        x + 1
    };
    println!("y: {:?}", y);
    println!("num: {:?}", number(5));
    println!("pass exam: {:?}", pass_exam(60));
    // println!(judge());

    let score = if rand::thread_rng().gen::<f64>() > 0.5 { 60 } else { -1 };
    println!("{}", score);
}
