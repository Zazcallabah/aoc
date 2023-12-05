use std::cmp;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Range {
    from: u64,
    length: u64,
}
impl Range {
    fn new(from: u64, length: u64) -> Range {
        assert_ne!(0, length);
        Range { from, length }
    }
}

struct Almanac {
    seeds: Vec<u64>,
    maps: Vec<Map>,
}
impl Almanac {
    fn new(data: &str) -> Almanac {
        let firstline = data.lines().next().unwrap();
        let seeds: Vec<u64> = firstline["seeds: ".len()..]
            .split(' ')
            .map(|n| n.parse().unwrap())
            .collect();
        let split = data.split("\n\n");

        let maps = split.skip(1).map(Map::new).collect();
        Almanac { seeds, maps }
    }
    fn follow(&self, source: u64) -> u64 {
        let mut interim = source;
        for map in &self.maps {
            interim = map.follow(interim);
        }
        interim
    }

    fn transform(&self, source: Range) -> Vec<Range> {
        let mut interim: Vec<Range> = vec![source];
        for map in &self.maps {
            interim = map.transform_ranges(&interim);
        }
        interim
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct MapLine {
    destination_range_start: u64,
    source_range_start: u64,
    range_length: u32,
}
impl MapLine {
    fn new(destination_range_start: u64, source_range_start: u64, range_length: u32) -> MapLine {
        MapLine {
            destination_range_start,
            source_range_start,
            range_length,
        }
    }
    fn in_full_range(&self, source: &Range) -> bool {
        // ...r----->...
        // self |-|
        return self.source_range_start >= source.from
            && (self.source_range_start + self.range_length as u64)
                <= (source.from + source.length);
    }
    fn in_partial_range(&self, source: &Range) -> bool {
        // ...r--------->...
        // |..|
        let selfend_excl = self.source_range_start + self.range_length as u64;
        let sourceend_excl = source.from + source.length;
        let source_ends_before_range = sourceend_excl <= self.source_range_start;
        let source_starts_after_range = source.from >= selfend_excl;
        return !(source_ends_before_range || source_starts_after_range);
    }
    fn tranform_if_ok(&self, source: &Range) -> Option<Range> {
        match self.in_partial_range(source) {
            true => Some(self.transform(source)),
            false => None,
        }
    }
    fn transform(&self, source: &Range) -> Range {
        let start = cmp::max(source.from, self.source_range_start);
        let transform = start - self.source_range_start;

        let selfend = self.source_range_start + self.range_length as u64;
        let sourceend = source.from + source.length;
        let min = cmp::min(selfend, sourceend);
        let result = Range::new(self.destination_range_start + transform, min - start);
        return result;
    }
    fn in_range(&self, source: u64) -> bool {
        source >= self.source_range_start
            && source < self.source_range_start + (self.range_length as u64)
    }
    fn follow(&self, source: u64) -> u64 {
        let diff = source - &self.source_range_start;
        return &self.destination_range_start + diff;
    }
}

struct Map {
    entries: Vec<MapLine>,
}
impl Map {
    fn transform_ranges(&self, source: &[Range]) -> Vec<Range> {
        let mut result: Vec<Range> = Vec::new();

        for s in source {
            let r = self.transform_range(s);
            let rl = result.len();
            result.splice((rl)..(rl), r.into_iter());
        }
        return result;
    }
    fn transform_range(&self, source: &Range) -> Vec<Range> {
        self.entries
            .iter()
            .filter_map(|l| l.tranform_if_ok(source))
            .collect()
    }
    fn follow(&self, source: u64) -> u64 {
        for line in &self.entries {
            if line.in_range(source) {
                return line.follow(source);
            }
        }
        source
    }

    fn new(data: &str) -> Map {
        //        println!("{}", data.lines().next().unwrap());
        let mut entries: Vec<MapLine> = data
            .lines()
            .skip(1)
            .map(|l| {
                let split: Vec<&str> = l.split(" ").collect();
                MapLine::new(
                    split[0].parse().unwrap(),
                    split[1].parse().unwrap(),
                    split[2].parse().unwrap(),
                )
            })
            .collect();
        let mut extra = Vec::new();
        entries.sort_unstable_by_key(|f| f.source_range_start);
        let mut current_start = 0u64;
        for entry in &entries {
            if entry.source_range_start > current_start {
                let tmp = MapLine::new(
                    current_start,
                    current_start,
                    (entry.source_range_start) as u32,
                );
                //            println!("{:?}", tmp);
                extra.push(tmp);
            }
            current_start = entry.source_range_start + entry.range_length as u64;
            //      println!("{:?}", entry);
        }
        let last = &entries.last().unwrap();
        let from = last.source_range_start + last.range_length as u64;
        extra.push(MapLine::new(from, from, u32::MAX));
        //    println!("{:?}", extra.last().unwrap());

        let rl = entries.len();
        entries.splice((rl)..(rl), extra.into_iter());
        Map { entries }
    }
}

fn find(almanac: Almanac) -> u64 {
    let mut min = u64::MAX;
    for ix in (0usize..(almanac.seeds.len() - 1)).step_by(2) {
        let now = std::time::SystemTime::now();
        let start: u64 = *almanac.seeds.get(ix).unwrap();
        let length: u64 = *almanac.seeds.get(ix + 1).unwrap();
        println!("looking for {} - {}", start, length);

        let result = almanac.transform(Range::new(start, length));
        println!("{:?}", result);
        for r in &result {
            if r.from < min {
                println!("found {}", r.from);
                min = r.from
            }
        }

        let took = now.elapsed().unwrap().as_millis();
        println!("size {}, took {}ms", result.len(), took);
    }
    println!("min: {}", min);
    return min;
}

fn main() {
    let data = std::fs::read_to_string("2023/5.txt").unwrap();
    let almanac = Almanac::new(&data);
    let _ = find(almanac);
}

#[cfg(test)]
mod tests {
    use super::*;
    static TEST_DATA: &str = r"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
    #[test]
    fn test_finder() {
        let a = Almanac::new(TEST_DATA);
        let r = find(a);
        assert_eq!(46, r);
        // let mut interim: Vec<Range> = vec![source];
        // for map in &self.maps {
        //     interim = map.transform_ranges(&interim);
        // }
        // interim
    }
    #[test]
    fn test_ex3() {
        let a = Almanac::new(TEST_DATA);

        let t = a.transform(Range::new(79, 14));
        assert_eq!(3, t.len());
        assert_eq!(3, t.get(0).unwrap().length);
        assert_eq!(82, t.get(0).unwrap().from);
        let a = Almanac::new(TEST_DATA);

        let t = a.transform(Range::new(55, 13));
        assert_eq!(4, t.len());
    }
    #[test]
    fn test_ex2() {
        let a = Almanac::new(TEST_DATA);

        let t0: Vec<Range> = vec![Range::new(82, 1)];
        let t1 = a.maps[0].transform_ranges(&t0);
        assert_eq!(1, t1.len());
        assert_eq!(84, t1[0].from);
        assert_eq!(1, t1[0].length);

        let t1 = a.maps[1].transform_ranges(&t1);
        assert_eq!(1, t1.len());
        assert_eq!(84, t1[0].from);
        assert_eq!(1, t1[0].length);

        let t1 = a.maps[2].transform_ranges(&t1);
        assert_eq!(1, t1.len());
        assert_eq!(84, t1[0].from);
        assert_eq!(1, t1[0].length);

        let t1 = a.maps[3].transform_ranges(&t1);
        assert_eq!(1, t1.len());
        assert_eq!(77, t1[0].from);
        assert_eq!(1, t1[0].length);
        for e in &a.maps[4].entries {
            println!("{:?}", e);
        }
        let t1 = a.maps[4].transform_ranges(&t1);
        println!("{:?}", t1);
        assert_eq!(1, t1.len());
        assert_eq!(45, t1[0].from);
        assert_eq!(1, t1[0].length);
        let t1 = a.maps[5].transform_ranges(&t1);
        assert_eq!(1, t1.len());
        assert_eq!(46, t1[0].from);
        assert_eq!(1, t1[0].length);
    }
    #[test]
    fn test_ex1() {
        let a = Almanac::new(TEST_DATA);
        assert_eq!(4, a.seeds.len());
        assert_eq!(7, a.maps.len());

        assert_eq!(79, a.seeds[0]);
        assert_eq!(81, a.maps[0].follow(79));
        assert_eq!(14, a.seeds[1]);
        assert_eq!(14, a.maps[0].follow(14));
        assert_eq!(55, a.seeds[2]);
        assert_eq!(57, a.maps[0].follow(55));
        assert_eq!(13, a.seeds[3]);
        assert_eq!(13, a.maps[0].follow(13));
        assert_eq!(82, a.follow(79))
    }
    #[test]
    fn test_almanac_new() {
        let a = Almanac::new("seeds: 1 2 3 4\n\ntest map:\n50 98 2\n52 50 48");
        assert_eq!(4, a.seeds.len());
        assert_eq!(3, a.seeds[2]);
        assert_eq!(1, a.maps.len());
        assert_eq!(50, a.maps[0].follow(98));
    }

    #[test]
    fn test_mapline_in_partial_range_length_1() {
        let mapline = MapLine::new(50, 98, 3);
        assert_eq!(false, mapline.in_partial_range(&Range::new(97, 1)));
        assert_eq!(true, mapline.in_partial_range(&Range::new(98, 1)));
        assert_eq!(true, mapline.in_partial_range(&Range::new(99, 1)));
        assert_eq!(true, mapline.in_partial_range(&Range::new(100, 1)));
        assert_eq!(false, mapline.in_partial_range(&Range::new(101, 1)));
    }

    #[test]
    fn test_mapline_in_partial_range_length_2() {
        let mapline = MapLine::new(50, 98, 3);
        assert_eq!(false, mapline.in_partial_range(&Range::new(96, 2)));
        assert_eq!(true, mapline.in_partial_range(&Range::new(97, 2)));
        assert_eq!(true, mapline.in_partial_range(&Range::new(98, 2)));
        assert_eq!(true, mapline.in_partial_range(&Range::new(99, 2)));
        assert_eq!(true, mapline.in_partial_range(&Range::new(100, 2)));
        assert_eq!(false, mapline.in_partial_range(&Range::new(101, 2)));
    }

    #[test]
    fn test_mapline_in_partial_range_length_3() {
        let mapline = MapLine::new(50, 98, 1);
        assert_eq!(false, mapline.in_partial_range(&Range::new(96, 2)));
        assert_eq!(true, mapline.in_partial_range(&Range::new(97, 2)));
        assert_eq!(true, mapline.in_partial_range(&Range::new(98, 2)));
        assert_eq!(false, mapline.in_partial_range(&Range::new(99, 2)));
        assert_eq!(false, mapline.in_partial_range(&Range::new(100, 2)));
        assert_eq!(false, mapline.in_partial_range(&Range::new(101, 2)));
    }

    #[test]
    fn test_mapline_in_partial_range() {
        let mapline = MapLine::new(50, 98, 3);
        assert_eq!(true, mapline.in_partial_range(&Range::new(98, 3)));
        assert_eq!(true, mapline.in_partial_range(&Range::new(99, 1)));
        assert_eq!(true, mapline.in_partial_range(&Range::new(90, 30)));
        assert_eq!(false, mapline.in_partial_range(&Range::new(91, 7)));
        assert_eq!(true, mapline.in_partial_range(&Range::new(91, 8)));
        assert_eq!(true, mapline.in_partial_range(&Range::new(100, 8)));
    }
    #[test]
    fn test_mapline_in_partial_range_2() {
        let mapline = MapLine::new(68, 64, 13);
        assert_eq!(false, mapline.in_partial_range(&Range::new(77, 1)));
    }

    #[test]
    fn test_mapline_transform() {
        let mapline = MapLine {
            destination_range_start: 50,
            source_range_start: 98,
            range_length: 3,
        };
        assert_eq!(Range::new(50, 3), mapline.transform(&Range::new(98, 3)));
        assert_eq!(Range::new(51, 1), mapline.transform(&Range::new(99, 1)));
        assert_eq!(Range::new(50, 3), mapline.transform(&Range::new(90, 30)));
        assert_eq!(Range::new(50, 1), mapline.transform(&Range::new(91, 8)));
        assert_eq!(Range::new(52, 1), mapline.transform(&Range::new(100, 8)));
    }

    #[test]
    fn test_mapline_in_full_range() {
        let mapline = MapLine {
            destination_range_start: 50,
            source_range_start: 98,
            range_length: 2,
        };
        assert_eq!(true, mapline.in_full_range(&Range::new(98, 2)));
        assert_eq!(true, mapline.in_full_range(&Range::new(97, 3)));
        assert_eq!(true, mapline.in_full_range(&Range::new(90, 30)));
        assert_eq!(false, mapline.in_full_range(&Range::new(98, 1)));
        assert_eq!(false, mapline.in_full_range(&Range::new(97, 2)));
        assert_eq!(false, mapline.in_full_range(&Range::new(99, 5)));
    }
    #[test]
    fn test_mapline_follow() {
        let mapline = MapLine {
            destination_range_start: 50,
            source_range_start: 98,
            range_length: 2,
        };
        assert_eq!(50, mapline.follow(98));
        assert_eq!(51, mapline.follow(99));
    }

    #[test]
    fn test_mapline_in_range() {
        let mapline = MapLine {
            destination_range_start: 50,
            source_range_start: 98,
            range_length: 2,
        };
        assert_eq!(false, mapline.in_range(97));
        assert_eq!(true, mapline.in_range(98));
        assert_eq!(true, mapline.in_range(99));
        assert_eq!(false, mapline.in_range(100));
        assert_eq!(false, mapline.in_range(101));
    }

    #[test]
    fn test_map_follow() {
        let map = Map::new("test map:\n50 98 2\n52 50 48");
        assert_eq!(0, map.follow(0));
        assert_eq!(1, map.follow(1));
        assert_eq!(48, map.follow(48));
        assert_eq!(49, map.follow(49));
        assert_eq!(52, map.follow(50));
        assert_eq!(53, map.follow(51));
        assert_eq!(98, map.follow(96));
        assert_eq!(99, map.follow(97));
        assert_eq!(50, map.follow(98));
        assert_eq!(51, map.follow(99));
    }

    #[test]
    fn test_map_transform() {
        let map = Map::new("test map:\n50 98 2\n52 50 48");
        let result = map.transform_range(&Range {
            from: 79,
            length: 14,
        });
        assert_eq!(1, result.len());
        assert_eq!(81, result.get(0).unwrap().from);
        assert_eq!(14, result.get(0).unwrap().length);
    }
    #[test]
    fn test_map_coverage() {
        let map = Map::new("test map:\n50 98 2\n52 50 48");
        assert_eq!(4, map.entries.len());
    }
}
