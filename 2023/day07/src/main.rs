use std::fs::File;
use std::io::{BufRead, BufReader};
use std::cmp::Ordering;

type Hand = Vec<char>;
type DataStruct = Vec<(Hand, u64)>;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

const CARDS: [char; 13] = ['2','3','4','5','6','7','8','9','T','J','Q','K','A'];
const CARDS_P2: [char; 13] = ['J', '2','3','4','5','6','7','8','9','T','Q','K','A'];

fn main() {    
    let mut game = parse_file("./input.data").unwrap();
    
    // PART 1
    game.sort_by(|a, b| compare_hands(&a.0, &b.0, false));
    let bids = game.iter().map(|x| x.1);
    let values = bids.enumerate().map(|(i, x)| x * (i as u64 + 1));
    let winnings: u64 = values.sum();
    println!("Result part 1: {}", winnings);

    game.sort_by(|a, b| compare_hands(&a.0, &b.0, true));
    let bids = game.iter().map(|x| x.1);
    let values = bids.enumerate().map(|(i, x)| x * (i as u64 + 1));
    let winnings: u64 = values.sum();
    println!("Result part 2: {}", winnings);
}

fn compare_hands(left: &Hand, right: &Hand, part2: bool) -> Ordering {
    let left_type = hand_type(left, part2);
    let right_type = hand_type(right, part2);
    let card_order = if part2 {
        CARDS_P2
    } else {
        CARDS
    };
    if left_type == right_type {
        for (a, b) in left.iter().zip(right.iter()) {
            if a==b {
                continue;
            } else {
                let a_weight = card_order.iter().position(|x| x==a);
                let b_weight = card_order.iter().position(|x| x==b);
                return a_weight.cmp(&b_weight);
            }
        }
        return Ordering::Equal;
    } else {
        return left_type.cmp(&right_type);
    }
}

fn hand_type(hand: &Hand, part2: bool) -> HandType {
    let mut card_counts = Vec::new();
    for card in &CARDS {
        let num = hand.iter().filter(|x| x==&card).count();
        card_counts.push(num);
    }
    
    let jokers = if part2 {card_counts.remove(9)} else {0};  // remove J as joker
    card_counts.sort();
    card_counts.reverse();

    // always optimal to add jokers to largest non-joker cardtype
    match (card_counts[0] + jokers, card_counts[1]) {
        (5, _) => return HandType::FiveOfAKind,
        (4, _) => return HandType::FourOfAKind,
        (3, 2) => return HandType::FullHouse,
        (3, _) => return HandType::ThreeOfAKind,
        (2, 2) => return HandType::TwoPair,
        (2, _) => return HandType::OnePair,
        _ => return HandType::HighCard,
        
    }
}

fn parse_file(filepath: &str) -> Result<DataStruct, std::io::Error> {
    let file = File::open(filepath)?;
    let mut lines = BufReader::new(file).lines();

    let mut result = Vec::new();
    while let Some(line) = lines.next() {
        let string = line?;
        
        let args: Vec<&str> = string.split_whitespace().collect();
        let hand: Hand = args[0].chars().collect();
        let num = args[1].parse().unwrap();
        
        result.push((hand, num));
    }

    return Ok(result);
}