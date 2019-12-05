use std::collections::HashMap;

#[derive(Debug,Clone,PartialEq,Eq)]
struct Node {
	x: isize,
	y: isize,
	wires:u8,
	manhattan:isize,
	cost:Vec<u32>,
}

impl Node {
	fn new(pointer:&[isize],wires:u8,cost:u32) -> Node {
		let manhattan = pointer[0].abs() + pointer[1].abs();
		Node{x:pointer[0],y:pointer[1],wires,manhattan,cost:vec![cost]}
	}
	fn is(&self,wires:u8) -> bool {
		wires == self.wires
	}
}
struct Grid {
	nodes: HashMap<(isize,isize),Node>,
	pointer: Vec<isize>,
	wire_count: u8,
	total_cost: u32,
}

impl Grid {
	fn new() -> Grid {
		Grid{ nodes:HashMap::new(), pointer:vec![0,0], wire_count:0, total_cost: 0 }
	}
	fn touchnode(&mut self,wire:u8){
		let x = self.pointer[0];
		let y = self.pointer[1];
		if let Some(existingnode) = self.nodes.get_mut(&(x,y)){
			existingnode.wires |= wire;
			if existingnode.cost.len() == self.wire_count as usize {
				existingnode.cost.push(self.total_cost);
			}
		}
		else {
			self.nodes.insert((x,y), Node::new(&self.pointer,wire,self.total_cost));
		}
	}
	fn read(&mut self, dir:char, length: isize, wire:u8) {
		// (axis,dir)
		let waldo = match dir {
			'U' => (1,1),
			'D' => (1,-1),
			'R' => (0,1),
			'L' => (0,-1),
			_ => panic!("invalid direction"),
		};
		for _ in 0..length {
			self.pointer[waldo.0] += waldo.1;
			self.total_cost += 1;
			self.touchnode(wire);
		}
	}
	fn parse_line(&mut self, data:&str, wire:u8){
		self.pointer = vec![0,0];
		self.total_cost = 0;
		for token in data.split(',').map( |s| s.chars().collect::<Vec<char>>() ) {
			let c = token[0];
			let s : String = token[1..].iter().collect();
			let n : isize = s.parse::<isize>().unwrap();
			self.read(c,n,wire);
		}
	}
	fn parse_file(&mut self, data:&str){
		for (i,line) in data.lines().enumerate(){
			self.parse_line(line, 1 << i);
			self.wire_count += 1;
		}
	}
	fn get_wire(&self, x:isize, y:isize) -> Option<&Node> {
		self.nodes.get(&(x,y) )
	}
	fn wire_ident(&self) -> u8 {
		2u8.pow(self.wire_count as u32) - 1
	}
	fn cheapest(&self) -> u32 {
		let ident = self.wire_ident();
		let val = self.nodes.values().clone();
		let mut filtered = Vec::new();
		for item in val {
			if item.is( ident )
			{
				filtered.push(item);
			}
		}
		filtered.sort_by_cached_key(|n| {
			n.cost.iter().sum::<u32>()
		}
		 );
		filtered[0].cost.iter().sum::<u32>()
	}
	fn closest(&self) -> isize {
		let ident = self.wire_ident();
		let val = self.nodes.values().clone();
		let mut filtered = Vec::new();
		for item in val {
			if item.is( ident )
			{
				filtered.push(item);
			}
		}
		filtered.sort_by_key(|n| n.manhattan );
		filtered[0].manhattan
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn test_iswire(){
		let mut g = Grid::new();
		g.touchnode(0b10);
		g.touchnode(0b01);
		assert!(g.nodes.values().next().unwrap().is(0b11));
	}

	#[test]
	fn test_isntwire(){
		let mut g = Grid::new();
		g.touchnode(0b10);
		assert!(!g.nodes.values().next().unwrap().is(0b11));
	}

	#[test]
	fn test_addtogrid(){
		let mut g = Grid::new();
		g.touchnode(1);
		assert_eq!(1,g.nodes.values().next().unwrap().wires);
	}

	#[test]
	fn test_read(){
		let mut g = Grid::new();
		g.read('R', 2, 1);

		assert_eq!(None,g.get_wire(0,0));
		assert_eq!(1,g.get_wire(1,0).unwrap().wires);
		assert_eq!(1,g.get_wire(2,0).unwrap().wires);
		assert_eq!(None,g.get_wire(3,0));
	}
	#[test]
	fn test_combine(){
		let mut g = Grid::new();
		g.read('R', 2, 1);
		g.read('L', 1, 2);

		assert_eq!(None,g.get_wire(0,0));
		assert_eq!(3,g.get_wire(1,0).unwrap().wires);
		assert_eq!(1,g.get_wire(2,0).unwrap().wires);
		assert_eq!(None,g.get_wire(3,0));
	}

	#[test]
	fn test_parse_file(){
		let mut g = Grid::new();
		g.parse_file("R8,U5,L5,D3\nU7,R6,D4,L4");
		assert_eq!(6, g.closest());
	}

	#[test]
	fn test_parse_file2(){
		let mut g = Grid::new();
		g.parse_file("R8,U5,L5,D3\nU7,R6,D4,L4");
		assert_eq!(30, g.cheapest());
	}
}

fn main() {
	let mut g = Grid::new();
	g.parse_file(&std::fs::read_to_string("2019/3.txt").unwrap());
	println!("closest: {}",g.closest());

	println!("cheapest: {}",g.cheapest());
}