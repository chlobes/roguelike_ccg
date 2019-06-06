use crate::prelude::*;

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct Equipment {
	pub helmet: Option<Helmet>,
	pub potions: Vec<Potion>,
}

impl Equipment {
	pub fn new() -> Self {
		Self {
			helmet: None,
			potions: Vec::new(),
		}
	}
	
	pub fn get_loot(&mut self, item: Item, player: &mut Player, commands: &terminal::Terminal<crate::terminal_command::Command>) -> bool {
		use crate::events::get_statement;
		match item {
			Item::Potion(p) => {
				self.potions.push(p);
				true
			},
			Item::Helmet(mut h) => {
				if self.helmet.is_some() {
					println!("are you sure you wish to replace {}, with {}?",self.helmet.as_ref().unwrap(),h);
					if !get_statement(commands) {
						return false;
					}
				}
				let mut h1 = mem::replace(&mut self.helmet, None);
				if h1.as_mut().map(|h| h.de_equip(player, self)).unwrap_or(true) {
					h.equip(player, self);
					self.helmet = Some(h);
					true
				} else {
					self.helmet = h1;
					false
				}
			},
		}
	}
	
}

impl Interrupt for Equipment {
	fn interrupt(&self, a: Action, p: &Player, e: &Vec<Enemy>) -> bool {
		if self.helmet.as_ref().map(|h| h.interrupt(a, p, e)).unwrap_or(false) {
			true
		//} else if 
			
		} else {
			false
		}
	}
}

impl Trigger for Equipment {
	fn trigger(&mut self, a: Action, p: &mut Player, e: &mut Vec<Enemy>) -> bool {
		if self.helmet.as_mut().map(|h| h.trigger(a, p, e)).unwrap_or(false) {
			let mut h = mem::replace(&mut self.helmet, None).unwrap();
			if !h.de_equip(p, self) {
				self.helmet = Some(h);
			}
		}
		false
	}
}
