use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Range {
    from: u64,
    length: u64,
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
    fn new(data: &str, joker: bool) -> Game {
        let hands = data.lines().map(|l| Hand::new(l, joker)).collect();
        Game { hands }
    }
    fn get_winnings(&mut self) -> u64 {
        self.hands.sort_unstable_by_key(|f| f.points);
        let mut total = 0u64;
        for (i, hand) in self.hands.iter().enumerate() {
            total += (i as u64 + 1) * hand.bid as u64;
        }
        total
    }
}
fn get_card_value_joker(c: &char) -> u64 {
    match c {
        'J' => 0,
        '2' => 1,
        '3' => 2,
        '4' => 3,
        '5' => 4,
        '6' => 5,
        '7' => 6,
        '8' => 7,
        '9' => 8,
        'T' => 9,
        'Q' => 10,
        'K' => 11,
        'A' => 12,
        _ => panic!("invalid card"),
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
fn get_hand_value_joker(hand: &Hand) -> u64 {
    let mut sum = 0u64;
    let base = 13;
    for (i, x) in hand.cards.chars().rev().enumerate() {
        let factor = u32::pow(base, i as u32);
        sum += get_card_value_joker(&x) * factor as u64;
    }
    sum
}
fn get_hand_type(hand: &Hand) -> u64 {
    if hand.counts.values().any(|&v| v >= 5u32) {
        return 10;
    }
    if hand.counts.values().any(|&v| v >= 4u32) {
        return 9;
    }
    let threes = hand.counts.values().filter(|&v| v == &3u32).count();
    let twos = hand.counts.values().filter(|&v| v == &2u32).count();
    if threes == 1 && twos == 1 {
        return 8;
    }
    if threes == 1 {
        return 7;
    }
    if twos == 2 {
        return 6;
    }
    if twos == 1 {
        return 5;
    }
    return 2;
}
fn get_hand_type_joker(hand: &Hand) -> u64 {
    let leeway = hand.counts.get(&'J').unwrap_or(&0);
    let nonjoker_counts: Vec<&u32> = hand
        .counts
        .iter()
        .filter_map(|c| if c.0 != &'J' { Some(c.1) } else { None })
        .collect();
    if leeway == &5 || nonjoker_counts.iter().any(|&v| *v >= 5u32 - leeway) {
        return 10;
    }
    if leeway == &4 || nonjoker_counts.iter().any(|&v| *v >= 4u32 - leeway) {
        return 9;
    }
    let threes = hand.counts.values().filter(|&v| v == &3u32).count();
    let twos = hand.counts.values().filter(|&v| v == &2u32).count();
    if leeway == &0 && threes == 1 && twos == 1 {
        return 8;
    }
    if leeway == &1 && twos == 2 {
        return 8;
    }

    if leeway == &0 && threes == 1 {
        return 7;
    }
    if leeway == &1 && twos == 1 {
        return 7;
    }
    if leeway == &2 {
        return 7;
    }
    if leeway > &0 {
        return 5;
    }
    if twos == 2 {
        return 6;
    }
    if twos == 1 {
        return 5;
    }

    2
}
fn get_points(hand: &Hand, joker: bool) -> u64 {
    let cardvalue = if joker {
        get_hand_value_joker(&hand)
    } else {
        get_hand_value(&hand)
    };
    let steplimit = 371292;
    let handtype = if joker {
        get_hand_type_joker(hand)
    } else {
        get_hand_type(hand)
    };
    return handtype * steplimit + cardvalue;
}
impl Hand {
    fn set_point(&mut self, joker: bool) {
        self.points = get_points(&self, joker);
    }
    fn new(data: &str, joker: bool) -> Hand {
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
        h.set_point(joker);
        h
    }
}
fn main() {
    let data = std::fs::read_to_string("2023/7.txt").unwrap();
    let mut g = Game::new(&data, false);
    let w = &g.get_winnings();
    println!("regular: {}", w);
    let mut g = Game::new(&data, true);
    let w = &g.get_winnings();
    println!("joker: {}", w);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_parse_hand() {
        let c = Hand::new("AAAAA 33", false);
        assert_eq!(33, c.bid);
        assert_eq!(&5, c.counts.get(&'A').unwrap());
        assert_eq!(371292, get_hand_value(&c))
    }

    #[test]
    fn test_can_game() {
        let str = "32T3K 765\nT55J5 684\nKK677 28\nKTJJT 220\nQQQJA 483";
        let mut g = Game::new(str, false);
        let w = &g.get_winnings();
        assert_eq!(&6440, w);
    }
    #[test]
    fn test_can_game_joker() {
        let str = "32T3K 765\nT55J5 684\nKK677 28\nKTJJT 220\nQQQJA 483";
        let mut g = Game::new(str, true);
        let w = &g.get_winnings();
        assert_eq!(&5905, w);
    }

    // FiveKind, 10
    // FourKind,9
    // FullHouse, 8
    // ThreeKind, 7
    // TwoPair,6
    // OnePair,5
    // HighCard,2

    #[test]
    fn test_hand_types_highcard() {
        assert_eq!(2, get_hand_type_joker(&Hand::new("2K354 33", true)));
    }
    #[test]
    fn test_hand_types_pair() {
        assert_eq!(5, get_hand_type_joker(&Hand::new("22354 33", true)));
        assert_eq!(5, get_hand_type_joker(&Hand::new("J2354 33", true)));
    }
    #[test]
    fn test_hand_types_twopair() {
        assert_eq!(6, get_hand_type_joker(&Hand::new("22334 33", true)));
    }
    #[test]
    fn test_hand_types_threekind() {
        assert_eq!(7, get_hand_type_joker(&Hand::new("22234 33", true)));
        assert_eq!(7, get_hand_type_joker(&Hand::new("J2234 33", true)));
        assert_eq!(7, get_hand_type_joker(&Hand::new("JJ234 33", true)));
    }
    #[test]
    fn test_hand_types_fullhouse() {
        assert_eq!(8, get_hand_type_joker(&Hand::new("22233 33", true)));
        assert_eq!(8, get_hand_type_joker(&Hand::new("J2233 33", true)));
    }

    #[test]
    fn test_hand_types_fourkind() {
        assert_eq!(9, get_hand_type_joker(&Hand::new("22223 33", true)));
        assert_eq!(9, get_hand_type_joker(&Hand::new("2J223 33", true)));
        assert_eq!(9, get_hand_type_joker(&Hand::new("22J3J 33", true)));
        assert_eq!(9, get_hand_type_joker(&Hand::new("23JJJ 33", true)));
    }
    #[test]
    fn test_hand_types_fivekind() {
        assert_eq!(10, get_hand_type_joker(&Hand::new("22222 33", true)));
        assert_eq!(10, get_hand_type_joker(&Hand::new("2J222 33", true)));
        assert_eq!(10, get_hand_type_joker(&Hand::new("22J2J 33", true)));
        assert_eq!(10, get_hand_type_joker(&Hand::new("22JJJ 33", true)));
        assert_eq!(10, get_hand_type_joker(&Hand::new("JJJJ2 33", true)));
        assert_eq!(10, get_hand_type_joker(&Hand::new("JJJJJ 33", true)));
    }
    #[test]
    fn test_hand_types() {
        assert_eq!(5, get_hand_type_joker(&Hand::new("32T3K 33", true)));
        assert_eq!(6, get_hand_type_joker(&Hand::new("KK677 33", true)));
        assert_eq!(9, get_hand_type_joker(&Hand::new("T55J5 33", true)));
        assert_eq!(9, get_hand_type_joker(&Hand::new("KTJJT 33", true)));
        assert_eq!(9, get_hand_type_joker(&Hand::new("QQQJA 33", true)));
    }
}
