use crate::prelude::*;

use crate::terminal_command::Command;
use self::Command::*;

use std::rc::Rc;

pub fn gen_event_pool() -> Vec<(bool, Rc<Fn(&mut Player, &mut Vec<Enemy>, &terminal::Terminal<Command>)>)> {
	vec!(
		(true, Rc::new(andrews_thing)),
		(false, Rc::new(hp_for_curses)),
	)
}

pub fn get_statement(c: &terminal::Terminal<Command>) -> bool {
	loop {
		match c.next() {
			Yes => return true,
			No => return false,
			_ => println!("enter yes or no"),
		}
	}
}

pub fn andrews_thing(p: &mut Player, _e: &mut Vec<Enemy>, c: &terminal::Terminal<Command>) {
	println!("you meet a wild andrew, he offers to turn you into a turtle. [y/n]");
	if get_statement(c) {
		println!("with a wave of his toenails all your strikes become worse and all your defends become better");
		let a = p.deck.deck.iter_mut();
		let b = p.deck.inf.iter_mut();
		for card in a.chain(b) {
			if card.typ == CardType::Strike {
				card.damage_modifier -= 1;
			} else if card.typ == CardType::Defend {
				card.block_modifier += 1;
			}
		}
	} else {
		println!("that's a shame, turtles are really nice");
	}
}

pub fn hp_for_curses(p: &mut Player, _e: &mut Vec<Enemy>, c: &terminal::Terminal<Command>) {
	println!("you come across a sacred healing fountain, but some kind of dark contaminant has seeped into the water");
	println!("drink from the fountain? [y/n]");
	if get_statement(c) {
		println!("your body is reinvigorated but your mind is struck with malaise");
		p.hp += 200;
		p.deck.add_cards(crate::card::gen_curses(random::<usize>() % 200));
	} else {
		println!("better not to risk it");
	}
}
