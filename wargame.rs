
extern crate time;

use std::os;
use std::fmt;
use time::precise_time_ns;
use std::rand::{task_rng, Rng};
use std::io;

#[deriving(Clone)]
enum Value {
	Two,
	Three,
	Four,
	Five,
	Six,
	Seven,
	Eight,
	Nine,
	Ten,
	Jack,
	Queen,
	King,
	Ace
}

impl fmt::Show for Value {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let name = match *self {
			Value::Two => "2",
			Value::Three => "3",
			Value::Four => "4",
			Value::Five => "5",
			Value::Six => "6",
			Value::Seven => "7",
			Value::Eight => "8",
			Value::Nine => "9",
			Value::Ten => "10",
			Value::Jack => "Jack",
			Value::Queen => "Queen",
			Value::King => "King",
			Value::Ace => "Ace",
		};
		write!(f, "{}", name)
	}
}

#[deriving(Clone)]
enum Suit {
	Clubs,
	Hearts,
	Diamonds,
	Spades
}

impl fmt::Show for Suit {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let name = match *self {
			Suit::Clubs => "Clubs",
			Suit::Hearts => "Hearts",
			Suit::Diamonds => "Diamonds",
			Suit::Spades => "Spades",
		};
		write!(f, "{}", name)
	}
}

#[deriving(Clone)]
struct Card {
	value: Value,
	suit: Suit
}

impl Card {
	fn new(value: Value, suit: Suit) -> Card {
		Card {value: value, suit: suit}
	}

	fn get_value(&self) -> uint {
		let v:uint = match self.value {
			Value::Two => 2,
			Value::Three => 3,
			Value::Four => 4,
			Value::Five => 5,
			Value::Six => 6,
			Value::Seven => 7,
			Value::Eight => 8,
			Value::Nine => 9,
			Value::Ten => 10,
			Value::Jack => 11,
			Value::Queen => 12,
			Value::King => 13,
			Value::Ace => 14,
		};
		return v;
	}
}

impl fmt::Show for Card {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{} of {}", self.value, self.suit)
	}
}

/*
	Allows direct comparison of cards.

	Annoying amount of code to implement something so straight forward.
*/
impl PartialEq for Card {
	fn eq(&self, other: &Card) -> bool {
		(self.get_value() - other.get_value()) == 0
	}
}
impl PartialOrd for Card {
    fn lt(&self, other: &Card) -> bool {
        match self.cmp(other) { Less => true, _ => false}
    }
    fn partial_cmp(&self, other: &Card) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Eq for Card {}
impl Ord for Card {
	fn cmp(&self, other: &Card) -> Ordering {
		let v1 = self.get_value();
		let v2 = other.get_value();
		if v1 < v2 {return Less;}
		if v1 > v2 {return Greater;}
		return Equal;
	}
}

#[deriving(Clone)]
struct Deck(Vec<Card>);

impl Deck {

	/*
		Makes a fresh deck of 52 regular cards.
	*/
	fn new_fresh_deck() -> Deck {

		let mut cards:Vec<Card> = Vec::with_capacity(52);
		for suit in [Suit::Spades, Suit::Hearts, Suit::Diamonds, Suit::Clubs].iter() {
			for value in [
				Value::Two, Value::Three, Value::Four,
				Value::Five, Value::Six, Value::Seven, 
				Value::Eight, Value::Nine, Value::Ten, 
				Value::Jack, Value::Queen, Value::King,
				Value::Ace  
			].iter() {
				cards.push(Card::new(value.clone(), suit.clone()));
			}
		}	
		Deck(cards)
	}

	fn new() -> Deck {
		let mut cards:Vec<Card> = Vec::with_capacity(52);
		Deck(cards)	
	}

	fn split(&mut self) -> Deck {
		let &Deck(ref mut cards) = self;
		let length = cards.len();
		let half = length / 2u;

		// a vector(52) for awaiting cards
		let mut _cards:Vec<Card> = Vec::with_capacity(52);
		for _ in range(0, half) {
			let c:Card = match cards.pop() {
				None => continue,
				Some(v) => v
			};
			_cards.push(c);
		}

		// returns a new deck
		Deck(_cards)
	}

	fn shuffle(&mut self) {
		let &Deck(ref mut cards) = self;
		
		let mut rng = std::rand::task_rng();
		rng.shuffle(cards.as_mut_slice());

	}

