use std::cmp::Ordering;
use std::io;
use std::iter::Iterator;

#[derive(Clone, Copy, PartialOrd, PartialEq, Eq, Ord, Debug)]
enum CardType {
    A = 0,
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
            _ => unreachable!(),
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
    for (i, card) in hand.iter_mut().enumerate() {
        *card = CardType::new(s.as_bytes()[i] as char)
    }
    hand
}

fn frequencies(hand: Hand) -> [i32; 13] {
    let mut freqs = [0; 13];
    for card in hand {
        freqs[card as usize] += 1;
    }
    freqs
}

fn classify_hand(hand: Hand) -> HandType {
    use crate::HandType::*;
    let freqs = frequencies(hand);
    if freqs.iter().any(|v| *v == 5) {
        return FiveOfAKind;
    }
    if freqs.iter().any(|v| *v == 4) {
        return FourOfAKind;
    }
    if freqs.iter().any(|v| *v == 3) && freqs.iter().any(|v| *v == 2) {
        return FullHouse;
    }
    if freqs.iter().any(|v| *v == 3) {
        return ThreeOfAKind;
    }
    if freqs.iter().filter(|v| **v == 2).count() == 2 {
        return TwoPair;
    }
    if freqs.iter().any(|v| *v == 2) {
        return OnePair;
    }
    assert!(freqs.iter().all(|v| *v <= 1));

    HighCard
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
        if classify_cmp == Ordering::Equal {
            hand1.cmp(hand2).reverse()
        } else {
            classify_cmp.reverse()
        }
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

    let contents = std::fs::read_to_string("input.txt").unwrap();
    let lines = contents.lines().collect();
    println!("{}", part1(lines));

    Ok(())
}
