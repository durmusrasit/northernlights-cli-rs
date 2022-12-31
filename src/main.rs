use std::io::{self, Write};

mod commands;

fn read_line(line: &mut String) {
    io::stdout().flush().expect("Failed to flush");
    io::stdin().read_line(line).expect("Failed to read line");
}

fn main() {
    println!("INIT");

    'promptLoop: loop {
        print!("< ");
        let mut line: String = String::new();

        read_line(&mut line);

        let strline: &str = line.trim();

        let params = strline.split_whitespace();

        let mut param_array: [&str; 2] = ["", ""];
        let mut param_array_index = 0;

        for param in params {
            param_array[param_array_index] = param;
            param_array_index = param_array_index + 1;
        }

        let command = param_array[0];

        let response = match command {
            "test" => commands::test_command("test"),
            "alphabet" => commands::alphabet_command(),
            "su" => commands::su_command(param_array[1]),
            _ => "COMMAND NOT FOUND.",
        };

        if command == "exit" {
            break 'promptLoop;
        }

        println!("> {}", response);
    }
}
