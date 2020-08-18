use parser::Parser;
use std::thread;
use tokio::time::{delay_for, Duration};

#[tokio::main]
async fn main() {
    let mut prs = Parser::new();
    println!("start");
    prs.start();
    println!("then");

    for i in 0..20 {
        delay_for(Duration::from_secs(1)).await;
        prs.parse(format!("test {}", i));
    }

    loop {}
}
