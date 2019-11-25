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
	fn lookup_register(name: &str) -> isize {
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
	fn tgl(&self, state: &mut State, ops: &[Box<dyn Op>]) -> Option<(usize,Box<dyn Op>)>;
	fn tgld(&self) -> Box<dyn Op>;
	fn ident(&self) -> u32;
}

struct Cpy {
	source: isize,
	target: isize,
}
impl Op for Cpy {
	fn exec(&self, state: &mut State){
		if self.target >= 0 && self.target < state.reg.len() as isize {
			state.reg[self.target as usize] = state.reg[self.source as usize];
		}
		state.pc += 1;
	}
	fn tgl(&self, state: &mut State, ops: &[Box<dyn Op>]) -> Option<(usize,Box<dyn Op>)> {
		panic!("invalid op");
	}
	fn tgld(&self) -> Box<dyn Op> {
		Box::new(Jnz{reg:self.source,jmp:self.target})
	}
	fn ident(&self) -> u32 {
		1
	}
}

struct Cpi {
	val: i32,
	target: isize,
}
impl Op for Cpi {
	fn exec(&self, state: &mut State) {
		if self.target >= 0 && self.target < state.reg.len() as isize {
			state.reg[self.target as usize] = self.val;
		}
		state.pc += 1;
	}
	fn tgl(&self, state: &mut State, ops: &[Box<dyn Op>]) -> Option<(usize,Box<dyn Op>)> {
		panic!("invalid op");
	}
	fn tgld(&self) -> Box<dyn Op> {
		Box::new(Jit{val:self.val,target:self.target})
	}
	fn ident(&self) -> u32 {
		2
	}
}

struct Inc {
	reg: isize,
}
impl Op for Inc {
	fn exec(&self, state: &mut State) {
		state.reg[self.reg as usize] += 1;
		state.pc += 1;
	}
	fn tgl(&self, state: &mut State, ops: &[Box<dyn Op>]) -> Option<(usize,Box<dyn Op>)> {
		panic!("invalid op");
	}
	fn tgld(&self) -> Box<dyn Op> {
		Box::new(Dec{reg:self.reg})
	}
	fn ident(&self) -> u32 {
		3
	}
}

struct Dec {
	reg: isize,
}
impl Op for Dec {
	fn exec(&self, state: &mut State) {
		state.reg[self.reg as usize] -= 1;
		state.pc += 1;
	}
	fn tgl(&self, state: &mut State, ops: &[Box<dyn Op>]) -> Option<(usize,Box<dyn Op>)> {
		panic!("invalid op");
	}
	fn tgld(&self) -> Box<dyn Op> {
		Box::new(Inc{reg:self.reg})
	}
	fn ident(&self) -> u32 {
		4
	}
}

struct Nop {
	jmp: isize,
	val: i32,
}
impl Op for Nop {
	fn exec(&self, state: &mut State) {
		state.pc += 1;
	}
	fn tgl(&self, state: &mut State, ops: &[Box<dyn Op>]) -> Option<(usize,Box<dyn Op>)> {
		panic!("invalid op");
	}
	fn tgld(&self) -> Box<dyn Op> {
		Box::new(Ji{val:self.val,jmp:self.jmp})
	}
	fn ident(&self) -> u32 {
		100
	}
}

struct JNop {
	jmp: isize,
	reg: isize,
}
impl Op for JNop {
	fn exec(&self, state: &mut State) {
		state.pc += 1;
	}
	fn tgl(&self, state: &mut State, ops: &[Box<dyn Op>]) -> Option<(usize,Box<dyn Op>)> {
		panic!("invalid op");
	}
	fn tgld(&self) -> Box<dyn Op> {
		Box::new(Jnz{reg:self.reg,jmp:self.jmp})
	}
	fn ident(&self) -> u32 {
		101
	}
}

struct Ji {
	jmp: isize,
	val: i32,
}
impl Ji {
	fn jmp(state: &mut State, jmp: isize) {
		state.pc = (state.pc as isize + jmp) as usize;
	}
}
impl Op for Ji {
	fn exec(&self, state: &mut State) {
		if self.val != 0 {
			Ji::jmp(state,self.jmp);
		} else {
			state.pc += 1;
		}
	}
	fn tgl(&self, state: &mut State, ops: &[Box<dyn Op>]) -> Option<(usize,Box<dyn Op>)> {
		panic!("invalid op");
	}
	fn tgld(&self) -> Box<dyn Op> {
		Box::new(Nop{val:self.val,jmp:self.jmp})
	}
	fn ident(&self) -> u32 {
		5
	}
}

