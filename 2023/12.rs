use std::fmt::write;

use regex::Regex;
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum MapSegment {
    Operational,
    Damaged,
    Unknown,
}

struct SegmentVector(Vec<MapSegment>);

impl MapSegment {
    fn to_vec(data: &str) -> Vec<MapSegment> {
        data.chars()
            .map(|c| match c {
                '.' => MapSegment::Operational,
                '#' => MapSegment::Damaged,
                '?' => MapSegment::Unknown,
                _ => panic!("invalid input for mapsegment"),
            })
            .collect()
    }
}
impl std::fmt::Display for SegmentVector {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut result: Vec<u8> = Vec::new();
        for i in &self.0 {
            let mapped = match i {
                MapSegment::Damaged => '#',
                MapSegment::Operational => '.',
                MapSegment::Unknown => '?',
            };
            result.push(mapped as u8);
        }
        write!(f, "{}", &String::from_utf8(result).unwrap())
    }
}
struct Map {
    record: Record,
    map: Vec<MapSegment>,
}
impl Map {
    fn find_next_unknown(bench: &[MapSegment], from: usize) -> Option<usize> {
        for ix in from.. {
            if let Some(segment) = bench.get(ix) {
                if *segment == MapSegment::Unknown {
                    return Some(ix);
                }
            } else {
                return None;
            }
        }
        None
    }
    fn count_matches_inner(&self, bench: &mut [MapSegment], current: usize) -> u32 {
        // current should be pointing to an unknown
        let next_unknown_index = Map::find_next_unknown(&bench, current + 1);

        let mut recursed_sum = 0;
        bench[current] = MapSegment::Damaged;
        let a_branch_can_match = self.can_possibly_match(bench);
        if a_branch_can_match {
            if let Some(ix) = next_unknown_index {
                recursed_sum += self.count_matches_inner(bench, ix);
            } else {
                //                println!("{} matches", SegmentVector(bench.to_owned()));
                assert!(self.map[current] == MapSegment::Unknown);
                bench[current] = MapSegment::Unknown; // restore
                return 1;
            }
        }

        bench[current] = MapSegment::Operational;
        let b_branch_can_match = self.can_possibly_match(bench);
        if b_branch_can_match {
            if let Some(ix) = next_unknown_index {
                recursed_sum += self.count_matches_inner(bench, ix);
            } else {
                //                println!("{} matches", SegmentVector(bench.to_owned()));
                assert!(self.map[current] == MapSegment::Unknown);
                bench[current] = self.map[current]; // restore
                return 1;
            }
        }
        bench[current] = self.map[current]; // restore
        return recursed_sum;
    }
    fn count_matches(&self) -> u32 {
        if let Some(first_index) = Map::find_next_unknown(&self.map, 0) {
            return self.count_matches_inner(&mut self.map.clone(), first_index);
        } else {
            return 0;
        }
    }
    fn minimum_width(&self, from_group: usize) -> Option<u8> {
        let group_count = self.record.entries.len();
        if from_group >= group_count {
            return None;
        }
        let mut sum: u8 = 0;
        for g in from_group..group_count {
            sum += self.record.entries.get(g).unwrap() + 1;
        }
        Some(sum - 1)
    }
    fn can_possibly_match(&self, map: &[MapSegment]) -> bool {
        let mut marker = 0u8;
        let mut groupsize = 0u8;
        let mut groupnumber = 0usize;
        let mut currently_counting_damaged_group = false;

        for s in map {
            if currently_counting_damaged_group {
                match *s {
                    MapSegment::Damaged => groupsize += 1,

                    MapSegment::Operational => {
                        match self.record.entries.get(groupnumber) {
                            Some(record) => {
                                if *record != groupsize {
                                    return false;
                                }
                            }
                            None => return false,
                        }
                        currently_counting_damaged_group = false;
                        groupsize = 0;
                        groupnumber += 1;
                    }
                    MapSegment::Unknown => {
                        if let Some(cc) = self.minimum_width(groupnumber) {
                            // counting from where this group started, how much remains of width
                            let remaining_unchecked = map.len() - (marker - groupsize) as usize;
                            if cc > remaining_unchecked as u8 {
                                println!(
                                    "CU {} r{:?}",
                                    SegmentVector(map.to_owned()),
                                    self.record.entries
                                );
                                return false;
                            }
                        } else {
                            println!(
                                "CX {} r{:?}",
                                SegmentVector(map.to_owned()),
                                self.record.entries
                            );
                            return false;
                        }
                        if let Some(record) = self.record.entries.get(groupnumber) {
                            if *record < groupsize {
                                return false;
                            }
                            return true;
                        }
                        return false;
                    }
                }
            } else {
                if *s == MapSegment::Unknown {
                    if let Some(cc) = self.minimum_width(groupnumber as usize) {
                        let remaining_unchecked = map.len() - marker as usize;
                        if cc > remaining_unchecked as u8 {
                            println!(
                                "EU {} r{:?}",
                                SegmentVector(map.to_owned()),
                                self.record.entries
                            );
                            return false;
                        }
                        // } else {
                        //     println!(
                        //         "EX {} r{:?}",
                        //         SegmentVector(map.to_owned()),
                        //         self.record.entries
                        //     );
                        //     return false;
                    }
                    return true;
                }

                if *s == MapSegment::Damaged {
                    currently_counting_damaged_group = true;
                    groupsize += 1;
                }
            }
            marker += 1;
        }
        if currently_counting_damaged_group {
            if self.record.entries.len() != groupnumber as usize + 1 {
                println!(
                    "NX {} r{:?}",
                    SegmentVector(map.to_owned()),
                    self.record.entries
                );
                return false;
            }
            match self.record.entries.get(groupnumber as usize) {
                Some(record) => {
                    if *record != groupsize as u8 {
                        return false;
                    }
                }
                None => return false,
            }
        }

        return true;
    }
    fn new(data: &str) -> Map {
        let spl: Vec<&str> = data.split(' ').collect();
        let record = Record::new(spl[1]);
        let map: Vec<MapSegment> = MapSegment::to_vec(spl[0]);
        Map { record, map }
    }
}
struct Record {
    entries: Vec<u8>,
    regex: Regex,
}
impl Record {
    fn new(data: &str) -> Record {
        let entries: Vec<u8> = data
            .split(',')
            .into_iter()
            .filter_map(|f: &str| match f.parse::<u8>() {
                Err(_) => None,
                Ok(d) => Some(d),
            })
            .collect();
        let rstring = entries
            .iter()
            .fold(String::new(), |res, n| res + &format!("#{{{}}}\\.*", n));
        let regex = Regex::new(format!("^\\.*{}\\.*$", rstring).as_str()).unwrap();

        Record { entries, regex }
    }

