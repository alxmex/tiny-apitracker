//Network mod
mod network;
mod files;

use std::fs;
use std::collections::HashMap;

use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct EndPoint {
    url: String,
    auth: String
}

#[tokio::main]
async fn main() {
    let endpoints_file = fs::read_to_string("endpoints.json").expect("Unable to read endpoints.");
    let ep: Vec<EndPoint> = serde_json::from_str(&endpoints_file).unwrap();

    for json_object in ep.iter(){
        let response: Vec<String> = network::request::send_request(&json_object.url).await.expect("Endpoint cant be called");
        
        //Bad for the memoery but could remove K.
        let mut response_hashmap: HashMap<&str, Vec<String>> = HashMap::new();
        response_hashmap.insert(&json_object.url, response);

        ////Store response 
        let write = files::file::write_file(&response_hashmap);
    };
}
