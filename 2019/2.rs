fn main(){
	let mut program = Prg{data: vec![1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,1,9,19,1,19,5,23,2,23,13,27,1,10,27,31,2,31,6,35,1,5,35,39,1,39,10,43,2,9,43,47,1,47,5,51,2,51,9,55,1,13,55,59,1,13,59,63,1,6,63,67,2,13,67,71,1,10,71,75,2,13,75,79,1,5,79,83,2,83,9,87,2,87,13,91,1,91,5,95,2,9,95,99,1,99,5,103,1,2,103,107,1,10,107,0,99,2,14,0,0]};
	let mut pc = 0;
	program.data[1] = 12;
	program.data[2] = 2;
	while program.do_opcode(pc) {
		pc += 4;
	}
	println!("ended at {}, {:?}",pc,program.data);
}

struct Prg {
	data: Vec<i32>
}

impl Prg {
	fn do_opcode( &mut self, pc: usize ) -> bool {
		let opcode = self.data[pc];
		if opcode == 99 {
			return false
		}

		let ix_a = self.data[pc+1] as usize;
		let ix_b = self.data[pc+2] as usize;
		let ix_c = self.data[pc+3] as usize;

		match opcode {
			1 => self.data[ix_c] = self.data[ix_b] + self.data[ix_a],
			2 => self.data[ix_c] = self.data[ix_b] * self.data[ix_a],
			_ => panic!( "bad opcode at {}, {:?}",pc,self.data),
		}
		true
	}
}