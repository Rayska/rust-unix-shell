use std::fs;
use std::io::{self, Write};
use std::path;

use crate::shell::structures::*;

pub fn write_usage() {
	println!("write	-	Write user input to a pre-existing or new file. Pre-existing file's contents are deleted.\n");

	println!("Enter input line by line to a file indicated by <name> and stop by giving \"q\" as input");
	println!("Usage: write <name>");
}

fn take_input() -> io::Result<String> {
	let mut user_input = String::new();
	io::stdin().read_line(&mut user_input)?;
	Ok(user_input)
}


pub fn write(cwd: &path::PathBuf, args: Vec<String>) -> CmdResult {
	let target_string: String;

	if args.len() == 2 {
		target_string = args[1].clone();
	} else {
		if args.len() == 1 {
			eprintln!("write: No target provided");
		} else if args.len() > 2 {
			eprintln!("write: Too many arguments")
		}
		write_usage();
		return CmdResult::Ok;
	}

	let p = path::Path::new(&target_string);
	let candidate_pathbuf = if p.is_absolute() {
		p.to_path_buf()
	} else {
		cwd.join(p)
	};

	if candidate_pathbuf.exists() && candidate_pathbuf.is_dir() {
		let name = candidate_pathbuf
			.file_name()
			.map(|s| s.to_string_lossy().into_owned())
			.unwrap_or_else(|| candidate_pathbuf.display().to_string());
		eprintln!("write: A directory by the name \"{}\" already exists", name);
		return CmdResult::Ok;
	}

	let mut file = match fs::File::create(&candidate_pathbuf) {
		Ok(f) => f,
		Err(e) => {
			eprintln!("write: could not create file {}: {}", candidate_pathbuf.display(), e);
			return CmdResult::Ok;
		}
	};

	loop {
		match take_input() {
			Ok(line) => {
				if line.trim() == "q" {
					break;
				}

				if let Err(e) = file.write_all(line.as_bytes()) {
					eprintln!("write: failed writing to file: {}", e);
					break;
				}
			}
			Err(e) => {
				eprintln!("write: failed reading input: {}", e);
				break;
			}
		}
	}

	CmdResult::Ok
}