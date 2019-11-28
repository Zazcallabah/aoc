#[macro_use]
extern crate lazy_static;

#[derive(Debug, PartialEq)]
enum Op {
	Jnz,
	Cpy,
	Dec,
	Inc,
	Tgl,
	Fac,
	Add,
	Mul,
}

impl From<&str> for Op {
	fn from(c: &str) -> Op {
		match c {
			"jnz" => Op::Jnz,
			"cpy" => Op::Cpy,
			"dec" => Op::Dec,
			"inc" => Op::Inc,
			"tgl" => Op::Tgl,
			"fac" => Op::Fac,
			"add" => Op::Add,
			"mul" => Op::Mul,
			_ => panic!("bad op"),
		}
	}
}

#[derive(Debug, PartialEq)]
enum R {
	Int(i32),
	Reg(u8),
}

#[derive(Debug, PartialEq)]
struct Line {
	op: Op,
	r1: R,
	r2: Option<R>,
}

impl std::fmt::Display for Line {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{:?} {:?} {:?}", self.op, self.r1, self.r2)
	}
}

impl From<&str> for R {
	fn from(c: &str) -> R {
		match c {
			"a" => R::Reg(0),
			"b" => R::Reg(1),
			"c" => R::Reg(2),
			"d" => R::Reg(3),
			num => R::Int(num.parse().unwrap()),
		}
	}
}

