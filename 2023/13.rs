struct Pattern {
    data: Vec<String>,
    width: usize,
    height: usize,
}

impl std::fmt::Display for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in &self.data {
            if let Err(e) = writeln!(f, "{}", row) {
                return Err(e);
            }
        }
        Ok(())
    }
}
impl Pattern {
    fn is_vertical_candidate(&self, x: isize, row: usize) -> bool {
        let s = self.data[row].as_bytes();
        let s_size = s.len() as isize;
        for ix in 0isize.. {
            let left_ix: isize = x - ix;
            if left_ix < 0 {
                return true;
            }
            let right_ix: isize = (x + 1) + ix;
            if right_ix >= s_size {
                return true;
            }
            if s[left_ix as usize] != s[right_ix as usize] {
                return false;
            }
        }
        false
    }

    fn is_vertical_valid(&self, x: isize) -> bool {
        //row 0 already checked
        for row in 1..self.height {
            if !self.is_vertical_candidate(x, row) {
                return false;
            }
        }
        return true;
    }

    fn find_vertical(&self) -> Option<usize> {
        let mut candidates: Vec<usize> = Vec::new();
        for c in 0..(&self.width - 1) {
            if self.is_vertical_candidate(c as isize, 0) {
                candidates.push(c);
            }
        }

        for c in candidates {
            if self.is_vertical_valid(c as isize) {
                return Some(c);
            }
        }

        None
    }

    fn new_vec(data: &str) -> Vec<Pattern> {
        let mut r: Vec<Pattern> = Vec::new();
        let mut collector: Vec<String> = Vec::new();
        for line in data.lines() {
            if line == "" {
                r.push(Pattern::new(collector));
                collector = Vec::new();
            } else {
                collector.push(line.to_owned());
            }
        }
        if collector.len() > 0 {
            r.push(Pattern::new(collector));
        }
        r
    }
    fn new(data: Vec<String>) -> Pattern {
        let width = data.get(0).unwrap().len();
        let height = data.len();
        Pattern {
            data,
            width,
            height,
        }
    }
}
fn main() {
    let data = std::fs::read_to_string("2018/7.txt").unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    static TEST_DATA: &str = r"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
";
    #[test]
    fn test_can_determine_is_candidate() {
        let p = Pattern::new_vec(TEST_DATA);
        assert!(!p[0].is_vertical_candidate(0, 0));
        assert!(!p[0].is_vertical_candidate(1, 0));
        assert!(!p[0].is_vertical_candidate(2, 0));
        assert!(!p[0].is_vertical_candidate(3, 0));
        assert!(p[0].is_vertical_candidate(4, 0));
        assert!(!p[0].is_vertical_candidate(5, 0));
    }
    #[test]
    fn test_can_count_matches() {
        let p = Pattern::new_vec(TEST_DATA);
        assert_eq!(2, p.len());
    }
}
