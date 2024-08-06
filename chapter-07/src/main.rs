struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // 定义了一个area方法
    fn area(&self) -> u32 {
        // &self == self: &Self，不可变引用
        self.width * self.height
    }

    fn add_width(&mut self) {
        // 可变引用
        self.width = self.width + 1
    }

    fn area2(self) -> u32 {
        self.width * self.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    // mut 会将整个结构体标记为可变的，通过user1.xxx读写字段
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    user1.active = false;
    println!(
        "Hello, world!, {}, {}, {}",
        user1.email, user1.username, user1.sign_in_count
    );

    let mut rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area() // 调用方法
    );

    rect1.add_width();
    println!("The width of the rectangle is {} pixels.", rect1.width);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area2() // area2用了self作为参数移交了rect1的所有权，下面会报错
    );

    // println!(
    //     "The width of the rectangle is {} pixels.",
    //     rect1.width // error
    // );
    let sq = Rectangle::square(3);

    println!(
        "The area of the rectangle is {} square pixels.",
        sq.area()
    );
}
