extern crate rand;
#[macro_use]
extern crate log;
extern crate env_logger;

pub mod wg;
pub mod benchmark;


fn main() {
	env_logger::init().unwrap();

	// let value = Value::Two;
	// let suit = Suit::Hearts;

	// let card = Card::new(value, suit);

	// print!("Value: {}\n", value);
	// print!("Suit: {}\n", suit);

	// print!("Card: {}\n", card);

	print!("\n\n\n\n\t\tdone\n\n\n\n");

	wg::game();


}
