use crate::prelude::*;
use crate::equipment::Equipment;

#[derive(Serialize,Deserialize)]
pub struct Player {
	pub deck: Deck,
	pub hand: Vec<Card>,
	pub mana: i64,
	pub hp: i64,
	pub block: i64,
	pub discarded: bool,
	pub hand_size: usize,
	pub card_draws: usize,
	pub equipment: Equipment,
	pub strength: i64,
	pub interrupts: HashMap<i64, Box<Interrupt>>,
	pub triggers: HashMap<i64, Box<Trigger>>,
}

impl Player {
	pub fn new() -> Self {
		use crate::card::CardType::*;
		let mut deck = Deck::new(vec!(Strike.into(), Strike.into(), Defend.into()));
		let mut hand = Vec::new();
		for _ in 0..7 {
			hand.push(deck.next());
		}
		Self {
			deck,
			hand,
			mana: 3,
			hp: 200,
			block: 0,
			discarded: false,
			hand_size: 7,
			card_draws: 3,
			equipment: Equipment::new(),
			strength: 0,
			interrupts: HashMap::new(),
			triggers: HashMap::new(),
		}
	}
	
	pub fn start_turn(&mut self, enemies: &mut Vec<Enemy>) {
		//you cannot interrupt turn starting so no need to check
		for _ in 0..self.hand.len() {
			let mut card = self.hand.pop().unwrap();
			if !card.trigger(Action::StartTurn, self, enemies) {
				self.hand.insert(0, card);
			}
		}
		for _ in 0..enemies.len() {
			let mut enemy = enemies.pop().unwrap();
			if !enemy.trigger(Action::StartTurn, self, enemies) {
				enemies.insert(0, enemy);
			} else {
				enemy.die(self, enemies, &mut 0);
			}
		}
		let mut e = mem::replace(&mut self.equipment, Equipment::new());
		let _ = e.trigger(Action::StartTurn, self, enemies);
		self.equipment = e;
		let ids = self.triggers.keys().cloned().collect::<Vec<_>>();
		for id in ids {
			let mut t = mem::replace(self.triggers.get_mut(&id).unwrap(), Box::new(BlankTriggerImpl));
			if !t.trigger(Action::StartTurn, self, enemies) {
				self.triggers.insert(id, t);
			}
		}
		self.block = 0;
		self.mana = (self.mana + 3).max(1);
		self.discarded = false;
		let mut n = 0;
		while {
			let mut card = self.deck.next();
			card.draw(self, enemies);
			self.hand.push(card);
			n += 1;
			self.hand.len() < self.hand_size && n < self.card_draws
		} {}
	}
	
	pub fn end_turn(&mut self, discard: usize, enemies: &mut Vec<Enemy>) -> bool {
		let mana_tmp = self.mana;
		if self.hand.iter().any(|c| c.interrupt(Action::EndTurn, self, enemies)) || enemies.iter().any(|e| e.interrupt(Action::EndTurn, self, enemies))|| self.equipment.interrupt(Action::EndTurn, self, enemies) || self.interrupts.values().any(|i| i.interrupt(Action::EndTurn, self, enemies)) {
			false
		} else if {
			if self.has_discardable_card(enemies) {
				let tmp = self.discarded;
				self.discarded = false;
				let r = self.discard(discard, true, enemies);
				self.discarded = tmp;
				r
			} else { true }
		} {
			for _ in 0..self.hand.len() {
				let mut card = self.hand.pop().unwrap();
				if !card.trigger(Action::EndTurn, self, enemies) {
					self.hand.insert(0, card);
				}
			}
			for _ in 0..enemies.len() {
				let mut enemy = enemies.pop().unwrap();
				if !enemy.trigger(Action::EndTurn, self, enemies) {
					enemies.insert(0, enemy);
				} else {
					enemy.die(self, enemies, &mut 0);
				}
			}
			let mut e = mem::replace(&mut self.equipment, Equipment::new());
			let _ = e.trigger(Action::EndTurn, self, enemies);
			self.equipment = e;
			let ids = self.triggers.keys().cloned().collect::<Vec<_>>();
			for id in ids {
				let mut t = mem::replace(self.triggers.get_mut(&id).unwrap(), Box::new(BlankTriggerImpl));
				if !t.trigger(Action::EndTurn, self, enemies) {
					self.triggers.insert(id, t);
				}
			}
			for _ in 0..self.hand.len() {
				let card = self.hand.pop().unwrap();
				if !card.ethereal() {
					self.hand.insert(0, card);
				}
			}
			self.mana -= mana_tmp;
			true
		} else {
			false
		}
	}
	
