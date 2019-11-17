use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::collections::{HashMap,HashSet};

#[derive(Debug, Clone, Copy, Hash, Eq, Ord, PartialEq, PartialOrd)]
struct Gen {
	floor: i8,
	id: char,
}

#[derive(Debug, Clone, Copy, Hash, Eq, Ord, PartialEq, PartialOrd)]
struct Chip {
	floor: i8,
	id: char,
}

#[derive(Debug, Clone)]
struct State {
	chips: HashMap<char,Chip>,
	gens: HashMap<char,Gen>,
	ev: i8,
}

impl std::fmt::Display for State {
	fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		for f in 1..=4 {
			let e = if self.ev == f { "E" } else {"."};
			let cid : Vec<char> = self.chips(f).iter().map(|c| c.id).collect();
			let gid : Vec<char> = self.gens(f).iter().map(|g| g.id.to_ascii_uppercase()).collect();

			let _ = writeln!(fmt, "F{} {} {:?} {:?}", f, e, cid, gid );
		}
		Ok(())
	}
}

impl Hash for State {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.ev.hash(state);
		let mut c : Vec<&Chip> = self.chips.iter().map(|(_,c)| c).collect();
		c.sort();
		for cx in c.iter() {
			cx.hash(state);
		}
		let mut g : Vec<&Gen> = self.gens.iter().map(|(_,c)| c).collect();
		g.sort();
		for gx in g.iter() {
			gx.hash(state);
		}
	}
}

impl State {
	fn new(chips:Vec<Chip>,gens:Vec<Gen>) -> State {
		let chips2 = chips.into_iter().map(|c|(c.id,c)).collect();
		let gens2 = gens.into_iter().map(|g|(g.id,g)).collect();
		State{chips:chips2,gens:gens2,ev:1}
	}
	fn data() -> State {
		let chips = vec![Chip{floor:1,id:'s'},Chip{floor:1,id:'p'},Chip{floor:3,id:'t'},Chip{floor:2,id:'r'},Chip{floor:2,id:'c'}];
		let gens = vec![  Gen{floor:1,id:'s'}, Gen{floor:1,id:'p'}, Gen{floor:2,id:'t'}, Gen{floor:2,id:'r'}, Gen{floor:2,id:'c'}];
		State::new(chips,gens)
	}
	fn data2() -> State {
		let chips = vec![Chip{floor:1,id:'s'},Chip{floor:1,id:'p'},Chip{floor:3,id:'t'},Chip{floor:2,id:'r'},Chip{floor:2,id:'c'},Chip{floor:1,id:'e'},Chip{floor:1,id:'d'}];
		let gens = vec![  Gen{floor:1,id:'s'}, Gen{floor:1,id:'p'}, Gen{floor:2,id:'t'}, Gen{floor:2,id:'r'}, Gen{floor:2,id:'c'}, Gen{floor:1,id:'e'}, Gen{floor:1,id:'d'}];
		State::new(chips,gens)
	}
	fn test() -> State {
		let chips = vec![Chip{floor:1,id:'h'},Chip{floor:1,id:'l'}];
		let gens = vec![Gen{floor:2,id:'h'},Gen{floor:3,id:'l'}];
		State::new(chips,gens)
	}

	fn get_hash(&self) -> u64 {
		let mut s = DefaultHasher::new();
		self.hash(&mut s);
		s.finish()
	}

	fn get_chip(&mut self, id:&char) -> &mut Chip {
		self.chips.get_mut(&id).unwrap()
	}
	fn get_gen(&mut self, id:&char) -> &mut Gen {
		self.gens.get_mut(&id).unwrap()
	}

