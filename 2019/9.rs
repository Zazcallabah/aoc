use std::collections::VecDeque;
use std::collections::HashMap;

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_mempointer(){
		let mut p = Prg::new("204,-7");
		p.relative_base = 50;
		let r = p.get_mempointer(204, 1);
		assert_eq!(43,r);
	}

	#[test]
	fn test_relative_base(){
		let mut p = Prg::new("109,19,204,-34");
		p.relative_base = 2000;
		assert_eq!(2,p.off(109));
		assert_eq!(2019,p.relative_base);
		p.pc = 2;
		assert_eq!(1985,p.get_mempointer(204,1));
	}
	#[test]
	fn test_mem(){
		let mut p = Prg::new("109,19,204,-34");
		p.data.write(10000, 1125899906842624);
		assert_eq!(1125899906842624,p.data.read(10000));
	}

	#[test]
	fn test_output(){
		let mut p = Prg::new("109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99");
		p.run(false);
		assert_eq!(vec![109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99],p.output.into_iter().collect::<Vec<i64>>());
	}
	#[test]
	fn test_output_large(){
		let mut p = Prg::new("104,1125899906842624,99");
		p.run(false);
		assert_eq!(vec![1125899906842624],p.output.into_iter().collect::<Vec<i64>>());
	}
}

fn main() {
	let mut p = Prg::new(&std::fs::read_to_string("2019/9.txt").unwrap());
	p.input.push_back(1);
	p.run(false);

	println!("keycode: {}",p.output.pop_front().unwrap());

	let mut p = Prg::new(&std::fs::read_to_string("2019/9.txt").unwrap());
	p.input.push_back(2);
	p.run(false);

	println!("coordinates: {}",p.output.pop_front().unwrap());
}

struct MemBank {
	prg: Vec<i64>,
	data:HashMap<usize,Vec<i64>>,
}

struct Prg {
	data: MemBank,
	input: VecDeque<i64>,
	output: VecDeque<i64>,
	relative_base: i64,
	pc: usize,
}

impl MemBank {
	fn new(prg:Vec<i64>) -> MemBank {
		MemBank{prg,data: HashMap::new()}
	}
	fn read(&self,pointer:usize) -> i64 {
		if pointer < self.prg.len() {
			return self.prg[pointer]
		}

		let segment = pointer / 1024;
		if self.data.contains_key(&segment) {
			let local = pointer % 1024;
			return self.data.get(&segment).unwrap()[local]
		}
		0
	}

	fn write(&mut self, pointer: usize, data: i64) {
		if pointer < self.prg.len() {
			self.prg[pointer] = data;
		}
		else {
			let segment = pointer / 1024;
			if !self.data.contains_key(&segment) {
				self.data.insert(segment, vec![0;1024]);
			}
				let local = pointer % 1024;
				self.data.get_mut(&segment).unwrap()[local] = data;
		}
	}
}

impl Prg {
	fn new(s: &str) -> Prg {
		let data: Vec<i64> = s
			.split(',')
			.map(|a| a.parse().unwrap())
			.collect::<Vec<i64>>();
		Prg { data: MemBank::new(data), input: VecDeque::new(), output: VecDeque::new(), pc: 0, relative_base: 0 }
	}

	fn mode_for(op: i64, ix:u32) -> i64 {
		(op / 10i64.pow(1+ix)) % 10
	}

	fn get_mempointer(&mut self, op: i64, ix: u32) -> i64 {
		let mode = Prg::mode_for(op,ix);
		let param_pointer = self.pc + ix as usize;

		match mode {
			0 => self.data.read(param_pointer),
			1 => param_pointer as i64,
			2 => self.data.read(param_pointer) + self.relative_base,
			_ => panic!("invalid mode"),
		}
	}

	fn get_param(&mut self, op: i64, ix: u32) -> i64 {
		let pointer = self.get_mempointer(op,ix);
		self.data.read(pointer as usize)
	}

	fn mul(&mut self, op: i64) -> usize {
		let a = self.get_param( op, 1);
		let b = self.get_param( op, 2);
		let ix_c = self.get_mempointer(op,3) as usize;
		self.data.write(ix_c, a*b);
		4
	}

	fn add(&mut self, op: i64 ) -> usize {
		let a = self.get_param( op, 1);
		let b = self.get_param( op, 2);
		let ix_c = self.get_mempointer(op,3) as usize;
		self.data.write(ix_c, a+b);
		4
	}

	fn jnz(&mut self, op: i64) -> usize {
		let a = self.get_param( op, 1);
		if a != 0 {
			self.get_param(op, 2) as usize
		}
		else {
			self.pc + 3
		}
	}

	fn jz(&mut self, op: i64 ) -> usize {
		let a = self.get_param( op, 1);
		if a == 0 {
			self.get_param(op, 2) as usize
		}
		else {
			self.pc + 3
		}
	}

	fn lt(&mut self, op: i64 ) -> usize {
		let a = self.get_param( op, 1);
		let b = self.get_param( op, 2);
		let ix_c = self.get_mempointer(op,3) as usize;
		self.data.write(ix_c, if a < b { 1 } else {0});
		4
	}

	fn eq(&mut self, op: i64 ) -> usize {
		let a = self.get_param( op, 1);
		let b = self.get_param( op, 2);
		let ix_c = self.get_mempointer(op,3) as usize;
		self.data.write(ix_c, if a == b { 1 } else {0});
		4
	}

	fn read(&mut self, op:i64 ) -> usize {
		let pointer = self.get_mempointer(op,1) as usize;
		if let Some(x) = self.input.pop_front() {
			self.data.write(pointer,x);
		}
		else {
			panic!("input buffer empty");
		}
		2
	}

	fn off(&mut self, op:i64) -> usize {
		let pointer = self.get_mempointer(op,1) as usize;
		self.relative_base += self.data.read(pointer);
		2
	}

	fn run(&mut self, halt_on_write:bool ) -> bool {
		loop {
			let opcode = self.data.read(self.pc);
			if opcode == 99 {
				return true
			}

			match opcode % 100 {
				1 => self.pc += self.add(opcode),
				2 => self.pc += self.mul(opcode),
				3 => self.pc += self.read(opcode),
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
				9 => self.pc += self.off(opcode),
				_ => panic!("bad opcode at {}", self.pc),
			}
		}
	}
}
