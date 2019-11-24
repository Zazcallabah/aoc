#[macro_use]
extern crate lazy_static;
use regex::Regex;
use std::collections::VecDeque;

struct Scramble {
	data:VecDeque<char>,
}

impl std::fmt::Display for Scramble {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		let row : String = self.data.iter().collect();
		write!(f, "{}", row)
	}
}

impl Scramble {
	fn swap_pos(&mut self,x:usize,y:usize){
		let tmp : char = self.data[x];
		self.data[x]=self.data[y];
		self.data[y]=tmp;
	}

	fn index_of(&self,c:char) -> Option<usize> {
		for (i,p) in self.data.iter().enumerate() {
			if *p == c {
				return Some(i)
			}
		}
		None
	}

	fn swap_let(&mut self,a:char,b:char){
		let first = self.index_of(a).unwrap();
		let second = self.index_of(b).unwrap();
		self.swap_pos(first, second);
	}

	fn rot_left(&mut self, l:usize) {
		for _ in 0..l {
			let tmp = self.data.pop_front().unwrap();
			self.data.push_back(tmp);
		}
	}

	fn rot_right(&mut self, l:usize){
		for _ in 0..l{
			let tmp = self.data.pop_back().unwrap();
			self.data.push_front(tmp);
		}
	}

	fn rot_index(&mut self, a:char){
		let ix = self.index_of(a).unwrap();
		if ix >= 4 {
			self.rot_right( ix+2 );
		}
		else {
			self.rot_right( ix+1 );
		}
	}
	fn rot_index_rev(&mut self, a:char){
// 7 from bcdefgha -> abcdefgh right 7 left 1
// 0 from abcdefgh -> habcdefg right 7 left 1
// 4 from efghabcd -> ghabcdef right 2 left 6
// 1 from habcdefg -> fghabcde right 6 left 2
// 5 from defghabc -> efghabcd right 1 left 7
// 2 from ghabcdef -> defghabc right 5 left 3
// 6 from cdefghab -> cdefghab same
// 3 from fghabcde -> bcdefgha right 4 left 4
		let lookup = [1,1,6,2,7,3,0,4];
		let ix = self.index_of(a).unwrap();
		self.rot_left( lookup[ix] );
	}

	fn rev(&mut self,mut x:usize,mut y:usize){
		while y>x {
			self.swap_pos(x, y);
			x += 1;
			y -= 1;
		}
	}

	fn mv(&mut self,x:usize,y:usize) {
		let tmp = self.data.remove(x).unwrap();
		self.data.insert(y,tmp);
	}

	fn new(s:&str) -> Scramble {
		let data : VecDeque<char> = s.chars().collect();
		Scramble{data}
	}

