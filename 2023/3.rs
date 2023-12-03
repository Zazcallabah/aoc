#[macro_use]
extern crate lazy_static;
use regex::Regex;

#[derive(Debug, Clone, Copy)]
struct PartNumber {
    line: usize,
    start: usize,
    end: usize,
    value: u32,
}

struct Symbol {
    line: usize,
    index: usize,
    value: char,
}
struct Gear {
    ratio: u32,
}
fn main() {
    let data = std::fs::read_to_string("2023/3.txt").unwrap();
    let inp = parse_input(&data);

    println!("sum: {}", sum(&inp));
    let gears = get_gears(&inp.1, &inp.0);
    let mut sum = 0;
    for g in gears {
        sum += g.ratio;
    }
    println!("gears: {:?}", sum);
}
fn sum(inp: &(Vec<PartNumber>, Vec<Symbol>)) -> u32 {
    let mut sum = 0;
    for part in inp.0.iter().filter(|p| has_adjacent_symbol(p, &inp.1)) {
        sum += part.value;
    }

    return sum;
}
fn is_symbol_location(line: usize, ix: usize, s: &[Symbol]) -> bool {
    for symbol in s {
        if symbol.line == line && symbol.index == ix {
            return true;
        }
    }
    false
}
fn has_adjacent_symbol(p: &PartNumber, s: &[Symbol]) -> bool {
    if p.start > 0 && is_symbol_location(p.line, p.start - 1, s) {
        return true;
    }
    if is_symbol_location(p.line, p.end, s) {
        return true;
    }
    let start = if p.start > 0 { p.start - 1 } else { p.start };
    for col in start..=(p.end) {
        if p.line > 0 && is_symbol_location(p.line - 1, col, s) {
            return true;
        }
        if is_symbol_location(p.line + 1, col, s) {
            return true;
        }
    }
    false
}

fn parse_input(data: &str) -> (Vec<PartNumber>, Vec<Symbol>) {
    let mut symbols: Vec<Symbol> = Vec::new();
    let mut parts: Vec<PartNumber> = Vec::new();
    let mut line_number = 0;
    for l in data.lines() {
        let r = parse_line(l, line_number);
        line_number += 1;

        let pl = parts.len();
        let sl = symbols.len();
        parts.splice((pl)..(pl), r.0.into_iter());
        symbols.splice((sl)..(sl), r.1.into_iter());
    }
    (parts, symbols)
}

fn parse_line(line: &str, line_number: usize) -> (Vec<PartNumber>, Vec<Symbol>) {
    lazy_static! {
        static ref SYMBOL_FINDER: Regex = Regex::new("[^0-9.]").unwrap();
        static ref PART_FINDER: Regex = Regex::new("\\d+").unwrap();
    }
    let mut symbols: Vec<Symbol> = Vec::new();
    let res = SYMBOL_FINDER.find_iter(line);
    for m in res {
        let index = m.start();
        let value = line.as_bytes()[m.start()] as char;
        symbols.push(Symbol {
            index,
            value,
            line: line_number,
        });
    }
    let mut parts: Vec<PartNumber> = Vec::new();
    let res = PART_FINDER.find_iter(line);
    for m in res {
        let value = m.as_str().parse().unwrap();
        parts.push(PartNumber {
            line: line_number,
            start: m.start(),
            end: m.end(),
            value,
        });
    }
    return (parts, symbols);
}
fn is_adjacent_part(line: usize, ix: usize, part: &PartNumber) -> bool {
    // same line
    if line == part.line {
        return part.end == ix || (part.start > 0 && part.start - 1 == ix);
    }
    // above/below
    if line == part.line + 1 || line + 1 == part.line {
        let left_limit = if part.start > 0 {
            part.start - 1
        } else {
            part.start
        };
        let right_limit = part.end;
        return ix >= left_limit && ix <= right_limit;
    }
    return false;
}
fn get_adjacent_parts(line: usize, ix: usize, parts: &[PartNumber]) -> Vec<&PartNumber> {
    parts
        .iter()
        .filter(|part| is_adjacent_part(line, ix, part))
        .collect()
}
fn get_gears(symbols: &[Symbol], parts: &[PartNumber]) -> Vec<Gear> {
    let mut gears = Vec::new();
    for symbol in symbols {
        if symbol.value == '*' {
            let adj = get_adjacent_parts(symbol.line, symbol.index, parts);
            if adj.len() == 2 {
                gears.push(Gear {
                    ratio: adj.get(0).unwrap().value * adj.get(1).unwrap().value,
                });
            }
        }
    }
    return gears;
}
#[cfg(test)]
mod tests {
    use super::*;

    static TEST_DATA: &str = r"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    // In this schematic, there are two gears.
    // The first is in the top left;
    // it has part numbers 467 and 35, so its gear ratio is .
    // The second gear is in the lower right; its gear ratio is 451490.
    // (The * adjacent to 617 is not a gear because it is only adjacent to one part number.)
    // Adding up all of the gear ratios produces 467835.

