#[macro_use]
extern crate lazy_static;

struct State {
	reg: [i32; 4],
	pc: usize,
}

impl State {
	fn new() -> State {
		State::from([0, 0, 0, 0])
	}
	fn from(reg: [i32; 4]) -> State {
		State { reg, pc: 0 }
	}
	fn lookup_register(name: &str) -> usize {
		match name {
			"a" => 0,
			"b" => 1,
			"c" => 2,
			"d" => 3,
			_ => panic!("bad name"),
		}
	}
}

trait Op {
	fn exec(&self, state: &mut State);
	fn ident(&self) -> u32;
}

struct Cpy {
	source: usize,
	target: usize,
}
impl Op for Cpy {
	fn exec(&self, state: &mut State) {
		state.reg[self.target] = state.reg[self.source];
		state.pc += 1;
	}
	fn ident(&self) -> u32 {
		1
	}
}

struct Cpi {
	val: i32,
	target: usize,
}
impl Op for Cpi {
	fn exec(&self, state: &mut State) {
		state.reg[self.target] = self.val;
		state.pc += 1;
	}
	fn ident(&self) -> u32 {
		2
	}
}

struct Inc {
	reg: usize,
}
impl Op for Inc {
	fn exec(&self, state: &mut State) {
		state.reg[self.reg] += 1;
		state.pc += 1;
	}
	fn ident(&self) -> u32 {
		3
	}
}

struct Dec {
	reg: usize,
}
impl Op for Dec {
	fn exec(&self, state: &mut State) {
		state.reg[self.reg] -= 1;
		state.pc += 1;
	}
	fn ident(&self) -> u32 {
		4
	}
}

struct Ji {
	jmp: isize,
}
impl Ji {
// jnz x y jumps to an instruction y away (positive means forward;
// negative means backward), but only if x is not zero.
// The jnz instruction moves relative to itself: an offset of -1
// would continue at the previous instruction, while an offset of 2
// would skip over the next instruction.

	fn jmp(state: &mut State, jmp: isize) {
		state.pc = (state.pc as isize + jmp) as usize;
	}
}
impl Op for Ji {
	fn exec(&self, state: &mut State) {
		Ji::jmp(state,self.jmp);
	}
	fn ident(&self) -> u32 {
		5
	}
}

struct Jnz {
	jmp: isize,
	reg: usize,
}
impl Op for Jnz {
	fn exec(&self, state: &mut State) {
		if state.reg[self.reg] != 0 {
			Ji::jmp(state,self.jmp);
		} else {
			state.pc += 1;
		}
	}
	fn ident(&self) -> u32 {
		6
	}
}

struct Prog {
	state: State,
	ops: Vec<Box<dyn Op>>,
}
impl Prog {
	fn run(&mut self) {
		while self.state.pc < self.ops.len() {
			let next = &self.ops[self.state.pc];
			next.exec(&mut self.state);
		}
	}

	fn from(data:&str) -> Prog {
		let mut v = Vec::new();
		for line in data.lines() {
			let op = Prog::parse(line);
			v.push( op );
		}

		Prog {
			state: State::new(),
			ops: v,
		}
	}

	fn parse(line: &str) -> Box<dyn Op> {
		lazy_static! {
			static ref PARSE: regex::Regex = regex::Regex::new(
				r"(?x)
		(?P<op>cpy|inc|dec|jnz)
		\s+
		(
			(?P<source>[a-d])
			|
			(?P<imm>-?[0-9]+)
		)
		(
			\s+
			(
			(?P<target>[a-d])
			|
			(?P<jmp>-?[0-9]+)
			)
		)?$"
			)
			.unwrap();
		}

		if let Some(cap) = PARSE.captures(line) {
			match cap.name(&"op").unwrap().as_str() {
				"cpy" => {
					let target = State::lookup_register(cap.name(&"target").unwrap().as_str());
					if let Some(imm) = cap.name(&"imm") {
						let val = imm.as_str().parse().unwrap();
						return Box::new(Cpi { val, target });
					} else {
						let source = State::lookup_register(cap.name(&"source").unwrap().as_str());
						return Box::new(Cpy { source, target });
					}
				}
				"inc" => {
					let reg = State::lookup_register(cap.name(&"source").unwrap().as_str());
					return Box::new(Inc { reg });
				}
				"dec" => {
					let reg = State::lookup_register(cap.name(&"source").unwrap().as_str());
					return Box::new(Dec { reg });
				}
				"jnz" => {
					if let Some(result) = cap.name(&"jmp") {
						let jmp : isize = result.as_str().parse().unwrap();
						if let Some(imm) = cap.name(&"imm") {
							let val : i32= imm.as_str().parse().unwrap();
							if val == 0 { panic!("invalid jump instr"); }
							return Box::new(Ji { jmp });
						} else {
							let reg = State::lookup_register(cap.name(&"source").unwrap().as_str());
							return Box::new(Jnz { reg, jmp, });
						}
					}
					else {
						panic!("dynamic jmp not implemented");
					}
				}
				_ => (),
			}
		}
		panic!("parsing error");
	}
}

