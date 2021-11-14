
struct Grid {
	cells: Vec<Cell>,
	width: usize,
	height: usize,
}

impl Grid {
	fn new(data:&str)->Grid{
		let cells = Vec::with_capacity(data.len());
		for c in data.c
		Grid { cells, width: 16, height: 10 }
	}
}

struct Cell {
	own: usize,
	links: Vec<usize>
}

fn main(){
	let i = Grid::new(&std::fs::read_to_string("kami/map.txt").unwrap());

}
