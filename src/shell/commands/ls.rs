use std::fs;
use std::path;
use std::os::unix::fs::PermissionsExt;

use chrono::{DateTime, Local};

use crate::shell::structures::*;

pub fn ls_usage() {
	println!("ls	-	List directory contents\n");

	println!("Usage: ls [-<options>] [<path>]");
	println!("For example:");
	println!("ls -l -a .");
	println!("");
	println!("Available options:");
	println!("-l:	List entries with additional information");
	println!("-a:	Also list hidden files (show entries starting with .)");
	println!("-h	This print");
}

fn entry_permissions(path: &path::Path) -> Option<String> {
	let meta = fs::symlink_metadata(path).ok()?;
	let file_type = meta.file_type();
	let mut result = String::new();

	if file_type.is_dir() {
		result.push('d');
	} else if file_type.is_symlink() {
		result.push('l');
	} else {
		result.push('-');
	}

	let mode = meta.permissions().mode();

	// owner
	result.push(if mode & 0o400 != 0 { 'r' } else { '-' });
	result.push(if mode & 0o200 != 0 { 'w' } else { '-' });
	result.push(if mode & 0o100 != 0 { 'x' } else { '-' });
	// group
	result.push(if mode & 0o040 != 0 { 'r' } else { '-' });
	result.push(if mode & 0o020 != 0 { 'w' } else { '-' });
	result.push(if mode & 0o010 != 0 { 'x' } else { '-' });
	// other
	result.push(if mode & 0o004 != 0 { 'r' } else { '-' });
	result.push(if mode & 0o002 != 0 { 'w' } else { '-' });
	result.push(if mode & 0o001 != 0 { 'x' } else { '-' });

	Some(result)
}

pub fn ls(path: &path::Path, args: Vec<String>) -> CmdResult {
	let rest: &[String];

	let mut options: CommandOptions = CommandOptions::default();

	if args.len() > 1 {
		rest = &args[1..];
	} else {
		rest = &[];
	}

	let options_collection: Vec<&str> = rest.iter()
		.filter(|s| s.starts_with('-'))
		.map(|s| s.as_str().strip_prefix('-').unwrap())
		.collect();

	for option in options_collection {
		for character in option.chars() {
			match character {
				'l' => options.l = true,
				'a' => options.a = true,
				char => {
					if char != 'h' {
						println!("Unknown option: {char}");
					}

					ls_usage();
					return CmdResult::Ok;
				}
			}
		}
	}

	let path_str = rest.iter().rev()
		.find(|s| !s.starts_with('-'))
		.map(|s| s.as_str());

	let p = match path_str {
		Some(str) => path::Path::new(str),
		None => path,
	};

	match fs::read_dir(p) {
		Err(error) => println!("Reading dir with path \"{}\" failed: {}", p.display(), error),
		Ok(entries) => {
			if options.l {
				let mut detailed: Vec<(String, String, u64)> = Vec::new();
				for entry_res in entries {
					let Ok(entry) = entry_res else {
						continue;
					};
					let filename: String = entry.file_name().to_string_lossy().into_owned();
					if filename.starts_with('.') && !options.a {
						continue;
					}

					let meta = entry.metadata().ok();
					let time_str = meta
						.as_ref()
						.and_then(|m| m.modified().or_else(|_| m.created()).ok())
						.map(|st| {
							let dt: DateTime<Local> = DateTime::from(st);
							dt.format("%Y-%m-%d %H:%M").to_string()
						})
						.unwrap_or_else(|| "unknown".to_string());
					let size = meta.as_ref().map(|m| m.len()).unwrap_or(0);
					detailed.push((filename, time_str, size));
				}

				detailed.sort_by(|a, b| a.0.cmp(&b.0));
				for (name, time_str, size) in detailed {
					let perms = entry_permissions(&p.join(&name)).unwrap_or_else(|| "<perm?>".into());
					println!("{:<10} {:>8} {} {}", perms, size, time_str, name);
				}
			} else {
				let mut simple_names: Vec<String> = Vec::new();
				for entry_res in entries {
					let Ok(entry) = entry_res else {
						continue;
					};
					let filename: String = entry.file_name().to_string_lossy().into_owned();
					if filename.starts_with('.') && !options.a {
						continue;
					}
					simple_names.push(filename);
				}
				simple_names.sort();
				println!("{}", simple_names.join("\t"));
			}

		},
	}

	return CmdResult::Ok;
}