	fn length(&mut self) -> uint {
		let &Deck(ref mut cards) = self;

		return cards.len()
	}

	fn has_cards(&mut self) -> bool {
		let &Deck(ref mut cards) = self;
		cards.len() > 0u
	}

	/*
		Get the card at the top of the deck.
	*/
	fn get_card(&mut self) -> Card {
		let &Deck(ref mut cards) = self;
		cards[0].clone()
	}

	/*
		Remove the card from the top of the deck.
	*/
	fn give_card(&mut self, deck: &mut Deck) -> () {
		let &Deck(ref mut cards) = self;
		let &Deck(ref mut cards2) = deck;

		let card:Card = match cards.remove(0) {
			None => return (),
			Some(c) => c
		};
		cards2.push(card)
	}

}

impl fmt::Show for Deck {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let &Deck(ref cards) = self;

		// a better method might be map to connect these strings
		let mut text = String::new();
		let mut i = 0;
		for card in cards.iter() {
			text.push_str(format!("{}", card).as_slice());
			i+=1;
			if i < cards.capacity() {
				text.push_str(", ");
			}
		}
		write!(f, "{}", text)
	}
}

fn log(s: String) {
	let debug = false;
	if ( debug ) {
		println!("{}", s);
	}
}

fn game() {

	let mut player1 = Deck::new_fresh_deck();

	player1.shuffle();

	let mut player2 = player1.split();

	let mut turns = 0u;

	'base: while player1.has_cards() && player2.has_cards() {
		turns = turns + 1;

		log(format!("Turn #{}", turns));

		log(format!("P1 {}; P2 {}", player1.length(), player2.length()));

		let mut winner:Deck = Deck::new();

		let mut card1:Card = player1.get_card();
		let mut card2:Card = player2.get_card();

		player1.give_card(&mut winner);
		player2.give_card(&mut winner);

		log(format!("P1: {}; P2: {}", card1, card2));

		if card1 == card2 {

			let mut wars = 0u;

			'war: while {

				log(format!("P1: {} = P2: {}", card1, card2));

				if player1.length() < 4 || player2.length() < 4 {
					log(format!("Not enough cards for war!"));
					break 'base;
				}
				wars = wars + 1;
				log(format!("War #{}", wars));

				// each player provides 3 cards to the winner
				for _ in range(0, 3u) {
					player1.give_card(&mut winner);
					player2.give_card(&mut winner);
				}

				// get the top cards
				card1 = player1.get_card();
				card2 = player2.get_card();

				// send the top cards to the winner deck
				player1.give_card(&mut winner);
				player2.give_card(&mut winner);

				if card1 < card2 {
					log(format!("P1: {} < P2: {}; W {}", card1, card2, winner.length()));
					winner.shuffle();
					for _ in range(0, winner.length()) {
						winner.give_card(&mut player2);
					}					
				} else if card1 > card2 {
					log(format!("P1: {} > P2: {}; W {}", card1, card2, winner.length()));
					winner.shuffle();
					for _ in range(0, winner.length()) {
						winner.give_card(&mut player1);
					}				
				} else {
					// perform another war
					// the cards are equal
				}

				// this must be the last line to emulate the do-while structure
				// please, tell me how to convert this into a non-hack
				// and it shall be done.
				card1 == card2
			} {}
			
			log(format!("War has ended"));

		} else if card1 < card2 {
			log(format!("P1: {} < P2: {}; W {}", card1, card2, winner.length()));
			winner.shuffle();
			for _ in range(0, winner.length()) {
				winner.give_card(&mut player2);
			}
		} else if card1 > card2 {
			log(format!("P1: {} > P2: {}; W {}", card1, card2, winner.length()));
			winner.shuffle();
			for _ in range(0, winner.length()) {
				winner.give_card(&mut player1);
			}
		}
		

	}


	log(format!("Total turns: {}", turns));
	log(format!("P1: {}; P2: {}", player1.length(), player2.length()));

}

fn backprint(s: String) {
	print!("\r{}", s);
}

