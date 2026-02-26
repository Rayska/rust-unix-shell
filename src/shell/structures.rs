pub enum CmdResult {
	Ok,
	Exit,
	Unknown,
}

pub fn display_arg(arg: &str) -> String {
	if arg.contains(' ') {
		format!("\"{}\"" , arg)
	} else {
		arg.to_string()
	}
}
