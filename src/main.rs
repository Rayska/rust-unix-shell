use std::io::{self, Write};
use std::path;

mod shell;
use shell::commands::*;
use shell::structures::*;


fn take_input() -> Vec<String> {
	let mut user_input: String = String::new();
	io::stdin().read_line(&mut user_input).expect("Error reading input");

	let trimmed = user_input.trim();
	if trimmed.is_empty() {
		return Vec::new();
	}

	return trimmed.split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>();
}

fn main() {
	let mut command_args: Vec<String>;
	let mut current_dir = path::PathBuf::from(".");
	let home_dir = path::PathBuf::from(".");

	loop {
		print!("$ ");

		if let Err(error) = io::stdout().flush() {
			eprintln!("Flushing stdout() failed: {error}");
			break;
		}

		command_args = take_input();

		if command_args.is_empty() {
			continue;
		}

		let action: CmdResult = match command_args.first().unwrap().trim() {
			"ls" => ls(&current_dir, command_args),
			"cd" => cd(&home_dir, &mut current_dir, command_args),
			"pwd" => pwd(&mut current_dir),
			"exit" => CmdResult::Exit,
			_ => CmdResult::Unknown,
		};

		match action {
			CmdResult::Ok => {},
			CmdResult::Exit => {
				println!("exiting...");
				break;
			},
			CmdResult::Unknown => println!("Unknown command, see \"help\""),
		}

	}
}