	pub fn draw(&mut self, enemies: &mut Vec<Enemy>) {
		let mut card = self.deck.next();
		card.draw(self, enemies);
		self.hand.push(card);
	}
	
	pub fn start_fight(&mut self, enemies: &mut Vec<Enemy>) {
		self.block = 0;
		self.mana = 3;
		self.discarded = false;
		while {
			let mut card = self.deck.next();
			card.draw(self, enemies);
			self.hand.push(card);
			self.hand.len() < self.hand_size
		} {}
	}
	
	pub fn play(&mut self, n: usize, target1: Option<usize>, target2: Option<usize>, enemies: &mut Vec<Enemy>) -> bool {
		if n < self.hand.len() {
			let mut card = self.hand.remove(n);
			if self.hand.iter().any(|c| c.interrupt(Action::Playing(&mut card), self, enemies)) || enemies.iter().any(|e| e.interrupt(Action::Playing(&mut card), self, enemies)) || self.equipment.interrupt(Action::Playing(&mut card), self, enemies) || self.interrupts.values().any(|i| i.interrupt(Action::Playing(&mut card), self, enemies)) {
				self.hand.insert(n, card);
				false
			} else {
				if card.can_play(self.mana) {
					if let Err(e) = card.play(target1, target2, self, enemies) {
						use crate::card::PlayError::*;
						match e {
							NeedsTarget => {
								println!("this cards needs more targets");
							},
							BadTarget => {
								println!("this card cannot target that");
							},
							Unplayable => {
								println!("you cannot play this card right now");
							},
						}
						self.hand.insert(n, card);
						false
					} else {
						self.mana -= card.mana_cost().expect(&format!("card was playable but had no mana cost: {:?}",card));
						for _ in 0..self.hand.len() {
							let mut c = self.hand.pop().unwrap();
							if !c.trigger(Action::Playing(&mut card), self, enemies) {
								self.hand.insert(0, c);
							}
						}
						for _ in 0..enemies.len() {
							let mut enemy = enemies.pop().unwrap();
							if !enemy.trigger(Action::Playing(&mut card), self, enemies) {
								enemies.insert(0, enemy);
							} else {
								enemy.die(self, enemies, &mut 0);
							}
						}
						let mut e = mem::replace(&mut self.equipment, Equipment::new());
						let _ = e.trigger(Action::Playing(&mut card), self, enemies);
						self.equipment = e;
						let ids = self.triggers.keys().cloned().collect::<Vec<_>>();
						for id in ids {
							let mut t = mem::replace(self.triggers.get_mut(&id).unwrap(), Box::new(BlankTriggerImpl));
							if !t.trigger(Action::Playing(&mut card), self, enemies) {
								self.triggers.insert(id, t);
							}
						}
						true
					}
				} else {
					if card.mana_cost().is_none() {
						println!("that card is not playable");
					} else {
						println!("not enough mana");
					}
					self.hand.insert(n, card);
					false
				}
			}
		} else {
			println!("hand has {} cards but you chose {}",self.hand.len(),n);
			false
		}
	}
	
