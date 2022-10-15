use std::fs;
use std::fs::File;
use std::io::prelude::*;

pub fn write_file(response: &Vec<String>, endpoint: &str) -> Result<(), Box<dyn std::error::Error>>{
    let mut file = File::open("response.txt")?;
    Ok(())
}

pub fn strip_dot(file: String) -> String{
    let modified_string = &file.replace(".","");
    modified_string.to_string()
    
}
