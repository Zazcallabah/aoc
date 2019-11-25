#[macro_use]
extern crate lazy_static;
use regex::Regex;

struct Node {
	x: u8,
	y: u8,
	size: u32,
	used: u32,
	avail: u32,
	usep: u8,
	marker: Option<char>,
}

impl std::fmt::Display for Node {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		if let Some(m) = self.marker {
			write!(f, "{}", m)
		}
		else if self.used == 0 {
			write!(f, "_")
		}
		else if self.used > 100 {
			write!(f, "#")
		}
		else {
			write!(f, ".")
		}
	}
}

impl Node {
	fn viable(&self, b:&Node)->bool {
		if self.used == 0 || self.x == b.x && self.y == b.y {
			false
		}
		else {
			self.used < b.avail
		}
	}
	fn new(s:&str)->Node{
		lazy_static! {
			static ref PARSE: Regex = Regex::new(r"/dev/grid/node-x(\d+)-y(\d+)\s+(\d+)T\s+(\d+)T\s+(\d+)T\s+(\d+)%").unwrap();
		}
		let cap = PARSE.captures(s).unwrap();
		let x : u8 = cap.get(1).unwrap().as_str().parse().unwrap();
		let y : u8 = cap.get(2).unwrap().as_str().parse().unwrap();
		let size : u32 = cap.get(3).unwrap().as_str().parse().unwrap();
		let used : u32 = cap.get(4).unwrap().as_str().parse().unwrap();
		let avail : u32 = cap.get(5).unwrap().as_str().parse().unwrap();
		let usep : u8 = cap.get(6).unwrap().as_str().parse().unwrap();
		Node{x,y,size,used,avail,usep,marker:None}
	}
}

struct Grid {
	nodes: Vec<Node>,
	index: usize,
	row_size: usize,
	steps: u32,
}

impl std::fmt::Display for Grid {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		let mut grid = String::new();

		for row in self.nodes.chunks(self.row_size) {
			for n in row{
				grid += &n.to_string();
			}
			grid += "\n";
		}
		write!(f,"{}{}Steps: {}\n{}",termion::clear::All,termion::cursor::Goto(1,1),self.steps,grid)
	}
}

impl Grid {
	fn step(&mut self, dir:char){
		let newindex = match dir {
			'L' => self.index - 1,
			'R' => self.index + 1,
			'U' => self.index - self.row_size,
			'D' => self.index + self.row_size,
			_ => panic!("bad direction"),
		};
		self.nodes.swap(self.index,newindex);
		self.index = newindex;
		self.steps += 1;
	}
	fn new(data:&str)->Grid{
		let mut nodes = data.lines().skip(2).map(Node::new).collect::<Vec<Node>>();

		let mut row_size = 0u8;
		for n in &nodes {
			if row_size < n.x {
				row_size = n.x;
			}
		}

		nodes.sort_by(|a, b| {
			let o = a.y.cmp(&b.y);
			if o == std::cmp::Ordering::Equal {
				a.x.cmp(&b.x)
			}
			else {
				o
			}
		});

		let index : &usize = &nodes.iter().enumerate()
			.filter(|(_,n)| n.used == 0)
			.map(|(i,_)|i)
			.next().unwrap();
		nodes[row_size as usize].marker = Some('X');

		Grid{nodes,row_size:row_size as usize + 1,steps:0,index: *index}
	}

	fn count_viable(&self) -> u32 {

		let mut counter = 0u32;
		for a in 0..self.nodes.len(){
			for b in 0..self.nodes.len() {
				if self.nodes[a].viable(&self.nodes[b]) {
					counter += 1;
				}
			}
		}
		counter
	}
}

fn animate(grid:&mut Grid,steps:&str){
	for c in steps.chars() {
		grid.step(c);
		println!("{}",grid.to_string());

		std::thread::sleep(std::time::Duration::from_millis(50));
	}
}


fn main() {
	let mut grid = Grid::new(&std::fs::read_to_string("22.txt").unwrap());

	animate(&mut grid,"LLLLUUUUUUURRRRRRRRRRUUUUUR");
	for _ in 0..=32 {
		animate(&mut grid,"DLLUR");
	}

	println!("found {} pairs",grid.count_viable());

}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_newnode(){
		let n = Node::new("/dev/grid/node-x33-y1    94T   68T    26T   72%");
		assert_eq!(33,n.x);
		assert_eq!(1,n.y);
		assert_eq!(94,n.size);
		assert_eq!(68,n.used);
		assert_eq!(26,n.avail);
		assert_eq!(72,n.usep);
	}
}