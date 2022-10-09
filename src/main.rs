//Network mod
mod network;
use crate::network::request::*;

#[tokio::main]
async fn main() {
    match send_request("https://jsonplaceholder.typicode.com/posts").await {
        Ok(_res) => println!("Success"),
        Err(err) => println!("{:?}", err),
    };
}
