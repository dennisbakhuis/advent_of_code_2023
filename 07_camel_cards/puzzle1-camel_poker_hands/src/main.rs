use std::collections::HashMap;
use std::fmt;


const INPUT_DATA: &str = include_str!("../../../data/07_camel_poker_hands.txt");
// const INPUT_DATA: &str = "\
// 32T3K 765
// T55J5 684
// KK677 28
// KTJJT 220
// QQQJA 483
// ";

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
enum HandRank {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
struct Hand {
    rank: HandRank,
    cards: Vec<Card>,
    bid: u128,
    hand_value: u128,
}

impl Hand {
    fn new(row_string: &str) -> Hand {
        let mut parts = row_string.split_whitespace();
        let cards_string = parts.next().unwrap();
        let bid_string = parts.next().unwrap();

        let mut cards: Vec<Card> = Vec::new();
        for c in cards_string.chars() {
            match c {
            '2' => cards.push(Card::Two),
            '3' => cards.push(Card::Three),
            '4' => cards.push(Card::Four),
            '5' => cards.push(Card::Five),
            '6' => cards.push(Card::Six),
            '7' => cards.push(Card::Seven),
            '8' => cards.push(Card::Eight),
            '9' => cards.push(Card::Nine),
            'T' => cards.push(Card::Ten),
            'J' => cards.push(Card::Jack),
            'Q' => cards.push(Card::Queen),
            'K' => cards.push(Card::King),
            'A' => cards.push(Card::Ace),
            _ => panic!("Invalid card: {}", c),
            }
        }

        let bid = bid_string.parse::<u128>().unwrap();

        let rank = Hand::determine_rank(&cards);
        
        let mut hand_value = 0;
        println!("\nrow_string: {}", row_string);
        for (ix, card) in cards.iter().enumerate() {
            let level = 100u128.pow(4 - ix as u32);
            let card_number = *card as u128 + 1;
            let card_value = card_number * level;
            hand_value += card_value;
            println!("level: {}", level);
            println!("card_number: {}", card_number);
            println!("card_value: {}", card_value);
        }

        match rank {
            HandRank::FiveOfAKind => {
                hand_value += 100_000_000_000_000_000_000;
            },
            HandRank::FourOfAKind => {
                hand_value += 1_000_000_000_000_000_000;
            },
            HandRank::FullHouse => {
                hand_value += 10_000_000_000_000_000;
            },
            HandRank::ThreeOfAKind => {
                hand_value += 100_000_000_000_000;
            },
            HandRank::TwoPair => {
                hand_value += 1_000_000_000_000;
            },
            HandRank::OnePair => {
                hand_value += 10_000_000_000;
            },
            HandRank::HighCard => {},
        }

        Hand {
            rank,
            cards,
            bid,
            hand_value,
        }

    }

    fn determine_rank(cards: &Vec<Card>) -> HandRank {
        let mut counts: HashMap<Card, u8> = HashMap::new();

        for card in cards {
            let count = counts.entry(*card).or_insert(0);
            *count += 1;
        }

        let mut has_three = false;
        let mut n_two = 0;
        for (_, count) in &counts {
            if *count == 5 {
                return HandRank::FiveOfAKind;
            }
            if *count == 4 {
                return HandRank::FourOfAKind;
            }
            if *count == 3 {
                has_three = true;
            }
            if *count == 2 {
                n_two += 1;
            }
        }
        if has_three && n_two == 1 {
            return HandRank::FullHouse;
        }
        if has_three {
            return HandRank::ThreeOfAKind;
        }
        if n_two == 2 {
            return HandRank::TwoPair;
        }
        if n_two == 1 {
            return HandRank::OnePair;
        }

        HandRank::HighCard
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Card::Two => write!(f, "2"),
            Card::Three => write!(f, "3"),
            Card::Four => write!(f, "4"),
            Card::Five => write!(f, "5"),
            Card::Six => write!(f, "6"),
            Card::Seven => write!(f, "7"),
            Card::Eight => write!(f, "8"),
            Card::Nine => write!(f, "9"),
            Card::Ten => write!(f, "T"),
            Card::Jack => write!(f, "J"),
            Card::Queen => write!(f, "Q"),
            Card::King => write!(f, "K"),
            Card::Ace => write!(f, "A"),
        }
    }
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for card in &self.cards {
            write!(f, "{} ", card)?;
        }

        // show rank
        write!(f, " rank: {}", self.rank)?;

        // show bid
        write!(f, " bid: {}", self.bid)?;

        // show order
        write!(f, " order: {}", self.hand_value)?;

        Ok(())
    }
}

impl fmt::Display for HandRank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            HandRank::FiveOfAKind => write!(f, "Five of a Kind"),
            HandRank::FourOfAKind => write!(f, "Four of a Kind"),
            HandRank::FullHouse => write!(f, "Full House"),
            HandRank::ThreeOfAKind => write!(f, "Three of a Kind"),
            HandRank::TwoPair => write!(f, "Two Pair"),
            HandRank::OnePair => write!(f, "One Pair"),
            HandRank::HighCard => write!(f, "High Card"),
        }
    }
}

fn main() {
    let lines = INPUT_DATA.lines().filter(|line| !line.is_empty());

    // insert cards into set
    let mut hands: Vec<Hand> = Vec::new();
    for line in lines {
        let hand = Hand::new(line);
        hands.push(hand);
    }

    // sort hands by hand_value
    hands.sort_by(|a, b| a.hand_value.cmp(&b.hand_value));

    for hand in &hands {
        println!("{}", hand);
    }

    // calculate score
    let mut score: u128 = 0;
    let mut counter = 0;
    for hand in &hands {
        counter += 1;
        score += hand.bid * counter;
    }

    println!("score: {}", score);
}
