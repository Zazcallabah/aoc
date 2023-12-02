use regex::{Match, Regex};

fn main() {
    let data = std::fs::read_to_string("2023/1.txt").unwrap();

    println!("sum: {}", get_summed_lines(data));
    // 53896 is too high
}

fn map_nr(s: &str) -> u32 {
    match s {
        "0" => 0,
        "1" => 1,
        "2" => 2,
        "3" => 3,
        "4" => 4,
        "5" => 5,
        "6" => 6,
        "7" => 7,
        "8" => 8,
        "9" => 9,
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => panic!("bad input"),
    }
}

fn get_last(line: &String, r: Regex, mut ix: usize) -> Option<Match> {
    let mut lastmatch = None;
    loop {
        match r.find_at(line, ix) {
            None => return lastmatch,
            Some(m) => {
                lastmatch = Some(m);
                ix += 1;
            }
        };
    }
}

fn get_calibration_values(line: &String) -> (u32, u32) {
    let re = Regex::new("one|two|three|four|five|six|seven|eight|nine|\\d").unwrap();
    let first = re.find_at(line, 0).unwrap();
    let last = match get_last(line, re, 1) {
        Some(m) => m,
        None => first.clone(),
    };
    return (map_nr(first.as_str()), map_nr(last.as_str()));
}
fn get_combined(line: &String) -> u32 {
    let res = get_calibration_values(line);
    println!("{1}-{2} :: {0}", line, res.0, res.1);
    return res.0 * 10 + res.1;
}
fn get_summed(lines: Vec<String>) -> u32 {
    return lines.iter().map(get_combined).sum();
}
fn get_summed_lines(data: String) -> u32 {
    return get_summed(data.lines().map(|l| l.to_owned()).collect());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oneeight() {
        assert_eq!((1, 8), get_calibration_values(&"oneight".to_owned()))
    }

    #[test]
    fn test_value_extraction() {
        assert_eq!((1, 2), get_calibration_values(&String::from("1abc2")));
    }
    #[test]
    fn test_value_combined() {
        assert_eq!(12, get_combined(&String::from("1abc2")));
        assert_eq!(38, get_combined(&String::from("pqr3stu8vwx")));
        assert_eq!(15, get_combined(&String::from("a1b2c3d4e5f")));
        assert_eq!(77, get_combined(&String::from("treb7uchet")));
    }
    #[test]
    fn test_value_summed() {
        assert_eq!(
            142,
            get_summed_lines(String::from("1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet"))
        );
    }

    #[test]
    fn test_value_combined_second_part() {
        assert_eq!(29, get_combined(&String::from("two1nine")));
        assert_eq!(83, get_combined(&String::from("eightwothree")));
        assert_eq!(13, get_combined(&String::from("abcone2threexyz")));
        assert_eq!(24, get_combined(&String::from("xtwone3four")));
        assert_eq!(42, get_combined(&String::from("4nineeightseven2")));
        assert_eq!(14, get_combined(&String::from("zoneight234")));
        assert_eq!(76, get_combined(&String::from("7pqrstsixteen")));
    }

    #[test]
    fn test_value_summed_second_part() {
        assert_eq!(281,get_summed_lines(String::from("two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen")));
    }
}