impl Line {
	fn new(line: &str) -> Line {
		lazy_static! {
			static ref PARSE: regex::Regex =
				regex::Regex::new(r"^(cpy|inc|dec|jnz|tgl|fac|mul|add)\s(\S+)\s?(\S+)?$").unwrap();
		}
		if let Some(cap) = PARSE.captures(line) {
			let r2: Option<R> = if let Some(c) = cap.get(3) {
				Some(R::from(c.as_str()))
			} else {
				None
			};
			let r1: R = R::from(cap.get(2).unwrap().as_str());
			let op = Op::from(cap.get(1).unwrap().as_str());
			return Line { op, r1, r2 };
		}
		panic!("parsing error");
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn test_line() {
		let l = Line::new("cpy a b");
		assert_eq!(R::Reg(0), l.r1);
		assert_eq!(Some(R::Reg(1)), l.r2);
		let l = Line::new("cpy c d");
		assert_eq!(R::Reg(2), l.r1);
		assert_eq!(Some(R::Reg(3)), l.r2);
		let b = Line::new("cpy -41 a");
		assert_eq!(R::Int(-41), b.r1);
		assert_eq!(Some(R::Reg(0)), b.r2);
		assert_eq!(Op::Cpy, b.op);
		let b = Line::new("cpy 41 a");
		assert_eq!(R::Int(41), b.r1);
		assert_eq!(Some(R::Reg(0)), b.r2);
		assert_eq!(Op::Cpy, b.op);

		let l = Line::new("jnz c -5");
		assert_eq!(R::Reg(2), l.r1);
		assert_eq!(Some(R::Int(-5)), l.r2);
		assert_eq!(Op::Jnz, l.op);

		let l = Line::new("tgl c");
		assert_eq!(R::Reg(2), l.r1);
		assert_eq!(Op::Tgl, l.op);

		let l = Line::new("inc c");
		assert_eq!(R::Reg(2), l.r1);
		assert_eq!(Op::Inc, l.op);

		let l = Line::new("dec c");
		assert_eq!(R::Reg(2), l.r1);
		assert_eq!(Op::Dec, l.op);
	}

	#[test]
	fn test_prog_jmp() {
		let mut p = Program::new("cpy 5 a\ndec a\njnz a -1");
		let state = p.run();
		assert_eq!(vec![0, 0, 0, 0], state);
	}

	#[test]
	fn test_prog() {
		let mut p = Program::new("inc b\ndec d");
		let state = p.run();
		assert_eq!(vec![0, 1, 0, -1], state);
	}

	#[test]
	fn test_example() {
		let mut p = Program::new(
			r"cpy 2 a
tgl a
tgl a
tgl a
cpy 1 a
dec a
dec a",
		);
		let state = p.run();

		assert_eq!(vec![3, 0, 0, 0], state);
	}
}

fn main() {
	let mut p = Program::new(
		r"cpy 7 a
cpy a b
dec b
cpy a d
cpy 0 a
cpy b c
inc a
dec c
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
jnz c -5",
	);
	let r = p.run();

	println!("result state: {:?}", r);

	// having verified that the tgl only ever touches ops downstream,
	// lets collapse some of thos dec-inc loops into adds
	let mut p = Program::new(
		r"cpy 7 a
cpy a b
dec b
cpy a d
cpy 0 a
cpy b c
add c a
dec d
jnz d -3
dec b
cpy b c
add b c
tgl c
cpy -11 c
jnz 1 c
cpy 94 c
jnz 80 d
inc a
inc d
jnz d -2
inc c
jnz c -5",
	);
	let r = p.run();
	println!("do add instead, state: {:?}", r);

	// that double loop in the original program was obviously a mul op
	let mut p = Program::new(
		r"cpy 7 a
cpy a b
dec b
cpy a d
cpy 0 a
cpy b c
mul d c
add c a
dec b
cpy b c
add b c
tgl c
cpy -10 c
jnz 1 c
cpy 94 c
jnz 80 d
inc a
inc d
jnz d -2
inc c
jnz c -5",
	);
	let r = p.run();
	println!("do mul also, state: {:?}", r);

	// reg b will contain numbers counting down toward 0.
	// the stuff just before togl is just c=b*2
	// so tgl will toggle 10, 8, 6, etc, before pc reaches any of them
	// the final tgl will be 'jnz 1 c' -> cpy 1 c
	// this it what it looks pre-toggld:

	// cpy -10 c
	// cpy 1 c
	// cpy 94 c
	// cpy 80 d
	// inc a
	// dec d
	// jnz d -2
	// dec c
	// jnz c -5

	// that double jnz loop looks like another mul + add

	// cpy 94 c
	// mul 80 c
	// add c a

	// but the tgl will destroy it, so we have to short out the tgl first
	let mut p = Program::new(
		r"cpy 7 a
cpy a b
dec b
cpy a d
cpy 0 a
cpy b c
mul d c
add c a
dec b
cpy b c
add b c
jnz b -8
cpy 94 c
cpy 80 d
inc a
dec d
jnz d -2
dec c
jnz c -5",
	);
	let r = p.run();
	println!("skip tgl, state: {:?}", r);


	// but the tgl will destroy it, so we have to short out the tgl first
	let mut p = Program::new(
		r"cpy 7 a
cpy a b
dec b
cpy a d
cpy 0 a
cpy b c
mul d c
add c a
dec b
cpy b c
add b c
jnz b -8
cpy 94 d
mul 80 d
add d a",
	);
	let r = p.run();
	println!("more mul & add, state: {:?}", r);

	// we observe that the first lines calculate a! into register a
	// lets do than in an op, and clean up some cpy:s
	let mut p = Program::new(
		r"fac 7 a
cpy 94 d
mul 80 d
add d a",
	);
	let r = p.run();
	println!("final fac state, state: {:?}", r);


	// now lets do it for input 12
	let mut p = Program::new(
		r"fac 12 a
cpy 94 d
mul 80 d
add d a",
	);
	let r = p.run();
	println!("state: {:?}", r);


	println!("part 2 solution: {}", r[0]);


}

struct Program {
	lines: Vec<Line>,
}

impl Program {
	fn new(p: &str) -> Program {
		Program {
			lines: p.lines().map(|l| Line::new(l)).collect::<Vec<Line>>(),
		}
	}

