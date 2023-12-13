use std::collections::HashMap;

use regex::Regex;
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum MapSegment {
    Operational,
    Damaged,
    Unknown,
}
struct PossibleMatch {
    matched: bool,
    group_count: usize,
}
impl PossibleMatch {
    fn no() -> PossibleMatch {
        PossibleMatch {
            matched: false,
            group_count: 0,
        }
    }
    fn yes(group_count: usize) -> PossibleMatch {
        PossibleMatch {
            matched: true,
            group_count,
        }
    }
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
    id: usize,
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
    fn cached_or_inner(
        &self,
        depth_cache: &mut HashMap<(usize, usize, usize), u64>,
        bench: &mut [MapSegment],
        next_index: usize,
        depth: usize,
        group_count: usize,
    ) -> u64 {
        // let slow_ids = [18, 22, 26, 29, 32, 35, 48, 49, 51, 52];
        // if !slow_ids.contains(&self.id) {
        //     return self.cached_or_inner_only_verify(
        //         depth_cache,
        //         bench,
        //         next_index,
        //         depth,
        //         group_count,
        //     );
        // }
        if let Some(x) = depth_cache.get(&(depth, group_count, next_index)) {
            return *x;
            // println!(
            //     "{} = {} ({},{},{},{})",
            //     v,
            //     x,
            //     depth,
            //     group_count,
            //     next_index,
            //     SegmentVector(bench.to_owned())
            // );
            // assert_eq!(v, *x);
            // return v;
        }
        let v = self.count_matches_inner(depth_cache, bench, next_index, depth + 1);
        depth_cache.insert((depth, group_count, next_index), v);
        return v;
    }
    fn cached_or_inner_only_verify(
        &self,
        depth_cache: &mut HashMap<(usize, usize, usize), u64>,
        bench: &mut [MapSegment],
        next_index: usize,
        depth: usize,
        group_count: usize,
    ) -> u64 {
        let v = self.count_matches_inner(depth_cache, bench, next_index, depth + 1);

        if let Some(x) = depth_cache.get(&(depth, group_count, next_index)) {
            // println!(
            //     "{} = {} ({},{},{},{})",
            //     v,
            //     x,
            //     depth,
            //     group_count,
            //     next_index,
            //     SegmentVector(bench.to_owned())
            // );
            assert_eq!(v, *x);

            return v;
        }
        depth_cache.insert((depth, group_count, next_index), v);
        return v;
    }

