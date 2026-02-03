use std::io::{self, Write};
use std::fs;

enum CmdResult {
	Ok(String),
	Exit,
	Unknown,
}

fn take_input() -> String {
	let mut user_input: String = String::new();
	io::stdin().read_line(&mut user_input).expect("Error reading input");
	return user_input;
}

// TODO: implement options like -l and -a
fn ls() -> CmdResult {
	let path: String = String::from(".");
	match fs::read_dir(".") {
		Err(error) => return CmdResult::Ok(format!("Reading dir with path {} failed with {}", path, error)),
		Ok(entries) => {
			let mut names: Vec<String> = Vec::new();

			for entry_res in entries {
				if let Ok(entry) = entry_res {
					let filename: String = entry.file_name().to_string_lossy().into_owned();
					if !filename.starts_with(".") {
						names.push(filename);
					}
				}
			}

			names.sort();
			return CmdResult::Ok(names.join("\t"));
		},
	}
}

fn main() {
	let mut input: String;

	loop {
		print!("$ ");

		if let Err(error) = io::stdout().flush() {
			eprintln!("Flushing stdout() failed: {error}");
			break;
		}

		input = take_input();
		// TODO: implement parsing into n-sized vector/iterator for command-option separation
		let action: CmdResult = match input.trim() {
			"ls" => ls(),
			"exit" => CmdResult::Exit,
			_ => CmdResult::Unknown,
		};

		match action {
			CmdResult::Ok(ref msg) => println!("{msg}"),
			CmdResult::Exit => {
				println!("exiting...");
				break;
			},
			CmdResult::Unknown => println!("Unknown command, see \"help\""),
		}

	}
}