	fn run(&mut self) -> Vec<i32> {
		let mut pc = 0usize;
		let mut reg = vec![0i32; 4];
		while pc < self.lines.len() {
			//print!("{} {:?}: {} ->",pc,reg,self.lines[pc]);
			let jmp = match self.lines[pc].op {
				Op::Tgl => tgl(&mut self.lines, &mut reg, pc),
				Op::Dec => dec(&mut self.lines, &mut reg, pc),
				Op::Inc => inc(&mut self.lines, &mut reg, pc),
				Op::Jnz => jnz(&mut self.lines, &mut reg, pc),
				Op::Cpy => cpy(&mut self.lines, &mut reg, pc),
				Op::Fac => fac(&mut self.lines, &mut reg, pc),
				Op::Add => add(&mut self.lines, &mut reg, pc),
				Op::Mul => mul(&mut self.lines, &mut reg, pc),
			};
			//	println!("{:?}",reg);
			pc = (pc as isize + jmp) as usize;
		}
		reg
	}
}

fn factorize(n: i32) -> i32 {
	if n == 2 {
		2
	} else {
		n * factorize(n - 1)
	}
}

fn mul(program: &mut [Line], reg: &mut [i32], pc: usize) -> isize {
	let line = &program[pc];
	let target = match line.r2 {
		Some(R::Reg(r)) => r as usize,
		_ => panic!("invalid fac instruction"),
	};
	let sourcevalue = match line.r1 {
		R::Reg(r) => reg[r as usize],
		R::Int(i) => i,
	};

	reg[target] *= sourcevalue;
	1
}

fn add(program: &mut [Line], reg: &mut [i32], pc: usize) -> isize {
	let line = &program[pc];
	let target = match line.r2 {
		Some(R::Reg(r)) => r as usize,
		_ => panic!("invalid fac instruction"),
	};
	let sourcevalue = match line.r1 {
		R::Reg(r) => reg[r as usize],
		R::Int(i) => i,
	};

	reg[target] += sourcevalue;
	1
}

fn fac(program: &mut [Line], reg: &mut [i32], pc: usize) -> isize {
	let line = &program[pc];
	let target = match line.r2 {
		Some(R::Reg(r)) => r as usize,
		_ => panic!("invalid fac instruction"),
	};
	let sourcevalue = match line.r1 {
		R::Reg(r) => reg[r as usize],
		R::Int(i) => i,
	};

	reg[target] = factorize(sourcevalue);
	1
}

fn jnz(program: &mut [Line], reg: &mut [i32], pc: usize) -> isize {
	let line = &program[pc];
	let distance = match line.r2 {
		Some(R::Int(i)) => i as isize,
		Some(R::Reg(r)) => reg[r as usize] as isize,
		None => panic!("invalid jump instruction"),
	};
	let sourcevalue = match line.r1 {
		R::Reg(r) => reg[r as usize],
		R::Int(i) => i,
	};

	if sourcevalue != 0 {
		distance
	} else {
		1
	}
}

fn cpy(program: &mut [Line], reg: &mut [i32], pc: usize) -> isize {
	let line = &program[pc];
	let target = if let Some(R::Reg(r)) = line.r2 {
		r as usize
	} else {
		return 1;
	};
	let sourcevalue = match line.r1 {
		R::Reg(r) => reg[r as usize],
		R::Int(i) => i,
	};
	reg[target] = sourcevalue;
	1
}

fn dec(program: &mut [Line], reg: &mut [i32], pc: usize) -> isize {
	let line = &program[pc];
	let register = if let R::Reg(r) = line.r1 { r } else { return 1 };
	reg[register as usize] -= 1;
	1
}

fn inc(program: &mut [Line], reg: &mut [i32], pc: usize) -> isize {
	let line = &program[pc];
	let register = if let R::Reg(r) = line.r1 { r } else { return 1 };
	reg[register as usize] += 1;
	1
}

fn tgl(program: &mut [Line], reg: &mut [i32], pc: usize) -> isize {
	let line = &program[pc];
	let register = if let R::Reg(r) = line.r1 {
		r
	} else {
		panic!("invalid tgl");
	};
	let target = pc as isize + reg[register as usize] as isize;

	if target >= 0 && (target as usize) < program.len() {
		program[target as usize].op = match program[target as usize].op {
			Op::Cpy => Op::Jnz,
			Op::Jnz => Op::Cpy,
			Op::Dec => Op::Inc,
			Op::Inc => Op::Dec,
			Op::Tgl => Op::Inc,
			Op::Mul => Op::Jnz,
			Op::Add => Op::Jnz,
			Op::Fac => Op::Jnz,
		};
	}
	1
}