    fn count_matches_inner(
        &self,
        depth_cache: &mut HashMap<(usize, usize, usize), u64>,
        bench: &mut [MapSegment],
        current: usize,
        depth: usize,
    ) -> u64 {
        // current should be pointing to an unknown
        let next_unknown_index = Map::find_next_unknown(&bench, current + 1);

        let mut recursed_sum = 0u64;
        bench[current] = MapSegment::Damaged;
        let a_branch_can_match = self.can_possibly_match(bench);
        if a_branch_can_match.matched {
            if let Some(ix) = next_unknown_index {
                recursed_sum += self.cached_or_inner(
                    depth_cache,
                    bench,
                    ix,
                    depth + 1,
                    a_branch_can_match.group_count,
                );
            } else {
                //                let as_str = SegmentVector(bench.to_owned());
                // if !self.record.str_match(&as_str.to_string()) {
                //     self.print_match(bench, "match_a");
                //     return 0;
                // }
                assert!(self.map[current] == MapSegment::Unknown);
                bench[current] = MapSegment::Unknown; // restore
                return 1;
            }
        }

        bench[current] = MapSegment::Operational;
        let b_branch_can_match = self.can_possibly_match(bench);
        if b_branch_can_match.matched {
            if let Some(ix) = next_unknown_index {
                recursed_sum += self.count_matches_inner(depth_cache, bench, ix, depth + 1);
            } else {
                let as_str = SegmentVector(bench.to_owned());
                if !self.record.str_match(&as_str.to_string()) {
                    self.print_match(bench, "match_b");
                    return 0;
                }
                assert!(self.map[current] == MapSegment::Unknown);
                bench[current] = self.map[current]; // restore
                return 1;
            }
        }
        bench[current] = self.map[current]; // restore
        return recursed_sum;
    }
    fn count_matches(&self) -> u64 {
        let mut cache = HashMap::new();
        if let Some(first_index) = Map::find_next_unknown(&self.map, 0) {
            return self.count_matches_inner(&mut cache, &mut self.map.clone(), first_index, 1);
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
    fn print_match(&self, map: &[MapSegment], label: &str) {
        // println!(
        //     "{} {} r{:?}",
        //     label,
        //     SegmentVector(map.to_owned()),
        //     self.record.entries
        // );
    }
    fn print_return(&self, map: &[MapSegment], label: &str, ret: PossibleMatch) -> PossibleMatch {
        // println!(
        //     "{}-{} {} r{:?}",
        //     if ret { 'o' } else { 'x' },
        //     label,
        //     SegmentVector(map.to_owned()),
        //     self.record.entries
        // );
        ret
    }
    fn can_possibly_match(&self, map: &[MapSegment]) -> PossibleMatch {
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
                                    return PossibleMatch::no();
                                }
                            }
                            None => return PossibleMatch::no(),
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
                                return self.print_return(map, "CU", PossibleMatch::no());
                            }
                        } else {
                            return self.print_return(map, "CX", PossibleMatch::no());
                        }
                        if let Some(record) = self.record.entries.get(groupnumber) {
                            if *record < groupsize {
                                return PossibleMatch::no();
                            }
                            return PossibleMatch::yes(groupnumber);
                        }
                        return PossibleMatch::no();
                    }
                }
            } else {
                if *s == MapSegment::Unknown {
                    if let Some(cc) = self.minimum_width(groupnumber as usize) {
                        let remaining_unchecked = map.len() - marker as usize;
                        if cc > remaining_unchecked as u8 {
                            return self.print_return(map, "EU", PossibleMatch::no());
                        } else {
                            return PossibleMatch::yes(groupnumber); //self.print_return(map, "IU", true);
                        }
                    }
                    return PossibleMatch::yes(groupnumber);
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
                return self.print_return(map, "NX", PossibleMatch::no());
            }
            match self.record.entries.get(groupnumber as usize) {
                Some(record) => {
                    if *record != groupsize as u8 {
                        return PossibleMatch::no();
                    }
                }
                None => return PossibleMatch::no(),
            }
        } else {
            if (groupnumber as usize) < self.record.entries.len() {
                return self.print_return(map, "NY", PossibleMatch::no());
            }
        }
        return PossibleMatch::yes(groupnumber);
    }
    fn count_unknowns(&self) -> usize {
        self.map
            .iter()
            .filter(|&i| *i == MapSegment::Unknown)
            .count()
    }
    fn new(data: &str) -> Map {
        let spl: Vec<&str> = data.split(' ').collect();
        let record = Record::new(spl[1]);
        let map: Vec<MapSegment> = MapSegment::to_vec(spl[0]);
        Map { record, map, id: 0 }
    }
    fn expand(data: &str, separator: char) -> String {
        let mut out = String::from(data);
        for _ in 0..4 {
            out.push(separator);
            out.push_str(data);
        }
        out
    }
    fn new_expanded(data: &str) -> Map {
        let spl: Vec<&str> = data.split(' ').collect();
        let record = Record::new(&Map::expand(spl[1], ','));
        let map: Vec<MapSegment> = MapSegment::to_vec(&Map::expand(spl[0], '?'));
        Map { record, map, id: 0 }
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
    // fn bool_match(&self, bools: &[bool]) -> bool {
    //     let entry_ix = 0;
    //     let mut finding = true;
    //     let mut counting = 0u8;

    //     for b in bools {
    //         if finding {
    //             if *b {
    //                 finding = false;
    //                 counting = 1;
    //             }
    //         } else {
    //             if *b {
    //                 counting += 1;
    //             } else {
    //                 match &self.entries.get(entry_ix) {
    //                     Some(x) => {
    //                         if &counting != *x {
    //                             return false;
    //                         }
    //                     }
    //                     None => return false,
    //                 }
    //             }
    //         }
    //     }

    //     false
    // }
}
fn main() {
    let data = std::fs::read_to_string("2023/12.txt").unwrap();
    let mut sum = 0;
    for line in data.lines() {
        sum += Map::new(line).count_matches();
    }
    println!("summed: {}", sum);
    let data = std::fs::read_to_string("2023/12.txt").unwrap();
    let mut sum = 0;
    let mut id = 0;
    for line in data.lines() {
        let mut map = Map::new_expanded(line);
        map.id = id;
        println!("{} {} ({})", id, line, map.count_unknowns());
        sum += map.count_matches();
        id += 1;
    }
    println!("expanded: {}", sum);
    // 351 is too low
    // 2596401949 is too low
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expand() {
        assert_eq!("a,a,a,a,a", Map::expand("a", ','))
    }
    #[test]
    fn test_can_count_matches() {
        assert_eq!(1, Map::new("???.### 1,1,3").count_matches());
        assert_eq!(4, Map::new(".??..??...?##. 1,1,3").count_matches());
        assert_eq!(1, Map::new("?#?#?#?#?#?#?#? 1,3,1,6").count_matches());
        assert_eq!(1, Map::new("????.#...#... 4,1,1").count_matches());
        assert_eq!(4, Map::new("????.######..#####. 1,6,5").count_matches());

        assert_eq!(10, Map::new("?###???????? 3,2,1").count_matches());
    }
    #[test]
    fn test_can_count_matches_expanded() {
        assert_eq!(1, Map::new_expanded("???.### 1,1,3").count_matches());
        assert_eq!(
            16384,
            Map::new_expanded(".??..??...?##. 1,1,3").count_matches()
        );
        assert_eq!(
            1,
            Map::new_expanded("?#?#?#?#?#?#?#? 1,3,1,6").count_matches()
        );
        assert_eq!(16, Map::new_expanded("????.#...#... 4,1,1").count_matches());
        assert_eq!(
            2500,
            Map::new_expanded("????.######..#####. 1,6,5").count_matches()
        );
        assert_eq!(
            506250,
            Map::new_expanded("?###???????? 3,2,1").count_matches()
        );
    }
    #[test]
    fn test_recursion_matches() {
        let m = Map::new(".###.....??? 3,2,1");
        let n = Map::find_next_unknown(&m.map, 0);
        assert_eq!(Some(9), n);
        let mut bench = m.map.clone();

        let r = m.count_matches_inner(&mut HashMap::new(), &mut bench, 9, 1);
        assert_eq!(0, r)
    }
    #[test]
    fn test_can_verify_cached() {
        Map::new_expanded("???.### 1,1,3").count_matches();
        Map::new_expanded("?.?#?#??#?.?#????? 4,2,5").count_matches();
        Map::new_expanded("?##??.#??#.???.# 4,4,2,1").count_matches();
        Map::new_expanded("#?#?.##???.?.? 4,2,1,1").count_matches();
        Map::new_expanded("...?#??#?#????#.. 10,1").count_matches();
        Map::new_expanded("?????#?#.??????????? 1,6,1,1,1,4").count_matches();
        Map::new_expanded("???????##???#???#??? 10,1,5").count_matches();
        Map::new_expanded(".?#??????.#.?#? 7,1,3").count_matches();
        Map::new_expanded("..#..??#???#???#?#?# 1,1,11,1").count_matches();
        Map::new_expanded(".??##?.??#?#?#?..? 5,7").count_matches();
        Map::new_expanded("?#???#?#??#???##?.? 2,9,4").count_matches();
        Map::new_expanded(".?#.?#?###????#???? 1,11,1").count_matches();
        Map::new_expanded("?..?#.??##??#??.? 2,9").count_matches();
        Map::new_expanded("??##.?#?????? 4,5,1").count_matches();
        Map::new_expanded("?.???#??#?#?#? 9,1").count_matches();
        Map::new_expanded("#??#.?????####. 4,9").count_matches();
        Map::new_expanded("???#????#?.??#? 9,2").count_matches();
        Map::new_expanded("#????.??#????? 1,1,1,7").count_matches();
        Map::new_expanded("?#??.##?###?##?. 3,6,2").count_matches();
    }
    #[test]
    fn test_can_find_specific() {
        let map = Map::new(&".###.##....? 3,2,1");
        assert_eq!(true, map.can_possibly_match(&map.map).matched);
    }
    //    #[test]
    fn test_can_run_on_expanded() {
        let map = Map::new_expanded(&"??????#..????##??? 1,1,1,1,5");
        let count = map.count_matches();
        assert_eq!(1, count);
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
        assert_eq!(true, map.can_possibly_match(&map.map).matched);
        let map = Map::new(&".###.##..#.. 3,2,1");
        assert_eq!(true, map.can_possibly_match(&map.map).matched);
        let map = Map::new(&".###.##...#. 3,2,1");
        assert_eq!(true, map.can_possibly_match(&map.map).matched);
        let map = Map::new(&".###.##....# 3,2,1");
        assert_eq!(true, map.can_possibly_match(&map.map).matched);
        let map = Map::new(&".###..##.#.. 3,2,1");
        assert_eq!(true, map.can_possibly_match(&map.map).matched);
        let map = Map::new(&".###..##..#. 3,2,1");
        assert_eq!(true, map.can_possibly_match(&map.map).matched);
        let map = Map::new(&".###..##...# 3,2,1");
        assert_eq!(true, map.can_possibly_match(&map.map).matched);
        let map = Map::new(&".###...##.#. 3,2,1");
        assert_eq!(true, map.can_possibly_match(&map.map).matched);
        let map = Map::new(&".###...##..# 3,2,1");
        assert_eq!(true, map.can_possibly_match(&map.map).matched);
        let map = Map::new(&".###....##.# 3,2,1");
        assert_eq!(true, map.can_possibly_match(&map.map).matched);
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
        assert_eq!(false, m.can_possibly_match(&m.map).matched);
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
            assert_eq!(false, map.can_possibly_match(&map.map).matched);
        }
    }
    #[test]
    fn test_can_possibly_match() {
        let line = Map::new("?###???????? 3,2,1");
        assert!(
            &line
                .can_possibly_match(&MapSegment::to_vec("?###????????"))
                .matched
        );
        assert!(
            !&line
                .can_possibly_match(&MapSegment::to_vec("####????????"))
                .matched
        );
        assert!(
            &line
                .can_possibly_match(&MapSegment::to_vec(".###????????"))
                .matched
        );
        assert!(
            &line
                .can_possibly_match(&MapSegment::to_vec(".###.???????"))
                .matched
        );
        assert!(
            !&line
                .can_possibly_match(&MapSegment::to_vec(".####???????"))
                .matched
        );
        let map = Map::new("?#?#?#?#?#?#?#? 1,3,1,6");
        assert_eq!(
            true,
            map.can_possibly_match(&MapSegment::to_vec("?#?#?#?#?#?#?#?"))
                .matched
        );
        assert_eq!(
            false,
            map.can_possibly_match(&MapSegment::to_vec("##?#?#?#?#?#?#?"))
                .matched
        );
        assert_eq!(
            true,
            map.can_possibly_match(&MapSegment::to_vec(".#?#?#?#?#?#?#?"))
                .matched
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
    #[test]
    fn test_all_output1() {
        let data = std::fs::read_to_string("2023/12test.txt").unwrap();
        for line in data.lines() {
            let map = Map::new(line);
            assert_eq!(false, map.can_possibly_match(&map.map).matched);
        }
    }
}
