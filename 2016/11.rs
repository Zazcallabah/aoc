use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::collections::{HashSet};

#[derive(Debug, Clone, Copy, Hash, Eq, Ord, PartialEq, PartialOrd)]
struct Gen {
	floor: i8,
	id: u8,
}

#[derive(Debug, Clone, Copy, Hash, Eq, Ord, PartialEq, PartialOrd)]
struct Chip {
	floor: i8,
	id: u8,
}

#[derive(Debug, Clone, Hash, Eq, Ord, PartialEq, PartialOrd)]
struct State {
	chips: Vec<Chip>,
	gens: Vec<Gen>,
	ev: i8,
}

const CLOOKUP: [char; 7] = ['s','p','t','r','c','e','d'];

impl std::fmt::Display for State {
	fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		for f in 1..=4 {
			let e = if self.ev == f { "E" } else {"."};
			let cid : Vec<char> = self.chips(f).iter().map(|c| CLOOKUP[c.id as usize]).collect();
			let gid : Vec<char> = self.gens(f).iter().map(|g| CLOOKUP[g.id as usize].to_ascii_uppercase()).collect();

			let _ = writeln!(fmt, "F{} {} {:?} {:?}", f, e, cid, gid );
		}
		Ok(())
	}
}

impl State {
	fn new(chip_floors:&[i8],gen_floors:&[i8]) -> State {
		let chips = chip_floors.iter().enumerate().map(|(i,f)|Chip{id:i as u8,floor:*f}).collect();
		let gens = gen_floors.iter().enumerate().map(|(i,f)|Gen{id:i as u8,floor:*f}).collect();
		State{chips,gens,ev:1}
	}
	fn data() -> State {
		let chips = vec![1,1,3,2,2];
		let gens = vec![1,1,2,2,2];
		State::new(&chips,&gens)
	}
	fn data2() -> State {
		let chips = vec![1,1,3,2,2,1,1];
		let gens = vec![1,1,2,2,2,1,1];
		State::new(&chips,&gens)
	}
	fn test() -> State {
		State::new(&[1,1],&[2,3])
	}

	fn get_hash(&self) -> u64 {
		let mut s = DefaultHasher::new();
		self.hash(&mut s);
		s.finish()
	}

	fn is_done(&self) -> bool {
		self.chips.iter().all(|c| c.floor == 4) && self.gens.iter().all(|g| g.floor == 4) && self.ev == 4
	}
	fn chips(&self,floor:i8) -> Vec<Chip>{
		self.chips.iter().filter(|c| c.floor == floor).cloned().collect()
	}
	fn gens(&self,floor:i8) -> Vec<Gen>{
		self.gens.iter().filter(|g| g.floor == floor ).cloned().collect()
	}
	fn is_valid(&self) -> bool {
		for f in 1..=4 {
			let floor_gens = self.gens(f);
			if !floor_gens.is_empty() {
				// no unpaired chips on a floor with any gen
				let floor_chips = self.chips(f);
				for chip in floor_chips {
					if floor_gens.iter().all(|g| g.id != chip.id) {
						return false
					}
				}
			}
		}
		true
	}

	fn movechip(&self,c:Chip,c2:Chip,up:bool)-> Option<State> {
		if up && c.floor == 4 || !up && c.floor == 1 {
			return None
		}
		let delta = if up { 1 } else { -1 };
		let mut s = self.clone();
		s.ev += delta;
		s.chips[c.id as usize].floor += delta;
		if c.id != c2.id {
			s.chips[c2.id as usize].floor += delta;
		}
		if s.is_valid() {
			Some(s)
		}
		else {
			None
		}
	}

	fn movegen(&self,c:Gen,c2:Gen,up:bool)-> Option<State> {
		if up && c.floor == 4 || !up && c.floor == 1 {
			return None
		}
		let delta = if up { 1 } else { -1 };
		let mut s = self.clone();
		s.ev += delta;
		s.gens[c.id as usize].floor += delta;
		if c.id != c2.id {
			s.gens[c2.id as usize].floor += delta;
		}
		if s.is_valid() {
			Some(s)
		}
		else {
			None
		}
	}

