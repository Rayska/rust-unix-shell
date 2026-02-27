use std::{fs, path};
use crate::shell::structures::*;

pub fn pwd_usage() {
	println!("pwd	-	Print absolute path of current/working directory\n");

	println!("Usage: pwd");
}

pub fn pwd(pathbuf: &path::Path, args: Vec<String>) -> CmdResult {
	for a in args {
		if a == "-h" {
			pwd_usage();
			return CmdResult::Ok;
		}
	}

	let canon = match fs::canonicalize(pathbuf) {
		Ok(c) => c,
		Err(e) => {
			eprintln!("pwd: {}", e);
			return CmdResult::Ok;
		}
	};

	let s = canon.to_string_lossy();
	println!("{}", display_arg(s.as_ref()));

	CmdResult::Ok
}