	pub fn discard(&mut self, n: usize, keep: bool, enemies: &mut Vec<Enemy>) -> bool {
		if n < self.hand.len() && !self.discarded {
			let mut card = self.hand.remove(n);
			if self.hand.iter().any(|c| c.interrupt(Action::Discarding(&mut card, keep), self, enemies)) || enemies.iter().any(|e| e.interrupt(Action::Discarding(&mut card, keep), self, enemies)) || self.equipment.interrupt(Action::Discarding(&mut card, keep), self, enemies) || self.interrupts.values().any(|i| i.interrupt(Action::Discarding(&mut card, keep), self, enemies)) {
				self.hand.insert(n, card);
				false
			} else if card.can_discard(self.mana) {
				self.mana += card.discard_mana().unwrap();
				card.discard(keep, self, enemies);
				for _ in 0..self.hand.len() {
					let mut c = self.hand.pop().unwrap();
					if !c.trigger(Action::Discarding(&mut card, keep), self, enemies) {
						self.hand.insert(0, c);
					}
				}
				for _ in 0..enemies.len() {
					let mut enemy = enemies.pop().unwrap();
					if !enemy.trigger(Action::Discarding(&mut card, keep), self, enemies) {
						enemies.insert(0, enemy);
					} else {
						enemy.die(self, enemies, &mut 0);
					}
				}
				let mut e = mem::replace(&mut self.equipment, Equipment::new());
				let _ = e.trigger(Action::Discarding(&mut card, keep), self, enemies);
				self.equipment = e;
				let ids = self.triggers.keys().cloned().collect::<Vec<_>>();
				for id in ids {
					let mut t = mem::replace(self.triggers.get_mut(&id).unwrap(), Box::new(BlankTriggerImpl));
					if !t.trigger(Action::Discarding(&mut card, keep), self, enemies) {
						self.triggers.insert(id, t);
					}
				}
				if (keep && !card.lose_on_discard()) || card.keep_on_discard() {
					self.deck.add_cards(vec!(card));
				}
				self.discarded = true;
				true
			} else {
				if card.discard_mana().is_some() {
					println!("not enough mana to discard this card");
				} else {
					println!("that card is not discardable");
				}
				self.hand.insert(n, card);
				false
			}
		} else {
			if self.discarded {
				println!("you can only discard one card per turn");
			} else {
				println!("hand has {} cards but you chose {}",self.hand.len(),n);
			}
			false
		}
	}
	
	pub fn has_discardable_card(&mut self, _enemies: &mut Vec<Enemy>) -> bool {
		self.hand.iter().any(|x| x.can_discard(self.mana))
	}
	
	pub fn damage(&mut self, amount: i64, attacker: Option<&mut Enemy>, enemies: &mut Vec<Enemy>) -> bool {
		//println!("{} attacks for {} damage",attacker.name(),amount);
		if self.hand.iter().any(|c| c.interrupt(Action::Damaged(&attacker), self, enemies)) || enemies.iter().any(|e| e.interrupt(Action::Damaged(&attacker), self, enemies)) || self.equipment.interrupt(Action::Damaged(&attacker), self, enemies) || self.interrupts.values().any(|i| i.interrupt(Action::Damaged(&attacker), self, enemies)) {
			false
		} else {
			for _ in 0..self.hand.len() {
				let mut card = self.hand.pop().unwrap();
				if !card.trigger(Action::Damaged(&attacker), self, enemies) {
					self.hand.insert(0, card);
				}
			}
			for _ in 0..enemies.len() {
				let mut enemy = enemies.pop().unwrap();
				if !enemy.trigger(Action::Damaged(&attacker), self, enemies) {
					enemies.insert(0, enemy);
				} else {
					enemy.die(self, enemies, &mut 0);
				}
			}
			let mut e = mem::replace(&mut self.equipment, Equipment::new());
			let _ = e.trigger(Action::Damaged(&attacker), self, enemies);
			self.equipment = e;
			let ids = self.triggers.keys().cloned().collect::<Vec<_>>();
			for id in ids {
				let mut t = mem::replace(self.triggers.get_mut(&id).unwrap(), Box::new(BlankTriggerImpl));
				if !t.trigger(Action::Damaged(&attacker), self, enemies) {
					self.triggers.insert(id, t);
				}
			}
			let new_amount = (amount - self.block).max(0);
			self.block = (self.block - amount).max(0);
			self.hp -= new_amount;
			true
		}
	}
	
