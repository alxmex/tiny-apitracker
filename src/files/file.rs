use std::fs::File;
use std::io::prelude::*;

pub fn write_file(response: &Vec<String>, endpoint: &str) -> Result<(), Box<dyn std::error::Error>>{
    let mut file = File::options().append(true).open("response.txt")?;
    let string_to_write = format!("\n{}", endpoint);
    file.write_all(string_to_write.as_bytes())?;
    for key in response{
        file.write_all(format!(" | {}",key).as_bytes())?;
    }
    Ok(())
}

pub fn strip_dot(file: String) -> String{
    let modified_string = &file.replace(".","");
    modified_string.to_string()
    
}
