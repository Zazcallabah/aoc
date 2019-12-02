fn main(){
	let p = [1,0,0,3,
		1,1,2,3,
		1,3,4,3,
		1,5,0,3,
		2,1,9,19,
		1,19,5,23,
		2,23,13,27,
		1,10,27,31,
		2,31,6,35,
		1,5,35,39,
		1,39,10,43,
		2,9,43,47,
		1,47,5,51,
		2,51,9,55,
		1,13,55,59,
		1,13,59,63,
		1,6,63,67,
		2,13,67,71,
		1,10,71,75,
		2,13,75,79,
		1,5,79,83,
		2,83,9,87,
		2,87,13,91,
		1,91,5,95,
		2,9,95,99,
		1,99,5,103,
		1,2,103,107,
		1,10,107,0,
		99,2,14,0,
		0];
	let mut program = Prg::new(p.to_vec());
	let result = program.run(12,2);
	println!("result {}",result);

	// lets look at just one of the inputs
	for a in 0..10 {
		let mut program = Prg::new(p.to_vec());
		let result = program.run(a,0);
		println!("{},{}: {}",a,0,result);
	}
	// a linear equation. a*303750+250703. what about b?
	for b in 0..10 {
		let mut program = Prg::new(p.to_vec());
		let result = program.run(1,b);
		println!("{},{}: {}",1,b,result);
	}
	// ok so its a*303750+250703+b
	// find whatever a is closest but lower than our target, then maths to find b
	let mut previous = 0;
	for a in 0.. {
		let mut program = Prg::new(p.to_vec());
		let result = program.run(a,0);
		if result > 19690720 {
			println!("a: {}",a-1);
			println!("b: {}",19690720-previous);
			println!("final: {}",(a-1)*100+(19690720-previous));
			break;
		}
		previous = result;
	}
}

struct Prg {
	data: Vec<i32>
}

impl Prg {
	fn new(data:Vec<i32>)->Prg{
		Prg{data}
	}
	fn run(&mut self, a:i32,b:i32) -> i32 {
		let mut pc = 0;
		self.data[1] = a;
		self.data[2] = b;
		while self.do_opcode(pc) {
			pc += 4;
		}
		self.data[0]
	}
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