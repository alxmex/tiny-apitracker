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

    let old_responses = files::file::read_file();

    for json_object in ep.iter(){
        let response: Vec<String> = network::request::send_request(&json_object.url).await.expect("Do you have internet connection?");
        
        //Bad for the memoery but could remove K.
        let mut response_hashmap: HashMap<&str, Vec<String>> = HashMap::new();
        let converted_url = &json_object.url.to_string().replace("https://", "");

        let old = match old_responses.get(converted_url){
            Some(x) => x,
            None => continue,
        };

        //New keys from the response.
        for value in &response{
            match old.contains(&value){
                true => println!("{}, exists", value),
                false => println!("{} dont exists", value),
            };
        };



        response_hashmap.insert(&converted_url, response);

        ////Store response 
        let write = files::file::write_file(&response_hashmap);
    };



    
}
