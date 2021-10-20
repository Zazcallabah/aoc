use std::collections::HashMap;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::thread;
use std::sync::mpsc;

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

}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Direction { N, E, S, W }
type Coord = (isize,isize);
struct Robot {
	dir: Direction,
	pos: Coord
}
impl Robot {
	fn new() -> Robot {
		Robot { dir: Direction::N, pos: (0,0) }
	}
	fn go(&mut self, turn_right: bool) {
		match self.dir {
			 Direction::N => if turn_right {
				self.dir = Direction::E
			} else {
				self.dir = Direction::W
			},
			Direction::E => if turn_right {
				self.dir = Direction::S
			} else {
				self.dir = Direction::N
			},
			Direction::S => if turn_right {
				self.dir = Direction::W
			} else {
				self.dir = Direction::E
			},
			Direction::W => if turn_right {
				self.dir = Direction::N
			} else {
				self.dir = Direction::S
			}
		}
		match self.dir {
			Direction::N => self.pos.1 += 1,
			Direction::E => self.pos.0 += 1,
			Direction::S => self.pos.1 -= 1,
			Direction::W => self.pos.0 -= 1,
		}
	}
}
struct Grid {
	quads: HashMap<Coord,i64>
}
impl Grid {
	fn new() -> Grid {
		Grid{quads:HashMap::new()}
	}
	fn paint(&mut self, pos:Coord, color:i64){
		self.quads.insert(pos, color);
	}
}
fn main() {

	let mut p = Prg::new(&std::fs::read_to_string("2019/11.txt").unwrap());
	let (input_tx, input_rx) = mpsc::channel();
	let (output_tx, output_rx) = mpsc::channel();
	let handle = thread::spawn(move || {
		p.run(input_rx,output_tx,false);
	});


	let mut robot = Robot::new();
	let mut grid = Grid::new();
	grid.paint(robot.pos,1);
	let mut time = 0u64;
	loop{
		time += 1;
		let startcolor = grid.quads.entry(robot.pos).or_default();
		if let Err(e) = input_tx.send( startcolor.clone() ){
			panic!("write");
		}
		if let Ok(color) = output_rx.recv() {
			grid.paint(robot.pos,color);

			if let Ok( direction ) = output_rx.recv() {
				robot.go( direction == 1 );
			}
			else {
				panic!("direction");
			}
		}
		else {
			break;
		}
	}
	println!("{}",time);
	let mut minx = isize::MAX;
	let mut maxx = isize::MIN;
	let mut miny = isize::MAX;
	let mut maxy = isize::MIN;
	grid.quads.keys().for_each(|pos| {
		minx = isize::min(minx,pos.0);
		maxx = isize::max(maxx,pos.0);
		miny = isize::min(miny,pos.1);
		maxy = isize::max(maxy,pos.1);
	});
	println!("{} {} {} {}",minx,maxx,miny,maxy);
	let deltax = (maxx - minx)+1;
	let deltay = (maxy - miny)+1;
	println!("dx {}, dy {}",deltax,deltay);

	let mut strmaker = vec!['.';(deltax*deltay) as usize];

	grid.quads.keys().for_each(|pos| {
//		println!("p {} {}, m {} {}",pos.0,pos.1,minx,miny);
		let ix_x = pos.0 - minx;
		let ix_y = pos.1 - miny;

		let ix = deltax * ix_y + ix_x;
		strmaker[ix as usize] = if grid.quads[&(pos.0,pos.1)] == 1 { 'X' } else { ' ' };
	});

	for chunk in strmaker.chunks(deltax as usize).rev() {
		let s : String = chunk.into_iter().collect();
		println!("{}",s);
	}


//	let drawing = Vec::with_capacity(20);
//	println!("tiles visited: {}",grid.quads.len());

}

struct MemBank {
	prg: Vec<i64>,
	data:HashMap<usize,Vec<i64>>,
}
//input: VecDeque<i64>,
//output: VecDeque<i64>,

struct Prg {
	data: MemBank,
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
		Prg { data: MemBank::new(data), pc: 0, relative_base: 0 }
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

	fn off(&mut self, op:i64) -> usize {
		let pointer = self.get_mempointer(op,1) as usize;
		self.relative_base += self.data.read(pointer);
		2
	}
	fn write(&mut self, op:i64, tx: &Sender<i64> ) -> usize {
		let x = self.get_param(op,1);
		if let Err(e) = tx.send(x) {
			panic!("bad write");
		}
		return 2;
	}
	fn read(&mut self, op:i64, rx: &Receiver<i64> ) -> usize {
		let pointer = self.get_mempointer(op,1) as usize;
		if let Ok(x) = rx.recv() {
			self.data.write(pointer,x);
			return 2;
		}
		else {
			panic!("bad read");
		}
	}
	fn run(&mut self, input_rx:Receiver<i64>, output_tx:Sender<i64>, halt_on_write:bool ) -> bool {
		loop {
			let opcode = self.data.read(self.pc);
			if opcode == 99 {
				return true;
			}

			match opcode % 100 {
				1 => self.pc += self.add(opcode),
				2 => self.pc += self.mul(opcode),
				3 => self.pc += self.read(opcode,&input_rx),
				4 => {
					self.pc += self.write(opcode, &output_tx);
					if halt_on_write {
						return true;
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
