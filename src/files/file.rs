use std::fs::File;
use std::io::prelude::*;

pub fn create_file(file: &Vec<String>, filename: &str) -> Result<(), Box<dyn std::error::Error>>{
    let path = format!("src/files/responses/{}.txt", &filename);
    dbg!(path);
    //let mut file = File::create("src/files/responses/{}.txt")?;
    //let mut file = File::create(path)?;
    //file.write_all(b"Hello, world!")?;
    Ok(())
}

pub fn strip_dot(file: Vec<String>) -> String{


}
