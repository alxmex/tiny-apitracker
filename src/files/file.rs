use std::fs::File;
use std::io::prelude::*;

pub fn write_file(response: &Vec<String>, endpoint: &str) -> Result<(), Box<dyn std::error::Error>>{
    //TODO: Fix the path to scale on any machine.
    let mut file = File::options().append(true).open("/home/anorak/.rcheck/responses.txt")?;
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
