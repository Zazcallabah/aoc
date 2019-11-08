
// In this model, the same message is sent repeatedly. You've recorded the repeating
// message signal (your puzzle input), but the data seems quite corrupted - almost too badly to recover. Almost.
use std::collections::HashMap;

// All you need to do is figure out which character is most frequent for each position. For example, suppose you had recorded the following messages:
struct Collector {
	data : Vec<HashMap<char,i32>>
}

impl Collector {
	fn new(length:usize) -> Collector {
		let data = vec![HashMap::new();length];
		Collector{data}
	}

	fn add(&mut self, s:&str) {
		for (i,c) in s.chars().enumerate() {
			if self.data[i].contains_key(&c) {
				*self.data[i].get_mut(&c).unwrap() += 1;
			}
			else {
				self.data[i].insert(c,1);
			}
		}
	}

	fn common(&self, ix:usize, common:bool)->char{
		let v = &self.data[ix];
		let mut best : (&char,&i32) = if common { (&' ',&0) } else { (&' ',&std::i32::MAX) };
		for (letter,count) in v.iter() {
			if common && count > best.1 {
				best = (letter,count);
			}
			if !common && count < best.1 {
				best = (letter,count);
			}
		}
		best.0.clone()
	}

	fn read_word(&self, common: bool) -> String {
		let mut s = String::new();
		for i in 0..self.data.len() {
			s.push(self.common(i,common));
		}
		s
	}
}

fn main() {
	let mut c = Collector::new(8);

	for l in std::fs::read_to_string("6.txt").unwrap().lines() {
		c.add(&l);
	}
	println!("common {}",c.read_word(true));
	println!("uncommon {}",c.read_word(false));
}

#[cfg(test)]
mod tests {
	use super::*;


	#[test]
	fn test_get_word() {
		let mut c = Collector::new(4);
		c.add("aoeu");
		c.add("abcd");
		c.add("xyza");
		c.add("yyea");

		assert_eq!("ayea",c.read_word(true));
	}

	#[test]
	fn test_uncommon() {
		let mut c = Collector::new(4);
		c.add("ayeu");
		c.add("abca");
		c.add("xyea");
		c.add("ayea");

		assert_eq!("xbcu",c.read_word(false));
	}

	#[test]
	fn test_can_count() {
		let mut c = Collector::new(4);
		c.add("aoeu");
		c.add("abcd");
		c.add("xyza");
		c.add("yyea");

		assert_eq!('a',c.common(0,true));
		assert_eq!('y',c.common(1,true));
		assert_eq!('e',c.common(2,true));
		assert_eq!('a',c.common(3,true));
	}

	#[test]
	fn test_can_add() {
		let mut c = Collector::new(3);
		c.add("aoe");

		assert_eq!(1,c.data[1][&'o']);
		assert!(!c.data[1].contains_key(&'a'));
	}

	#[test]
	fn test_collector() {
		let mut c = Collector::new(3);
		c.data[1].insert('c',1);

		assert_eq!(0,c.data[0].len());
		assert_eq!(1,c.data[1][&'c']);
	}
}

// eedadn
// drvtee
// eandsr
// raavrd
// atevrs
// tsrnev
// sdttsa
// rasrtv
// nssdts
// ntnada
// svetve
// tesnvt
// vntsnd
// vrdear
// dvrsen
// enarar

// The most common character in the first column is e; in the second, a; in the third, s, and so on. Combining these characters returns the error-corrected message, easter.

// Given the recording in your puzzle input, what is the error-corrected version of the message being sent?