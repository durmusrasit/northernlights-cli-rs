pub fn test_command(_text: &str) -> &str {
    "test" as &str
}

pub fn alphabet_command() -> &'static str {
    let mut alphabet = String::from("");
    for c in b'a'..=b'z' {
        let character = c as char;
        alphabet.push_str(&character.to_string());
    }
    println!("{}", alphabet);
    ""
}

pub fn su_command(string: &str) -> &'static str {
    println!("Switch to the user {}", string);
    ""
}
