use std::io;

fn main() {
    loop {
        let mut line: String = String::new();

        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        let strline: &str = line.as_str().trim();

        let response = match strline {
            "test" => test("test"),
            "alphabet" => abc(),
            _ => "COMMAND NOT FOUND.",
        };

        println!("> {}", response);
    }
}

fn test(_text: &str) -> &str {
    "test" as &str
}

fn abc() -> &'static str {
    "abcdef...."
}
