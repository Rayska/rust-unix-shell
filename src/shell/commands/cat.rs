use std::fs;
use std::path;

use crate::shell::structures::*;

pub fn cat_usage() {
	println!("cat	-	concatenate files and print in stdout.");

	println!("Usage: cat <file> [<file>...]");
	println!("Concatenate files and print to stdout.");
}

pub fn cat(cwd: &path::Path, args: Vec<String>) -> CmdResult {
	if args.len() < 2 {
		eprintln!("cat: No target provided");
		cat_usage();
		return CmdResult::Ok;
	}

	for arg in &args[1..] {
		if arg == "-h" {
			cat_usage();
			return CmdResult::Ok;
		}
	}

	for target in &args[1..] {
		let p = path::Path::new(target);
		let candidate = if p.is_absolute() {
			p.to_path_buf()
		} else {
			cwd.join(p)
		};

		if candidate.is_dir() {
			let name = candidate
				.file_name()
				.map(|s| s.to_string_lossy().into_owned())
				.unwrap_or_else(|| candidate.display().to_string());
			eprintln!("cat: {}: is a directory", display_arg(&name));
			continue;
		}

		match fs::read_to_string(&candidate) {
			Ok(contents) => print!("{contents}"),
			Err(e) => eprintln!("cat: {}: {}", candidate.display(), e),
		}
	}

	CmdResult::Ok
}
