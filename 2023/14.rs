use md5;
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Rock {
    Round,
    Square,
    Empty,
}

impl Rock {
    fn from_char(c: char) -> Rock {
        match c {
            '#' => Rock::Square,
            'O' => Rock::Round,
            _ => Rock::Empty,
        }
    }
}

struct Platform {
    data: Vec<Vec<Rock>>,
    width: usize,
    height: usize,
}
impl Platform {
    fn new(strdata: &str) -> Platform {
        let mut data: Vec<Vec<Rock>> = Vec::new();
        for line in strdata.lines() {
            data.push(line.chars().map(Rock::from_char).collect());
        }
        let height = data.len();
        let width = data.get(0).unwrap().len();
        Platform {
            data,
            height,
            width,
        }
    }
    fn rotate_cw(&mut self) {
        let mut data: Vec<Vec<Rock>> = (0..(self.width)).map(|_| Vec::new()).collect();

        self.data.reverse();
        for row in &self.data {
            for (ix, rock) in row.iter().enumerate() {
                data[ix].push(*rock);
            }
        }
        let tmp = self.width;
        self.width = self.height;
        self.height = tmp;
        self.data = data;
    }
    fn rotate_ccw(&mut self) {
        let mut data: Vec<Vec<Rock>> = (0..(self.width)).map(|_| Vec::new()).collect();

        for row in self.data.iter_mut() {
            row.reverse();
            for (ix, rock) in row.iter().enumerate() {
                data[ix].push(*rock);
            }
        }
        let tmp = self.width;
        self.width = self.height;
        self.height = tmp;
        self.data = data;
    }
    fn sort_segmented_row(data: &mut Vec<Rock>, from: usize, to: usize, rock_count: usize) {
        // to is an index, not included in sort
        let delta = to - from;
        if delta <= 1 {
            return;
        }
        assert!(delta >= rock_count);
        let empty_count = delta - rock_count;
        let empty_limit = to - rock_count;
        for ix in from..to {
            let d = data.get_mut(ix).unwrap();
            *d = if ix >= empty_limit {
                Rock::Round
            } else {
                Rock::Empty
            };
        }
    }
    fn tilt_row_right(&mut self, row: usize) {
        let mut v = self.data.get_mut(row).unwrap();
        let mut left_edge = 0;
        let mut rock_count = 0;
        for ix in 0..self.width {
            match v[ix] {
                Rock::Square => {
                    Platform::sort_segmented_row(&mut v, left_edge, ix, rock_count);
                    left_edge = ix + 1;
                    rock_count = 0;
                }
                Rock::Round => {
                    rock_count += 1;
                }
                _ => (),
            }
        }
        if left_edge < v.len() - 1 {
            let end = v.len();
            Platform::sort_segmented_row(&mut v, left_edge, end, rock_count);
        }
    }
    fn tilt_right(&mut self) {
        for r in 0..self.height {
            self.tilt_row_right(r);
        }
    }
    fn cycle(&mut self) {
        self.tilt_right();
        self.rotate_cw();
        self.tilt_right();
        self.rotate_cw();
        self.tilt_right();
        self.rotate_cw();
        self.tilt_right();
        self.rotate_cw();
    }
    fn sum(&self) -> usize {
        let mut sum = 0;
        for (ix, row) in self.data.iter().enumerate() {
            let one_rock_weight = self.height - ix;
            let rock_count = row.iter().filter(|&r| r == &Rock::Round).count();
            sum += one_rock_weight * rock_count;
        }
        sum
    }
}
impl std::fmt::Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in &self.data {
            let a = row.iter().map(|q| match q {
                Rock::Square => b'#',
                Rock::Round => b'O',
                _ => b'.',
            });
            let s = String::from_utf8(a.collect()).unwrap();
            if let Err(e) = writeln!(f, "{}", s) {
                return Err(e);
            }
        }
        Ok(())
    }
}
fn main() {
    let data = std::fs::read_to_string("2023/14.txt").unwrap();
    let mut p = Platform::new(&data);
    p.rotate_cw();
    p.tilt_right();
    p.rotate_ccw();
    println!("summed: {}", p.sum());
    let data = std::fs::read_to_string("2023/14.txt").unwrap();
    let mut p = Platform::new(&data);
    p.rotate_cw();
    // hypothesis, repeats stable
    // seems to have a freq of 9
    for x in 1..=595 {
        p.cycle();
        let str = p.to_string();
        let digest = md5::compute(str);
        p.rotate_ccw();
        let sum = p.sum();
        p.rotate_cw();
        println!("after {} {:x} {}", x, digest, sum);
    }
    p.rotate_ccw();

    println!("spinned: {}", p.sum());

    // 91100 is too low
    // 95619 is too low
    // 95648 is too low
}

#[cfg(test)]
mod tests {
    use super::*;
    static TEST_DATA: &str = r"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
    #[test]
    fn test_spin_cycle() {
        let mut p = Platform::new(TEST_DATA);
        p.rotate_cw();
        p.cycle();
        p.rotate_ccw();
        assert_eq!(
            r".....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#....
",
            p.to_string()
        );

        p.rotate_cw();
        p.cycle();
        p.rotate_ccw();
        assert_eq!(
            r".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#..OO###..
#.OOO#...O
",
            p.to_string()
        );

        p.rotate_cw();
        p.cycle();
        p.rotate_ccw();
        assert_eq!(
            r".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O
",
            p.to_string()
        );

        // p.rotate_cw();
        // for _ in 0..1_000 {
        //     p.cycle();
        // }
        //        assert_eq!(64, p.sum());
    }

    #[test]
    fn test_can_tilt_right() {
        let mut p = Platform::new("O..........OOO#.O.O.#.....OO...O.#..#...OO....OO#O.#OOO..OO..#.OO.....#O##.O..#.OOO...#.#O#...#.OO..");
        p.tilt_row_right(0);
        assert_eq!("..........OOOO#...OO#.........OOO#..#.......OOOO#.O#....OOOOO#......OO#O##...O#....OOO#.#O#...#...OO\n",p.to_string())
    }
    #[test]
    fn test_can_tilt_right_oops_all_rocks() {
        let mut p = Platform::new("OOOO");
        p.tilt_row_right(0);
        assert_eq!("OOOO\n", p.to_string())
    }
    #[test]
    fn test_can_make_platform() {
        let p = Platform::new(&TEST_DATA);
        assert_eq!(Rock::Square, *p.data.get(1).unwrap().get(4).unwrap());
        assert_eq!(10, p.width);
        assert_eq!(10, p.height);
    }
    #[test]
    fn test_can_tostring() {
        assert_eq!(TEST_DATA, Platform::new(&TEST_DATA).to_string().trim())
    }
    #[test]
    fn test_can_rotate_cw() {
        let mut p = Platform::new(&TEST_DATA);
        p.rotate_cw();
        p.rotate_cw();
        p.rotate_cw();
        p.rotate_cw();
        assert_eq!(TEST_DATA, p.to_string().trim());
        p.rotate_cw();
        p.rotate_ccw();
        assert_eq!(TEST_DATA, p.to_string().trim());
        let mut p = Platform::new(&"#...\n..O.");
        p.rotate_cw();
        assert_eq!(".#\n..\nO.\n..\n", p.to_string());
        assert_eq!(2, p.width);
        assert_eq!(4, p.height);
        p.rotate_ccw();
        assert_eq!("#...\n..O.\n", p.to_string());
    }
}
