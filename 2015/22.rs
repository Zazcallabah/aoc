
#[derive(Clone,Debug)]
struct State {
	boss_hp : i32,
	boss_atk : i32,
	hp: i32,
	mana: i32,
	shield: i32,
	poison: i32,
	recharge: i32,
	spent: i32,
}

type PlayerWin = bool;

impl State {
	fn simulate(&mut self, hard:bool) -> Option<i32> {
		let mut r = 100000;

		for next_choice in 0..=4 {
			if let Some(result) = self.whatif(next_choice,r,hard) {
				if result < r {
					r = result;
				}
			}
		}
		Some(r)
	}

	fn choice_is_valid(&mut self, choice: i8) -> bool {
		if self.shield > 1 && choice == 2 {
			false
		}
		else if self.recharge > 1 && choice == 4 {
			false
		}
		else if self.poison > 1 && choice == 3 {
			false
		}
		else {
			true
		}
	}

	fn handle_effects(&mut self) {
		if self.shield > 0 {
			self.shield -= 1;
		}
		if self.recharge > 0 {
			self.mana += 101;
			self.recharge -= 1;
		}
		if self.poison > 0 {
			self.poison -= 1;
			self.boss_hp -= 3;
		}
	}

	fn do_choice(&mut self, choice: i8, hard: bool) -> Option<PlayerWin> {
		if ! self.choice_is_valid(choice) {
			return Some(false)
		}

		if hard {
			self.hp -= 1;
			if self.hp <= 0 {
				return Some(false)
			}
		}

		self.handle_effects();

		if self.boss_hp <= 0 {
			return Some(true)
		}

		let result = match choice {
			4 => self.recharge(),
			1 => self.drain(),
			2 => self.shield(),
			3 => self.poison(),
			0 => self.missile(),
			_ => panic!("bad choice"),
		};

		if let Some(win) = result {
			return Some(win)
		}

		self.boss_turn()
	}

	fn whatif(&mut self, choice: i8, best: i32, hard: bool) -> Option<i32> {
		let mut next = self.clone();

		let result = next.do_choice(choice, hard);

		if let Some(win) = result {
			if win && next.spent < best {
				return Some(next.spent)
			}
			else {
				return None
			}
		}
		if next.spent >= best {
			return None
		}

		let mut r = best;

		for next_choice in 0..=4 {
			if let Some(result) = next.whatif(next_choice,r,hard) {
				if result < r {
					r = result;
				}
			}
		}
		Some(r)
	}

	fn missile(&mut self) -> Option<PlayerWin> {
		if self.mana < 53 {
			return Some(false)
		}
		self.mana -= 53;
		self.spent += 53;
		self.boss_hp -= 4;
		if self.boss_hp <= 0 {
			Some(true)
		}
		else {
			None
		}
	}

	fn drain(&mut self) -> Option<PlayerWin> {
		if self.mana < 73 {
			return Some(false)
		}
		self.mana -= 73;
		self.spent += 73;
		self.hp += 2;
		self.boss_hp -= 2;
		if self.boss_hp <= 0 {
			Some(true)
		}
		else {
			None
		}
	}

	fn shield(&mut self) -> Option<PlayerWin> {
		if self.shield > 0 || self.mana < 113 {
			return Some(false)
		}
		self.mana -= 113;
		self.spent += 113;
		self.shield = 6;
		None
	}

	fn poison(&mut self) -> Option<PlayerWin> {
		if self.poison > 0 || self.mana < 173 {
			return Some(false)
		}
		self.mana -= 173;
		self.spent += 173;
		self.poison = 6;
		None
	}

	fn recharge(&mut self) -> Option<PlayerWin> {
		if self.recharge > 0 || self.mana < 229 {
			return Some(false)
		}
		self.mana -= 229;
		self.spent += 229;
		self.recharge = 5;
		None
	}

	fn boss_turn(&mut self) -> Option<PlayerWin> {
		if self.shield == 1 {
			panic!("shield should never have this state here")
		}
		if self.poison == 1 {
			panic!("poison should never have this state here")
		}
		self.handle_effects();
		if self.boss_hp <= 0 {
			return Some(true)
		}
		self.hp -= if self.shield > 0 { self.boss_atk - 7 } else { self.boss_atk };
		if self.hp <= 0 {
			Some(false)
		}
		else {
			None
		}
	}

	fn new() -> State {
		State {
			boss_atk : 9,
			boss_hp : 51,
			hp : 50,
			mana : 500,
			shield : 0,
			poison : 0,
			recharge : 0,
			spent : 0,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_poison() {
		let mut s = State::new();
		let r = s.poison();
		assert_eq!(None,r);
		assert_eq!(173,s.spent);
		assert_eq!(6,s.poison);

		let mut s = State::new();
		s.mana = 172;
		let r = s.poison();
		assert_eq!(Some(false),r);
		assert_eq!(0,s.spent);
		assert_eq!(0,s.poison);
		assert_eq!(172,s.mana);

		let mut s = State::new();
		s.poison = 1;
		let r = s.poison();
		assert_eq!(Some(false),r);
		assert_eq!(0,s.spent);
		assert_eq!(1,s.poison);
	}

	#[test]
	fn test_simulate() {
		let mut s = State::new();
		assert_eq!(Some(900),s.simulate(false));
	}
	#[test]
	fn test_simulate_hard() {
		let mut s = State::new();
		assert_eq!(Some(1216),s.simulate(true));
	}

	#[test]
	fn test_can_clone() {
		let s = State::new();
		let mut s2 = s.clone();
		s2.mana = 3;
		assert_eq!(500,s.mana);
	}

	#[test]
	fn test_final_round() {
		let mut s = State::new();
		s.hp = 2;
		s.mana = 77;
		s.boss_hp = 10;
		s.poison = 5;

		let r = s.whatif(0, 100000,false);

		assert_eq!(Some(53), r);
	}

	#[test]
	fn test_boss_turn() {
		let mut s = State::new();
		s.hp = 100;
		s.boss_hp = 100;
		s.mana = 0;
		s.poison = 2;
		s.shield = 2;
		s.recharge = 1;
		let r = s.boss_turn();

		assert_eq!(None,r);
		assert_eq!(101,s.mana);
		assert_eq!(97,s.boss_hp);
		assert_eq!(98,s.hp);
		assert_eq!(1,s.poison);
		assert_eq!(1,s.shield);
		assert_eq!(0,s.recharge);

		let mut s = State::new();
		s.hp = 1;
		s.boss_hp = 3;
		s.mana = 0;
		s.poison = 2;
		let r = s.boss_turn();
		assert_eq!(Some(true),r);
		assert_eq!(0,s.mana);
		assert_eq!(0,s.boss_hp);
		assert_eq!(1,s.hp);
		assert_eq!(1,s.poison);
		assert_eq!(0,s.shield);
		assert_eq!(0,s.recharge);
	}
}