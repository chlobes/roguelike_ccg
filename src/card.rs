use crate::prelude::*;
use crate::enemy::ETrigger;

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct Card {
	pub damage_modifier: i64,
	pub block_modifier: i64,
	pub cost_modifier: i64,
	pub typ: CardType,
	pub data: Vec<i64>,
}

impl Card {
	pub fn play(&mut self, target: Option<usize>, target2: Option<usize>, player: &mut Player, enemies: &mut Vec<Enemy>) -> Result<(), PlayError> {
		use self::PlayError::*;
		match self.typ {
			Strike => if enemies.is_empty() {
				Err(NeedsTarget)
			} else if enemies.len() == 1 {
				let mut enemy = enemies.remove(0);
				self.attack(5, player, &mut enemy, enemies);
				enemies.insert(0, enemy);
				Ok(())
			} else if let Some(target) = target {
				if enemies.len() > target {
					let mut enemy = enemies.remove(target);
					self.attack(5, player, &mut enemy, enemies);
					enemies.insert(target, enemy);
					Ok(())
				} else {
					Err(BadTarget)
				}
			} else {
				Err(NeedsTarget)
			},
			Defend => {
				player.block += (4 + self.block_modifier).max(1);
				Ok(())
			},
			PommelStrike => if enemies.is_empty() {
				Err(NeedsTarget)
			} else if enemies.len() == 1 {
				let mut enemy = enemies.remove(0);
				self.attack(3, player, &mut enemy, enemies);
				enemies.insert(0, enemy);
				player.draw(enemies);
				Ok(())
			} else if let Some(target) = target {
				if enemies.len() > target {
					let mut enemy = enemies.remove(target);
					self.attack(3, player, &mut enemy, enemies);
					enemies.insert(target, enemy);
					player.draw(enemies);
					Ok(())
				} else {
					Err(BadTarget)
				}
			} else {
				Err(NeedsTarget)
			},
			CripplingStabs => if enemies.is_empty() {
				Err(NeedsTarget)
			} else if enemies.len() == 1 {
				let mut enemy = enemies.remove(0);
				self.attack(3, player, &mut enemy, enemies);
				self.attack(3, player, &mut enemy, enemies);
				enemy.add_strength(-2);
				enemies.insert(0, enemy);
				Ok(())
			} else if let Some(target) = target {
				if enemies.len() > target {
					let mut enemy = enemies.remove(target);
					self.attack(3, player, &mut enemy, enemies);
					self.attack(3, player, &mut enemy, enemies);
					enemy.add_strength(-2);
					enemies.insert(target, enemy);
					Ok(())
				} else {
					Err(BadTarget)
				}
			} else {
				Err(NeedsTarget)
			},
			SwordDraw => {
				#[derive(Serialize,Deserialize)]
				struct F(usize);
				impl Trigger for F {
					fn trigger(&mut self, a: Action, p: &mut Player, _e: &mut Vec<Enemy>) -> bool {
						if let Action::EndTurn = a {
							self.0 -= 1;
						}
						if self.0 == 0 {
							p.strength -= 1;
							true
						} else {
							false
						}
					}
				}
				if enemies.is_empty() {
					Err(NeedsTarget)
				} else if enemies.len() == 1 {
					let mut enemy = enemies.remove(0);
					self.attack(2, player, &mut enemy, enemies);
					player.strength += 1;
					let id: i64 = random();
					//self.data.push(id);
					player.triggers.insert(id, Box::new(F(3)));
					enemies.insert(0, enemy);
					Ok(())
				} else if let Some(target) = target {
					if enemies.len() > target {
						let mut enemy = enemies.remove(target);
						self.attack(2, player, &mut enemy, enemies);
						player.strength += 1;
						let id = random();
						//self.data.push(id);
						player.triggers.insert(id, Box::new(F(3)));
						enemies.insert(target, enemy);
						Ok(())
					} else {
						Err(BadTarget)
					}
				} else {
					Err(NeedsTarget)
				}
			},
			Barrier => {
				player.block += (100 + self.block_modifier).max(1);
				Ok(())
			},
			Fortify => {
				#[derive(Serialize,Deserialize)]
				struct F(usize);
				impl Trigger for F {
					fn trigger(&mut self, a: Action, p: &mut Player, _e: &mut Vec<Enemy>) -> bool {
						if let Action::StartTurn = a {
							self.0 -= 1;
							p.block += 2;
						}
						self.0 == 0
					}
				}
				let id = random();
				player.triggers.insert(id, Box::new(F(5)));
				Ok(())
			},
			BlazeOfInsanity => {
				if enemies.is_empty() {
					Err(NeedsTarget)
				} else if enemies.len() == 1 {
					let mut enemy = enemies.remove(0);
					let len = player.hand.len() as i64;
					player.hand = Vec::new();
					for _ in 0..len {
						self.attack(6, player, &mut enemy, enemies);
					}
					enemies.insert(0, enemy);
					Ok(())
				} else if let Some(target) = target {
					if enemies.len() > target {
						let mut enemy = enemies.remove(target);
						let len = player.hand.len() as i64;
						player.hand = Vec::new();
						for _ in 0..len {
							self.attack(6, player, &mut enemy, enemies);
						}
						enemies.insert(target, enemy);
						Ok(())
					} else {
						Err(BadTarget)
					}
				} else {
					Err(NeedsTarget)
				}
			},
			RecklessStrike => {
				#[derive(Serialize,Deserialize)]
				struct F1(usize);
				impl Trigger for F1 {
					fn trigger(&mut self, a: Action, p: &mut Player, _e: &mut Vec<Enemy>) -> bool {
						if let Action::EndTurn = a {
							self.0 -= 1;
						}
						if self.0 == 0 {
							p.strength -= 2;
							true
						} else {
							false
						}
					}
				}
				#[derive(Serialize,Deserialize)]
				struct F2(usize);
				impl ETrigger for F2 {
					fn trigger(&mut self, e: &mut Enemy, _p: &mut Player, _es: &mut Vec<Enemy>) -> bool {
						self.0 -= 1;
						if self.0 == 0 {
							e.add_strength(-2);
							true
						} else {
							false
						}
					}
				}
				if enemies.is_empty() {
					Err(NeedsTarget)
				} else if enemies.len() == 1 {
					let mut enemy = enemies.remove(0);
					self.attack(10, player, &mut enemy, enemies);
					player.strength += 2;
					let id = random();
					player.triggers.insert(id, Box::new(F1(3)));
					enemies.insert(0, enemy);
					for enemy in enemies.iter_mut() {
						enemy.add_strength(2);
						enemy.triggers.insert(id, Box::new(F2(3)));
					}
					Ok(())
				} else if let Some(target) = target {
					if enemies.len() > target {
						let mut enemy = enemies.remove(target);
						self.attack(10, player, &mut enemy, enemies);
						player.strength += 2;
						let id = random();
						player.triggers.insert(id, Box::new(F1(3)));
						enemies.insert(target, enemy);
						for enemy in enemies.iter_mut() {
							enemy.add_strength(2);
							enemy.triggers.insert(id, Box::new(F2(3)));
						}
						Ok(())
					} else {
						Err(BadTarget)
					}
				} else {
					Err(NeedsTarget)
				}
			},
			Madness => {
				if (target2.is_none() || target2.map(|i| if i > player.hand.len() { true } else { player.hand[i].mana_cost().is_none() }).unwrap_or(false)) && player.hand.len() > 1 {
					Err(NeedsTarget)
				} else {
					if enemies.is_empty() {
						return Err(NeedsTarget);
					} else if enemies.len() == 1 {
						let mut enemy = enemies.remove(0);
						self.attack(5, player, &mut enemy, enemies);
						enemies.insert(0, enemy);
					} else if let Some(target) = target {
						if enemies.len() > target {
							let mut enemy = enemies.remove(target);
							self.attack(5, player, &mut enemy, enemies);
							enemies.insert(target, enemy);
						} else {
							return Err(BadTarget);
						}
					} else {
						return Err(NeedsTarget);
					}
					match player.hand.len() {
						0 => {},
						1 => player.hand[0].cost_modifier -= 2,
						_ => player.hand[target2.unwrap()].cost_modifier -= 2,
					}
						Ok(())
				}
			},
			DoubleEdgedSword => if enemies.is_empty() {
				Err(NeedsTarget)
			} else if enemies.len() == 1 {
				let mut enemy = enemies.remove(0);
				self.attack(25, player, &mut enemy, enemies);
				enemies.insert(0, enemy);
				player.damage(10, None, enemies);
				Ok(())
			} else if let Some(target) = target {
				if enemies.len() > target {
					let mut enemy = enemies.remove(target);
					self.attack(25, player, &mut enemy, enemies);
					enemies.insert(target, enemy);
					player.damage(10, None, enemies);
					Ok(())
				} else {
					Err(BadTarget)
				}
			} else {
				Err(NeedsTarget)
			},
			_ => Err(PlayError::Unplayable),
		}
	}
	
