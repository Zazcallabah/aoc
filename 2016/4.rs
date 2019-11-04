// Finally, you come across an information kiosk with a list of rooms. Of course, the list is encrypted and full of
// decoy data, but the instructions to decode the list are barely hidden nearby. Better remove the decoy data first.

// Each room consists of an encrypted name (lowercase letters separated by dashes) followed by a dash, a sector ID, and a checksum in square brackets.

// A room is real (not a decoy) if the checksum is the five most common letters in the encrypted name,
// in order, with ties broken by alphabetization. For example:

// Of the real rooms from the list above, the sum of their sector IDs is 1514.

// What is the sum of the sector IDs of the real rooms?
use regex::Regex;

struct Room {
	chk: String,
	id: u32,
	name: String,
}

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";

impl Room {
	fn real(&self) -> bool {
		self.checksum() == self.chk
	}

	fn decrypt(&self) -> String {
		let mut s = String::with_capacity(self.name.len());
		for c in self.name.chars() {
			if c == '-' {
				s.push(' ');
			}
			else {
				let index : usize = (c as usize - 'a' as usize + self.id as usize) % ALPHABET.len();
				let lookup = &ALPHABET[index..=index];
				s.push_str(&lookup);
			}
		}
		s
	}

	fn checksum(&self) -> String {
		let mut hash = std::collections::HashMap::<char, u8>::new();
		for c in self.name.chars().filter(|c| c != &'-') {
			if hash.contains_key(&c) {
				*hash.get_mut(&c).unwrap() += 1;
			} else {
				hash.insert(c, 1);
			}
		}

		let mut result: Vec<(char, u8)> = Vec::new();
		for r in hash {
			result.push(r);
		}

		result.sort_by(|a, b| {
			if a.1 == b.1 {
				b.0.cmp(&a.0)
			} else {
				a.1.cmp(&b.1)
			}
		});
		let mut s = String::with_capacity(5);

		for _ in 0..5 {
			s.push(result.pop().unwrap().0);
		}

		s
	}
}

pub fn get_match(cap: &regex::Captures, name: &str) -> Option<u8> {
	if let Some(m) = cap.name(&name) {
		Some(m.as_str().parse().unwrap())
	} else {
		None
	}
}

fn parse(data: &str) -> Vec<Room> {
	let re =
		Regex::new(r"(?x)(?P<name>[a-z-]+)-(?P<id>[0-9]+)\[(?P<chk>[a-z]{5})\](\r\n|\n)?").unwrap();
	let mut v = Vec::with_capacity(re.captures_len());
	for cap in re.captures_iter(&data) {
		let chk = cap.name(&"chk").unwrap().as_str().to_owned();
		let id: u32 = cap.name(&"id").unwrap().as_str().parse().unwrap();
		let name = cap.name(&"name").unwrap().as_str().to_owned();
		v.push(Room { chk, id, name });
	}
	v
}

fn main() {
	let data = std::fs::read_to_string("4.txt").unwrap();
	let r: Vec<Room> = parse(&data);
	let mut sum = 0u32;
	for room in r.iter().filter(|r| r.real() ) {
		sum += room.id;
		let d = room.decrypt();
		if d.contains("north") {
			println!("{} -> {}",room.id,d);
		}
	}
	println!("sum is {}",sum);
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_regex() {
		let r: Vec<Room> = parse("aaaaa-bbb-z-y-x-123[abxyz]");

		assert_eq!(1, r.len());
		assert_eq!("abxyz", r[0].chk);
		assert_eq!(123, r[0].id);
		assert_eq!("aaaaa-bbb-z-y-x", r[0].name);
	}

	#[test]
	fn test_calchash() {
		let r: Vec<Room> = parse("aaaaa-bbb-z-y-x-123[abxyz]");
		assert_eq!("abxyz", r[0].checksum());
	}

	#[test]
	fn test_testdata() {
		let r: Vec<Room> = parse("aaaaa-bbb-z-y-x-123[abxyz]\r\na-b-c-d-e-f-g-h-987[abcde]\nnot-a-real-room-404[oarel]\ntotally-real-room-200[decoy]");
		assert!(r[0].real());
		assert!(r[1].real());
		assert!(r[2].real());
		assert!(!r[3].real());
	}

	#[test]
	fn test_decrypt() {
		let r: Vec<Room> = parse("qzmt-zixmtkozy-ivhz-343[aaaaa]");
		assert_eq!("very encrypted name",r[0].decrypt());
	}
}