    fn str_match(&self, test: &str) -> bool {
        self.regex.is_match(test)
    }

    // input bool array where true = broken
    fn bool_match(&self, bools: &[bool]) -> bool {
        let mut entry_ix = 0;
        let mut finding = true;
        let mut counting = 0u8;

        for b in bools {
            if finding {
                if *b {
                    finding = false;
                    counting = 1;
                }
            } else {
                if *b {
                    counting += 1;
                } else {
                    match &self.entries.get(entry_ix) {
                        Some(x) => {
                            if &counting != *x {
                                return false;
                            }
                        }
                        None => return false,
                    }
                }
            }
        }

        false
    }
}
fn main() {
    let data = std::fs::read_to_string("2018/7.txt").unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    static TEST_DATA: &str = r"";
    #[test]
    fn test_can_count_matches() {
        // assert_eq!(1, Map::new("???.### 1,1,3").count_matches());
        // assert_eq!(4, Map::new(".??..??...?##. 1,1,3").count_matches());
        // assert_eq!(1, Map::new("?#?#?#?#?#?#?#? 1,3,1,6").count_matches());
        // assert_eq!(1, Map::new("????.#...#... 4,1,1").count_matches());
        // assert_eq!(4, Map::new("????.######..#####. 1,6,5").count_matches());

        assert_eq!(10, Map::new("?###???????? 3,2,1").count_matches());
    }
    #[test]
    fn test_recursion_matches() {
        let m = Map::new(".###.....??? 3,2,1");
        let n = Map::find_next_unknown(&m.map, 0);
        assert_eq!(Some(9), n);
        let mut bench = m.map.clone();
        let r = m.count_matches_inner(&mut bench, 9);
        assert_eq!(0, r)
    }
    #[test]
    fn test_can_find_specific() {
        let map = Map::new(&".###.##....? 3,2,1");
        assert_eq!(true, map.can_possibly_match(&map.map));
    }

    #[test]
    fn test_can_find_next_unknown() {
        assert_eq!(
            Some(0),
            Map::find_next_unknown(&Map::new("???.### 1,1,3").map, 0)
        );
        assert_eq!(
            Some(1),
            Map::find_next_unknown(&Map::new(".??..??...?##. 1,1,3").map, 0)
        );
        assert_eq!(
            Some(6),
            Map::find_next_unknown(&Map::new("?#?#?#?#?#?#?#? 1,3,1,6").map, 5)
        );
        assert_eq!(
            Some(3),
            Map::find_next_unknown(&Map::new("????.#...#... 4,1,1").map, 3)
        );
        assert_eq!(
            None,
            Map::find_next_unknown(&Map::new("????.######..#####. 1,6,5").map, 4)
        );
        assert_eq!(
            Some(4),
            Map::find_next_unknown(&Map::new("?###???????? 3,2,1").map, 1)
        );
    }
    #[test]
    fn test_can_line_match_exact() {
        let map = Map::new(&".###.##.#... 3,2,1");
        assert_eq!(true, map.can_possibly_match(&map.map));
        let map = Map::new(&".###.##..#.. 3,2,1");
        assert_eq!(true, map.can_possibly_match(&map.map));
        let map = Map::new(&".###.##...#. 3,2,1");
        assert_eq!(true, map.can_possibly_match(&map.map));
        let map = Map::new(&".###.##....# 3,2,1");
        assert_eq!(true, map.can_possibly_match(&map.map));
        let map = Map::new(&".###..##.#.. 3,2,1");
        assert_eq!(true, map.can_possibly_match(&map.map));
        let map = Map::new(&".###..##..#. 3,2,1");
        assert_eq!(true, map.can_possibly_match(&map.map));
        let map = Map::new(&".###..##...# 3,2,1");
        assert_eq!(true, map.can_possibly_match(&map.map));
        let map = Map::new(&".###...##.#. 3,2,1");
        assert_eq!(true, map.can_possibly_match(&map.map));
        let map = Map::new(&".###...##..# 3,2,1");
        assert_eq!(true, map.can_possibly_match(&map.map));
        let map = Map::new(&".###....##.# 3,2,1");
        assert_eq!(true, map.can_possibly_match(&map.map));
    }
    #[test]
    fn test_minimum_width() {
        let m = Map::new(".###.....??? 3,2,1");
        assert_eq!(Some(8), m.minimum_width(0));
        assert_eq!(Some(4), m.minimum_width(1));
        assert_eq!(Some(1), m.minimum_width(2));
        assert_eq!(None, m.minimum_width(3));
    }

    #[test]
    fn test_not_match_remaining_groups_dont_fit() {
        let m = Map::new(".###.....??? 3,2,1");
        assert_eq!(false, m.can_possibly_match(&m.map));
    }

    #[test]
    fn test_can_line_not_match_exact() {
        let testdata = r".###.##.#.#. 3,2,1
.###.##..##. 3,2,1
.######...#. 3,2,1
.###.##....# 4,2,1
####..##.#.. 3,2,1
.###..##..#. 3,3,1
.###..##...# 3,2,2
.###..###.#. 3,2,1
.###.#.##..# 3,2,1
.###.##.##.# 3,2,1";
        for test in testdata.lines() {
            let map = Map::new(test);
            println!("{}", test);
            assert_eq!(false, map.can_possibly_match(&map.map));
        }
    }
    #[test]
    fn test_can_possibly_match() {
        let line = Map::new("?###???????? 3,2,1");
        assert!(&line.can_possibly_match(&MapSegment::to_vec("?###????????")));
        assert!(!&line.can_possibly_match(&MapSegment::to_vec("####????????")));
        assert!(&line.can_possibly_match(&MapSegment::to_vec(".###????????")));
        assert!(&line.can_possibly_match(&MapSegment::to_vec(".###.???????")));
        assert!(!&line.can_possibly_match(&MapSegment::to_vec(".####???????")));
        let map = Map::new("?#?#?#?#?#?#?#? 1,3,1,6");
        assert_eq!(
            true,
            map.can_possibly_match(&MapSegment::to_vec("?#?#?#?#?#?#?#?"))
        );
        assert_eq!(
            false,
            map.can_possibly_match(&MapSegment::to_vec("##?#?#?#?#?#?#?"))
        );
        assert_eq!(
            true,
            map.can_possibly_match(&MapSegment::to_vec(".#?#?#?#?#?#?#?"))
        );
    }
    #[test]
    fn test_can_parse_line() {
        let line = Map::new("?#.#?#?#?#?#?#? 1,3,1,6");
        assert_eq!(15, line.map.len());
        assert_eq!(MapSegment::Unknown, *line.map.get(0).unwrap());
        assert_eq!(MapSegment::Damaged, *line.map.get(1).unwrap());
        assert_eq!(MapSegment::Operational, *line.map.get(2).unwrap());
    }
    #[test]
    fn test_can_match_entry() {
        let r = Record::new("3,2,1");
        let testdata = r".###.##.#...
.###.##..#..
.###.##...#.
.###.##....#
.###..##.#..
.###..##..#.
.###..##...#
.###...##.#.
.###...##..#
.###....##.#";
        for test in testdata.lines() {
            assert_eq!(true, r.str_match(test));
        }
        assert_eq!(false, r.str_match(".######..#.."));
    }
}
