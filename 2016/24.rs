use std::collections::HashMap;

type Node = (usize, usize);
type Map = HashMap<usize, HashMap<usize, Vec<Node>>>;

#[derive(Copy, Clone, Debug)]
struct Marker {
	node: Node,
	sign: char,
}

struct Maze {
	data: Vec<Vec<char>>,
	markers: Vec<Marker>,
}

impl Marker {
	fn new(node: Node, sign: char) -> Marker {
		Marker { node, sign }
	}
}

impl Maze {
	fn new(mazestr: &str) -> Maze {
		let mut data: Vec<Vec<char>> = Vec::new();
		let mut markers: Vec<Marker> = Vec::new();

		for (y, row) in mazestr.lines().enumerate() {
			let mut r = Vec::with_capacity(row.len());
			for (x, c) in row.chars().enumerate() {
				match c {
					'#' => r.push('.'),
					'.' => r.push(' '),
					_ => {
						r.push(' ');
						markers.push(Marker::new((x, y), c));
					}
				}
			}
			data.push(r);
		}

		Maze { data, markers }
	}
	fn is_wall(&self, node: Node) -> bool {
		self.data[node.1][node.0] == '.'
	}
	fn get_sign(&self, node: Node) -> char {
		if let Some(m) = self.markers.iter().find(|s| s.node == node) {
			m.sign
		} else {
			self.data[node.1][node.0]
		}
	}
}

impl std::fmt::Display for Maze {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		let mut grid = String::with_capacity(self.data.len());

		for (y, row) in self.data.iter().enumerate() {
			for (x, pos) in row.iter().enumerate() {
				grid.push(self.get_sign((x, y)));
			}
			grid.push('\n');
		}
		write!(f, "{}", grid)
	}
}

fn heuristic(node: Node, goal: Node) -> usize {
	let dx = node.0 as isize - goal.0 as isize;
	let dy = node.1 as isize - goal.1 as isize;
	(dx.abs() + dy.abs()) as usize
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
				g_score.insert(*neighbor, tentative_g);
				f_score.insert(*neighbor, tentative_g + heuristic(*neighbor, goal));
				if open.iter().all(|n| n != neighbor) {
					open.push(*neighbor);
				}
			}
		}
	}

	panic!("no path found")
}
fn travel(
	from: Option<usize>,
	to: usize,
	dist: usize,
	past: &[usize],
	remain: &[usize],
	map: &Map,
	besteffort: usize,
	goback: Option<()>,
) -> Option<(usize, Vec<usize>)> {
	let mut be = besteffort;
	let mut newdistance = if let Some(f) = from {
		dist + map.get(&f).unwrap().get(&to).unwrap().len() - 1
	} else {
		0
	};

	if newdistance >= be {
		return None;
	}

	let mut newpast = past.to_vec();
	newpast.push(to);

	if remain.is_empty() {
		if goback.is_some() {
			newdistance += map.get(&to).unwrap().get(&0).unwrap().len() - 1;
			newpast.push(0);
		}
		return Some((newdistance, newpast));
	}
	let mut rem_cpy = remain.to_vec();
	let mut best: Vec<usize> = Vec::new();

	for i in 0..rem_cpy.len() {
		let item = rem_cpy.remove(i);
		let result = travel(
			Some(to),
			item,
			newdistance,
			&newpast,
			&rem_cpy,
			&map,
			be,
			goback,
		);
		if let Some((newbesteffort, foundpast)) = result {
			be = newbesteffort;
			best = foundpast;
		}
		rem_cpy.insert(i, item);
	}
	if be < besteffort {
		Some((be, best))
	} else {
		None
	}
}

fn find_paths(mut maze: &mut Maze) -> Map {
	let mut markers = maze.markers.clone();
	markers.sort_by_key(|m| m.sign);
	let mut paths: Map = HashMap::new();
	for start in 0..markers.len() - 1 {
		for to in start + 1..markers.len() {
			let path = a_star(markers[start].node, markers[to].node, &mut maze, &markers);
			let starthash = paths.entry(start).or_insert_with( HashMap::new );
			starthash.insert(to,path.clone());
			let tohash = paths.entry(to).or_insert_with( HashMap::new );
			tohash.insert(start,path);
		}
	}
	paths
}

fn decorate_maze(maze:&mut Maze, past: &[usize], markers: &[Marker], paths:&Map){
	maze.markers = markers.to_vec();
	for i in 0..past.len() - 1 {
		maze.markers.extend_from_slice(
			&paths
				.get(&past[i])
				.unwrap()
				.get(&past[i + 1])
				.unwrap()
				.iter()
				.map(|&n| Marker::new(n, 'o'))
				.collect::<Vec<Marker>>(),
		);
	}
}

fn main() {
	let ms = std::fs::read_to_string("24.txt").unwrap();
	let mut maze = Maze::new(&ms);
	let markers = maze.markers.clone();
	let paths = find_paths(&mut maze);

	let (result, past) = travel(
		None,
		0,
		0,
		&[],
		&[1usize, 2, 3, 5, 6, 4, 7],
		&paths,
		std::usize::MAX,
		None,
	)
	.unwrap();

	decorate_maze(&mut maze,&past,&markers,&paths);

	println!("\nEnd wherever\n{}\npath length {}", maze, result);
	println!("{:?}", past);
	let (result, past) = travel(
		None,
		0,
		0,
		&[],
		&[1usize, 2, 3, 5, 6, 4, 7],
		&paths,
		std::usize::MAX,
		Some(()),
	)
	.unwrap();

	decorate_maze(&mut maze,&past,&markers,&paths);

	println!("\nEnd at start\n{}\npath length {}", maze, result);
	println!("{:?}", past);
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_() {
		let mut maze = Maze::new(
			r"###########
#0.1.6...2#
#.#######5#
#4.......3#
###########",
		);
		let p = find_paths(&mut maze);

		let t = vec![1, 2, 3, 4, 5, 6];
		let (result, past) = travel(None, 0, 0, &[], &t, &p, std::usize::MAX, None).unwrap();
		assert_eq!(14, result);
		assert_eq!(vec![0, 4, 1, 6, 2, 5, 3], past);
	}
	#[test]
	fn test_2() {
		let mut maze = Maze::new(
			r"#######################
#....................2#
#.#.###.#.###.#.###.#.#
#4........#.......#...#
#.###.#.###.#.#.#.#.#.#
#.#....1#.....#...#...#
#.#.#######.#.#.#.#.###
#.#.#...#.........#...#
#.###.#.#.#.#.#######.#
#...#.#.3....0..#.#...#
#.#.#.#.#.#.###.#.#.#.#
#.....#.#.....#.......#
#######################",
		);
		let p = find_paths(&mut maze);
		for (from, m2) in &p {
			for (to, d) in m2 {
				println!("{} >> {} :: {}", from, to, d.len() - 1);
			}
		}

		let t = vec![1, 2, 3, 4];
		let (result, past) = travel(None, 0, 0, &[], &t, &p, std::usize::MAX, None).unwrap();
		assert_eq!(vec![0,3,2,1,4], past);
		assert_eq!(52, result);
	}
}
