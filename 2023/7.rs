use std::{cmp, collections::HashMap};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Range {
    from: u64,
    length: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Type {
    FiveKind,
    FourKind,
    FullHouse,
    ThreeKind,
    TwoPair,
    OnePair,
    HighCard,
}

struct Hand {
    bid: u32,
    cards: String,
    counts: HashMap<char, u32>,
    points: u64,
}
struct Game {
    hands: Vec<Hand>,
}
impl Game {
    fn new(data: &str) -> Game {
        let hands = data.lines().map(|l| Hand::new(l)).collect();
        Game { hands }
    }
    fn get_winnings(&mut self) -> u64 {
        self.hands.sort_unstable_by_key(|f| f.points);
        println!("winnings");
        let mut total = 0u64;
        for (i, hand) in self.hands.iter().enumerate() {
            println!("{}: {} : {:?}", i + 1, hand.cards, hand.bid);
            total += (i as u64 + 1) * hand.bid as u64;
        }
        total
    }
}
fn get_card_value(c: &char) -> u64 {
    match c {
        '2' => 0,
        '3' => 1,
        '4' => 2,
        '5' => 3,
        '6' => 4,
        '7' => 5,
        '8' => 6,
        '9' => 7,
        'T' => 8,
        'J' => 9,
        'Q' => 10,
        'K' => 11,
        'A' => 12,
        _ => panic!("invalid card"),
    }
}
// base 13 interpretation of hand
fn get_hand_value(hand: &Hand) -> u64 {
    let mut sum = 0u64;
    let base = 13;
    for (i, x) in hand.cards.chars().rev().enumerate() {
        let factor = u32::pow(base, i as u32);
        sum += get_card_value(&x) * factor as u64;
    }
    sum
}
fn get_points(hand: &Hand) -> u64 {
    let cardvalue = get_hand_value(&hand);
    let steplimit = 371292;

    if hand.counts.values().any(|v| v == &5u32) {
        return 10 * steplimit + cardvalue;
    }
    if hand.counts.values().any(|v| v == &4u32) {
        return 9 * steplimit + cardvalue;
    }
    let threes = hand.counts.values().filter(|&v| v == &3u32).count();
    let twos = hand.counts.values().filter(|&v| v == &2u32).count();
    if threes == 1 && twos == 1 {
        return 8 * steplimit + cardvalue;
    }
    if threes == 1 {
        return 7 * steplimit + cardvalue;
    }
    if twos == 2 {
        return 6 * steplimit + cardvalue;
    }
    if twos == 1 {
        return 5 * steplimit + cardvalue;
    }
    return 2 * steplimit + cardvalue;
}
impl Hand {
    fn set_point(&mut self) {
        self.points = get_points(&self);
    }
    fn new(data: &str) -> Hand {
        let spl: Vec<&str> = data.split(' ').collect();
        let bid: u32 = spl.get(1).unwrap().parse().unwrap();
        let cards: String = spl.get(0).unwrap().to_string();
        let mut counts: HashMap<char, u32> = HashMap::new();
        for card in cards.chars() {
            let v = counts.entry(card).or_insert(0);
            *v += 1;
        }
        let mut h = Hand {
            bid,
            cards,
            counts,
            points: 0,
        };
        h.set_point();
        h
    }
}
fn main() {
    let data = std::fs::read_to_string("2023/7.txt").unwrap();
    let mut g = Game::new(&data);
    let w = &g.get_winnings();
    println!("{}", w);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_parse_hand() {
        let c = Hand::new("AAAAA 33");
        assert_eq!(33, c.bid);
        assert_eq!(&5, c.counts.get(&'A').unwrap());
        assert_eq!(371292, get_hand_value(&c))
    }

    #[test]
    fn test_can_game() {
        let str = "32T3K 765\nT55J5 684\nKK677 28\nKTJJT 220\nQQQJA 483";
        let mut g = Game::new(str);
        let w = &g.get_winnings();
        assert_eq!(&6440, w);
        // assert_eq!(&5, c.counts.get(&'A').unwrap());
    }
}
