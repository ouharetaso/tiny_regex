#[allow(dead_code)]

use tiny_regex::TinyRegex;

fn main() -> Result<(), String>{
    let regex_str = "[a-zA-Z0-9][a-zA-Z0-9]*";
    let re = TinyRegex::new(regex_str).unwrap();

    let print_is_matched = |s: &str| {
        if re.is_match(s) {
            println!("{} has a match regex \"{}\"", s, regex_str);
        }
        else {
            println!("{} does not have a match regex \"{}\"", s, regex_str);
        }
    };

    print_is_matched("hello");
    print_is_matched("hello123");
    print_is_matched("123");
    print_is_matched("123hello");
    print_is_matched("hello123world");
    print_is_matched("野獣先輩");

    Ok(())
}
