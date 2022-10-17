//Network mod
mod network;
mod files;

use std::fs;

use serde_derive::Deserialize;

#[derive(Deserialize)]
struct EndPoint {
    url: String,
    auth: String
}

#[tokio::main]
async fn main() {
    let endpoints_file = fs::read_to_string("endpoints.json").expect("HEH");
    let ep: EndPoint = serde_json::from_str(&endpoints_file).unwrap();

    let end_point = ep.url.to_string();
    let response: Vec<String> = network::request::send_request(&end_point).await.expect("Endpoint cant be called");

    ////Store response 
    let end_point = files::file::strip_dot(end_point);
    let write = files::file::write_file(&response, &end_point);
    match write {
        Ok(_) => println!("Write successful."),
        Err(err) => println!("{:?}", err),
    }
}
