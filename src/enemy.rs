use crate::prelude::*;

use self::Intent::*;
use self::EnemyType::*;

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct Enemy {
	typ: EnemyType,
	pub hp: i64,
	block: i64,
	strength: i64,
	intent: Intent,
	data: Vec<i64>,
}

impl Enemy {
	pub fn damage(&mut self, _player: &mut Player, amount: i64) {
		let new_amount = (amount - self.block).max(0);
		self.block = (self.block - amount).max(0);
		self.hp -= new_amount;
		match self.typ {
			Louse => {
				self.block += self.data[0];
				self.data[0] = 0;
			},
			_ => {},
		}
	}
	
	pub fn activate(&mut self, player: &mut Player, enemies: &mut Vec<Enemy>) {
		self.block = 0;
		match self.intent {
			Attack(n) => {
				self.attack(n, player, enemies);
			},
			Block(n) => {
				self.block += n;
			},
			x => match self.typ {
				Louse => {
					match x {
						Debuff => player.mana -= 1,
						Buff => self.strength += 3,
						_ => unreachable!(),
					}
				},
				Sentry => {
					let c: Card = CardType::Dazed.into();
					player.deck.add_cards(vec!(c.clone(), c.clone(), c.clone()));
				},
			}
		}
		let i = match self.typ {
			Louse => {
				if let Attack(_) = self.intent {
					match random::<u64>() % 3 {
						0 => Attack((random::<u64>() % 3 + 4) as i64),
						1 => Debuff,
						_ => Buff,
					}
				} else {
					Attack((random::<u64>() % 3 + 4) as i64)
				}
			},
			Sentry => {
				self.data[0] += 1;
				self.data[0] %= 3;
				if self.data[0] == 1 {
					Debuff
				} else {
					Attack(8)
				}
			},
		};
		self.intent = i;
		if let Attack(ref mut n) = self.intent {
			*n += self.strength;
		}
	}
	
	pub fn add_strength(&mut self, s: i64) {
		if let Attack(ref mut n) = self.intent {
			*n = (s + *n).max(1);
		}
		self.strength += s;
	}
	
	pub fn start_fight(&mut self, _player: &mut Player, _enemies: &mut Vec<Enemy>) {
		match self.typ {
			_ => {},
		}
	}
	
	fn attack(&mut self, n: i64, player: &mut Player, enemies: &mut Vec<Enemy>) {
		player.damage(n.max(1), Some(self), enemies);
	}
	
	pub fn name(&self) -> String {
		format!("{}",self.typ)
	}
	
	pub fn die(self, _player: &mut Player, _enemies: &mut Vec<Enemy>, _idx: &mut usize) {
		println!("you have killed an enemy");
	}
}

impl Interrupt for Enemy {
	fn interrupt(&self, _a: Action, _p: &Player, _e: &Vec<Enemy>) -> bool {
		//use self::Action::*;
		match self.typ {
			_ => false,
		}
	}
}

impl Trigger for Enemy {
	fn trigger(&mut self, _a: Action, _p: &mut Player, _e: &mut Vec<Enemy>) -> bool {
		//use self::Action::*;
		match self.typ {
			_ => {},
		};
		false
	}
}

impl fmt::Display for Enemy {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f,"{}: hp: {}, block: {}, intent: {}",self.name(),self.hp,self.block,self.intent)
	}
}

impl From<EnemyType> for Enemy {
	fn from(e: EnemyType) -> Self {
		match e {
			Louse => Self {
				typ: e,
				hp: (random::<u64>() % 11 + 20) as i64,
				block: 0,
				strength: 0,
				intent: Attack((random::<u64>() % 3 + 4) as i64),
				data: vec!((random::<u64>() % 3 + 5) as i64),
			},
			Sentry => {
				let x = (random::<u64>() % 3) as i64;
				Self {
					typ: e,
					hp: 40,
					block: 0,
					strength: 0,
					intent: if x == 1 { Debuff } else { Attack(8) },
					data: vec!(x),
				}
			},
		}
	}
}

#[derive(Debug,Copy,Clone,Serialize,Deserialize)]
pub enum EnemyType {
	Louse,
	Sentry,
}

impl fmt::Display for EnemyType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Louse => write!(f,"Louse"),
			Sentry => write!(f,"Sentry"),
		}
	}
}

#[derive(Debug,Copy,Clone,Serialize,Deserialize)]
pub enum Intent {
	Attack(i64),
	Block(i64),
	Debuff,
	StrongDebuff,
	Buff,
	Unknown
}

impl fmt::Display for Intent {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Attack(n) => write!(f,"attacking for {}",n.max(&1)),
			Block(_n) => write!(f,"blocking"),
			Debuff => write!(f,"inflicting a debuff"),
			StrongDebuff => write!(f,"inflicting a powerful debuff"),
			Buff => write!(f,"using a buff"),
			Unknown => write!(f,"intentions are unknown"),
		}
	}
}
