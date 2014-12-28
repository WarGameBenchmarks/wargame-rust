#![feature(phase)] 
#[phase(plugin, link)] extern crate log; 

use std::fmt;
use std::rand::{task_rng, Rng};

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
		
		let mut rng = task_rng();
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
		Removes card from the top of this deck and gives the card to the given deck.
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

	/*
		Gives card from this deck to the given deck.
	*/
	fn give_cards(&mut self, deck: &mut Deck) -> () {
		self.shuffle();
		for _ in range(0, self.length()) {
			self.give_card(deck);
		}		
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


pub fn game() {

	let mut player1 = Deck::new_fresh_deck();

	player1.shuffle();

	let mut player2 = player1.split();

	let mut turns = 0u;

	let mut winner:Deck = Deck::new();

	'base: while player1.has_cards() && player2.has_cards() {
		turns = turns + 1;

		debug!(format!("Turn #{}", turns));

		debug!(format!("P1 {}; P2 {}", player1.length(), player2.length()));

		let mut card1:Card = player1.get_card();
		let mut card2:Card = player2.get_card();

		player1.give_card(&mut winner);
		player2.give_card(&mut winner);

		debug!(format!("P1: {}; P2: {}", card1, card2));

		if card1 == card2 {

			let mut wars = 0u;

			'war: while {

				debug!(format!("P1: {} = P2: {}", card1, card2));

				if player1.length() < 4 || player2.length() < 4 {
					debug!(format!("Not enough cards for war!"));
					break 'base;
				}
				wars = wars + 1;
				debug!(format!("War #{}", wars));

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
					debug!(format!("P1: {} < P2: {}; W {}", card1, card2, winner.length()));
					winner.give_cards(&mut player2);
				} else if card1 > card2 {
					debug!(format!("P1: {} > P2: {}; W {}", card1, card2, winner.length()));
					winner.give_cards(&mut player1);
				} else {
					// perform another war
					// the cards are equal
				}

				// this must be the last line to emulate the do-while structure
				// please, tell me how to convert this into a non-hack
				// and it shall be done.
				card1 == card2
			} {}
			
			debug!(format!("War has ended"));

		} else if card1 < card2 {
			debug!(format!("P1: {} < P2: {}; W {}", card1, card2, winner.length()));
			winner.shuffle();
			for _ in range(0, winner.length()) {
				winner.give_cards(&mut player2);
			}
		} else if card1 > card2 {
			debug!(format!("P1: {} > P2: {}; W {}", card1, card2, winner.length()));
			winner.shuffle();
			for _ in range(0, winner.length()) {
				winner.give_cards(&mut player1);
			}
		}
		

	}


	debug!(format!("Total turns: {}", turns));
	debug!(format!("P1: {}; P2: {}", player1.length(), player2.length()));

}

pub fn backprint(s: String) {
	print!("\r{}", s);
}
