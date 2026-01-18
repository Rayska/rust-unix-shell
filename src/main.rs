use std::io;

fn take_input() -> String {
	let mut user_input: String = String::new();
	io::stdin().read_line(&mut user_input).expect("Error reading input");
	return user_input;
}

fn main() {
	let mut input: String;
	loop {
		input = take_input();
		match input.trim() {
			"$ ls" => println!("Received: $ ls"),
			"$ ls -l" => println!("Received: $ ls -l"),
			"$ cd" => println!("Received: $ cd"),
			"$ cd folder_name" => println!("Received: $ cd folder_name"),
			"$ pwd" => println!("Received: $ pwd"),
			"$ write |name|" => println!("Received: $ write |name|"),
			"$ help" => println!("Received: $ help"),
			"$ exit" => break,
			_ => println!("Invalid command. Try again."),
		}
	}
}