	fn moveboth(&self,c:Chip,g:Gen,up:bool)-> Option<State> {
		if up && c.floor == 4 || !up && c.floor == 1 {
			return None
		}
		let delta = if up { 1 } else { -1 };
		let mut s = self.clone();
		s.chips[c.id as usize].floor += delta;
		s.gens[g.id as usize].floor += delta;
		s.ev += delta;
		if s.is_valid() {
			Some(s)
		}
		else {
			None
		}
	}

	fn get_moves(&self) -> Vec<State> {
		// only do down moves with one item
		let mut moves = Vec::new();
		let chips = self.chips(self.ev);
		let gens = self.gens(self.ev);
		for (i,chip) in chips.iter().enumerate() {
			for chip2 in &chips[i..] {
				if let Some(s) = self.movechip(*chip,*chip2,true) {
					moves.push(s);
				}
				if chip.id == chip2.id {
					if let Some(s) = self.movechip(*chip,*chip2,false) {
						moves.push(s);
					}
				}
			}
		}
		for (i,gen) in gens.iter().enumerate() {
			for gen2 in &gens[i..] {
				if let Some(s) = self.movegen(*gen,*gen2,true) {
					moves.push(s);
				}
				if gen.id == gen2.id {
					if let Some(s) = self.movegen(*gen,*gen2,false) {
						moves.push(s);
					}
				}
			}
			for chip in &chips {
				if let Some(s) = self.moveboth(*chip,*gen,true) {
					moves.push(s);
				}
			}
		}
		moves
	}
}

fn depth_search(mut current : Vec<State>, seen: &mut HashSet<u64>) -> u32 {
	let mut v = Vec::new();
	for o in current.drain(..) {

		let mut new_moves : Vec<State> = o.get_moves().into_iter().filter(|m| !seen.contains(&m.get_hash())).collect();

		for nm in &new_moves {
			if nm.is_done() {
				return 1
			}

			seen.insert(nm.get_hash());
		}
		v.append(&mut new_moves);
	}
	depth_search(v,seen) + 1
}

fn count( start: State ) -> u32 {
	let mut seen : HashSet<u64> = HashSet::new();
	seen.insert(start.get_hash());
	depth_search(vec![start],&mut seen)
}

fn main() {

	let now = std::time::SystemTime::now();
	let moves = count(State::test());
	println!("test done at count {}, took {}ms",moves,now.elapsed().unwrap().as_millis());
	let now = std::time::SystemTime::now();
	let moves = count(State::data());
	println!("part 1 done at count {}, took {}ms",moves,now.elapsed().unwrap().as_millis());
	let now = std::time::SystemTime::now();
	let moves = count(State::data2());
	println!("part 2 done at count {}, took {}ms",moves,now.elapsed().unwrap().as_millis());

}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_canmakestate() {
		assert_eq!(2,State::test().chips.len());
	}

	#[test]
	fn test_can_get_forfloor(){
		let t = State::test();
		assert_eq!(0,t.chips(4).len());
		assert_eq!(2,t.chips(1).len());
	}

	#[test]
	fn test_isvalid(){
		let mut t = State::test();
		assert!(t.is_valid());
		t.gens[0].floor = 1;
		assert!(!t.is_valid());
		assert!(State::data().is_valid());
	}

	#[test]
	fn test_canmovechipup(){
		let t = State::test();
		let c = t.chips[0];
		let newstate = t.movechip(c, c, true).unwrap();

		assert!(newstate.is_valid());
	}

	#[test]
	fn test_can_count_testexample(){
		assert_eq!(11,count(State::test()));
	}
	#[test]
	fn test_can_count_part1(){
		assert_eq!(37,count(State::data()));
	}
}
