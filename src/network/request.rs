use regex::Regex;

pub async fn send_request(url: &str) -> Result<(), Box<dyn std::error::Error>> {
let resp = reqwest::get(url)
        .await?
        .text()
        .await?;

    let _keys = fetch_keys(&resp);


    Ok(())

}


pub fn fetch_keys(text: &str){
    let splitted: Vec<&str> = text.split("{").collect();
    let splitted2: Vec<&str> = splitted[splitted.len()-2].split(":").collect();
    let re = Regex::new(r"\b(\w+)\W*$").expect("No matches for regex");
    for row in splitted2{
        let hehe = re.find(row).unwrap();
        let luls = hehe.as_str().replace('"', "").replace("}", "").replace(",","").replace('\n', "").replace(" ", "");
        dbg!(luls);


    }
}
