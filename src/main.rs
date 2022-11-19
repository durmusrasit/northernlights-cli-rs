use std::io;

fn main() {
    println!("INIT");
    loop {
        let mut line: String = String::new();

        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        let strline: &str = line.as_str().trim();

        let params = strline.split_whitespace();

        let mut param_array: [&str; 2] = ["", ""];
        let mut param_array_index = 0;

        for param in params {
            param_array[param_array_index] = param;
            param_array_index = param_array_index + 1;
        }

        let command = param_array[0];

        let response = match command {
            "test" => test_command("test"),
            "alphabet" => alphabet_command(),
            "su" => su_command(param_array[1]),
            _ => "COMMAND NOT FOUND.",
        };

        println!("> {}", response);
    }
}

fn test_command(_text: &str) -> &str {
    "test" as &str
}

fn alphabet_command() -> &'static str {
    "abcdef...."
}

fn su_command(string: &str) -> &'static str {
    println!("Switch to the user {}", string);
    ""
}
