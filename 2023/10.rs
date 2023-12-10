use std::{fmt, fs::File, io::Write};

impl fmt::Display for Loop {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let strings: Vec<String> = self
            .map
            .iter()
            .map(|v| v.iter().collect::<String>())
            .collect();
        let s = strings.join("\n");

        write!(f, "{}", s)
    }
}

struct Loop {
    map: Vec<Vec<char>>,
}
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Coord {
    line: isize,
    ix: isize,
}
impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.ix, self.line)
    }
}
impl Coord {
    fn new(line: isize, ix: isize) -> Coord {
        Coord { line, ix }
    }
}
impl Loop {
    fn new_sized(line_count: usize, row_size: usize) -> Loop {
        let map = vec![vec![' '; row_size]; line_count];
        Loop { map }
    }
    fn new(s: &str) -> Loop {
        let map = s.lines().map(|l| l.chars().collect()).collect();
        Loop { map }
    }
    fn at(&self, line: usize, ix: usize) -> char {
        *self.map.get(line).unwrap().get(ix).unwrap()
    }
    fn find(&self, c: char) -> Coord {
        let height = self.map.len();
        let width = self.map.get(0).unwrap().len();
        for l in 0..height {
            for w in 0..width {
                if self.at(l, w) == c {
                    return Coord {
                        line: l as isize,
                        ix: w as isize,
                    };
                }
            }
        }
        panic!("invalid character");
    }
    fn set(&mut self, line: isize, ix: isize, c: char) {
        if let Some(l) = self.map.get_mut(line as usize) {
            if let Some(i) = l.get_mut(ix as usize) {
                if *i != 'x' {
                    *i = c;
                }
            }
        }
    }
    fn mask(&self, marks: &Loop) -> Loop {
        let height = self.map.len();
        let width = self.map[0].len();
        let mut l = Loop::new_sized(height, width);

        for line in 0..height {
            for ix in 0..width {
                let m = marks.at(line, ix);
                let cc = if m == 'x' { self.at(line, ix) } else { m };
                l.set(line as isize, ix as isize, cc)
            }
        }

        l
    }

    fn filter_pos(&mut self, line: isize, ix: isize, wipe: &str) {
        if let Some(c) = self.map.get_mut(line as usize) {
            if let Some(s) = c.get_mut(ix as usize) {
                if wipe.contains(s.to_owned()) {
                    *s = ' ';
                }
            }
        }
    }
    fn mark_location_as_above_open_tile(&mut self, l: isize, w: isize) {
        self.filter_pos(l, w, "|7F");
    }
    fn mark_location_as_below_open_tile(&mut self, l: isize, w: isize) {
        self.filter_pos(l, w, "|LJ");
    }
    fn mark_location_as_left_of_open_tile(&mut self, l: isize, w: isize) {
        self.filter_pos(l, w, "-FL");
    }
    fn mark_location_as_right_of_open_tile(&mut self, l: isize, w: isize) {
        self.filter_pos(l, w, "-J7");
    }

    fn clean_edges(&mut self) {
        let height = self.map.len() as isize;
        let width = self.map.get(0).unwrap().len() as isize;
        for i in 0..width {
            self.mark_location_as_below_open_tile(0, i);
            self.mark_location_as_above_open_tile(height - 1, i);
        }
        for i in 0..height {
            self.mark_location_as_right_of_open_tile(i, 0);
            self.mark_location_as_left_of_open_tile(i, width - 1);
        }
    }

    fn clean_pass(&mut self) {
        let height = self.map.len() as isize;
        let width = self.map.get(0).unwrap().len() as isize;
        for l in 0..height {
            for w in 0..width {
                let c = self.map.get(l as usize).unwrap().get(w as usize).unwrap();
                match c {
                    '|' => {
                        self.mark_location_as_left_of_open_tile(l, w - 1);
                        self.mark_location_as_right_of_open_tile(l, w + 1);
                    }
                    '-' => {
                        self.mark_location_as_above_open_tile(l - 1, w);
                        self.mark_location_as_below_open_tile(l + 1, w);
                    }
                    'F' => {
                        self.mark_location_as_left_of_open_tile(l, w - 1);
                        self.mark_location_as_above_open_tile(l - 1, w);
                    }
                    '7' => {
                        self.mark_location_as_right_of_open_tile(l, w + 1);
                        self.mark_location_as_above_open_tile(l - 1, w);
                    }
                    'J' => {
                        self.mark_location_as_right_of_open_tile(l, w + 1);
                        self.mark_location_as_below_open_tile(l + 1, w);
                    }
                    'L' => {
                        self.mark_location_as_left_of_open_tile(l, w - 1);
                        self.mark_location_as_below_open_tile(l + 1, w);
                    }
                    '.' => {
                        self.mark_location_as_above_open_tile(l - 1, w);
                        self.mark_location_as_below_open_tile(l + 1, w);
                        self.mark_location_as_left_of_open_tile(l, w - 1);
                        self.mark_location_as_right_of_open_tile(l, w + 1)
                    }
                    ' ' => {
                        self.mark_location_as_above_open_tile(l - 1, w);
                        self.mark_location_as_below_open_tile(l + 1, w);
                        self.mark_location_as_left_of_open_tile(l, w - 1);
                        self.mark_location_as_right_of_open_tile(l, w + 1);
                    }
                    _ => (),
                }
            }
        }
    }
}