struct Jnz {
	jmp: isize,
	reg: isize,
}
impl Op for Jnz {
	fn exec(&self, state: &mut State) {
		if state.reg[self.reg as usize] != 0 {
			Ji::jmp(state,self.jmp);
		} else {
			state.pc += 1;
		}
	}
	fn tgl(&self, state: &mut State, ops: &[Box<dyn Op>]) -> Option<(usize,Box<dyn Op>)> {
		panic!("invalid op");
	}
	fn tgld(&self) -> Box<dyn Op> {
		Box::new(JNop{reg:self.reg,jmp:self.jmp})
	}
	fn ident(&self) -> u32 {
		6
	}
}

struct Jit {
	target: isize,
	val: i32,
}
impl Op for Jit {
	fn exec(&self, state: &mut State) {
		if self.val != 0 {
			Ji::jmp(state,state.reg[self.target as usize] as isize);
		} else {
			state.pc += 1;
		}
	}
	fn tgl(&self, state: &mut State, ops: &[Box<dyn Op>]) -> Option<(usize,Box<dyn Op>)> {
		panic!("invalid op");
	}
	fn tgld(&self) -> Box<dyn Op> {
		Box::new(Cpi{val:self.val,target:self.target})
	}
	fn ident(&self) -> u32 {
		11
	}
}

struct Jnzt {
	target: isize,
	reg: isize,
}
impl Op for Jnzt {
	fn exec(&self, state: &mut State) {
		if state.reg[self.reg as usize] != 0 {
			Ji::jmp(state,state.reg[self.target as usize] as isize);
		} else {
			state.pc += 1;
		}
	}
	fn tgl(&self, state: &mut State, ops: &[Box<dyn Op>]) -> Option<(usize,Box<dyn Op>)> {
		panic!("invalid op");
	}
	fn tgld(&self) -> Box<dyn Op> {
		Box::new(Cpy{source:self.reg,target:self.target})
	}
	fn ident(&self) -> u32 {
		6
	}
}
struct Tgl {
	ix: isize,
}

impl Op for Tgl {
	fn exec(&self, state: &mut State) {
		panic!("invalid op");
	}
	fn tgl(&self, state: &mut State, ops: &[Box<dyn Op>]) -> Option<(usize,Box<dyn Op>)> {
		let targetix = state.pc as isize + state.reg[self.ix as usize] as isize;
		if targetix < 0 || targetix as usize >= ops.len() {
			return None;
		}
		let targetop = &ops[targetix as usize];
		let newop = targetop.tgld();
		state.pc += 1;
		Some((targetix as usize,newop))
	}
	fn ident(&self) -> u32 {
		10
	}
	fn tgld(&self) -> Box<dyn Op> {
		Box::new(Inc{reg:self.ix})
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
			if next.ident() == 10 {
				if let Some((ix,op)) = next.tgl(&mut self.state,&self.ops) {
					self.ops[ix] = op;
				}
			}
			else {
				next.exec(&mut self.state);
			}
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
		(?P<op>cpy|inc|dec|jnz|tgl)
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
							return Box::new(Ji { jmp, val });
						} else {
							let reg = State::lookup_register(cap.name(&"source").unwrap().as_str());
							return Box::new(Jnz { reg, jmp, });
						}
					}
					else {
						let target = State::lookup_register(cap.name(&"target").unwrap().as_str());
						if let Some(imm) = cap.name(&"imm") {
							let val : i32= imm.as_str().parse().unwrap();
							return Box::new(Jit { target, val });
						} else {
							let reg = State::lookup_register(cap.name(&"source").unwrap().as_str());
							return Box::new(Jnzt { reg, target });
						}
						}
				},
				"tgl" => {
					let ix = State::lookup_register(cap.name(&"source").unwrap().as_str());
					return Box::new(Tgl { ix });
				}
				_ => (),
			}
		}
		panic!("parsing error");
	}
}

fn main() {
	let mut p = Prog::from(r"cpy a b
dec b
cpy a d
cpy 0 a // 0 6 0 7
cpy b c // 0 6 6 7
inc a // 1 11 1 7
dec c // 1 11 0 7
jnz c -2
dec d
jnz d -5
dec b
cpy b c
cpy c d
dec d
inc c
jnz d -2
tgl c
cpy -16 c
jnz 1 c
cpy 94 c
jnz 80 d
inc a
inc d
jnz d -2
inc c
jnz c -5");
	p.state.reg[0] = 7;
	p.run();

	println!("end state part 1 {:?}",p.state.reg);


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
	fn test_example() {
		let mut p = Prog::from(r"cpy 2 a
tgl a
tgl a
tgl a
cpy 1 a
dec a
dec a");
		p.run();

		assert_eq!([3,0,0,0], p.state.reg);
		assert_eq!(vec![2, 10, 10, 3, 11, 4, 4], p.ops.iter().map(|o| o.ident() ).collect::<Vec<u32>>());
	}

}

