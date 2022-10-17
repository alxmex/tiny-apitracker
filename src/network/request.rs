use regex::Regex;
use std::collections::HashMap;


pub async fn send_request(url: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let resp = client
        .get(url)
        .header("john", "doe")
        .send()
        .await?
        .text()
        .await?;
        let keys = fetch_keys(&resp);
        Ok(keys)
}

pub fn fetch_keys(text: &str) -> Vec<String>{
    let mut stack = Vec::new();
    let splitted: Vec<&str> = text.split("{").collect();
    let splitted2: Vec<&str> = splitted[splitted.len()-2].split(":").collect();
    let re = Regex::new(r"\b(\w+)\W*$").unwrap();
    for row in splitted2{
        let regex_match = re.find(row).expect("No matches found with regex string");
        let replacing_non_chars = regex_match.as_str().replace('"', "").replace("}", "").replace(",","").replace('\n', "").replace(" ", "");
        stack.push(replacing_non_chars);
    }
    stack
}
