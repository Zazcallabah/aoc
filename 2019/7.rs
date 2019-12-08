use std::collections::VecDeque;
use permutohedron::heap_recursive;

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_run() {
		assert_eq!(43210, run("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0",0,&[],0,0));
		assert_eq!(54321, run("3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0",0,&[],0,0));
		assert_eq!(65210, run("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0",0,&[],0,0));
	}

	#[test]
	fn test_do_loopback(){
		assert_eq!(139629729,do_loopback(&[9,8,7,6,5],"3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5"));
	}

	#[test]
	fn test_find_loopback(){
		assert_eq!(18216,find_loopback("3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10"));
	}
}

fn run(s:&str,depth:usize,history:&[usize],input:i32,mut max:i32) -> i32 {
	for a in 0..=4 {
		if !history.contains(&a) {
			let mut program = Prg::new(&s);
			program.input.push_back(a as i32);
			program.input.push_back(input);
			let _ = program.run(false);
			let result = program.output.pop_front().unwrap();
			if depth == 4 {
				if result > max {
					max = result;
				}
			}
			else {
				let mut nhistory = history.to_vec();
				nhistory.push(a);
				max = run(s,depth+1,&nhistory,result,max)
			}
		}
	}
	max
}

fn main() {
	let p = std::fs::read_to_string("2019/7.txt").unwrap();
	let result = run(&p,0,&[],0,0);
	println!("part 1: {}",result);

	let result = find_loopback(&p);
	println!("part 2: {}",result);
}

fn find_loopback(s:&str) -> i32 {
	let mut data = [5,6,7,8,9];
	let mut max = 0;
	heap_recursive(&mut data, |permutation| {
		let result = do_loopback(permutation,s);
		if result > max {
			max = result;
		}
	});
	max
}

fn do_loopback(setting:&[i32],s:&str) -> i32 {
	let mut prgs :Vec<Prg> = setting.iter().map(|i| {
		let mut p = Prg::new(&s);
		p.input.push_back(*i);
		p
	}).collect::<Vec<Prg>>();

	let mut result = 0;
	loop {
		for p in prgs.iter_mut() {
			p.input.push_back(result);
			if p.run(true) {
				return result
			}
			if let Some(x) = p.output.pop_front() {
				result = x;
			}
			else {
				panic!("didnt get output");
			}
		}
	}
}

struct Prg {
	data: Vec<i32>,
	input: VecDeque<i32>,
	output: VecDeque<i32>,
	pc: usize,
}

impl Prg {
	fn new(s: &str) -> Prg {
		let data: Vec<i32> = s
			.split(',')
			.map(|a| a.parse().unwrap())
			.collect::<Vec<i32>>();
		Prg { data, input: VecDeque::new(), output: VecDeque::new(), pc: 0 }
	}

	fn mode_for(op: i32, ix:u32) -> i32 {
		(op / 10i32.pow(1+ix)) % 10
	}

	fn get_param_from_mode(&mut self, val: i32, mode: i32 ) -> i32 {
		match mode {
			0 => self.data[val as usize],
			1 => val,
			_ => panic!("invalid mode"),
		}
	}

	fn get_param(&mut self, op: i32, ix: u32) -> i32 {
		let mode = Prg::mode_for(op,ix);
		let val = self.data[self.pc + ix as usize];
		self.get_param_from_mode(val,mode)
	}

	fn mul(&mut self, op: i32) -> usize {
		let a = self.get_param( op, 1);
		let b = self.get_param( op, 2);
		let ix_c = self.data[self.pc + 3] as usize;
		self.data[ix_c] = a*b;
		4
	}

	fn add(&mut self, op: i32 ) -> usize {
		let a = self.get_param( op, 1);
		let b = self.get_param( op, 2);
		let ix_c = self.data[self.pc + 3] as usize;
		self.data[ix_c] = a+b;
		4
	}

	fn jnz(&mut self, op: i32) -> usize {
		let a = self.get_param( op, 1);
		if a != 0 {
			self.get_param(op, 2) as usize
		}
		else {
			self.pc + 3
		}
	}

	fn jz(&mut self, op: i32 ) -> usize {
		let a = self.get_param( op, 1);
		if a == 0 {
			self.get_param(op, 2) as usize
		}
		else {
			self.pc + 3
		}
	}

	fn lt(&mut self, op: i32 ) -> usize {
		let a = self.get_param( op, 1);
		let b = self.get_param( op, 2);
		let ix_c = self.data[self.pc + 3] as usize;
		self.data[ix_c] = if a < b { 1 } else {0};
		4
	}

	fn eq(&mut self, op: i32 ) -> usize {
		let a = self.get_param( op, 1);
		let b = self.get_param( op, 2);
		let ix_c = self.data[self.pc + 3] as usize;
		self.data[ix_c] = if a == b { 1 } else {0};
		4
	}

	fn run(&mut self, halt_on_write:bool ) -> bool {
		while self.pc < self.data.len() {
			let opcode = self.data[self.pc];
			if opcode == 99 {
				return true
			}

			match opcode % 100 {
				1 => self.pc += self.add(opcode),
				2 => self.pc += self.mul(opcode),
				3 => {
					let ix_a = self.data[self.pc + 1] as usize;
					if let Some(x) = self.input.pop_front() {
						self.data[ix_a] = x;
					}
					else {
						panic!("input buffer empty");
					}
					self.pc += 2;
				},
				4 => {
					let x = self.get_param(opcode,1);
					self.output.push_back(x);
					self.pc += 2;
					if halt_on_write {
						return false
					}
				},
				5 => self.pc = self.jnz(opcode),
				6 => self.pc = self.jz(opcode),
				7 => self.pc += self.lt(opcode),
				8 => self.pc += self.eq(opcode),
				_ => panic!("bad opcode at {}, {:?}", self.pc, self.data),
			}
		}

		panic!("halted at {}, {:?}", self.pc, self.data);
	}
}
