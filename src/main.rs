//Network mod
mod network;
use crate::network::request::*;

#[tokio::main]
async fn main() {
    let response: Vec<String> = send_request("https://jsonplaceholder.typicode.com/posts").await.expect("Endpoint cant be called");
    println!("Response: {:?} acquired.", response);
}
