use std::{fs, path};
use crate::shell::structures::*;

pub fn pwd_usage() {
	println!("pwd	-	Print absolute path of current/working directory\n");

	println!("Usage: pwd");
}

pub fn pwd(pathbuf: &mut path::PathBuf, args: Vec<String>) -> CmdResult {
	for a in args {
		if a == "-h" {
			pwd_usage();
			return CmdResult::Ok;
		}
	}

	let canon = match fs::canonicalize(pathbuf.as_path()) {
		Ok(c) => c,
		Err(e) => {
			eprintln!("pwd: {}", e);
			return CmdResult::Ok;
		}
	};


	*pathbuf = canon;

	let s = pathbuf.to_string_lossy();
	println!("{}", display_arg(s.as_ref()));

	return CmdResult::Ok;
}