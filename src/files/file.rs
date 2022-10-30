use std::fs;
use std::io::prelude::*;

use std::collections::HashMap;

pub fn write_file(endpoint: &HashMap<&str,Vec<String>>) -> Result<(), Box<dyn std::error::Error>>{
    //TODO: Fix the path to scale on any machine.
    let mut file = fs::File::options().append(true).open("response.txt")?;
    serde_json::to_writer(&file, endpoint)?;
    file.write_all(b"\n").expect("Cant write new line");
    Ok(())
}


pub fn read_file(){
    let mut contents = fs::read_to_string("response.txt").expect("Should have been able to read the file");
    let mut read_from_file: HashMap<&str, Vec<String>> = HashMap::new();
    let mut keys: Vec<String> = Vec::new();
    for row in contents.lines(){
        let make_split = row.split(":");
        let splitted: Vec<&str> = make_split.collect();


        for value in splitted[1].lines(){
            let value = value.replace("}", "");
            let do_split: Vec<&str> = value.split(",").collect();
            for v in do_split.iter(){
                let cleaned = v.replace('"', "").replace("[","").replace("]", "");
                keys.push(cleaned);
            };

        };


        //Cleaning URL 
        //let cleaned_url = splitted[0].replace("{", "").to_string();
        // FIX THIS BLACK MAGIC

        read_from_file.insert("Insert here", keys.to_owned());
    };



    //println!("{:?}", read_from_file);

    //let ep: Vec<EndPoint> = serde_json::from_str(&endpoints_file).unwrap();

}
