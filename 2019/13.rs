use std::collections::HashMap;
use std::io::Stdout;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::thread;
use std::sync::mpsc;
use std::time::Duration;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::raw::RawTerminal;
use std::io::{Write, stdout, stdin};

fn as_char(output:i64)->char{
	match output {
		0 => ' ',
		1 => '#',
		2 => '.',
		3 => '"',
		4 => 'o',
		_ => panic!("bad output from intcode"),
	}
}

fn main() {
	let mut p = Prg::new(&std::fs::read_to_string("2019/13.txt").unwrap());
	let (_, input_rx) = mpsc::channel();
	let (output_tx, output_rx) = mpsc::channel();
	let handle = thread::spawn(move || {
		p.run(input_rx,output_tx,false);
	});

	let mut tileset:HashMap<i64,i32> = HashMap::new();
	loop {
		if let Ok(x) = output_rx.recv_timeout(Duration::from_millis(100)) {
			if let Ok(y) = output_rx.recv() {
				if let Ok(o) = output_rx.recv() {
					let counter = tileset.entry(o).or_default();
					*counter += 1;
				}
			}
		}
		else {
			break;
		}
	}
	if let Err(_) = handle.join() {
		panic!("couldnt join");
	}
	for (a,b) in tileset {
		println!("{}: {}",as_char(a),b);
	//  _: 1
	//   : 442
	//  #: 76
	//  .: 236   // assuming linear score, but incorrect. different layers are probably worth different scores
	//  o: 1
	// guessed 15340

	}

	let mut p = Prg::new(&std::fs::read_to_string("2019/13.txt").unwrap());
	p.data.write(0,2);
	let (input_tx, input_rx) = mpsc::channel();
	let (output_tx, output_rx) = mpsc::channel();
	let handle = thread::spawn(move || {
		p.run(input_rx,output_tx,false);
	});
	let mut keys = stdin().keys();
	let mut stdout = stdout().into_raw_mode().unwrap();
	write!(stdout,"{}",termion::clear::All).unwrap();
	let mut paddle_x = -1;
	let mut ball = (0,0);
	let mut ball_dir = (true,true);
	let mut score = 0;
	loop{
		if let Ok(x) = output_rx.recv_timeout(Duration::from_millis(100)) {
			if let Ok( y ) = output_rx.recv() {
				if let Ok( t ) = output_rx.recv() {
					if x != -1 && t == 4 {
						if x != ball.0 {
							ball_dir = (x > ball.0,ball_dir.1);
						}
						if y != ball.1 {
							ball_dir = (ball_dir.0,y > ball.1);
						}
						ball = (x,y);
						write!(stdout,"{}ball x {} y {}",termion::cursor::Goto(2,26),x,y).unwrap();
						write!(stdout,"{}ball _ {} _ {}",
							termion::cursor::Goto(2,27),
							if ball_dir.0 {'0'}else{'1'},
							if ball_dir.1 {'0'}else{'1'}
						).unwrap();
					} else if t == 3 {
						paddle_x = x;
						write!(stdout,"{}pad {}",termion::cursor::Goto(2,24),x).unwrap();
					}
					if x==-1 && y==0 {
						score = t;
						if score > 0 {
							write!(stdout,"{}score: {} ",termion::cursor::Goto(15,2),t).unwrap();
						}
					}
					else {
						let c = as_char(t);
						write!(stdout,"{}{}",termion::cursor::Goto((x+2) as u16,(y +2)as u16),c).unwrap();
					}
				}
				else {
					panic!("failed to read t");
				}
			}
			else {
				panic!("failed to read y");
			}
		}
		else {
			write!(stdout,"{}",termion::cursor::Goto(1,1)).unwrap();
			stdout.flush().unwrap();
			let wait = if score == 0 {
				500
			} else if score < 100 {
				300
			} else if score < 200 {
				100
			} else {
				1
			};
			std::thread::sleep(std::time::Duration::from_millis(wait));
			let data = decision(ball,ball_dir,paddle_x,&mut stdout);
			match input_tx.send(data) {
				Ok(_) => {},
				Err(_) => break,
			}
		}
	}


}

fn decision(ball:(i64,i64),dball:(bool,bool),paddle:i64,stdout:&mut RawTerminal<Stdout>) -> i64 {
	if ball.1 > 14 && dball.1 {
		let delta_y = 18 - ball.1;
		let absolute_x_ball = ball.0 + if dball.0 { delta_y } else { -1 * delta_y };
		let expected_x_ball =
			if absolute_x_ball < 0 {
				(absolute_x_ball * -1) +1 // accounting for the wall
			} else if absolute_x_ball > 34 {
				32 + ( absolute_x_ball - 34 )
			} else { absolute_x_ball };
		write!(stdout,
			"{}                                           {}{} {}",
			termion::cursor::Goto(0,23),
			termion::cursor::Goto((expected_x_ball+1) as u16,23),
			'x',
			expected_x_ball
		).unwrap();
		if paddle == expected_x_ball {
			0
		}
		else if paddle > expected_x_ball {
			-1
		} else {
			1
		}
	} else if paddle == ball.0 {
		if dball.0 { 1 } else { -1 }
	} else if paddle > ball.0 {
		-1
	} else {
		1
	}
}

struct MemBank {
	prg: Vec<i64>,
	data:HashMap<usize,Vec<i64>>,
}

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
