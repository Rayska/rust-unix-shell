use std::fs;
use std::path;

use crate::shell::structures::*;

#[derive(Default)]
struct RmOptions {
	recursive: bool,
}

pub fn rm_usage() {
	println!("rm	-	Remove the provided item(s).\n");

	println!("Usage: rm [-r] <file> [<file>...]");
	println!("Remove provided file(s) or directory trees (-r).");
}

pub fn rm(cwd: &path::Path, args: Vec<String>) -> CmdResult {
	let mut opts = RmOptions::default();

	if args.len() < 2 {
		eprintln!("rm: No target provided");
		rm_usage();
		return CmdResult::Ok;
	}

	let rest: &[String] = &args[1..];

	for arg in rest.iter().filter(|s| s.starts_with('-')) {
		for ch in arg[1..].chars() {
			match ch {
				'r' => opts.recursive = true,
				'h' => {
					rm_usage();
					return CmdResult::Ok;
				}
				other => {
					eprintln!("rm: Unknown option: {other}");
					rm_usage();
					return CmdResult::Ok;
				}
			}
		}
	}

	let file_args: Vec<&String> = rest.iter().filter(|s| !s.starts_with('-')).collect();

	if file_args.is_empty() {
		eprintln!("rm: No target provided");
		rm_usage();
		return CmdResult::Ok;
	}

	for target in &file_args {
		let target = target.as_str();
		let p = path::Path::new(target);
		let candidate = if p.is_absolute() {
			p.to_path_buf()
		} else {
			cwd.join(p)
		};

		if candidate.symlink_metadata().map(|m| m.is_symlink()).unwrap_or(false) {
			if let Err(e) = fs::remove_file(&candidate) {
				eprintln!("rm: cannot remove '{}': {}", display_arg(target), e);
			}
		} else if candidate.is_dir() {
			if opts.recursive {
				if let Err(e) = fs::remove_dir_all(&candidate) {
					eprintln!("rm: cannot remove '{}': {}", display_arg(target), e);
				}
			} else {
				eprintln!("rm: {}: is a directory", display_arg(target));
			}
		} else if let Err(e) = fs::remove_file(&candidate) {
			eprintln!("rm: cannot remove '{}': {}", display_arg(target), e);
		}
	}

	CmdResult::Ok
}