	fn is_done(&self) -> bool {
		self.chips.iter().all(|(_,c)| c.floor == 4) && self.gens.iter().all(|(_,g)| g.floor == 4) && self.ev == 4
	}
	fn chips(&self,floor:i8) -> Vec<&Chip>{
		self.chips.iter().map(|(_,c)|c).filter(|c| c.floor == floor).collect()
	}
	fn gens(&self,floor:i8) -> Vec<&Gen>{
		self.gens.iter().map(|(_,g)|g).filter(|g| g.floor == floor ).collect()
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

	fn movechip(&self,c:&Chip,c2:&Chip,up:bool)-> Option<State> {
		if up && c.floor == 4 || !up && c.floor == 1 {
			return None
		}
		let delta = if up { 1 } else { -1 };
		let mut s = self.clone();
		s.ev += delta;
		s.get_chip(&c.id).floor += delta;
		if c.id != c2.id {
			s.get_chip(&c2.id).floor += delta;
		}
		if s.is_valid() {
			Some(s)
		}
		else {
			None
		}
	}

	fn movegen(&self,c:&Gen,c2:&Gen,up:bool)-> Option<State> {
		if up && c.floor == 4 || !up && c.floor == 1 {
			return None
		}
		let delta = if up { 1 } else { -1 };
		let mut s = self.clone();
		s.ev += delta;
		s.get_gen(&c.id).floor += delta;
		if c.id != c2.id {
			s.get_gen(&c2.id).floor += delta;
		}
		if s.is_valid() {
			Some(s)
		}
		else {
			None
		}
	}

	fn moveboth(&self,c:&Chip,g:&Gen,up:bool)-> Option<State> {
		if up && c.floor == 4 || !up && c.floor == 1 {
			return None
		}
		let delta = if up { 1 } else { -1 };
		let mut s = self.clone();
		s.get_chip(&c.id).floor += delta;
		s.get_gen(&g.id).floor += delta;
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
				if let Some(s) = self.movechip(chip,chip2,true) {
					moves.push(s);
				}
				if chip.id == chip2.id {
					if let Some(s) = self.movechip(chip,chip2,false) {
						moves.push(s);
					}
				}
			}
		}
		for (i,gen) in gens.iter().enumerate() {
			for gen2 in &gens[i..] {
				if let Some(s) = self.movegen(gen,gen2,true) {
					moves.push(s);
				}
				if gen.id == gen2.id {
					if let Some(s) = self.movegen(gen,gen2,false) {
						moves.push(s);
					}
				}
			}
			for chip in &chips {
				if let Some(s) = self.moveboth(chip,gen,true) {
					moves.push(s);
				}
			}
		}
		moves
	}
}

fn main() {

	let now = std::time::SystemTime::now();

	let mut seen : HashSet<u64> = HashSet::new();
	let s = State::data();
	println!("{}",s);
	seen.insert(s.get_hash());

	let mut backlog : Vec<State> = vec![s];

	let mut movecount = 0u32;
	'outer: loop {

		let moveset = backlog;
		backlog = Vec::new();
		println!("{} options", moveset.len());
		for o in moveset {

			seen.insert(o.get_hash());

			let mut new_moves : Vec<State> = o.get_moves().into_iter().filter(|m| !seen.contains(&m.get_hash())).collect();

			for nm in &new_moves {
				if nm.is_done() {
					println!("found final state after {} + 1 moves\n{}",movecount,nm);
					break 'outer;
				}

				let h = nm.get_hash();

				seen.insert(h);
			}
			backlog.append(&mut new_moves);
		}

		println!("for move {}",movecount);

		movecount += 1;

	}

	println!("done at count {}, took {}s",movecount+1,now.elapsed().unwrap().as_secs());

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
		t.gens.get_mut(&'h').unwrap().floor = 1;
		assert!(!t.is_valid());
		assert!(State::data().is_valid());
	}

	#[test]
	fn test_canhash(){
		let mut t = State::test();
		assert_ne!(State::data().get_hash(),t.get_hash());
		let t2 = State::test();
		assert_eq!(t2.get_hash(),t.get_hash());
		t.get_chip(&'h').floor = 4;
		assert_ne!(t2.get_hash(),t.get_hash());
	}

	#[test]
	fn test_canmovechipup(){
		let t = State::test();
		let c = t.chips.get(&'h').unwrap();
		let newstate = t.movechip(&c, &c, true).unwrap();

		assert!(newstate.is_valid());
	}
}


// The experimental RTGs have poor radiation containment, so they're dangerously radioactive. The chips are
// prototypes and don't have normal radiation shielding, but they do have the ability to generate an electromagnetic
// radiation shield when powered. Unfortunately, they can only be powered by their corresponding RTG. An RTG powering
// a microchip is still dangerous to other microchips.

// In other words, if a chip is ever left in the same area as another RTG, and it's not connected to its own RTG,
//  the chip will be fried. Therefore, it is assumed that you will follow procedure and keep chips connected to their
//   corresponding RTG when they're in the same room, and away from other RTGs otherwise.

// These microchips sound very interesting and useful to your current activities, and you'd like to try to retrieve them.
//  The fourth floor of the facility has an assembling machine which can make a self-contained, shielded computer for you
//   to take with you - that is, if you can bring it all of the RTGs and microchips.

