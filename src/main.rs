//Network mod
mod network;
mod files;

use std::fs;
use std::collections::HashMap;

use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct EndPoint {
    url: String,
}

#[tokio::main]
async fn main() {

    let endpoints_file = fs::read_to_string("endpoints.json").expect("Unable to read endpoints.");
    let ep: Vec<EndPoint> = serde_json::from_str(&endpoints_file).unwrap();
    let old_responses = files::file::read_file();

    for json_object in ep.iter(){
        let response: Vec<String> = network::request::send_request(&json_object.url).await.expect("Do you have internet connection?");
        
        //Bad for the memoery but could remove K.
        let converted_url = &json_object.url.to_string().replace("https://", "");
        let mut response_hashmap: HashMap<&str, &Vec<String>> = HashMap::new();
       
        response_hashmap.insert(&converted_url, &response);



        let old = match old_responses.get(converted_url){
            Some(x) => x.to_owned(),
            None => files::file::if_is_none(&response_hashmap),
        };


        //New keys from the response.
        println!("[{}]", converted_url);
        for value in &response{
            match old.contains(&value){
                true => println!("{} | OK ", value),
                false => println!("{} | REMOVED", value),
            };
        };
        for value in &old{
            match response.contains(&value){
                true => continue,
                false => println!("{} | NEW", value),
            };
        };

        println!("----------------------------------");

        ////Store response 
        match files::file::write_file(&response_hashmap) {
            Ok(_s) => continue,
            Err(_) => println!("couldnt not write to file"),
        };

    };
}
