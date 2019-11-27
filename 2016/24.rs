// You've finally met your match; the doors that provide access to the roof are locked tight,
// and all of the controls and related electronics are inaccessible. You simply can't reach them.

// The robot that cleans the air ducts, however, can.

// It's not a very fast little robot, but you reconfigure it to be able to interface with some of
//  the exposed wires that have been routed through the HVAC system. If you can direct it to each
//  of those locations, you should be able to bypass the security controls.

// You extract the duct layout for this area from some blueprints you acquired and create a map
// with the relevant locations marked (your puzzle input). 0 is your current location, from which
//  the cleaning robot embarks; the other numbers are (in no particular order) the locations the
//  robot needs to visit at least once each. Walls are marked as #, and open passages are marked
//  as .. Numbers behave like open passages.

// For example, suppose you have a map like the following:

// ###########
// #0.1.....2#
// #.#######.#
// #4.......3#
// ###########

// To reach all of the points of interest as quickly as possible, you would have the robot take the following path:

//     0 to 4 (2 steps)
//     4 to 1 (4 steps; it can't move diagonally)
//     1 to 2 (6 steps)
//     2 to 3 (2 steps)

// Since the robot isn't very fast, you need to find it the shortest route. This path is the fewest steps (in the above
//  example, a total of 14) required to start at 0 and then visit every other location at least once.

// Given your actual map, and starting from location 0, what is the fewest number of steps required to visit every
// non-0 number marked on the map at least once?


// look at day 9 2015
// and day 13 2016

use std::collections::HashMap;

type Node = (usize, usize);

#[derive(Copy,Clone,Debug)]
struct Marker {
	node:Node,
	sign:char,
}

struct Maze {
	data: Vec<Vec<char>>,
	markers: Vec<Marker>,
}

impl Marker{
	fn new(node:Node,sign:char) -> Marker {
		Marker{node,sign}
	}
}

impl Maze {
	fn new(mazestr:&str) -> Maze {
		let mut data : Vec<Vec<char>> = Vec::new();
		let mut markers : Vec<Marker> = Vec::new();

		for (y,row) in mazestr.lines().enumerate() {
			let mut r = Vec::with_capacity(row.len());
			for (x,c) in row.chars().enumerate() {
				match c {
					'#' => r.push(c),
					'.' => r.push(' '),
					_ => {
						r.push(' ');
						markers.push(Marker::new((x,y),c));
					}
				}
			}
			data.push(r);
		}

		Maze{data,markers}
	}
	fn is_wall(&self, node: Node) -> bool {
		self.data[node.1][node.0] == '#'
	}
	fn get_sign(&self, node: Node) -> char {
		if let Some(m) = self.markers.iter().find(|s| s.node == node) {
			m.sign
		}
		else {
			self.data[node.1][node.0]
		}
	}
}

impl std::fmt::Display for Maze {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		let mut grid = String::with_capacity(self.data.len());

		for (y,row) in self.data.iter().enumerate() {
			for (x,pos) in row.iter().enumerate() {
				grid.push(self.get_sign((x,y)));
			}
			grid.push('\n');
		}
		write!(f,"{}",grid)
	}
}

fn heuristic(node: Node, goal: Node) -> usize {
	let dx = node.0 as isize - goal.0 as isize;
	let dy = node.1 as isize - goal.1 as isize;
	(dx * dx + dy * dy) as usize
}

fn pop_current(open: &mut Vec<Node>, score: &HashMap<Node, usize>) -> Node {
	let mut lowest: (usize, usize) = (0, std::usize::MAX);
	for (i, n) in open.iter().enumerate() {
		if let Some(f) = score.get(n) {
			if f <= &lowest.1 {
				lowest = (i, *f);
			}
		} else {
			panic!("missing f score");
		}
	}
	open.remove(lowest.0)
}

fn neighbours(node: Node) -> [Node; 4] {
	[
		(node.0 + 1, node.1),
		(node.0 - 1, node.1),
		(node.0, node.1 + 1),
		(node.0, node.1 - 1),
	]
}

