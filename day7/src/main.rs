use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::iter::Iterator;

#[derive(Clone, Copy, PartialOrd, PartialEq, Eq, Ord, Hash, Debug)]
enum CardType {
    A,
    K,
    Q,
    J,
    T,
    N9,
    N8,
    N7,
    N6,
    N5,
    N4,
    N3,
    N2,
}

impl CardType {
    fn new(c: char) -> Self {
        use CardType::*;
        match c {
            'A' => A,
            'K' => K,
            'Q' => Q,
            'J' => J,
            'T' => T,
            '9' => N9,
            '8' => N8,
            '7' => N7,
            '6' => N6,
            '5' => N5,
            '4' => N4,
            '3' => N3,
            '2' => N2,
            _ => unreachable!()
        }
    }
}

#[derive(Clone, Copy, PartialOrd, PartialEq, Eq, Ord, Debug)]
enum HandType {
    FiveOfAKind = 0,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

type Hand = [CardType; 5];

fn parse_hand(s: &str) -> Hand {
    use crate::CardType::*;
    let mut hand = [N2; 5];
    for i in 0..5 {
        hand[i] = CardType::new(s.as_bytes()[i] as char)
    }
    return hand;
}

fn frequencies<I, T>(iterator: I) -> HashMap<T, u32>
    where
        I: Iterator<Item=T>,
        T: Eq + std::hash::Hash + Copy
{
    let mut m = HashMap::new();
    for item in iterator {
        if m.contains_key(&item) {
            m.insert(item, m[&item] + 1);
        } else {
            m.insert(item, 1);
        }
    }

    m
}

fn classify_hand(hand: Hand) -> HandType {
    use crate::HandType::*;
    let freqs = frequencies(hand.into_iter());
    if freqs.values().any(|v| *v == 5) {
        return FiveOfAKind;
    }
    if freqs.values().any(|v| *v == 4) {
        return FourOfAKind;
    }
    if freqs.values().any(|v| *v == 3) && freqs.values().any(|v| *v == 2) {
        return FullHouse;
    }
    if freqs.values().any(|v| *v == 3) {
        return ThreeOfAKind;
    }
    if freqs.values().filter(|v| **v == 2).count() == 2 {
        return TwoPair;
    }
    if freqs.values().any(|v| *v == 2) {
        return OnePair;
    }
    assert!(freqs.values().all(|v| *v == 1));

    return HighCard;
}

fn part1(input: Vec<&str>) -> i32 {
    let mut hands_and_bids: Vec<(Hand, i32)> = vec![];
    for line in input {
        let parts: Vec<&str> = line.split(" ").collect();
        let hand = parse_hand(parts[0]);
        let bid = parts[1].parse::<i32>().unwrap();
        hands_and_bids.push((hand, bid));
    }
    hands_and_bids.sort_by(|(hand1, _), (hand2, _)| {
        let classify_cmp = classify_hand(*hand1).cmp(&classify_hand(*hand2));
        return if classify_cmp == Ordering::Equal {
            hand1.cmp(hand2).reverse()
        } else {
            classify_cmp.reverse()
        };
    });
    // println!("{:?}", hands_and_bids);
    let mut total = 0;
    for (i, (_, bid)) in hands_and_bids.iter().enumerate() {
        total += (i + 1) as i32 * bid;
    }
    total
}

fn main() -> io::Result<()> {
    let example = vec![
        "32T3K 765",
        "T55J5 684",
        "KK677 28",
        "KTJJT 220",
        "QQQJA 483",
    ];
    println!("{}", part1(example));
    let file = File::open("input.txt")?;

    let reader = io::BufReader::new(file);

    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();
    let lines: Vec<&str> = lines.iter().map(|s| s.as_str()).collect();
    println!("{}", part1(lines));

    Ok(())
}