impl Walker {
    fn new(coord: Coord, facing: Dir) -> Walker {
        Walker {
            coord,
            facing,
            stepcount: 0,
        }
    }
    fn step(&mut self, l: &Loop) {
        let here = self.coord;
        let mut dx = 0i8;
        let mut dy = 0i8;
        match l.at(here.line as usize, here.ix as usize) {
            '|' => dy = if self.facing == Dir::N { -1 } else { 1 },
            '-' => dx = if self.facing == Dir::W { -1 } else { 1 },
            'L' => {
                if self.facing == Dir::S {
                    self.facing = Dir::E;
                    dx = 1;
                } else {
                    self.facing = Dir::N;
                    dy = -1;
                }
            }
            'J' => {
                if self.facing == Dir::S {
                    self.facing = Dir::W;
                    dx = -1;
                } else {
                    self.facing = Dir::N;
                    dy = -1;
                }
            }
            '7' => {
                if self.facing == Dir::N {
                    self.facing = Dir::W;
                    dx = -1;
                } else {
                    self.facing = Dir::S;
                    dy = 1;
                }
            }
            'F' => {
                if self.facing == Dir::N {
                    self.facing = Dir::E;
                    dx = 1;
                } else {
                    self.facing = Dir::S;
                    dy = 1;
                }
            }
            'S' => dx = if self.facing == Dir::W { -1 } else { 1 },
            _ => {}
        }
        self.coord.line += dy as isize;
        self.coord.ix += dx as isize;
        self.stepcount += 1
    }
    fn step_and_mark(&mut self, l: &Loop, marks: &mut Loop) {
        let here = self.coord;
        marks.set(here.line, here.ix, 'x');

        match l.at(here.line as usize, here.ix as usize) {
            '|' => {
                if self.facing == Dir::N {
                    marks.set(here.line, here.ix - 1, '.');
                    marks.set(here.line, here.ix + 1, 'I');
                } else {
                    marks.set(here.line, here.ix + 1, '.');
                    marks.set(here.line, here.ix - 1, 'I');
                }
            }
            'S' => {
                if self.facing == Dir::E {
                    marks.set(here.line - 1, here.ix, '.');
                    marks.set(here.line + 1, here.ix, 'I');
                } else {
                    marks.set(here.line + 1, here.ix, '.');
                    marks.set(here.line - 1, here.ix, 'I');
                }
            }
            '-' => {
                if self.facing == Dir::E {
                    marks.set(here.line - 1, here.ix, '.');
                    marks.set(here.line + 1, here.ix, 'I');
                } else {
                    marks.set(here.line + 1, here.ix, '.');
                    marks.set(here.line - 1, here.ix, 'I');
                }
            }
            'L' => {
                if self.facing == Dir::S {
                    marks.set(here.line, here.ix - 1, 'I');
                    marks.set(here.line + 1, here.ix, 'I');
                } else {
                    marks.set(here.line, here.ix - 1, '.');
                    marks.set(here.line + 1, here.ix, '.');
                }
            }
            'J' => {
                if self.facing == Dir::S {
                    marks.set(here.line, here.ix + 1, '.');
                    marks.set(here.line + 1, here.ix, '.');
                } else {
                    marks.set(here.line, here.ix + 1, 'I');
                    marks.set(here.line + 1, here.ix, 'I');
                }
            }
            '7' => {
                if self.facing == Dir::N {
                    marks.set(here.line, here.ix + 1, 'I');
                    marks.set(here.line - 1, here.ix, 'I');
                } else {
                    marks.set(here.line, here.ix + 1, '.');
                    marks.set(here.line - 1, here.ix, '.');
                }
            }
            'F' => {
                if self.facing == Dir::N {
                    marks.set(here.line, here.ix - 1, '.');
                    marks.set(here.line - 1, here.ix, '.');
                } else {
                    marks.set(here.line, here.ix - 1, 'I');
                    marks.set(here.line - 1, here.ix, 'I');
                }
            }
            _ => {}
        }

        self.step(l);
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Dir {
    N,
    E,
    S,
    W,
}
struct Walker {
    coord: Coord,
    facing: Dir,
    stepcount: u32,
}

fn experiment_1() {
    let data = "...........\n.F-------7.\n.|F-----7|.\n.||.....||.\n.||.....||.\n.|L-7.F-J|.\n.|..|.|..|.\n.L--J.L--J.\n...........";
    let mut l = Loop::new(&data);
    let paint = walk_mark_loop(&mut l, Coord { line: 1, ix: 1 }, Dir::N);
    println!("{}", paint)
}
fn experiment_2() {
    let data =".F----7F7F7F7F-7....\n.|F--7||||||||FJ....\n.||.FJ||||||||L7....\nFJL7L7LJLJ||LJ.L-7..\nL--J.L7...LJF7F-7L7.\n....F-J..F7FJ|L7L7L7\n....L7.F7||L7|.L7L7|\n.....|FJLJ|FJ|F7|.LJ\n....FJL-7.||.||||...\n....L---J.LJ.LJLJ...";
    let mut l = Loop::new(&data);
    let paint = walk_mark_loop(&mut l, Coord { line: 1, ix: 1 }, Dir::N);
    println!("{}", paint)
}
fn main() {
    let data = std::fs::read_to_string("2023/10.txt").unwrap();
    let mut strs = Vec::new();
    let mut l = Loop::new(&data);
    let marks = walk_mark_loop(&l, Coord { line: 15, ix: 22 }, Dir::N);

    strs.push(l.to_string());
    l.clean_edges();
    strs.push(l.to_string());
    let mut counter = 0;
    loop {
        counter += 1;
        l.clean_pass();
        let tmp = l.to_string();
        if *strs.last().unwrap() == tmp {
            break;
        }
        strs.push(tmp);
    }

    println!("clean iterations {}", counter);
    let start = l.find('S');
    println!("start at {}", start);
    let steps = iter_loop(&mut l, start);
    println!("steps: {}", steps);

    let mut buffer = File::create("./clean.txt").unwrap();
    buffer.write_all(l.to_string().as_bytes()).unwrap();

    let mut tile_count = 0;
    let masked = l.mask(&marks);
    for line in &masked.map {
        let inside: Vec<&char> = line.iter().filter(|&x| *x == 'I').collect();
        tile_count += inside.len();
    }
    // also sum the space in the middle
    for l in 60..85 {
        for x in 50..100 {
            if &masked.at(l, x) == &' ' {
                tile_count += 1;
            }
        }
    }
    println!("inner area: {}", tile_count);

    // 887 is too high!
    // 260 is too low
}
fn walk_mark_loop(l: &Loop, start: Coord, facing: Dir) -> Loop {
    let mut walker = Walker::new(start, facing);
    let mut marks = Loop::new_sized(l.map.len(), l.map[0].len());
    walker.step_and_mark(l, &mut marks);
    loop {
        if walker.coord == start {
            break;
        }
        walker.step_and_mark(l, &mut marks);
    }
    return marks;
}
fn iter_loop(l: &mut Loop, start: Coord) -> u32 {
    // cheat, we know first step is horizontal in both dirs
    let mut walkers = vec![
        Walker::new(Coord::new(start.line, start.ix - 1), Dir::W),
        Walker::new(Coord::new(start.line, start.ix + 1), Dir::E),
    ];
    let mut marks = Loop::new_sized(l.map.len(), l.map[0].len());
    marks.set(start.line, start.ix, 'x');
    loop {
        if walkers[0].coord == walkers[1].coord {
            break;
        }
        walkers[0].step(l);
        marks.set(walkers[0].coord.line, walkers[0].coord.ix, 'x');
        walkers[1].step(l);
        marks.set(walkers[1].coord.line, walkers[1].coord.ix, 'x');
    }

    let height = l.map.len();
    let width = l.map.get(0).unwrap().len();

    for line in 0..height {
        for ix in 0..width {
            if marks.at(line, ix) != 'x' {
                l.set(line as isize, ix as isize, ' ');
            }
        }
    }

    let stepcount = walkers[0].stepcount + 1;
    return stepcount;
}

#[cfg(test)]
mod tests {
    use super::*;
    static TEST_DATA: &str = r"7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";
    #[test]
    fn test_can_make_loop() {
        let d = Loop::new(TEST_DATA);
        assert_eq!('7', *d.map.get(0).unwrap().get(0).unwrap());
    }
    #[test]
    fn test_can_clean_edges() {
        let mut d = Loop::new("|JL7.");
        d.clean_edges();
        assert_eq!(r"    .", d.to_string());
        let mut d = Loop::new("...\n-FF\nJ.|\nL-J");
        d.clean_edges();
        assert_eq!("...\n F \n .|\nL-J", d.to_string());
    }
    #[test]
    fn test_can_clean() {
        let mut d = Loop::new(TEST_DATA);
        d.clean_edges();
        d.clean_pass();

        assert_eq!("  F7 \n.FJ| \nSJ L7\n|F--J\nLJ.  ", d.to_string());
    }
    #[test]
    fn test_doesnt_clean_loop_parts() {
        let mut d = Loop::new("F-7\n|.|\nL-J");
        d.clean_edges();
        assert_eq!("F-7\n|.|\nL-J", d.to_string());
        d.clean_pass();

        assert_eq!("F-7\n|.|\nL-J", d.to_string());
    }
    #[test]
    fn test_coord_eq() {
        assert_eq!(Coord::new(3, 2), Coord::new(3, 2))
    }
    #[test]
    fn test_walker_can_step_marked_f() {
        let d = Loop::new("...\n.F.\n...");
        let mut m = Loop::new_sized(3, 3);
        let mut w = Walker::new(Coord::new(1, 1), Dir::N);
        w.step_and_mark(&d, &mut m);
        assert_eq!(Coord::new(1, 2), w.coord);
        assert_eq!(Dir::E, w.facing);
        assert_eq!(" . \n.x \n   ", m.to_string());

        let mut m = Loop::new_sized(3, 3);
        let mut w = Walker::new(Coord::new(1, 1), Dir::W);
        w.step_and_mark(&d, &mut m);
        assert_eq!(Coord::new(2, 1), w.coord);
        assert_eq!(Dir::S, w.facing);
        assert_eq!(" I \nIx \n   ", m.to_string());
    }
    #[test]
    fn test_walker_can_step_marked_7() {
        let d = Loop::new("...\n.7.\n...");
        let mut m = Loop::new_sized(3, 3);
        let mut w = Walker::new(Coord::new(1, 1), Dir::N);
        w.step_and_mark(&d, &mut m);
        assert_eq!(Coord::new(1, 0), w.coord);
        assert_eq!(Dir::W, w.facing);
        assert_eq!(" I \n xI\n   ", m.to_string());

        let mut m = Loop::new_sized(3, 3);
        let mut w = Walker::new(Coord::new(1, 1), Dir::E);
        w.step_and_mark(&d, &mut m);
        assert_eq!(Coord::new(2, 1), w.coord);
        assert_eq!(Dir::S, w.facing);
        assert_eq!(" . \n x.\n   ", m.to_string());
    }
    #[test]
    fn test_walker_can_step_marked_l() {
        let d = Loop::new("...\n.L.\n...");
        let mut m = Loop::new_sized(3, 3);
        let mut w = Walker::new(Coord::new(1, 1), Dir::S);
        w.step_and_mark(&d, &mut m);
        assert_eq!(Coord::new(1, 2), w.coord);
        assert_eq!(Dir::E, w.facing);
        assert_eq!("   \nIx \n I ", m.to_string());

        let mut m = Loop::new_sized(3, 3);
        let mut w = Walker::new(Coord::new(1, 1), Dir::W);
        w.step_and_mark(&d, &mut m);
        assert_eq!(Coord::new(0, 1), w.coord);
        assert_eq!(Dir::N, w.facing);
        assert_eq!("   \n.x \n . ", m.to_string());
    }
    #[test]
    fn test_walker_can_step() {
        let d = Loop::new("F-7\n|.|\nL-J");
        let mut w = Walker::new(Coord::new(1, 0), Dir::N);
        w.step(&d);
        assert_eq!(Coord::new(0, 0), w.coord);
        assert_eq!(Dir::N, w.facing);
        w.step(&d);
        assert_eq!(Coord::new(0, 1), w.coord);
        assert_eq!(Dir::E, w.facing);
        w.step(&d);
        assert_eq!(Coord::new(0, 2), w.coord);
        assert_eq!(Dir::E, w.facing);
        w.step(&d);
        assert_eq!(Coord::new(1, 2), w.coord);
        assert_eq!(Dir::S, w.facing);
        w.step(&d);
        assert_eq!(Coord::new(2, 2), w.coord);
        assert_eq!(Dir::S, w.facing);
        w.step(&d);
        assert_eq!(Coord::new(2, 1), w.coord);
        assert_eq!(Dir::W, w.facing);
        w.step(&d);
        assert_eq!(Coord::new(2, 0), w.coord);
        assert_eq!(Dir::W, w.facing);
        w.step(&d);
        assert_eq!(Coord::new(1, 0), w.coord);
        assert_eq!(Dir::N, w.facing);
    }
}
