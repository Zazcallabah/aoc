use std::collections::HashMap;

type Node = (i32, i32);

fn is_wall(fav: u32, node: Node) -> bool {
	if node.0 < 0 || node.1 < 0 {
		return true;
	}
	let calcd =
		fav as i32 + node.0 * node.0 + 3 * node.0 + 2 * node.0 * node.1 + node.1 + node.1 * node.1;
	calcd.count_ones() % 2 == 1
}
fn draw(fav: u32, from: Node, to: Node) -> String {
	let w = get_walls(fav, from, to);
	let row_size = to.0 - from.0;
	let mut s = String::new();
	for row in w.chunks(row_size as usize) {
		let rowstr: String = row.iter().map(|b| if *b { '#' } else { ' ' }).collect();
		s.push_str(&rowstr);
		s.push('\n');
	}
	s
}
fn add_path(grid: &Vec<bool>, path: &Vec<Node>, row_size: usize) -> String {
	let mut output = Vec::with_capacity(grid.len());
	for (y, row) in grid.chunks(row_size).enumerate() {
		for (x, n) in row.iter().enumerate() {
			if path.contains(&(x as i32, y as i32)) {
				output.push('.');
			} else {
				output.push(if *n { '#' } else { ' ' });
			}
		}
		output.push('\n');
	}
	output.into_iter().collect()
}
fn get_walls(fav: u32, from: Node, to: Node) -> Vec<bool> {
	let mut collected = Vec::new();
	for sy in from.1..to.1 {
		for sx in from.0..to.0 {
			collected.push(is_wall(fav, (sx, sy)));
		}
	}
	collected
}

fn heuristic(node: Node, goal: Node) -> i32 {
	let dx = node.0 - goal.0;
	let dy = node.1 - goal.1;
	dx * dx + dy * dy
}

fn pop_current(open: &mut Vec<Node>, score: &HashMap<Node, i32>) -> Node {
	let mut lowest: (usize, i32) = (0, std::i32::MAX);
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
fn depth_first(current: Vec<Node>, mut visited: Vec<Node>, depth: i32, fav: u32) -> Vec<Node> {
	let mut new = Vec::new();

	for node in current.into_iter() {
		let ns: Vec<Node> = neighbours(node)
			.to_vec()
			.into_iter()
			.filter(|&n| !is_wall(fav, n) && !visited.contains(&n))
			.collect();
		new.extend_from_slice(&ns);
		visited.extend_from_slice(&ns);
	}

	if depth == 1 {
		return visited;
	}
	depth_first(new, visited, depth - 1, fav)
}
fn neighbours(node: Node) -> [Node; 4] {
	[
		(node.0 + 1, node.1),
		(node.0 - 1, node.1),
		(node.0, node.1 + 1),
		(node.0, node.1 - 1),
	]
}

fn a_star(start: Node, goal: Node, fav: u32) -> Vec<Node> {
	let mut open = vec![start];
	let mut parent: HashMap<Node, Node> = HashMap::new();
	let mut g_score: HashMap<Node, i32> = HashMap::new();
	g_score.insert(start, 0);

	let mut f_score: HashMap<Node, i32> = HashMap::new();
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

		for neighbor in neighbours(current).iter().filter(|&n| !is_wall(fav, *n)) {
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
fn main() {
	let fav = 1352;
	println!("before\n{}", draw(fav, (0, 0), (50, 50)));
	let walls = get_walls(fav, (0, 0), (50, 50));
	let path = a_star((1, 1), (31, 39), fav);

	println!(
		"\n\n{}\npart 1 total {} nodes, {} moves",
		add_path(&walls, &path, 50),
		&path.len(),
		&path.len() - 1
	);

	let fill = depth_first(vec![(1,1)],vec![(1,1)],50,fav);

	println!("\n\n{}\npart 2 total {} nodes", add_path(&walls,&fill,50), fill.len());

}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_is_wall() {
		assert!(!is_wall(10, (0, 0)));
		assert!(is_wall(10, (1, 0)));
	}

	#[test]
	fn test_draw() {
		assert_eq!(
			r" # #### ##
  #  #   #
",
			draw(10, (0, 0), (10, 2))
		);
	}

	#[test]
	fn test_draw_path() {
		let walls = get_walls(10, (0, 0), (10, 4));
		let r = a_star((1, 1), (7, 4), 10);
		assert_eq!(
			" # #### ##\n .#  #   #\n#... ##   \n###.# ### \n",
			add_path(&walls, &r, 10)
		);
	}

	#[test]
	fn test_depth_search() {
		let walls = get_walls(10, (0, 0), (10, 4));
		let r = depth_first(vec![(1,1)],vec![(1,1)], 1, 10);
		assert_eq!(3,r.len());
		assert_eq!(
			" # #### ##\n..#  #   #\n#.   ##   \n### # ### \n",
			add_path(&walls, &r, 10)
		);
	}
	#[test]
	fn test_depth_search2() {
		let walls = get_walls(10, (0, 0), (10, 4));
		let r = depth_first(vec![(1,1)],vec![(1,1)], 3, 10);
		assert_eq!(6,r.len());
		assert_eq!(
			".# #### ##\n..#  #   #\n#... ##   \n### # ### \n",
			add_path(&walls, &r, 10)
		);
	}

	#[test]
	fn test_astar() {
		let r = a_star((1, 1), (7, 4), 10);

		assert_eq!(12, r.len());
		assert_eq!(
			vec![
				(7, 4),
				(6, 4),
				(6, 5),
				(5, 5),
				(4, 5),
				(4, 4),
				(3, 4),
				(3, 3),
				(3, 2),
				(2, 2),
				(1, 2),
				(1, 1)
			],
			r
		);
	}
}
