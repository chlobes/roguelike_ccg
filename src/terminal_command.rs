#[derive(Debug,Clone)]
pub enum Command {
	Numbers(Vec<isize>),
	End(usize),
	Discard(usize),
	Play(usize, Option<usize>),
	Yes,
	No,
	Stop,
}

use self::Command::*;
use terminal::Parse;

impl Parse for Command {
	fn parse(s: &String) -> Option<Self> {
		let s = s.to_lowercase();
		let mut words = s.split_whitespace().map(|w| w.trim()).rev().collect::<Vec<_>>();
		if words.is_empty() {
			return None
		}
		if let Some(w) = words.pop() {
			if let Ok(n) = w.parse() {
				let mut numbers = vec!(n);
				while let Some(Ok(n)) = words.pop().map(|w| w.parse()) {
					numbers.push(n);
				}
				return Some(Numbers(numbers));
			}
			match w {
				"end" => if let Some(w) = words.pop() {
					if let Ok(n) = w.parse() {
						Some(End(n))
					} else {
						println!("'{}' is not a number",w);
						None
					}
				} else {
					println!("input a card number to discard as you end your turn");
					None
				},
				"discard" => if let Some(w) = words.pop() {
					if let Ok(n) = w.parse() {
						Some(Discard(n))
					} else {
						println!("'{}' is not a number",w);
						None
					}
				} else {
					println!("input a card number to discard");
					None
				},
				"play" => if let Some(w) = words.pop() {
					if let Ok(n) = w.parse() {
						if let Some(w) = words.pop() {
							if let Ok(n1) = w.parse() {
								Some(Play(n, Some(n1)))
							} else {
								Some(Play(n, None))
							}
						} else {
							Some(Play(n, None))
						}
					} else {
						println!("'{}' is not a number",w);
						None
					}
				} else {
					println!("input a card number to play");
					None
				}
				"stop" => Some(Stop),
				_ => {
					if w.starts_with('y') {
						Some(Yes)
					} else if w.starts_with('n') {
						Some(No)
					} else {
						println!("commands are: end, discard, play, stop");
						None
					}
				}
			}
		} else {
			None
		}
	}
}
