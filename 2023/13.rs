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
    fn is_horizontal_candidate(&self, x: usize, row: isize) -> bool {
        let s: Vec<u8> = self
            .data
            .iter()
            .map(|st| *st.as_bytes().get(x as usize).unwrap())
            .collect();
        let s_size = s.len() as isize;

        for ix in 0isize.. {
            let top_ix: isize = row - ix;
            if top_ix < 0 {
                return true;
            }
            let bottom_ix: isize = (row + 1) + ix;
            if bottom_ix >= s_size {
                return true;
            }
            if s[top_ix as usize] != s[bottom_ix as usize] {
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
    fn is_horizontal_valid(&self, row: isize) -> bool {
        //col 0 already checked
        for x in 1..self.width {
            if !self.is_horizontal_candidate(x, row) {
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

    fn find_horizontal(&self) -> Option<usize> {
        let mut candidates: Vec<usize> = Vec::new();
        for r in 0..(&self.height - 1) {
            if self.is_horizontal_candidate(0, r as isize) {
                candidates.push(r);
            }
        }

        for c in candidates {
            if self.is_horizontal_valid(c as isize) {
                return Some(c);
            }
        }

        None
    }

    fn summarise(&self) -> usize {
        if let Some(c) = self.find_vertical() {
            return c + 1;
        }
        if let Some(r) = self.find_horizontal() {
            return (r + 1) * 100;
        }
        panic!("invalid pattern")
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
    let data = std::fs::read_to_string("2023/13.txt").unwrap();
    let p = Pattern::new_vec(&data);

    println!(
        "summarized: {}",
        p.iter().map(|f| f.summarise()).sum::<usize>()
    );
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
    fn test_can_summarize() {
        let p = Pattern::new_vec(TEST_DATA);

        assert_eq!(405usize, p.iter().map(|f| f.summarise()).sum());
    }
    #[test]
    fn test_find_vertical() {
        let p = Pattern::new_vec(TEST_DATA);
        assert_eq!(Some(4), p[0].find_vertical());
        assert_eq!(None, p[1].find_vertical());
    }
    #[test]
    fn test_find_horizontal() {
        let p = Pattern::new_vec(TEST_DATA);
        assert_eq!(None, p[0].find_horizontal());
        assert_eq!(Some(3), p[1].find_horizontal());
    }
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