    // What is the sum of all of the gear ratios in your engine schematic?
    fn par(line: usize, index: usize) -> PartNumber {
        PartNumber {
            line,
            start: index,
            end: index + 2,
            value: 33,
        }
    }
    #[test]
    fn test_gears() {
        let result = parse_input(TEST_DATA);
        let gears = get_gears(&result.1, &result.0);

        assert_eq!(16345, gears.get(0).unwrap().ratio);
        assert_eq!(451490, gears.get(1).unwrap().ratio);
    }
    #[test]
    fn test_is_adjacent_part_line_distanc() {
        // part is several lines away
        assert_eq!(false, is_adjacent_part(3, 5, &par(5, 5)));
        assert_eq!(true, is_adjacent_part(4, 5, &par(5, 5)));
        assert_eq!(false, is_adjacent_part(5, 5, &par(5, 5)));
        assert_eq!(true, is_adjacent_part(6, 5, &par(5, 5)));
        assert_eq!(false, is_adjacent_part(7, 5, &par(5, 5)));
    }
    #[test]
    fn test_is_adjacent_part_line_above() {
        // part is on line above symbol
        assert_eq!(false, is_adjacent_part(6, 5, &par(5, 2)));
        assert_eq!(true, is_adjacent_part(6, 5, &par(5, 3)));
        assert_eq!(true, is_adjacent_part(6, 5, &par(5, 4)));
        assert_eq!(true, is_adjacent_part(6, 5, &par(5, 5)));
        assert_eq!(true, is_adjacent_part(6, 5, &par(5, 6)));
        assert_eq!(false, is_adjacent_part(6, 5, &par(5, 7)));
    }
    #[test]
    fn test_is_adjacent_part_line_below() {
        // part is on line below symbol
        assert_eq!(false, is_adjacent_part(4, 5, &par(5, 2)));
        assert_eq!(true, is_adjacent_part(4, 5, &par(5, 3)));
        assert_eq!(true, is_adjacent_part(4, 5, &par(5, 4)));
        assert_eq!(true, is_adjacent_part(4, 5, &par(5, 5)));
        assert_eq!(true, is_adjacent_part(4, 5, &par(5, 6)));
        assert_eq!(false, is_adjacent_part(4, 5, &par(5, 7)));
    }
    #[test]
    fn test_is_adjacent_part_same_line() {
        // same line before
        assert_eq!(false, is_adjacent_part(5, 5, &par(5, 2)));
        assert_eq!(true, is_adjacent_part(5, 5, &par(5, 3)));
        // intersects
        assert_eq!(false, is_adjacent_part(5, 5, &par(5, 4)));
        assert_eq!(false, is_adjacent_part(5, 5, &par(5, 5)));
        // same line after
        assert_eq!(true, is_adjacent_part(5, 5, &par(5, 6)));
        assert_eq!(false, is_adjacent_part(5, 5, &par(5, 7)));
    }

    #[test]
    fn test_sum() {
        let result = parse_input(TEST_DATA);

        assert_eq!(4361, sum(&result));
    }

    fn sym(line: usize, index: usize) -> Symbol {
        Symbol {
            line,
            index,
            value: 'x',
        }
    }

    #[test]
    fn test_adjacent_lookup_directions() {
        // part is
        // 0: ......
        // 1: ..33..
        // 2: ......
        let part = PartNumber {
            start: 2,
            end: 4,
            line: 1,
            value: 33,
        };

        // partnumber itself is not "adjacent"
        assert_eq!(false, has_adjacent_symbol(&part, &[sym(1, 2)]));
        assert_eq!(false, has_adjacent_symbol(&part, &[sym(1, 3)]));
        // left and right edge is adjacent
        assert_eq!(true, has_adjacent_symbol(&part, &[sym(1, 1)]));
        assert_eq!(true, has_adjacent_symbol(&part, &[sym(1, 4)]));
        // lid (incl diagonal) is adjacent
        assert_eq!(true, has_adjacent_symbol(&part, &[sym(0, 1)]));
        assert_eq!(true, has_adjacent_symbol(&part, &[sym(0, 2)]));
        assert_eq!(true, has_adjacent_symbol(&part, &[sym(0, 3)]));
        assert_eq!(true, has_adjacent_symbol(&part, &[sym(0, 4)]));
    }

    #[test]
    fn test_adjacent_lookup() {
        let result = parse_input(TEST_DATA);

        assert_eq!(
            true,
            has_adjacent_symbol(&result.0.get(0).unwrap(), &result.1)
        );
        assert_eq!(
            false,
            has_adjacent_symbol(&result.0.get(1).unwrap(), &result.1)
        );
    }
    #[test]
    fn test_is_symbol() {
        let result = parse_input(TEST_DATA);

        assert_eq!(false, is_symbol_location(0, 3, &result.1));
        assert_eq!(true, is_symbol_location(1, 3, &result.1));
    }

    #[test]
    fn test_parse_input() {
        let result = parse_input(TEST_DATA);

        assert_eq!(6, result.1.len());
        assert_eq!('*', result.1.get(0).unwrap().value);
        assert_eq!(10, result.0.len());
        assert_eq!(35, result.0.get(2).unwrap().value);
    }

    #[test]
    fn test_parseline() {
        let result = parse_line(".....+.58.", 0);
        let sym = result.1.get(0).unwrap();
        let par = result.0.get(0).unwrap();

        assert_eq!(0, par.line);
        assert_eq!(58, par.value);
        assert_eq!(7, par.start);
        assert_eq!(9, par.end);

        assert_eq!(0, sym.line);
        assert_eq!(5, sym.index);
        assert_eq!('+', sym.value);
    }
}
