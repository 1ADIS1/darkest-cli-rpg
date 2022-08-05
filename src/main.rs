use std::io;

fn main() {
	println!("Welcome to the Darkest RPG! Please, enter your name: ");

	let mut player_name = String::new();

	io::stdin()
		.read_line(&mut player_name)
		.expect("Wrong name!");

	// Get rid of unnecessary symbols
	player_name = String::from(player_name.trim());

	println!("Hello, {player_name}, good luck in your journey. You will need it...");
}