	pub fn ethereal(&self) -> bool {
		match self.typ {
			Barrier => true,
			Dazed => true,
			Fear => true,
			Exhaustion => true,
			_ => false,
		}
	}
	
	pub fn discard(&mut self, _keep: bool, player: &mut Player, _enemies: &mut Vec<Enemy>) {
		match self.typ {
			Stress => player.hp = player.hp.saturating_sub(10),
			_ => {},
		}
	}
	
	pub fn draw(&mut self, player: &mut Player, _enemies: &mut Vec<Enemy>) {
		match self.typ {
			Fear => player.mana -= 3,
			_ => {},
		}
	}
	
	pub fn lose_on_discard(&self) -> bool { //override to make card always dissapear when discarded
		match self.typ {
			Unease | Stress => true,
			_ => false,
		}
	}
	
	pub fn keep_on_discard(&self) -> bool { //override to make card always get put into deck when discarded
		match self.typ {
			_ => false,
		}
	}
	
	pub fn mana_cost(&self) -> Option<i64> {
		match self.typ {
			Strike => Some(1),
			Defend => Some(1),
			PommelStrike => Some(1),
			CripplingStabs => Some(2),
			SwordDraw => Some(1),
			Barrier => Some(5),
			Fortify => Some(1),
			BlazeOfInsanity => Some(2),
			RecklessStrike => Some(1),
			Madness => Some(1),
			DoubleEdgedSword => Some(2),
			Dazed => None,
			Fear => None,
			Unease => None,
			Stress => None,
			Exhaustion => None,
		}.map(|x| (x + self.cost_modifier).max(0))
	}
	
