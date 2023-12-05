struct Almanac {
    seeds: Vec<u64>,
    maps: Vec<Map>,
}
struct MapLine {
    destination_range_start: u64,
    source_range_start: u64,
    range_length: u32,
}
struct Map {
    entries: Vec<MapLine>,
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
    // fn backwards_follow(&self, location: u64) -> u64 {
    //     let mut interim = location;

    //     for ix in [(&self.maps.len() - 1)..=0] {
    //         interim = &self.maps[ix].backwards_follow(interim);
    //     }
    //     interim
    // }
}
impl MapLine {
    fn in_backwards_range(&self, destination: u64) -> bool {
        destination >= self.destination_range_start
            && destination < self.destination_range_start + (self.range_length as u64)
    }
    fn in_range(&self, source: u64) -> bool {
        source >= self.source_range_start
            && source < self.source_range_start + (self.range_length as u64)
    }
    fn follow(&self, source: u64) -> u64 {
        let diff = source - &self.source_range_start;
        return &self.destination_range_start + diff;
    }
    fn backwards_follow(&self, destination: u64) -> u64 {
        let diff = destination - &self.destination_range_start;
        return &self.source_range_start + diff;
    }
}
impl Map {
    fn follow(&self, source: u64) -> u64 {
        for line in &self.entries {
            if line.in_range(source) {
                return line.follow(source);
            }
        }
        source
    }
    fn backwards_follow(&self, destination: u64) -> u64 {
        for line in &self.entries {
            if line.in_backwards_range(destination) {
                return line.backwards_follow(destination);
            }
        }
        destination
    }
    fn new(data: &str) -> Map {
        let entries = data
            .lines()
            .skip(1)
            .map(|l| {
                let split: Vec<&str> = l.split(" ").collect();
                MapLine {
                    destination_range_start: split[0].parse().unwrap(),
                    source_range_start: split[1].parse().unwrap(),
                    range_length: split[2].parse().unwrap(),
                }
            })
            .collect();
        Map { entries }
    }
}

fn main() {
    let data = std::fs::read_to_string("2023/5.txt").unwrap();
    let almanac = Almanac::new(&data);
    let mut min = u64::MAX;

    for ix in 0usize..(almanac.seeds.len() - 1) {
        let now = std::time::SystemTime::now();
        let start: u64 = *almanac.seeds.get(ix).unwrap();
        let length: u64 = *almanac.seeds.get(ix).unwrap();
        let end = start + length;
        println!("size {}", length);

        for ix2 in start..end {
            let tmp: u64 = almanac.follow(ix2);
            if tmp < min {
                min = tmp;
            }
            if ix2 % 1000000 == 0 {
                println!(" {}%", ix2 - start);
            }
        }
        let took = now.elapsed().unwrap().as_millis();
        println!("size {}, took {}ms", length, took);
    }
    println!("min: {}", min);
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
}
