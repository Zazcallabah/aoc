
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_in_out() {
		assert_eq!(vec![42], Prg::new("3,0,4,0,99").run(&[42]));
	}

	#[test]
	fn test_mul() {
		let mut p = Prg::new("1002,4,3,4,33");
		let r = p.run(&Vec::new());

		assert_eq!(0,r.len());
		assert_eq!(99,p.data[4]);
	}

	#[test]
	fn test_get_mode() {
		assert_eq!( 0, Prg::mode_for(1002,1));
		assert_eq!( 1, Prg::mode_for(1002,2));
		assert_eq!( 0, Prg::mode_for(1002,3));
	}

	#[test]
	fn test_cmp() {
		assert_eq!(vec![1], Prg::new("3,9,8,9,10,9,4,9,99,-1,8").run(&[8]));
		assert_eq!(vec![0], Prg::new("3,9,8,9,10,9,4,9,99,-1,8").run(&[9]));

		assert_eq!(vec![1], Prg::new("3,9,7,9,10,9,4,9,99,-1,8").run(&[7]));
		assert_eq!(vec![0], Prg::new("3,9,7,9,10,9,4,9,99,-1,8").run(&[8]));

		assert_eq!(vec![1], Prg::new("3,3,1108,-1,8,3,4,3,99").run(&[8]));
		assert_eq!(vec![0], Prg::new("3,3,1108,-1,8,3,4,3,99").run(&[9]));

		assert_eq!(vec![1], Prg::new("3,3,1107,-1,8,3,4,3,99").run(&[7]));
		assert_eq!(vec![0], Prg::new("3,3,1107,-1,8,3,4,3,99").run(&[8]));
	}

	#[test]
	fn test_jmp() {
		assert_eq!(vec![1], Prg::new("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9").run(&[8]));
		assert_eq!(vec![0], Prg::new("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9").run(&[0]));
		assert_eq!(vec![1], Prg::new("3,3,1105,-1,9,1101,0,0,12,4,12,99,1").run(&[7]));
		assert_eq!(vec![0], Prg::new("3,3,1105,-1,9,1101,0,0,12,4,12,99,1").run(&[0]));
	}
}

fn main() {
	let p = std::fs::read_to_string("2019/5.txt").unwrap();
	let mut program = Prg::new(&p);
	let result = program.run(&[1]);
	println!("result 1: {:?}", result);

	let mut program = Prg::new(&p);
	let result = program.run(&[5]);
	println!("result 2: {:?}", result);
}

struct Prg {
	data: Vec<i32>,
}

impl Prg {
	fn new(s: &str) -> Prg {
		let data: Vec<i32> = s
			.split(',')
			.map(|a| a.parse().unwrap())
			.collect::<Vec<i32>>();
		Prg { data }
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

	fn get_param(&mut self, op: i32, pc: usize, ix: u32) -> i32 {
		let mode = Prg::mode_for(op,ix);
		let val = self.data[pc + ix as usize];
		self.get_param_from_mode(val,mode)
	}

	fn mul(&mut self, op: i32, pc: usize) -> usize {
		let a = self.get_param( op, pc, 1);
		let b = self.get_param( op, pc, 2);
		let ix_c = self.data[pc + 3] as usize;
		self.data[ix_c] = a*b;
		4
	}

	fn add(&mut self, op: i32, pc: usize) -> usize {
		let a = self.get_param( op, pc, 1);
		let b = self.get_param( op, pc, 2);
		let ix_c = self.data[pc + 3] as usize;
		self.data[ix_c] = a+b;
		4
	}

	fn jnz(&mut self, op: i32, pc: usize) -> usize {
		let a = self.get_param( op, pc, 1);
		if a != 0 {
			self.get_param(op, pc, 2) as usize
		}
		else {
			pc + 3
		}
	}

	fn jz(&mut self, op: i32, pc: usize) -> usize {
		let a = self.get_param( op, pc, 1);
		if a == 0 {
			self.get_param(op, pc, 2) as usize
		}
		else {
			pc + 3
		}
	}

	fn lt(&mut self, op: i32, pc: usize) -> usize {
		let a = self.get_param( op, pc, 1);
		let b = self.get_param( op, pc, 2);
		let ix_c = self.data[pc + 3] as usize;
		self.data[ix_c] = if a < b { 1 } else {0};
		4
	}

	fn eq(&mut self, op: i32, pc: usize) -> usize {
		let a = self.get_param( op, pc, 1);
		let b = self.get_param( op, pc, 2);
		let ix_c = self.data[pc + 3] as usize;
		self.data[ix_c] = if a == b { 1 } else {0};
		4
	}

	fn run(&mut self, input: &[i32]) -> Vec<i32> {
		let mut output = Vec::new();
		let mut pc = 0;
		let mut inp = 0;
		while pc < self.data.len() {
			let opcode = self.data[pc];
			if opcode == 99 {
				return output;
			}

			match opcode % 100 {
				1 => pc += self.add(opcode, pc),
				2 => pc += self.mul(opcode, pc),
				3 => {
					let ix_a = self.data[pc + 1] as usize;
					self.data[ix_a] = input[inp];
					inp += 1;
					pc += 2;
				},
				4 => {
					output.push(self.get_param( opcode, pc, 1));
					pc += 2;
				},
				5 => pc = self.jnz(opcode,pc),
				6 => pc = self.jz(opcode,pc),
				7 => pc += self.lt(opcode, pc),
				8 => pc += self.eq(opcode, pc),
				_ => panic!("bad opcode at {}, {:?}", pc, self.data),
			}
		}

		panic!("halted at {}, {:?}", pc, self.data);
	}
}
