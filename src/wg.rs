use std::fmt;
use rand::Rng;
use rand::ThreadRng;
use std::cmp::Ordering;

#[derive(Clone, Copy)]
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

impl fmt::Display for Value {
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

#[derive(Clone, Copy)]
enum Suit {
	Clubs,
	Hearts,
	Diamonds,
	Spades
}

impl fmt::Display for Suit {
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

#[derive(Clone)]
struct Card {
	value: Value,
	suit: Suit
}

impl Card {
	fn new(value: Value, suit: Suit) -> Card {
		Card {value: value, suit: suit}
	}

	fn get_value(&self) -> i32 {
		let v:i32 = match self.value {
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

impl fmt::Display for Card {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{} of {}", self.value, self.suit)
	}
}

/*
	Comparing Cards.
*/
impl PartialEq for Card {
	fn eq(&self, other: &Card) -> bool {
		(self.get_value() - other.get_value()) == 0
	}
}
impl PartialOrd for Card {
    fn lt(&self, other: &Card) -> bool {
        match self.cmp(other) { Ordering::Less => true, _ => false}
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
		if v1 < v2 {return Ordering::Less;}
		if v1 > v2 {return Ordering::Greater;}
		return Ordering::Equal;
	}
}

#[derive(Clone)]
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
		let cards:Vec<Card> = Vec::with_capacity(52);
		Deck(cards)
	}

	fn split(&mut self) -> Deck {
		let &mut Deck(ref mut cards) = self;
		let length = cards.len();
		let half = length / 2;

		// a vector(52) for awaiting cards
		let mut _cards:Vec<Card> = Vec::with_capacity(52);
		for _ in 0..half {
			let c:Card = match cards.pop() {
				None => continue,
				Some(v) => v
			};
			_cards.push(c);
		}

		// returns a new deck
		Deck(_cards)
	}

	fn shuffle(&mut self, rng: &mut ThreadRng) {
		let &mut Deck(ref mut cards) = self;

		// let mut rng = rand::thread_rng();
		rng.shuffle(cards);

	}

	fn length(&mut self) -> usize {
		let &mut Deck(ref mut cards) = self;

		return cards.len()
	}

	fn has_cards(&mut self) -> bool {
		let &mut Deck(ref mut cards) = self;
		cards.len() > 0
	}

	/*
		Get the card at the top of the deck.
	*/
	fn get_card(&mut self) -> Card {
		let &mut Deck(ref mut cards) = self;
		cards[0].clone()
	}

	/*
		Removes card from the top of this deck and gives the card to the given deck.
	*/
	fn give_card(&mut self, deck: &mut Deck) -> () {
		let &mut Deck(ref mut cards) = self;
		let &mut Deck(ref mut cards2) = deck;

		if cards.len() == 0 {
			return ();
		}

		let card = cards.remove(0);
		cards2.push(card);

	}

	/*
		Gives card from this deck to the given deck.
	*/
	fn give_cards(&mut self, deck: &mut Deck) -> () {
		for _ in 0..self.length() {
			self.give_card(deck);
		}
	}

}

impl fmt::Display for Deck {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let &Deck(ref cards) = self;

		// a better method might be map to connect these strings
		let mut text = String::new();
		let mut i = 0;
		for card in cards.iter() {
			text.push_str(&card.to_string());
			i+=1;
			if i < cards.capacity() {
				text.push_str(", ");
			}
		}
		write!(f, "{}", text)
	}
}

pub fn game(rng: &mut ThreadRng) {

	let mut player1 = Deck::new_fresh_deck();

	player1.shuffle(rng);

	let mut player2 = player1.split();

	let mut turns = 0;

	let mut winner:Deck = Deck::new();

	'base: while player1.has_cards() && player2.has_cards() {
		turns = turns + 1;

		info!(target: "game_events", "Turn #{}", turns);

		info!(target: "game_events", "P1 {}; P2 {}", player1.length(), player2.length());

		let mut card1:Card = player1.get_card();
		let mut card2:Card = player2.get_card();

		player1.give_card(&mut winner);
		player2.give_card(&mut winner);

		info!(target: "game_events", "P1: `{}`; P2: `{}`", card1, card2);

		if card1 == card2 {

			let mut wars = 0;

			'war: while {

				info!(target: "game_events", "P1: `{}` = P2: `{}`", card1, card2);

				if player1.length() < 4 || player2.length() < 4 {
					//info!(target: "game_events", ("Not enough cards for war!"));
					break 'base;
				}
				wars = wars + 1;
				info!(target: "game_events", "War #{}", wars);

				// each player provides 3 cards to the winner
				for _ in 0..3 {
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
					info!(target: "game_events", "P1: `{}` < P2: `{}`; W {}", card1, card2, winner.length());
					winner.shuffle(rng);
					winner.give_cards(&mut player2);
				} else if card1 > card2 {
					info!(target: "game_events", "P1: `{}` > P2: `{}`; W {}", card1, card2, winner.length());
					winner.shuffle(rng);
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

		} else if card1 < card2 {
			info!(target: "game_events", "P1: `{}` < P2: `{}`; W {}", card1, card2, winner.length());
			winner.shuffle(rng);
			winner.give_cards(&mut player2);
		} else if card1 > card2 {
			info!(target: "game_events", "P1: `{}` > P2: `{}`; W {}", card1, card2, winner.length());
			winner.shuffle(rng);
			winner.give_cards(&mut player1);
		}


	}

	info!(target: "game_events", "Total turns: {}", turns);
	info!(target: "game_events", "P1: {}; P2: {}; W {}", player1.length(), player2.length(), winner.length());
}
