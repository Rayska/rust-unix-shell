use std::fs;
use std::os::unix::fs::MetadataExt;
use std::path;

use crate::shell::structures::*;

pub fn du_usage() {
	println!("du	-	estimate file space usage\n");

	println!("Usage: du [-<options>] [<path>]");
	println!("For example:");
	println!("du -h -a .\n");

	println!("Available options:");
	println!("-a	Write counts for all files, not just directories");
	println!("-b	Print sizes in bytes");
	println!("-c	Produce a grand total");
	println!("-h	Print sizes in human-readable format (e.g. 1K, 234M)");
}

#[derive(Default)]
struct DuOptions {
	all:	bool,
	bytes:	bool,
	total:	bool,
	human:	bool,
}

fn format_size(size: u64, opts: &DuOptions) -> String {
	if opts.bytes {
		return size.to_string();
	}
	if opts.human {
		let mut value = size as f64;
		let mut unit = "B";

		if value >= 1024.0 { value /= 1024.0; unit = "K"; }
		if value >= 1024.0 { value /= 1024.0; unit = "M"; }
		if value >= 1024.0 { value /= 1024.0; unit = "G"; }
		if value >= 1024.0 { value /= 1024.0; unit = "T"; }

		if unit == "B" || value >= 10.0 {
			return format!("{}{}", value as u64, unit);
		}
		return format!("{:.1}{}", value, unit);
	}
	let blocks = size.div_ceil(1024);
	blocks.to_string()
}

fn get_size(meta: &fs::Metadata, opts: &DuOptions) -> u64 {
	if opts.bytes {
		meta.len()
	} else {
		meta.blocks() * 512
	}
}

fn du_recurse(path: &path::Path, opts: &DuOptions) -> u64 {
	let meta = match fs::symlink_metadata(path) {
		Ok(m) => m,
		Err(e) => {
			eprintln!("du: cannot access \"{}\": {}", path.display(), e);
			return 0;
		}
	};

	if meta.is_dir() {
		let read_dir = match fs::read_dir(path) {
			Ok(rd) => rd,
			Err(e) => {
				eprintln!("du: cannot read dir \"{}\": {}", path.display(), e);
				return get_size(&meta, opts);
			}
		};

		let mut dir_total: u64 = get_size(&meta, opts);
		for entry in read_dir {
			let item = match entry {
				Ok(i) => i,
				Err(e) => {
					eprintln!("du: failed to retrieve entry: {e}");
					continue;
				}
			};
			dir_total += du_recurse(&item.path(), opts);
		}

		println!("{}	{}", format_size(dir_total, opts), path.display());
		dir_total
	} else {
		let size = get_size(&meta, opts);
		if opts.all {
			println!("{}	{}", format_size(size, opts), path.display());
		}
		size
	}
}

pub fn du(path: &path::Path, args: Vec<String>) -> CmdResult {
	let mut opts = DuOptions::default();
	let rest: &[String] = if args.len() > 1 { &args[1..] } else { &[] };

	for arg in rest.iter().filter(|s| s.starts_with('-')) {
		if arg == "--help" {
			du_usage();
			return CmdResult::Ok;
		}
		for ch in arg.strip_prefix('-').unwrap().chars() {
			match ch {
				'a' => opts.all  = true,
				'b' => opts.bytes = true,
				'c' => opts.total = true,
				'h' => opts.human = true,
				_ => {
					eprintln!("du: invalid option -- '{ch}'");
					du_usage();
					return CmdResult::Ok;
				}
			}
		}
	}

	let non_flag_args: Vec<String> = rest.iter()
		.filter(|s| !s.starts_with('-'))
		.cloned()
		.collect();

	let resolved: Vec<path::PathBuf> = if non_flag_args.is_empty() {
		vec![path.to_path_buf()]
	} else {
		non_flag_args.iter().map(|s| {
			let p = path::Path::new(s);
			if p.is_absolute() {
				p.to_path_buf()
			} else {
				path.join(p)
			}
		}).collect()
	};

	let mut grand_total: u64 = 0;
	for target in &resolved {
		let size = du_recurse(target, &opts);
		grand_total += size;
	}

	if opts.total {
		println!("{}	total", format_size(grand_total, &opts));
	}

	CmdResult::Ok
}
