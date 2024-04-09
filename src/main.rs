use std::env;
use std::io;
use std::process;

fn contains_digits(s: &str) -> bool {
    s.chars().any(|c| c.is_digit(10))
}

fn is_alphanumeric(s: &str) -> bool {
    s.chars().any(|c| c.is_alphanumeric())
}

fn match_pattern(input_line: &str, pattern: &str) -> bool {
    if pattern.chars().count() == 1 {
        return input_line.contains(pattern);
    }

    if pattern == "\\d" {
        return contains_digits(input_line);
    }

    if pattern == "\\w" {
        return is_alphanumeric(input_line);
    }

    // check for positive character group
    if pattern.starts_with("[") && pattern.ends_with("]") {
        // get the string between [ and ]
        let chars = pattern
            .chars()
            .skip(1)
            .take_while(|c| *c != ']')
            .collect::<String>();
        for char in chars.chars() {
            if input_line.contains(char) {
                return true;
            }
        }
        return false;
    }

    panic!("Unhandled pattern: {}", pattern)
}

// Usage: echo <input_text> | your_grep.sh -E <pattern>
fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    if env::args().nth(1).unwrap() != "-E" {
        println!("Expected first argument to be '-E'");
        process::exit(1);
    }

    let pattern = env::args().nth(2).unwrap();
    let mut input_line = String::new();

    io::stdin().read_line(&mut input_line).unwrap();

    if match_pattern(&input_line, &pattern) {
        process::exit(0)
    } else {
        process::exit(1)
    }
}
