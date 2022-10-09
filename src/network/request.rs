use regex::Regex;

pub async fn send_request(url: &str) -> Result<(), Box<dyn std::error::Error>> {
let resp = reqwest::get(url)
        .await?
        .text()
        .await?;
    let keys = fetch_keys(&resp) 

    Ok(())

}

pub fn fetch_keys(text: &str) -> Vec<String>{
    let mut stack = Vec::new();
    let splitted: Vec<&str> = text.split("{").collect();
    let splitted2: Vec<&str> = splitted[splitted.len()-2].split(":").collect();
    let re = Regex::new(r"\b(\w+)\W*$").expect("No matches for regex");
    for row in splitted2{
        let regex_match = re.find(row).unwrap();
        let replacing_non_chars = regex_match.as_str().replace('"', "").replace("}", "").replace(",","").replace('\n', "").replace(" ", "");
        stack.push(replacing_non_chars);
    }
    stack
}
