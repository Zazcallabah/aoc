use console::Term;
use std::{collections::HashMap, fs::File, io::Write};

#[derive(Copy, Clone, Eq, PartialEq)]
enum MarkState {
    Source,
    Empty,
    Owned,
    Claimed,
    Tied,
}
#[derive(Copy, Clone)]
struct Mark {
    owner: Option<u8>,
    state: MarkState,
}
impl Mark {
    fn to_char(&self) -> char {
        match self.owner {
            Some(o) => {
                let offset = if self.state == MarkState::Claimed {
                    'a' as u8
                } else {
                    'A' as u8
                };
                (o + offset) as char
            }
            None => '.',
        }
    }
}

struct Field {
    limit_top: isize,
    limit_bottom: isize,
    limit_left: isize,
    limit_right: isize,
    rows: HashMap<isize, HashMap<isize, Mark>>,
}
impl Field {
    fn new(data: &str) -> Field {
        let mut rows: HashMap<isize, HashMap<isize, Mark>> = HashMap::new();
        let mut index = 0;
        let mut limit_top = isize::MAX;
        let mut limit_bottom = isize::MIN;
        let mut limit_left = isize::MAX;
        let mut limit_right = isize::MIN;
        for line in data.lines() {
            let spl: Vec<isize> = line
                .split(", ")
                .map(|a| a.parse().unwrap())
                .collect::<Vec<isize>>();
            let coord: isize = *spl.get(0).unwrap();
            let r: isize = *spl.get(1).unwrap();
            let row: &mut HashMap<isize, Mark> = rows.entry(r).or_insert_with(|| HashMap::new());
            if coord > limit_right {
                limit_right = coord;
            }
            if coord < limit_left {
                limit_left = coord;
            }
            if r > limit_bottom {
                limit_bottom = r;
            }
            if r < limit_top {
                limit_top = r;
            }
            row.insert(
                coord,
                Mark {
                    owner: Some(index),
                    state: MarkState::Source,
                },
            );
            index += 1;
        }
        Field {
            rows,
            limit_bottom,
            limit_left,
            limit_right,
            limit_top,
        }
    }
    fn len(&self) -> usize {
        let mut sum = 0;
        for r in &self.rows {
            sum += r.1.len();
        }
        sum
    }
    fn at_edge(&self, row: isize, coord: isize) -> bool {
        row < self.limit_top
            || row > self.limit_bottom
            || coord < self.limit_left
            || coord > self.limit_right
    }
    fn get(&mut self, row: isize, coord: isize) -> Option<&mut Mark> {
        match self.rows.get_mut(&row) {
            Some(r) => match r.get_mut(&coord) {
                Some(m) => Some(m),
                None => None,
            },
            None => None,
        }
    }
    fn add(&mut self, row: isize, coord: isize, owner: Option<u8>) {
        let owner = match owner {
            Some(o) => o,
            None => panic!("invalid add"),
        };
        //  if !self.at_edge(row, coord) {
        match self.get(row, coord) {
            Some(s) => match s.state {
                MarkState::Source => (),
                MarkState::Claimed => {
                    if s.owner.unwrap() != owner {
                        (*s).state = MarkState::Tied;
                        (*s).owner = None;
                    }
                }
                MarkState::Empty => panic!("illegal operation"),
                MarkState::Tied => (),
                MarkState::Owned => (),
            },
            None => {
                let rr: &mut HashMap<isize, Mark> =
                    self.rows.entry(row).or_insert_with(|| HashMap::new());
                rr.insert(
                    coord,
                    Mark {
                        owner: Some(owner),
                        state: MarkState::Claimed,
                    },
                );
            }
        }
        //  }
    }
    fn step(&mut self, steptype: MarkState) {
        let mut coords: Vec<(isize, isize, Option<u8>)> = Vec::new();
        for (rowline, rowdata) in &self.rows {
            for (coord, entry) in rowdata {
                if entry.state == steptype {
                    coords.push((*rowline, *coord, entry.owner));
                }
            }
        }
        for (x, y, owner) in coords {
            self.add(x - 1, y, owner);
            self.add(x + 1, y, owner);
            self.add(x, y - 1, owner);
            self.add(x, y + 1, owner);
        }
    }
    fn commit(&mut self) {
        // commit all claims into owned tiles
        for (_, rowdata) in self.rows.iter_mut() {
            for (_, entry) in rowdata.iter_mut() {
                if entry.state == MarkState::Claimed {
                    entry.state = MarkState::Owned;
                }
            }
        }
    }
    fn to_string(&self, x: isize, y: isize, w: usize, h: usize) -> String {
        let mut lines = Vec::with_capacity(h);
        for row in y..(y + h as isize) {
            let mut rowdata = vec!['.'; w];
            //            rowdata.fill('.');
            match self.rows.get(&row) {
                None => {}
                Some(r) => {
                    for (index, entry) in r {
                        let delta = index - x;
                        if delta >= 0 && delta < w as isize {
                            rowdata[delta as usize] = entry.to_char();
                        }
                    }
                }
            }
            let linedata = rowdata.iter().collect::<String>();
            lines.push(linedata);
        }
        return lines.join(&"\n");
    }
}
fn navigate(field: &Field) {
    let stdout = Term::buffered_stdout();
    let mut x = 80isize;
    let mut y = 140isize;
    loop {
        println!("!");
        println!("{}", field.to_string(x, y, 70, 20));

        if let Ok(character) = stdout.read_char() {
            match character {
                'å' => y -= 1,
                'a' => x -= 1,
                'o' => y += 1,
                'e' => x += 1,
                _ => return,
            }
        }
    }
}
#[derive(Debug)]
struct Counter {
    infinite: bool,
    count: u64,
}
fn count_max(field: &Field) -> u64 {
    let mut counters: HashMap<u8, Counter> = HashMap::new();
    for (row, map) in &field.rows {
        for (coord, entry) in map {
            if let Some(e) = entry.owner {
                if entry.state == MarkState::Owned || entry.state == MarkState::Source {
                    let counter = counters.entry(e).or_insert(Counter {
                        count: 0,
                        infinite: false,
                    });
                    if field.at_edge(*row, *coord) {
                        counter.infinite = true;
                    }
                    counter.count += 1;
                }
            }
        }
    }
    let mut max = 0;
    for c in counters.iter().filter(|c| !c.1.infinite) {
        if max < c.1.count {
            max = c.1.count;
        }
    }
    max
    // let mut sum = 0;
    // for (_, c) in counters {
    //     if !c.infinite {
    //         sum += c.count;
    //     }
    // }
    // sum
}
fn main() {
    let data = std::fs::read_to_string("2018/6.txt").unwrap();
    let mut field = Field::new(&data);

    field.step(MarkState::Source);
    field.commit();

    for i in 0..450 {
        let before = field.len();
        field.step(MarkState::Owned);
        field.commit();
        let after = field.len();
        println!("{}: {} diff {}", i, after, after - before);
    }

    let printed = field.to_string(0, 0, 1000, 1000);
    let mut buffer = File::create("./result.txt").unwrap();
    buffer.write_all(printed.as_bytes()).unwrap();

    let max = count_max(&field);

    println!("max: {}", max);
    // 97189 is too high
    // 5820 is too high

    //navigate(&field);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_parse_field() {
        let c = Field::new("1, 1\n1, 6\n8, 3\n3, 4\n5, 5\n8, 9");
        assert_eq!(1, c.rows.get(&6).unwrap().len());
    }
    #[test]
    fn test_can_print_field() {
        let c = Field::new("1, 1\n1, 6\n8, 3\n3, 4\n5, 5\n8, 9");
        assert_eq!("....\nD...\n..E.\n....", c.to_string(3, 3, 4, 4));
        assert_eq!("....\n....\n.D..\n...E", c.to_string(2, 2, 4, 4));
    }
    #[test]
    fn test_has_limits() {
        let c = Field::new("1, 1\n1, 6\n8, 3\n3, 4\n5, 5\n8, 9");
        assert_eq!(1, c.limit_top);
        assert_eq!(1, c.limit_left);
        assert_eq!(8, c.limit_right);
        assert_eq!(9, c.limit_bottom);
    }
    #[test]
    fn test_can_detect_field_edge() {
        let mut c = Field::new("1, 1\n1, 6\n8, 3\n3, 4\n5, 5\n8, 9");
        c.step(MarkState::Source);
        assert_eq!(true, c.at_edge(-1, -1));
        assert_eq!(false, c.at_edge(1, 1));
    }

    #[test]
    fn test_can_init_step() {
        let mut c = Field::new("1, 1\n1, 6\n8, 3\n3, 4\n5, 5\n8, 9");
        c.step(MarkState::Source);
        assert_eq!("d...\nDde.\ndeEe\n..e.", c.to_string(3, 3, 4, 4));
    }
    #[test]
    fn test_can_commit_step() {
        let mut c = Field::new("1, 1\n1, 6\n8, 3\n3, 4\n5, 5\n8, 9");
        c.step(MarkState::Source);
        c.commit();
        assert_eq!("D...\nDDE.\nDEEE\n..E.", c.to_string(3, 3, 4, 4));
    }
    #[test]
    fn test_can_count_field() {
        let mut field = Field::new("1, 1\n1, 6\n8, 3\n3, 4\n5, 5\n8, 9");
        field.step(MarkState::Source);
        field.commit();
        for _ in 0..10 {
            field.step(MarkState::Owned);
            field.commit();
        }
        assert_eq!(17, count_max(&field))
    }
}
