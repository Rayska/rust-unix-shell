pub enum CmdResult {
	Ok,
	Exit,
	Unknown,
}

#[derive(Default)]
pub struct CommandOptions {
	pub a: bool,
	pub l: bool,
}