	pub fn attack(&mut self, amount: i64, enemy: &mut Enemy, enemies: &mut Vec<Enemy>) -> bool {
		let mut amount = amount + self.strength;
		if self.hand.iter().any(|c| c.interrupt(Action::Attacking(enemy, &mut amount), self, enemies)) || enemies.iter().any(|e| e.interrupt(Action::Attacking(enemy, &mut amount), self, enemies)) || self.equipment.interrupt(Action::Attacking(enemy, &mut amount), self, enemies) || self.interrupts.values().any(|i| i.interrupt(Action::Attacking(enemy, &mut amount), self, enemies)) {
			false
		} else {
			for _ in 0..self.hand.len() {
				let mut card = self.hand.pop().unwrap();
				if !card.trigger(Action::Attacking(enemy, &mut amount), self, enemies) {
					self.hand.insert(0, card);
				}
			}
			for _ in 0..enemies.len() {
				let mut e = enemies.pop().unwrap();
				if !e.trigger(Action::Attacking(enemy, &mut amount), self, enemies) {
					enemies.insert(0, e);
				} else {
					e.die(self, enemies, &mut 0); //TODO: its possible for some wierd ordering glitch to occur here that makes enemies swap places when dying sometimes
				}
			}
			let mut e = mem::replace(&mut self.equipment, Equipment::new());
			let _ = e.trigger(Action::Attacking(enemy, &mut amount), self, enemies);
			self.equipment = e;
			let ids = self.triggers.keys().cloned().collect::<Vec<_>>();
			for id in ids {
				let mut t = mem::replace(self.triggers.get_mut(&id).unwrap(), Box::new(BlankTriggerImpl));
				if !t.trigger(Action::Attacking(enemy, &mut amount), self, enemies) {
					self.triggers.insert(id, t);
				}
			}
			enemy.damage(self, amount);
			true
		}
	}
	
	pub fn get_loot(&mut self, mut loot: Loot, commands: &terminal::Terminal<crate::terminal_command::Command>) {
		for l in loot.0.drain(..) {
			use self::LootInner::*;
			match l {
				Cards(n, card) => {
					let mut c = Vec::new();
					for _ in 0..n {
						c.push(card.clone());
					}
					self.deck.add_cards(c);
				},
				LootItem(i) => {
					let mut e = mem::replace(&mut self.equipment, Equipment::new());
					if !e.get_loot(i, self, commands) {
						//unimplemented
					}
					self.equipment = e;
				},
			}
		}
	}
}

impl fmt::Display for Player {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f,"mana: {}, hp: {}, block: {}",self.mana,self.hp,self.block)?;
		for (i, card) in self.hand.iter().enumerate() {
			write!(f,"\n{} |   {}",i,card.fmt(&self))?;
		}
		Ok(())
	}
}

pub enum Action<'a> {
	EndTurn,
	StartTurn,
	Attacking(&'a mut Enemy, &'a mut i64),
	Damaged(&'a Option<&'a mut Enemy>),
	Playing(&'a mut Card),
	Discarding(&'a mut Card, bool),
}

pub trait Interrupt: serde_traitobject::Serialize + serde_traitobject::Deserialize {
	#[must_use]
	fn interrupt(&self, a: Action, p: &Player, e: &Vec<Enemy>) -> bool;
}

pub trait Trigger: serde_traitobject::Serialize + serde_traitobject::Deserialize {
	#[must_use]
	fn trigger(&mut self, a: Action, p: &mut Player, e: &mut Vec<Enemy>) -> bool;
}

#[derive(Serialize,Deserialize)]
struct BlankTriggerImpl;
impl Trigger for BlankTriggerImpl {
	fn trigger(&mut self, _a: Action, _p: &mut Player, _e: &mut Vec<Enemy>) -> bool {
		false
	}
}
