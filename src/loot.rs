use crate::prelude::*;
use crate::equipment::Equipment;

pub struct Loot(pub Vec<LootInner>);

impl Loot {
	pub fn fmt(&self, player: &Player) -> String {
		let mut r = String::new();
		for l in self.0.iter() {
			r.push_str(&format!(" {},",l.fmt(player)));
		}
		r
	}
}

use self::LootInner::*;

#[derive(Debug,Clone,Serialize,Deserialize)]
pub enum LootInner {
	Cards(usize, Card),
	LootItem(Item),
}

impl LootInner {
	pub fn fmt(&self, player: &Player) -> String {
		let mut r = String::new();
		match self {
			Cards(n, card) => {
				r.push_str(&format!("{} {}",n,card.fmt(player)));
			},
			LootItem(item) => {
				r.push_str(&format!("{}",item));
			}
		}
		r
	}
}

#[derive(Debug,Clone,Serialize,Deserialize)]
pub enum Item {
	Potion(Potion),
	Helmet(Helmet),
}

impl fmt::Display for Item {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Item::Potion(x) => write!(f,"{}",x),
			Item::Helmet(x) => write!(f,"{}",x),
		}
	}
}

#[derive(Debug,Clone,Serialize,Deserialize)]
pub enum Potion {
	
}

impl fmt::Display for Potion {
	fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			_ => unimplemented!(),
		}
	}
}

use self::Helmet::*;

#[derive(Debug,Clone,Serialize,Deserialize)]
pub enum Helmet {
	HelmOfIntelligence,
}

impl Helmet {
	pub fn equip(&mut self, player: &mut Player, _equipment: &mut Equipment) {
		match self {
			HelmOfIntelligence => {
				player.hand_size += 1;
				player.card_draws += 1;
			},
		}
	}
	
	pub fn de_equip(&mut self, player: &mut Player, _equipment: &mut Equipment) -> bool {
		match self {
			HelmOfIntelligence => {
				player.hand_size -= 1;
				player.card_draws -= 1;
				true
			},
		}
	}
	
	pub fn name(&self) -> String {
		match self {
			HelmOfIntelligence => "helm of intelligence".to_string(),
		}
	}
	
	pub fn description(&self) -> String {
		match self {
			HelmOfIntelligence => "draw one more card per turn while under hand size limit, and hand size limit increased by one".to_string(),
		}
	}
}

impl fmt::Display for Helmet {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f,"{}: {}",self.name(),self.description())
	}
}

impl Interrupt for Helmet {
	fn interrupt(&self, _a: Action, _p: &Player, _e: &Vec<Enemy>) -> bool {
		//use self::Action::*;
		match self {
			_ => false,
		}
	}
}

impl Trigger for Helmet {
	fn trigger(&mut self, _a: Action, _p: &mut Player, _e: &mut Vec<Enemy>) -> bool {
		//use self::Action::*;
		match self {
			_ => {},
		}
		false
	}
}
