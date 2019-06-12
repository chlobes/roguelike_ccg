mod terminal_command;
mod deck;
mod card;
mod player;
mod enemy;
mod loot;
mod equipment;
mod events;
//mod interrupt;
mod prelude;
use prelude::*;
use terminal_command::Command;
use self::Command::*;

fn main() {
	let mut player = Player::new();
	let mut player = &mut player;
	let mut wins = 0;
	let mut enemies = gen_enemies(wins, &mut player);
	let enemies = &mut enemies;
	let mut event_pool = crate::events::gen_event_pool();
	let commands = terminal::Terminal::start();
	
	'a: loop {
		if player.hp <= 0 {
			println!("you are dead");
			break 'a;
		}
		let mut i = 0;
		while i < enemies.len() {
			if enemies[i].hp <= 0 {
				let enemy = enemies.remove(i);
				enemy.die(player, enemies, &mut i);
			} else {
				i += 1;
			}
		}
		if enemies.is_empty() {
			wins += 1;
			let (mut n, mut loot) = gen_loot(wins);
			while n > 0 {
				println!("all enemies killed, choose up to {} items:",n);
				for (i, l) in loot.iter().enumerate() {
					println!("{} | {}",i,l.fmt(player));
				}
				match commands.next() {
					Numbers(num) => {
						for &num in num.iter() {
							let num = num as usize;
							if num < loot.len() {
								player.get_loot(loot.remove(num), &commands);
								n -= 1;
							} else {
								println!("that number loot does not exist");
							}
						}
					},
					Stop => n = 0,
					_ => {
						println!("enter a loot number or type stop to finish looting");
					},
				}
			}
			crate::events::run_event(wins, &mut event_pool, player, enemies, &commands);
			if enemies.is_empty() {
				*enemies = gen_enemies(wins, player);
			}
		}
		println!("\nenemies:");
		for (i, e) in enemies.iter().enumerate() {
			println!("{} | {}",i,e);
		}
		println!("-----------------------------------------");
		println!("\n{}",player);
		println!("-----------------------------------------");
		match commands.next() {
			End(n) => if player.end_turn(n, enemies) {
				for _ in 0..enemies.len() {
					let mut e = enemies.pop().unwrap();
					e.activate(player, enemies);
					enemies.insert(0, e);
				}
				player.start_turn(enemies);
			},
			Discard(n) => { player.discard(n, false, enemies); },
			Play(n, target1, target2) => { player.play(n, target1, target2, enemies); },
			Save(mut path) => {
				path += ".sav";
				if let Ok(file) = std::fs::File::create(path) {
					if let Err(e) = serialize_into(file, &(&wins, &player, &enemies, &event_pool)) {
						println!("error while saving: {:?}",e);
					} else {
						println!("saved");
					}
				}
			},
			Load(mut path) => {
				path += ".sav";
				if let Ok(file) = std::fs::File::open(path) {
					if let Ok((w, p, e, ep)) = deserialize_from(file) {
						wins = w;
						*player = p;
						*enemies = e;
						event_pool = ep;
					}
				}
			},
			Stop => break 'a,
			Numbers(_) | Yes | No => println!("unknown command"),
		}
	}
}

fn gen_enemies(wins: u64, player: &mut Player) -> Vec<Enemy> {
	let mut r = if wins % 2 == 0 {
		let a: Enemy = enemy::EnemyType::Louse.into();
		let b = enemy::EnemyType::Louse.into();
		let c = enemy::EnemyType::Louse.into();
		vec!(a, b, c)
	} else {
		let a: Enemy = enemy::EnemyType::Sentry.into();
		let b = enemy::EnemyType::Sentry.into();
		vec!(a, b)
	};
	let len = r.len();
	for _ in 0..len {
		let mut e = r.pop().unwrap();
		e.start_fight(player, &mut r);
		r.insert(0, e);
	}
	player.start_fight(&mut r);
	r
}

fn gen_loot(_wins: u64) -> (usize, Vec<Loot>) {
	use crate::card::CardType::*;
	(2,
	vec!(Loot(vec!(LootInner::Cards(50, PommelStrike.into()), LootInner::Cards(50, CripplingStabs.into()), LootInner::Cards(50, SwordDraw.into()).into())),
	Loot(vec!(LootInner::LootItem(Item::Helmet(Helmet::HelmOfIntelligence))))))
}
