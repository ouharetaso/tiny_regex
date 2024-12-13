use std::io::{self, Read};
use tiny_regex::TinyRegex;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let args = std::env::args().collect::<Vec<String>>();
 
    let usage = format!("\
substitute all substrings to given string matching the given regex by searching each line from stdin

Usage: {} [regex_str] [replace_str]
[regex_str]  : regex string to search
[replace_str]: string to substitute

arguments after the first two are ignored", args.get(0).unwrap());

    let regex_str = args.get(1).ok_or_else(|| {eprintln!("{}", usage); "regex string is not provided"})?;
    let replace_str = args.get(2).ok_or_else(|| {eprintln!("{}", usage); "replace string is not provided"})?;
    let re = TinyRegex::new(regex_str).unwrap();

    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    for line in buffer.lines() {
        let matches = re.find_all(line);

        for (i, c) in line.char_indices() {
            if let Some((start, end)) = matches.clone().map(|mat| (mat.start(), mat.end())).find(|(start, end)| *start <= i && i < *end) {
                if i == start {
                    print!("{}", replace_str);
                }
                if i == end - 1 {
                    continue;
                }
            }
            else {
                print!("{}", c);
            }
        }
        println!();
    }


    Ok(())
}