	pub fn discard_mana(&self) -> Option<i64> {
		match self.typ {
			Strike => Some(1), 
			Defend => Some(1),
			PommelStrike => Some(1),
			CripplingStabs => Some(1),
			SwordDraw => Some(1),
			Barrier => Some(2),
			Fortify => Some(1),
			BlazeOfInsanity => Some(1),
			RecklessStrike => Some(1),
			Madness => Some(0),
			DoubleEdgedSword => Some(1),
			Dazed => None,
			Fear => None,
			Unease => Some(-1),
			Stress => Some(0),
			Exhaustion => None,
		}
	}
	
	pub fn can_play(&self, mana: i64) -> bool {
		self.mana_cost().map(|x| mana >= x).unwrap_or(false)
	}
	
	pub fn can_discard(&self, mana: i64) -> bool {
		self.discard_mana().map(|x| (mana as i64) + x >= 0).unwrap_or(false)
	}
	
	fn attack(&mut self, n: i64, player: &mut Player, target: &mut Enemy, enemies: &mut Vec<Enemy>) {
		player.attack((n + self.damage_modifier).max(1), target, enemies);
	}
	
	pub fn description(&self, player: &Player) -> String {
		match self.typ {
			Strike => format!("deal {} damage to target",(5+self.damage_modifier+player.strength).max(1)),
			Defend => format!("gain {} block",(4+self.block_modifier).max(1)),
			PommelStrike => format!("deal {} to target and draw a card",(3+self.damage_modifier+player.strength).max(1)),
			CripplingStabs => format!("deal {} to target twice and reduce its strength by 2",(3+self.damage_modifier+player.strength).max(1)),
			SwordDraw => format!("deal {} to target and gain 1 strength for 3 turns",(2+self.damage_modifier+player.strength).max(1)),
			Barrier => format!("ethereal, gain {} block",(100+self.block_modifier).max(1)),
			Fortify => format!("at the start of your next 5 turns gain 2 block"),
			BlazeOfInsanity => format!("destroy all cards in your hand, deal {} damage to target for each card destroyed",(6+self.damage_modifier+player.strength).max(1)),
			RecklessStrike => format!("deal {} damage to target, gain 2 strength for 3 turns, all enemies gain 2 strength for 3 turns",(10+self.damage_modifier+player.strength).max(1)),
			Madness => format!("if this card is in your hand at the end of your turn lose 1 hp. when played, deal {} damage to target enemy, reduce play cost of target card by 2",(6+self.damage_modifier+player.strength)),
			DoubleEdgedSword => format!("deal {} damage to target, take 10 damage",(25+self.damage_modifier+player.strength)),
			Dazed => format!("unplayable, undiscardable, ethereal"),
			Fear => format!("unplayable, undiscardable, ethereal, when you draw this lose 3 mana"),
			Unease => format!("unplayable, fleeting"),
			Stress => format!("unplayable, fleeting, when you discard this, lose 10 hp"),
			Exhaustion => format!("unplayable, undiscardable, ethereal, whenever you play or discard a card, lose 4 hp"),
		}
	}
	