fn main() {
	let mut p = Prog::from(r"cpy 1 a
cpy 1 b
cpy 26 d
jnz c 2
jnz 1 5
cpy 7 c
inc d
dec c
jnz c -2
cpy a c
inc a
dec b
jnz b -2
cpy c b
dec d
jnz d -6
cpy 14 c
cpy 14 d
inc a
dec d
jnz d -2
dec c
jnz c -5");
	p.run();

	println!("end state part 1 {:?}",p.state.reg);

	let mut p = Prog::from(r"cpy 1 c
cpy 1 a
cpy 1 b
cpy 26 d
jnz c 2
jnz 1 5
cpy 7 c
inc d
dec c
jnz c -2
cpy a c
inc a
dec b
jnz b -2
cpy c b
dec d
jnz d -6
cpy 14 c
cpy 14 d
inc a
dec d
jnz d -2
dec c
jnz c -5");
	p.run();
	println!("end state part 2 {:?}",p.state.reg);

}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn test_prog_jmp() {
		let mut p = Prog::from("cpy 5 a\ndec a\njnz a -1");
		p.run();
		assert_eq!([0, 0, 0, 0], p.state.reg);
		assert_eq!(3, p.state.pc);
	}

	#[test]
	fn test_prog() {
		let mut p = Prog::from("inc b\ndec d");
		p.run();
		assert_eq!([0, 1, 0, -1], p.state.reg);
		assert_eq!(2, p.state.pc);
	}

	#[test]
	fn test_inc_dec() {
		let mut st = State::new();
		Prog::parse("inc b").exec(&mut st);
		assert_eq!([0, 1, 0, 0], st.reg);
		Prog::parse("dec d").exec(&mut st);
		assert_eq!([0, 1, 0, -1], st.reg);
		assert_eq!(2, st.pc);
	}

	#[test]
	fn test_cpy_pc_inc() {
		let mut st = State::from([1, 2, 3, 4]);
		Prog::parse("cpy b a").exec(&mut st);
		assert_eq!(1, st.pc);
		Prog::parse("cpy 3 d").exec(&mut st);
		assert_eq!(2, st.pc);
	}

	#[test]
	fn test_cpy_parse() {
		let mut st = State::from([1, 2, 3, 4]);
		Prog::parse("cpy b a").exec(&mut st);
		assert_eq!([2, 2, 3, 4], st.reg);
		Prog::parse("cpy c d").exec(&mut st);
		assert_eq!([2, 2, 3, 3], st.reg);
	}

	#[test]
	fn test_cpi_parse() {
		let mut st = State::new();
		Prog::parse("cpy 40 a").exec(&mut st);
		assert_eq!([40, 0, 0, 0], st.reg);
		Prog::parse("cpy -99 b").exec(&mut st);
		assert_eq!([40, -99, 0, 0], st.reg);
		Prog::parse("cpy 0 a").exec(&mut st);
		assert_eq!([0, -99, 0, 0], st.reg);
	}

	#[test]
	fn test_parse_ops() {
		let b: Box<dyn Op> = Prog::parse("cpy 41 a");
		assert_eq!(2, b.ident());
		let b: Box<dyn Op> = Prog::parse("cpy b a");
		assert_eq!(1, b.ident());
	}

	#[test]
	fn test_parse_jump_ops() {
		let b: Box<dyn Op> = Prog::parse("jnz 41 -3");
		assert_eq!(5, b.ident());
		let b: Box<dyn Op> = Prog::parse("jnz b 3");
		assert_eq!(6, b.ident());
	}
	#[test]
	fn test_do_cpy() {
		let op = Cpy {
			source: 0,
			target: 1,
		};
		let mut st = State::from([1, 0, 0, 0]);
		op.exec(&mut st);
		assert_eq!([1, 1, 0, 0], st.reg);
	}

	#[test]
	fn test_do_cpy_and_cpi() {
		let op1 = Cpi { val: 99, target: 0 };
		let op2 = Cpy {
			source: 0,
			target: 1,
		};
		let ops: Vec<&dyn Op> = vec![&op1, &op2];
		let mut st = State::from([1, 2, 3, 4]);
		for op in ops.iter() {
			op.exec(&mut st);
		}
		assert_eq!([99, 99, 3, 4], st.reg);
	}


	#[test]
	fn test_example() {
		let mut p = Prog::from(r"cpy 41 a
inc a
inc a
dec a
jnz a 2
dec a");
p.run();
		assert_eq!([42, 0,0,0], p.state.reg);
	}
}

