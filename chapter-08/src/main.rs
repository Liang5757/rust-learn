enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn send(&self) {
        // println!("Message: {:?}", self);
        println!("Message send");
    }
}

fn main() {
    let mes_quit = Message::Quit;
    let mes_write = Message::Write(String::from("hello"));
    mes_write.send();

    if let Message::Quit = mes_quit {
        println!("match quit");
    } else {
        println!("match default");
    }

    // 下面会报错
    // if mes_quit == Message::Quit {
    //     println!("match quit");
    // }
}