	pub fn fmt(&self, player: &Player) -> String {
		let desc = self.description(player);
		if let Some(cost) = self.mana_cost() {
			if let Some(mana) = self.discard_mana() {
				format!("{},{} {}: {}",cost,mana,self.typ,desc)
			} else {
				format!("{}   {}: {}",cost,self.typ,desc)
			}
		} else {
			if let Some(mana) = self.discard_mana() {
				format!("  {} {}: {}",mana,self.typ,desc)
			} else {
				format!("    {}: {}",self.typ,desc)
			}
		}
	}
}

impl Interrupt for Card {
	fn interrupt(&self, _a: Action, _p: &Player, _e: &Vec<Enemy>) -> bool {
		//use self::Action::*;
		match self.typ {
			_ => false,
		}
	}
}

impl Trigger for Card {
	fn trigger(&mut self, a: Action, p: &mut Player, _e: &mut Vec<Enemy>) -> bool {
		use self::Action::*;
		match self.typ {
			Madness => if let Action::EndTurn = a { p.hp -= 1; },
			Exhaustion => match a {
				Playing(_card) => p.hp -= 4,
				Discarding(_card, _keep) => p.hp -= 4,
				_ => {},
			},
			_ => {},
		}
		false
	}
}

impl From<CardType> for Card {
	fn from(typ: CardType) -> Self {
		Self {
			damage_modifier: 0,
			block_modifier: 0,
			cost_modifier: 0,
			typ,
			data: Vec::new(),
		}
	}
}

impl fmt::Display for CardType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self { //TODO: macro this
			Strike => write!(f,"strike"),
			Defend => write!(f,"defend"),
			PommelStrike => write!(f,"pommel strike"),
			CripplingStabs => write!(f,"crippling stabs"),
			SwordDraw => write!(f,"sword draw"),
			Barrier => write!(f,"barrier"),
			Fortify => write!(f,"fortify"),
			BlazeOfInsanity => write!(f,"blaze of insanity"),
			RecklessStrike => write!(f,"reckless strike"),
			Madness => write!(f,"madness"),
			DoubleEdgedSword => write!(f,"double edged sword"),
			Dazed => write!(f,"dazed"),
			Fear => write!(f,"fear"),
			Unease => write!(f,"unease"),
			Stress => write!(f,"stress"),
			Exhaustion => write!(f,"exhaustion"),
		}
	}
}

use self::CardType::*;

#[derive(Debug,Copy,Clone,Eq,PartialEq,Serialize,Deserialize)]
pub enum CardType {
	//basic
	Strike,
	Defend,
	//normal
	PommelStrike,
	CripplingStabs,
	SwordDraw,
	Barrier,
	Fortify,
	BlazeOfInsanity,
	RecklessStrike,
	Madness,
	DoubleEdgedSword,
	//status
	Dazed,
	Fear,
	//curses
	Unease,
	Stress,
	Exhaustion,
}

#[derive(Debug,Copy,Clone,Serialize,Deserialize)]
pub enum PlayError {
	NeedsTarget,
	BadTarget,
	Unplayable,
}

const CURSE_LIST: &[(usize, CardType)] = &[
	(1, Unease),
	(2, Stress),
	(3, Exhaustion),
];

pub fn gen_curses(n: usize) -> Vec<Card> {
	let mut i = n;
	let mut r = Vec::new();
	while i > 0 {
		let idx = random::<usize>() % CURSE_LIST.len();
		if i >= CURSE_LIST[idx].0 {
			r.push(CURSE_LIST[idx].1.into());
			i -= CURSE_LIST[idx].0;
		}
	}
	r
}
