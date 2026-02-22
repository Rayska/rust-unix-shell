use std::io::{self, Write};
use std::path;
use std::env;

mod shell;
use shell::commands::*;
use shell::structures::*;

fn tokenize_input(line: &str) -> Result<Vec<String>, String> {
	let mut tokens: Vec<String> = Vec::new();
	let mut current = String::new();
	let mut in_single = false;
	let mut in_double = false;
	let mut escape = false;

	for ch in line.chars() {
		if escape {
			current.push(ch);
			escape = false;
			continue;
		}

		if ch == '\\' && !in_single {
			escape = true;
			continue;
		}

		if in_single {
			if ch == '\'' {
				in_single = false;
			} else {
				current.push(ch);
			}
			continue;
		}

		if in_double {
			if ch == '"' {
				in_double = false;
			} else {
				current.push(ch);
			}
			continue;
		}

		if ch == '\'' {
			in_single = true;
		} else if ch == '"' {
			in_double = true;
		} else if ch.is_whitespace() {
			if !current.is_empty() {
				tokens.push(current.clone());
				current.clear();
			}
		} else {
			current.push(ch);
		}
	}

	if escape {
		return Err("Trailing backslash".to_string());
	}
	if in_single || in_double {
		return Err("Unterminated quote".to_string());
	}
	if !current.is_empty() {
		tokens.push(current);
	}

	Ok(tokens)
}

fn take_input() -> Vec<String> {
	let mut user_input: String = String::new();
	let bytes_read = io::stdin().read_line(&mut user_input).expect("Error reading input");

	if bytes_read == 0 {
		println!();
		return vec!["exit".to_string()];
	}

	let trimmed = user_input.trim();
	if trimmed.is_empty() {
		return Vec::new();
	}

	match tokenize_input(trimmed) {
		Ok(tokens) => tokens,
		Err(e) => {
			eprintln!("Parse error: {e}");
			Vec::new()
		}
	}
}

fn help(args: Vec<String>) -> CmdResult {
	if args.len() > 1 {
		match args[1].as_str() {
			"ls" => {
				ls_usage();
				return CmdResult::Ok;
			},
			"cd" => {
				cd_usage();
				return CmdResult::Ok;
			},
			"pwd" => {
				pwd_usage();
				return CmdResult::Ok;
			},
			"write" => {
				write_usage();
				return CmdResult::Ok;
			},
			"rm" => {
				rm_usage();
				return CmdResult::Ok;
			}
			"cat" => {
				cat_usage();
				return CmdResult::Ok;
			}
			_ => {},
		}
	}
	println!("Rust Programming Shell v1\n");

	println!("List of supported commands:");
	println!("Legend: <> = replace, [] = optional");
	println!("ls [-l] [-a] [<path>]: 	Shows the contents of the given or working directory with optional flags. See ls -h for more.");
	println!("cd [<path>]:		Change directory to the given path or home directory. See cd -h for more.");
	println!("pwd:				Show the aboslute path to working directory");
	println!("write <name>:		Write text line by line into a file named <name>. See write -h for more.");
	println!("rm [-r] <name>:		Remove a file or directory (-r). See rm -h for more.");
	println!("cat <file> [<file>...]:	Concatenate and print file(s). See cat -h for more.");
	println!("exit:				Exit program.");
	println!("help:				This print.\n");

	return CmdResult::Ok;
}

fn main() {
	let mut command_args: Vec<String>;
	let mut current_dir = path::PathBuf::from(".");
	let home_dir = env::var_os("HOME")
		.map(path::PathBuf::from)
		.or_else(|| env::current_dir().ok())
		.unwrap_or_else(|| path::PathBuf::from("."));

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
			"pwd" => pwd(&mut current_dir, command_args),
			"write" => write(&current_dir, command_args),
			"rm" => rm(&current_dir, command_args),
			"cat" => cat(&current_dir, command_args),
			"exit" => CmdResult::Exit,
			"help" => help(command_args),
			_ => CmdResult::Unknown,
		};

		match action {
			CmdResult::Ok => {},
			CmdResult::Exit => {
				break;
			},
			CmdResult::Unknown => println!("Unknown command, see \"help\""),
		}

	}
}
