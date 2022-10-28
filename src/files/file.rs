use std::fs::File;
use std::io::prelude::*;

use std::collections::HashMap;

pub fn write_file(endpoint: &HashMap<&str,Vec<String>>) -> Result<(), Box<dyn std::error::Error>>{
    //TODO: Fix the path to scale on any machine.
    let mut file = File::options().append(true).open("response.txt")?;
    serde_json::to_writer(&file, endpoint)?;
    file.write_all(b"\n").expect("Cant write new line");
    Ok(())
}



