struct Instr {
	command: Command,
	x: usize,
	y: usize,
}

#[derive(PartialEq,Debug)]
enum Command {
	Rect,
	Col,
	Row,
}

impl Command {
	fn from(input:&str)->Command {
		match input {
			"column" => Command::Col,
			"row" => Command::Row,
			_ => Command::Rect,
		}
	}
}

impl Instr {
	fn new(data:&str)->Instr{
		let re = regex::Regex::new(r"(?x)(rect|column|row)\D+(\d+)\D+(\d+)").unwrap();
		let caps = re.captures(data).unwrap();
		let command = Command::from(caps.get(1).unwrap().as_str());
		let x : usize = caps.get(2).unwrap().as_str().parse().unwrap();
		let y : usize = caps.get(3).unwrap().as_str().parse().unwrap();

		Instr{ command,x,y }
	}
}

struct Screen {
	size: (usize, usize),
	data: Vec<Vec<bool>>,
}

impl Screen {

	fn pxls(&self) -> usize {
		self.data.iter().flat_map(|v| v.iter()).filter(|&b| *b).collect::<Vec<&bool>>().len()
	}
	fn new() -> Screen {
		let size = (50, 6);
		Screen {
			size,
			data: vec![ vec![false;size.0] ; size.1],
		}
	}

	fn rect(&mut self, instr: &Instr) {
		for x in 0..instr.x {
			for y in 0..instr.y {
				self.data[y][x]=true;
			}
		}
	}
	fn colrot(&mut self, instr: &Instr) {
		let mut cpy : Vec<bool> = self.data.iter().map(|v| v[instr.x]).collect();
		cpy.rotate_right(instr.y);
		for ix in 0..self.size.1 {
			self.data[ix][instr.x] = cpy[ix];
		}
	}
	fn rowrot(&mut self, instr: &Instr) {
		self.data[instr.x].rotate_right(instr.y);
	}

	fn apply(&mut self, instr: &Instr) {
		match instr.command {
			Command::Rect => self.rect(&instr),
			Command::Row => self.rowrot(&instr),
			Command::Col => self.colrot(&instr),
		}
	}
}

impl std::fmt::Display for Screen {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		for row in &self.data {
			let s =
				String::from_utf8(row.iter().map(|&b| if b { 35 } else { 46 }).collect()).unwrap();
			if let Err(e) = writeln!(f, "{}", s) {
				return Err(e)
			}
		}
		Ok(())
	}
}

fn main() {
	let input = std::fs::read_to_string("8.txt").unwrap();
	let mut screen = Screen::new();
	for instr in input.lines().map(|l|Instr::new(&l)) {
		screen.apply(&instr);
		println!("{} c: {}",screen,screen.pxls());
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn test_can_count_pxels() {
		let i = Instr::new("rect 3x2");
		let mut s = Screen::new();
		s.apply(&i);
		assert_eq!(6,s.pxls());
	}
	#[test]
	fn test_can_do_instruction() {
		let i = Instr::new("rect 3x2");
		let mut s = Screen::new();
		s.apply(&i);
assert_eq!(r"###...............................................
###...............................................
..................................................
..................................................
..................................................
..................................................
", s.to_string());
s.apply(&Instr::new("rotate column x=1 by 1"));
assert_eq!(r"#.#...............................................
###...............................................
.#................................................
..................................................
..................................................
..................................................
", s.to_string());

s.apply(&Instr::new("rotate row y=0 by 4"));
assert_eq!(r"....#.#...........................................
###...............................................
.#................................................
..................................................
..................................................
..................................................
", s.to_string());
s.apply(&Instr::new("rotate column x=1 by 4"));
assert_eq!(r".#..#.#...........................................
#.#...............................................
..................................................
..................................................
..................................................
.#................................................
", s.to_string());
}

	#[test]
	fn test_can_parse_instr() {
		let i = Instr::new("rect 3x2");
		assert_eq!(Command::Rect,i.command);
		assert_eq!(2,i.y);
		assert_eq!(3,i.x);

		let i = Instr::new("rotate column x=1 by 4");
		assert_eq!(Command::Col,i.command);
		assert_eq!(4,i.y);
		assert_eq!(1,i.x);

		let i = Instr::new("rotate row y=0 by 4");
		assert_eq!(Command::Row,i.command);
		assert_eq!(4,i.y);
		assert_eq!(0,i.x);
	}

	#[test]
	fn test_screen_has_data() {
		let s = Screen::new();
		assert_eq!(6, s.data.len());
	}

	#[test]
	fn test_screen_has_tostring() {
		let mut s = Screen::new();
		s.data[0][33] = true;
		s.data[0][34] = true;
		assert_eq!(r".................................##...............
..................................................
..................................................
..................................................
..................................................
..................................................
", s.to_string());
	}
}
// The magnetic strip on the card you swiped encodes a series of instructions for the screen;
//  these instructions are your puzzle input. The screen is 50 pixels wide and 6 pixels tall,
//   all of which start off, and is capable of three somewhat peculiar operations:

//     rect AxB turns on all of the pixels in a rectangle at the top-left of the screen which is A wide and B tall.
//     rotate row y=A by B shifts all of the pixels in row A (0 is the top row) right by B pixels. Pixels that would
// 	fall off the right end appear at the left end of the row.
//     rotate column x=A by B shifts all of the pixels in column A (0 is the left column) down by B pixels. Pixels that would
// 	fall off the bottom appear at the top of the column.

// For example, here is a simple sequence on a smaller screen:

//     rect 3x2 creates a small rectangle in the top-left corner:

//     ###....
//     ###....
//     .......

//     rotate column x=1 by 1 rotates the second column down by one pixel:

//     #.#....
//     ###....
//     .#.....

//      rotates the top row right by four pixels:

//     ....#.#
//     ###....
//     .#.....

//      again rotates the second column down by one pixel, causing the bottom pixel to wrap back to the top:

//     .#..#.#
//     #.#....
//     .#.....

// As you can see, this display technology is extremely powerful, and will soon dominate the tiny-code-displaying-screen market.
// That's what the advertisement on the back of the display tries to convince you, anyway.

// There seems to be an intermediate check of the voltage used by the display: after you swipe your card, if the screen did work, how many pixels should be lit?
