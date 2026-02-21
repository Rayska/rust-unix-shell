use std::fs;
use std::path;

use crate::shell::structures::*;

pub fn cd_usage() {
	println!("cd	-	Change the shell working directory\n");

	println!("Usage: cd [<path>]");
	println!("Path can be absolute or relative.");
	println!("Pathless call will change working directory to be the home directory");
	println!("Home directory is taken from $HOME when available.");
}

pub fn cd(home_dir: &path::Path, path: &mut path::PathBuf, args: Vec<String>) -> CmdResult {
	if args.iter().any(|a| a == "-h") {
		cd_usage();
		return CmdResult::Ok;
	}

	let dest_string: String;
	if args.len() == 1 {
		dest_string = "~".to_string();
	} else if args.len() == 2 {
		dest_string = args[1].clone();
	} else {
		eprintln!("cd: too many arguments (hint: quote paths with spaces)");
		cd_usage();
		return CmdResult::Ok;
	}

	let candidate_pathbuf = if dest_string == "~" {
		home_dir.to_path_buf()
	} else if let Some(rest) = dest_string.strip_prefix("~/") {
		home_dir.join(rest)
	} else {
		let p = path::Path::new(&dest_string);

		if p.is_absolute() {
			p.to_path_buf()
		} else {
			path.join(p)
		}
	};

	match fs::canonicalize(&candidate_pathbuf) {
		Ok(canon) => {
			if canon.is_dir() {
				*path = canon;
			} else {
				let name = canon.file_name()
					.map(|s| s.to_string_lossy().into_owned())
					.unwrap_or_else(|| canon.display().to_string());
				eprintln!("cd: {}: Not a directory", name);
			}
		},
		Err(_) => eprintln!("cd: no such file or directory: {}", dest_string),
	}

	return CmdResult::Ok;
}