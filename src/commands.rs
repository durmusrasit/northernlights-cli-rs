pub fn test_command(_text: &str) -> &str {
    "test" as &str
}

pub fn alphabet_command() -> &'static str {
    "abcdef...."
}

pub fn su_command(string: &str) -> &'static str {
    println!("Switch to the user {}", string);
    ""
}