fn a_star(start: Node, goal: Node, maze: &mut Maze, startpoints: &[Marker]) -> Vec<Node> {
	maze.markers.clear();
	maze.markers.extend_from_slice(startpoints);
	println!(
		"{}{}Calculating paths:\n{}",
		termion::clear::All,
		termion::cursor::Goto(1,1),
		maze
	);
	let mut frames = 0u64;
	let mut open = vec![start];
	let mut parent: HashMap<Node, Node> = HashMap::new();
	let mut g_score: HashMap<Node, usize> = HashMap::new();
	g_score.insert(start, 0);

	let mut f_score: HashMap<Node, usize> = HashMap::new();
	f_score.insert(start, heuristic(start, goal));

	while !open.is_empty() {
		let current = pop_current(&mut open, &f_score);
		if current == goal {
			let mut path = vec![goal];
			loop {
				let p = parent.get(&path.last().unwrap()).unwrap();
				path.push(*p);
				if p == &start {
					return path;
				}
			}
		}

		for neighbor in neighbours(current).iter().filter(|&n| !maze.is_wall(*n)) {
			let tentative_g = g_score.get(&current).unwrap() + 1;
			if !g_score.contains_key(&neighbor) || tentative_g < *g_score.get(&neighbor).unwrap() {

				parent.insert(*neighbor, current);
				print!("{}{}",termion::cursor::Goto(neighbor.0 as u16 + 1,neighbor.1 as u16 +2),'.');
				g_score.insert(*neighbor, tentative_g);
				f_score.insert(*neighbor, tentative_g + heuristic(*neighbor, goal));
				if open.iter().all(|n| n != neighbor) {
					open.push(*neighbor);
				}
			}
		}


		frames += 1;
		if frames % 5 == 0 {
			maze.markers.clear();
			maze.markers.extend_from_slice(startpoints);

			maze.markers.append(&mut parent.keys().map(|k| Marker::new(k.clone(),'.')).collect::<Vec<Marker>>());
			maze.markers.append(&mut open.iter().map(|k| Marker::new(k.clone(),'?')).collect::<Vec<Marker>>());

		}
	}

	panic!("no path found")
}
fn travel(from:Option<usize>,to:usize,dist:usize,past:&[usize],remain:&[usize],map:&HashMap<usize,HashMap<usize,usize>>,longest:bool, mut besteffort: usize) -> Option<(usize,Vec<usize>)> {
	let newdistance = if let Some(f) = from {
		dist + map.get(&f).unwrap().get(&to).unwrap()
	}
	else {
		0
	};

	if !longest && newdistance >= besteffort {
		return None
	}

	let mut newpast = past.to_vec();
	newpast.push(to);

	if remain.len() == 0 {
		if longest && newdistance <= besteffort {
			return None
		}
		return Some((newdistance,newpast))
	}

	let mut rem_cpy = remain.to_vec();
	let mut best : Vec<usize> = Vec::new();

	for i in 0..rem_cpy.len() {
		let item = rem_cpy.remove(i);
		let result = travel(Some(to),item,newdistance,past,&rem_cpy[i..],&map,longest,besteffort);
		if let Some((newbesteffort,newbestpath)) = result {
			best = newbestpath;
			besteffort = newbesteffort;
		}
		rem_cpy.insert(i,item);
	}
	if best.len() == 0 {
		None
	}
	else {
		Some((besteffort,best))
	}
}
fn main() {
	let ms = std::fs::read_to_string("24.txt").unwrap();
	let mut maze = Maze::new(&ms);
	let mut markers = maze.markers.clone();
	let mut maxpath = 0;

	markers.sort_by_key(|m| m.sign );

	let mut paths : HashMap<usize,HashMap<usize,usize>>=HashMap::new();

	for start in 0..markers.len()-1 {
		for to in start+1..markers.len() {
			let path = a_star(markers[start].node, markers[to].node,&mut maze,&markers);
			if !paths.contains_key(&start) {
				paths.insert(start, HashMap::new());
			}
			if !paths.contains_key(&to) {
				paths.insert(to, HashMap::new());
			}
			maxpath += path.len();
			paths.get_mut(&start).unwrap().insert(to,path.len());
			paths.get_mut(&to).unwrap().insert(start,path.len());
		}
	}

	let mut targets = paths.keys().map(|k|k.clone()).collect::<Vec<usize>>();
	let mut best = maxpath;

	for i in 0..targets.len() {
		let item = targets.remove(i);
		let result = travel(None,item,0,&[],&targets,&paths,false,best);
		if let Some((bestef,bestp)) = result {
			best = bestef;
		}
		targets.insert(i,item);
	}

	println!("\n\n\n\nbest {}",best);





}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_astar() {

	}
}
