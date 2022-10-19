//Network mod
mod network;
mod files;

use std::fs;

use serde_derive::Deserialize;

#[derive(Deserialize)]
#[derive(Debug)]
struct EndPoint {
    url: String,
    auth: String
}

#[tokio::main]
async fn main() {
    let endpoints_file = fs::read_to_string("endpoints.json").expect("HEH");
    let ep: Vec<EndPoint> = serde_json::from_str(&endpoints_file).unwrap();

    for json_object in ep.iter(){
        let response: Vec<String> = network::request::send_request(&json_object.url).await.expect("Endpoint cant be called");

        ////Store response 
        let end_point = files::file::strip_dot(json_object.url.to_string());
        let write = files::file::write_file(&response, &end_point);
        match write {
            Ok(_) => println!("Write successful."),
            Err(err) => println!("{:?}", err),
        }
        };
}
