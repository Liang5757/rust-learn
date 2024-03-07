use std::error::Error;
use clap::Parser; // 命令行参数解析库
use reqwest::blocking::Client; // HTTP 客户端库
use reqwest::header::HeaderMap;

// 功能：执行http请求，打印相关信息
// 终端执行：cargo run -- --url https://juejin.cn/

#[derive(Parser)] // 宏
#[command(
    author,
    version,
    about = "Sends HTTP requests and print details information"
)]

struct Cli {
    #[arg(short, long, help = "Target URL", required = true)]
    url: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let response = send_request(&cli.url)?; // ? 是 Rust 中的错误传播语法糖

    print_response_details(response)?;

    Ok(())
}

// 发起一个 HTTP 请求
fn send_request(url: &str) -> Result<reqwest::blocking::Response, Box<dyn Error>> {
    let client = Client::builder().build()?;
    let response = client.get(url).send()?;
    Ok(response)
}

// 打印出 HTTP 相应的详细信息
fn print_response_details(response: reqwest::blocking::Response) -> Result<(), Box<dyn Error>> {
    println!("Status: {}", response.status());
    println!("Headers:");
    print_headers(response.headers());

    let body = response.text()?;
    println!("Body:\n{}", body);

    Ok(())
}

// 打印出 HTTP 相应头
fn print_headers(headers: &HeaderMap) {
    for (key, value) in headers.iter() {
        println!("{}: {}", key, value.to_str().unwrap_or("[unprintable]"));
    }
}