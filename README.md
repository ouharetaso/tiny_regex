# TinyRegex

TinyRegex is a toy regex engine for study written in Rust.

## Features
- concatenation
    - `ab`
- selection
    - `a|b`
- repetition
    - `a*`
- grouping
    - `(ab|c)*`
- character class
    - `[a-z]`
- negated character class
    - `[^a-z]`
- any character
    - `.`
- on-the-fly DFA
    - add `features = [ "on_the_fly" ]` to use it

## Usage
```rust
use tiny_regex::TinyRegex;

let re = TinyRegex::new("a(b|c)*d").unwrap();
assert!(re.is_match("abbbcd"));
assert!(!re.is_match("abbbce"));

let mat = re.find("wxyzabbbcdeffe").unwrap();
assert_eq!(mat.start(), 4);
assert_eq!(mat.end(), 10);
assert_eq!(mat.as_str(), "abbbcd");
assert_eq!(mat.range(), 4..10);
```

## License
This project is licensed under the [MIT License](LICENSE).

Copyright (c) 2024　ouharetaso