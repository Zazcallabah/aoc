use std::collections::{HashSet, VecDeque};

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
enum Dir {
    N,
    E,
    S,
    W,
}
struct Todo {
    row: isize,
    col: isize,
    from: Dir,
}
impl Todo {
    fn new(row: isize, col: isize, from: Dir) -> Todo {
        Todo { row, col, from }
    }
    fn up(tile: &Tile) -> Todo {
        Todo::new(tile.row - 1, tile.col, Dir::S)
    }
    fn down(tile: &Tile) -> Todo {
        Todo::new(tile.row + 1, tile.col, Dir::N)
    }
    fn left(tile: &Tile) -> Todo {
        Todo::new(tile.row, tile.col - 1, Dir::E)
    }
    fn right(tile: &Tile) -> Todo {
        Todo::new(tile.row, tile.col + 1, Dir::W)
    }
}
struct Tile {
    tile_type: char,
    row: isize,
    col: isize,
    entered_from: HashSet<Dir>,
    visit_count: u32,
    infinite_from: HashSet<Dir>,
}
impl Tile {
    fn new(tile_type: char, row: isize, col: isize) -> Tile {
        Tile {
            tile_type,
            col,
            row,
            entered_from: HashSet::new(),
            infinite_from: HashSet::new(),
            visit_count: 0,
        }
    }
    fn repr(&self, row: isize, col: isize) -> String {
        format!(
            "{}{}{}",
            self.tile_type,
            if self.infinite_from.len() > 0 {
                'i'
            } else if self.visit_count > 0 {
                '#'
            } else {
                '.'
            },
            if self.row == row && self.col == col {
                "X"
            } else {
                " "
            }
        )
    }
    fn propagate_normal(&self, from: Dir) -> Vec<Todo> {
        match from {
            Dir::N => vec![Todo::down(self)],
            Dir::S => vec![Todo::up(self)],
            Dir::E => vec![Todo::left(self)],
            Dir::W => vec![Todo::right(self)],
        }
    }
    fn propagate_pipe(&self, from: Dir) -> Vec<Todo> {
        match from {
            Dir::N => vec![Todo::down(self)],
            Dir::S => vec![Todo::up(self)],
            Dir::E => vec![Todo::up(self), Todo::down(self)],
            Dir::W => vec![Todo::up(self), Todo::down(self)],
        }
    }
    fn propagate_dash(&self, from: Dir) -> Vec<Todo> {
        match from {
            Dir::N => vec![Todo::left(self), Todo::right(self)],
            Dir::S => vec![Todo::left(self), Todo::right(self)],
            Dir::E => vec![Todo::left(self)],
            Dir::W => vec![Todo::right(self)],
        }
    }
    fn propagate_frontslash(&self, from: Dir) -> Vec<Todo> {
        match from {
            Dir::N => vec![Todo::left(self)],
            Dir::S => vec![Todo::right(self)],
            Dir::E => vec![Todo::down(self)],
            Dir::W => vec![Todo::up(self)],
        }
    }
    fn propagate_backslash(&self, from: Dir) -> Vec<Todo> {
        match from {
            Dir::N => vec![Todo::right(self)],
            Dir::S => vec![Todo::left(self)],
            Dir::E => vec![Todo::up(self)],
            Dir::W => vec![Todo::down(self)],
        }
    }
    fn pass(&mut self, from: Dir) -> Vec<Todo> {
        if self.infinite_from.contains(&from) {
            return Vec::new();
        }

        if self.entered_from.contains(&from) {
            self.infinite_from.insert(from);
        } else {
            self.entered_from.insert(from);
        }
        self.visit_count += 1;

        match self.tile_type {
            '.' => self.propagate_normal(from),
            '|' => self.propagate_pipe(from),
            '-' => self.propagate_dash(from),
            '/' => self.propagate_frontslash(from),
            '\\' => self.propagate_backslash(from),
            _ => panic!("not mapped type"),
        }
    }
}
struct Map {
    tiles: Vec<Vec<Tile>>,
    active_row: isize,
    active_col: isize,
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in &self.tiles {
            let str: String = row
                .iter()
                .map(|t| t.repr(self.active_row, self.active_col))
                .collect();
            if let Err(e) = writeln!(f, "{}\n", &str) {
                return Err(e);
            }
        }
        Ok(())
    }
}
impl Map {
    fn new(data: &str) -> Map {
        let tiles: Vec<Vec<Tile>> = data
            .lines()
            .enumerate()
            .map(|(row, line)| {
                line.chars()
                    .enumerate()
                    .map(|(col, c)| Tile::new(c, row as isize, col as isize))
                    .collect()
            })
            .collect();

        Map {
            tiles,
            active_col: 0,
            active_row: 0,
        }
    }
    fn walk(&mut self, row: isize, col: isize, from: Dir) {
        let mut todos = VecDeque::new();
        todos.push_back(Todo::new(row, col, from));
        self.active_col = col;
        self.active_row = row;
        // println!("{}", &self);

        loop {
            if todos.is_empty() {
                break;
            }
            let t = todos.pop_front().unwrap();
            if t.row < 0 || t.col < 0 {
                continue;
            }
            self.active_col = t.col;
            self.active_row = t.row;
            if let Some(row) = self.tiles.get_mut(t.row as usize) {
                if let Some(tile) = row.get_mut(t.col as usize) {
                    let result = tile.pass(t.from);
                    for next in result {
                        todos.push_back(next);
                    }
                    //      println!("{}", &self);
                }
            }
        }
    }
    fn count_visited(&self) -> u32 {
        let mut sum = 0;
        for row in &self.tiles {
            for t in row {
                if t.visit_count > 0 {
                    sum += 1;
                }
            }
        }
        sum
    }
    fn max(data: &str) -> u32 {
        let height = data.lines().collect::<Vec<&str>>().len() as isize;
        let width = data.lines().last().unwrap().len() as isize;
        let mut max = 0;
        for col in 0isize..width {
            // from N
            let mut map = Map::new(&data);
            map.walk(0, col, Dir::N);
            let v = map.count_visited();
            if v > max {
                max = v;
            }
            // from S
            let mut map = Map::new(&data);
            map.walk(height - 1, col, Dir::S);
            let v = map.count_visited();
            if v > max {
                max = v;
            }
        }
        for row in 0isize..height {
            // from W
            let mut map = Map::new(&data);
            map.walk(row, 0, Dir::W);
            let v = map.count_visited();
            if v > max {
                max = v;
            }
            // from E
            let mut map = Map::new(&data);
            map.walk(row, width - 1, Dir::E);
            let v = map.count_visited();
            if v > max {
                max = v;
            }
        }
        max
    }
}
fn main() {
    let data = std::fs::read_to_string("2023/16.txt").unwrap();
    let mut map = Map::new(&data);
    map.walk(0, 0, Dir::W);
    println!("visited: {}", map.count_visited());

    let max = Map::max(&data);

    println!("max: {}", max); // 7489 is too low
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_propagate_pipe() {
        let t = Tile::new('|', 1, 1);
        let n = t.propagate_pipe(Dir::N);
        assert_eq!(1, n.len());
        assert_eq!(Dir::N, n[0].from);
        assert_eq!(1, n[0].col);
        assert_eq!(2, n[0].row);
        let s = t.propagate_pipe(Dir::S);
        assert_eq!(1, s.len());
        assert_eq!(Dir::S, s[0].from);
        assert_eq!(1, s[0].col);
        assert_eq!(0, s[0].row);
        let w = t.propagate_pipe(Dir::W);
        assert_eq!(2, w.len());
        assert_eq!(Dir::S, w[0].from);
        assert_eq!(Dir::N, w[1].from);
    }

    #[test]
    fn test_op_new() {
        let s = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";
        let mut map = Map::new(&s);
        map.walk(0, 0, Dir::W);
        assert_eq!(46, map.count_visited());
    }

    #[test]
    fn test_find_max() {
        let s = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";
        let max = Map::max(&s);
        assert_eq!(51, max);
    }
}