fn multi(tasks: uint) {
	// how many tasks should we run
	// i.e. the level of concurrency

	let mut terminate_senders = Vec::<Sender<uint>>::new();
	let mut termination_receivers = Vec::<Receiver<uint>>::new();
	let mut completion_receivers = Vec::<Receiver<uint>>::new();

	for i in range(0, tasks) {

		// bind various channel ends to the arrays above
		// or into the closure of proc() below for
		// each task's use
		let (tx, rx): (Sender<uint>, Receiver<uint>) = channel();
		let (ctx, crx): (Sender<uint>, Receiver<uint>) = channel();
		let (ttx, trx): (Sender<uint>, Receiver<uint>) = channel();
		terminate_senders.push(ctx);
		termination_receivers.push(trx);
		completion_receivers.push(rx);

		// this starts the task,
		// which may or may not be a thread
		spawn(move || {
			let task_id = i;
			let mut iterations = 0;

			// infinitely loop the games,
			// second back the iteration count
			loop {

				game(); // simulation of game
				iterations = iterations + 1;

				tx.send(iterations);
				

				let result = crx.try_recv();
				match result {
					Ok(r) => {
						if r == 1 {
							// break out of this loop
							break;
						}
					},
					Err(e) => {
						// there are no errors here
					}
				}

			}
			// send the termination signal
			ttx.send(1);
		});			
	}

	let mut phase = 1u;

	let mut total_games = 0u64;
	let start_time = precise_time_ns();
	let mut current_time = precise_time_ns();
	let mut test_duration = 0f64;

	// 1 minute in nanoseconds
	// let prime_time = 500000000;
	// let prime_time = 10000000000;
	let prime_time = 60000000000;
	let maximum_tests = 100u;
	let percent_variation = 0.0001f64;

	let MS = 1000000u64;
	let NS = 1000000000u64;

	let mut tests = 0;

	let mut elasped_time = 0;
	let mut last_time = 0u64;
	let mut test_time = 0u64;

	let mut speed = 0f64;
	let mut rate = 0f64;

	let mut rate_low = 0f64;
	let mut rate_high = 0f64;
	let mut percent_rate = 0f64;

	let mut test_started = false;

	// the monitor loop collects the total game count
	// and the elasped time so far
	println!("\n{}. prime time has begun", phase); phase = 2;
	'monitor: loop {
		
		// the sum of all the tasks should be found here
		total_games = 0;
		for i in range(0, tasks) {
			let received = completion_receivers[i].recv() as u64;
			total_games = total_games + received;
		}

		// time calculations
		current_time = precise_time_ns();
		elasped_time = current_time - start_time;

		
		// NANOSECONDS per GAME
		rate = elasped_time as f64 / total_games as f64;

		// GAMES per NANOSECOND
		speed = 1f64 / rate as f64;
		let speed_v = speed * MS as f64;

		// the priming phase
		if !test_started && elasped_time >= prime_time {
			test_started = true;
			phase = 3;
			println!("\n{}. prime time has is over", phase);
			phase = 4;
		} else if test_started && elasped_time >= test_time {

			// testing phase
			if rate_low < rate && rate < rate_high || tests >= maximum_tests {
				// end the monitor infinite loop
				break 'monitor;
			} else {
				// calculate the details for the next testing phase
				test_duration = speed_v + 1f64;
				test_time = elasped_time + (test_duration * NS as f64) as u64;
				
				percent_rate = rate * percent_variation;

				rate_low = rate - percent_rate;
				rate_high = rate + percent_rate;
				tests = tests + 1;
			}

		}

		if  (current_time - last_time) > 50000000 {
			last_time = current_time;
			backprint(format!("{}. et = {}s, r = {} ms/g; s = {} g/ms; total = {} \t \t ", phase, elasped_time / NS, rate / MS as f64, speed_v, total_games));
		}

	}

	// cleanup after 'monitor has ended
	for i in range(0, tasks) {
		terminate_senders[i].send(1);
	}
	let mut end_collection = 0u;
	'end:loop {
		for i in range(0, tasks) {
			end_collection = end_collection + termination_receivers[i].recv();
		}
		if end_collection == tasks {
			phase = 5;
			println!("\n{}. {} tasks stopped", phase, end_collection);
			break 'end;
		}
	}


}

// view the source code on this gist
// https://gist.github.com/ryanmr/46097dc63c1ccf833f52
fn main() {

	let args = os::args();

	let tasks:uint = match args.len() {
		2 => match args[1].as_slice().trim().parse() {
			Some(x) => x,
			None => 1
		},
		_ => 1
	};

	println!("settings: tasks = {}", tasks);

	multi(tasks);
}
