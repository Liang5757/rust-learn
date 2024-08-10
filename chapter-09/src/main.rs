// 包管理
mod apple;
mod fruit;

fn deliver_order() {
    println!("deliver_order");
}

mod back_of_house {
    pub fn fix_incorrect_order() {
        super::deliver_order();
    }
}

use crate::fruit::pear as localPear;

fn main() {
    apple::eat_apple();
    localPear::eat_pear();
    back_of_house::fix_incorrect_order();
}
