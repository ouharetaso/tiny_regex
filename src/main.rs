#[allow(dead_code)]

use std::io::{self, Read};
use tiny_regex::TinyRegex;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let args = std::env::args().collect::<Vec<String>>();
    let usage = format!("Usage: {} [regex_str]\nafter the 1st arguments are ignored", args.get(0).unwrap());

    let regex_str = args.get(1).ok_or_else(|| {eprintln!("{}", usage); "regex string is not provided"})?;
    let re = TinyRegex::new(regex_str).unwrap();

    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).or_else(|e| Err(e))?;

    for mat in re.find_all(&buffer) {
        println!("{}", mat.as_str());
    }

    Ok(())
}
