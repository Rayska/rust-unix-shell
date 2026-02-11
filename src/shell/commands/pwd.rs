use std::{fs, path};
use crate::shell::structures::*;

pub fn pwd_usage() {
	println!("pwd	-	Print absolute path of current/working directory\n");

	println!("Usage: pwd");
}

pub fn pwd(pathbuf: &mut path::PathBuf, args: Vec<String>) -> CmdResult {
	let canon = match fs::canonicalize(pathbuf.as_path()) {
		Ok(c) => c,
		Err(e) => {
			eprintln!("pwd: {}", e);
			return CmdResult::Ok;
		}
	};

	for a in args {
		if a == "-h" {
			pwd_usage();
			return CmdResult::Ok;
		}
	}


	*pathbuf = canon;
	println!("{}", pathbuf.display());
	return CmdResult::Ok;
}