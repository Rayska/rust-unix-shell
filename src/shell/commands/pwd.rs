use std::{fs, path};
use crate::shell::structures::*;

pub fn pwd(pathbuf: &mut path::PathBuf) -> CmdResult {
	let canon = match fs::canonicalize(pathbuf.as_path()) {
		Ok(c) => c,
		Err(e) => {
			eprintln!("pwd: {}", e);
			return CmdResult::Ok;
		}
	};

	*pathbuf = canon;
	println!("{}", pathbuf.display());
	return CmdResult::Ok;
}