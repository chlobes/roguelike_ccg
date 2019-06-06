use crate::prelude::*;

const INITIAL_DECK_SIZE: usize = 150;
const CARD_REPLACEMENT_RATE: usize = 5;

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct Deck {
	pub deck: Vec<Card>,
	pub inf: Vec<Card>,
}

impl Deck {
	pub fn new(inf: Vec<Card>) -> Self {
		let mut deck = Vec::new();
		for _ in 0..INITIAL_DECK_SIZE {
			let idx = random::<usize>() % inf.len();
			deck.push(inf[idx].clone());
		}
		Self {
			deck,
			inf,
		}
	}
	
	pub fn add_cards(&mut self, cards: Vec<Card>) {
		self.deck.extend(cards.into_iter());
	}
	
	pub fn next(&mut self) -> Card {
		for _ in 0..CARD_REPLACEMENT_RATE {
			let a = (self.deck.len() as f64 + std::f64::consts::E - 1.0).ln();
			let a = a as usize + if a.fract() > random() { 1 } else { 0 };
			if random::<usize>() % a == 0 {
				let idx = random::<usize>() % self.inf.len();
				self.deck.push(self.inf[idx].clone());
			}
		}
		let idx = random::<usize>() % self.deck.len();
		self.deck.remove(idx)
	}
}