// Within the radiation-shielded part of the facility (in which it's safe to have these pre-assembly RTGs), there is
//  an elevator that can move between the four floors. Its capacity rating means it can carry at most yourself and two
//  RTGs or microchips in any combination. (They're rigged to some heavy diagnostic equipment - the assembling machine
//  will detach it for you.) As a security measure, the elevator will only function if it contains at least one RTG or
//  microchip. The elevator always stops on each floor to recharge, and this takes long enough that the items within it
//   and the items on that floor can irradiate each other. (You can prevent this if a Microchip and its Generator end up
// 	 on the same floor in this way, as they can be connected while the elevator is recharging.)

// You make some notes of the locations of each component of interest (your puzzle input). Before you don a hazmat
// suit and start moving things around, you'd like to have an idea of what you need to do.

// When you enter the containment area, you and the elevator will start on the first floor.

// For example, suppose the isolated area has the following arrangement:

// F4 .  .  .  .  .
// F3 .  .  .  LG .
// F2 .  HG .  .  .
// F1 E  .  HM .  LM

// Then, to get everything up to the assembling machine on the fourth floor, the following steps could be taken:

//     Bring the Hydrogen-compatible Microchip to the second floor, which is safe because it can get power from the Hydrogen Generator:

//     F4 .  .  .  .  .
//     F3 .  .  .  LG .
//     F2 E  HG HM .  .
//     F1 .  .  .  .  LM

//     Bring both Hydrogen-related items to the third floor, which is safe because the Hydrogen-compatible microchip is getting power from its generator:

//     F4 .  .  .  .  .
//     F3 E  HG HM LG .
//     F2 .  .  .  .  .
//     F1 .  .  .  .  LM

//     Leave the Hydrogen Generator on floor three, but bring the Hydrogen-compatible Microchip back down with you so you can still use the elevator:

//     F4 .  .  .  .  .
//     F3 .  HG .  LG .
//     F2 E  .  HM .  .
//     F1 .  .  .  .  LM

//     At the first floor, grab the Lithium-compatible Microchip, which is safe because Microchips don't affect each other:

//     F4 .  .  .  .  .
//     F3 .  HG .  LG .
//     F2 .  .  .  .  .
//     F1 E  .  HM .  LM

//     Bring both Microchips up one floor, where there is nothing to fry them:

//     F4 .  .  .  .  .
//     F3 .  HG .  LG .
//     F2 E  .  HM .  LM
//     F1 .  .  .  .  .

//     Bring both Microchips up again to floor three, where they can be temporarily connected to their corresponding generators while the elevator recharges, preventing either of them from being fried:

//     F4 .  .  .  .  .
//     F3 E  HG HM LG LM
//     F2 .  .  .  .  .
//     F1 .  .  .  .  .

//     Bring both Microchips to the fourth floor:

//     F4 E  .  HM .  LM
//     F3 .  HG .  LG .
//     F2 .  .  .  .  .
//     F1 .  .  .  .  .

//     Leave the Lithium-compatible microchip on the fourth floor, but bring the Hydrogen-compatible one so you can still use the elevator; this is safe because although the Lithium Generator is on the destination floor, you can connect Hydrogen-compatible microchip to the Hydrogen Generator there:

//     F4 .  .  .  .  LM
//     F3 E  HG HM LG .
//     F2 .  .  .  .  .
//     F1 .  .  .  .  .

//     Bring both Generators up to the fourth floor, which is safe because you can connect the Lithium-compatible Microchip to the Lithium Generator upon arrival:

//     F4 E  HG .  LG LM
//     F3 .  .  HM .  .
//     F2 .  .  .  .  .
//     F1 .  .  .  .  .

//     Bring the Lithium Microchip with you to the third floor so you can use the elevator:

//     F4 .  HG .  LG .
//     F3 E  .  HM .  LM
//     F2 .  .  .  .  .
//     F1 .  .  .  .  .

//     Bring both Microchips to the fourth floor:

//     F4 E  HG HM LG LM
//     F3 .  .  .  .  .
//     F2 .  .  .  .  .
//     F1 .  .  .  .  .

// In this arrangement, it takes 11 steps to collect all of the objects at the fourth floor for assembly. (Each elevator stop counts as one step, even if nothing is added to or removed from it.)

// In your situation, what is the minimum number of steps required to bring all of the objects to the fourth floor?