	fn parse_apply(&mut self, instr:&str, unscramble:bool){
		lazy_static! {
			static ref SWAPPO: Regex = Regex::new(r"swap position (\d+) with position (\d+)").unwrap();
			static ref SWAPLET: Regex = Regex::new(r"swap letter (\w+) with letter (\w+)").unwrap();
			static ref REVERSE: Regex = Regex::new(r"reverse positions (\d+) through (\d+)").unwrap();
			static ref ROTL: Regex = Regex::new(r"rotate left (\d+) step").unwrap();
			static ref ROTR: Regex = Regex::new(r"rotate right (\d+) step").unwrap();
			static ref MOV: Regex = Regex::new(r"move position (\d+) to position (\d+)").unwrap();
			static ref ROTBASED: Regex = Regex::new(r"rotate based on position of letter (\w+)").unwrap();
		}

		match &instr[0..8] {
			"swap pos" => {
				let cap = SWAPPO.captures(instr).unwrap();
				let x : usize = cap.get(1).unwrap().as_str().parse().unwrap();
				let y : usize = cap.get(2).unwrap().as_str().parse().unwrap();
				self.swap_pos(x,y);
			},
			"swap let" => {
				let cap = SWAPLET.captures(instr).unwrap();
				let x : char = cap.get(1).unwrap().as_str().chars().next().unwrap();
				let y : char = cap.get(2).unwrap().as_str().chars().next().unwrap();
				self.swap_let(x,y);
			},
			"reverse " => {
				let cap = REVERSE.captures(instr).unwrap();
				let x : usize = cap.get(1).unwrap().as_str().parse().unwrap();
				let y : usize = cap.get(2).unwrap().as_str().parse().unwrap();
				self.rev(x,y);
			},
			"rotate l" => {
				let cap = ROTL.captures(instr).unwrap();
				let x : usize = cap.get(1).unwrap().as_str().parse().unwrap();
				if unscramble {
					self.rot_right(x);
				}
				else {
					self.rot_left(x);
				}
			},
			"rotate r" => {
				let cap = ROTR.captures(instr).unwrap();
				let x : usize = cap.get(1).unwrap().as_str().parse().unwrap();
				if unscramble {
					self.rot_left(x);
				}
				else {
					self.rot_right(x);
				}
			},
			"rotate b" => {
				let cap = ROTBASED.captures(instr).unwrap();
				let x : char = cap.get(1).unwrap().as_str().chars().next().unwrap();
				if unscramble {
					self.rot_index_rev(x);
				}
				else {
					self.rot_index(x);
				}
			},
			"move pos" => {
				let cap = MOV.captures(instr).unwrap();
				let x : usize = cap.get(1).unwrap().as_str().parse().unwrap();
				let y : usize = cap.get(2).unwrap().as_str().parse().unwrap();
				if unscramble {
					self.mv(y,x);
				}
				else {
					self.mv(x,y);
				}
			},

			_ => panic!("bad instruction"),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_swap(){
		let mut s = Scramble::new("abcde");
		s.parse_apply("swap position 4 with position 0",false);
		assert_eq!("ebcda",s.to_string());
	}
	#[test]
	fn test_swap_letter(){
		let mut s = Scramble::new("ebcda");
		s.parse_apply("swap letter d with letter b",false);
		assert_eq!("edcba",s.to_string());
	}
	#[test]
	fn test_reverse(){
		let mut s = Scramble::new("edcba");
		s.parse_apply("reverse positions 0 through 4",false);
		assert_eq!("abcde",s.to_string());
	}
	#[test]
	fn test_move(){
		let mut s = Scramble::new("bcdea");
		s.parse_apply("move position 1 to position 4",false);
		assert_eq!("bdeac",s.to_string());
		s.parse_apply("move position 3 to position 0",false);
		assert_eq!("abdec",s.to_string());
	}
	#[test]
	fn test_rotate(){
		let mut s = Scramble::new("abcde");
		s.parse_apply("rotate left 1 step",false);
		assert_eq!("bcdea",s.to_string());
		s.parse_apply("rotate right 1 step",false);
		assert_eq!("abcde",s.to_string());
	}
	#[test]
	fn test_rotate_based(){
		let mut s = Scramble::new("abdec");
		s.parse_apply("rotate based on position of letter b",false);
		assert_eq!("ecabd",s.to_string());
		s.parse_apply("rotate based on position of letter d",false);
		assert_eq!("decab",s.to_string());
	}

	#[test]
	fn test_unscramble_simple(){
		let mut s = Scramble::new("abdec");
		for instr in r"swap position 4 with position 0
swap letter d with letter b
reverse positions 0 through 4
rotate left 1 step
move position 1 to position 4
move position 3 to position 0"
			.lines().rev() {
			s.parse_apply(instr,true);
		}
		assert_eq!("abcde",s.to_string());
	}

	#[test]
	fn test_unscramble_rotate_based(){
		let mut s = Scramble::new("habcdefg");
		s.parse_apply("rotate based on position of letter a",true);
		assert_eq!("abcdefgh",s.to_string());
		let mut s = Scramble::new("fghabcde");
		s.parse_apply("rotate based on position of letter a",true);
		assert_eq!("habcdefg",s.to_string());
		let mut s = Scramble::new("defghabc");
		s.parse_apply("rotate based on position of letter a",true);
		assert_eq!("ghabcdef",s.to_string());
		let mut s = Scramble::new("bcdefgha");
		s.parse_apply("rotate based on position of letter a",true);
		assert_eq!("fghabcde",s.to_string());
		let mut s = Scramble::new("ghabcdef");
		s.parse_apply("rotate based on position of letter a",true);
		assert_eq!("efghabcd",s.to_string());
		let mut s = Scramble::new("efghabcd");
		s.parse_apply("rotate based on position of letter a",true);
		assert_eq!("defghabc",s.to_string());
		let mut s = Scramble::new("cdefghab");
		s.parse_apply("rotate based on position of letter a",true);
		assert_eq!("cdefghab",s.to_string());
		let mut s = Scramble::new("abcdefgh");
		s.parse_apply("rotate based on position of letter a",true);
		assert_eq!("bcdefgha",s.to_string());
	}
}

fn main() {
	let mut s = Scramble::new( "abcdefgh" );
	let filestr = std::fs::read_to_string("21.txt").unwrap();
	for instr in filestr.lines() {
		s.parse_apply(instr,false);
	}
	println!("final scramble: {}",s);

	let mut s = Scramble::new( "fbgdceah" );
	for instr in filestr.lines().rev() {
		s.parse_apply(instr,true);
	}
	println!("final unscramble: {}",s);
}