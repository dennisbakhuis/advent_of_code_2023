use std::fs;
use std::fmt;
use regex::Regex;
use std::collections::HashMap;

const TEST_DATA: &str = "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";
const USE_TEST_DATA: bool = true;
const DATA_PATH: &str = "../../data/04_scratchcards.txt";


struct Card {
    id: u32,
    winning_numbers: Vec<u32>,
    numbers: Vec<u32>,
}

impl Card {
    pub fn score(&self) -> u32 {
        // amount of winning numbers on the card
        let n_winning_numbers = self.numbers.iter()
            .filter(|&x| self.winning_numbers.contains(x))
            .count() as u32;

        n_winning_numbers
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Card {:<3}: ", self.id)?;

        for number in self.winning_numbers.iter() {
            write!(f, "{:<2} ", number)?;
        }
        
        write!(f, "| ")?;
        
        for number in self.numbers.iter() {
            write!(f, "{:<2} ", number)?;
        }

        // show score
        write!(f, "({})", self.score())?;
        Ok(())
    }
}

struct Cards {
    originals: HashMap<u32, Card>,
    cards: Vec<u32>,
}

impl Cards {
    pub fn new(input_text: &str) -> Cards {
        let mut cards = Cards {
            cards: Vec::new(),
            originals: HashMap::new(),
        };

        for line in input_text.lines() {
            // check if 'Card' is in the line
            if !line.contains("Card") {
                continue;
            }

            // extract numbers from line using regex
            let re = Regex::new(r"\d+").unwrap();
            let numbers: Vec<u32> = re.find_iter(line)
                .filter_map(|digits| digits.as_str().parse().ok())
                 .collect();

            let card_id = numbers[0].clone();
            let winning_numbers = numbers[1..11].to_vec();
            let selected_numbers = numbers[11..36].to_vec();
            // let winning_numbers = numbers[1..6].to_vec();
            // let selected_numbers = numbers[6..14].to_vec();

            // fill in the card struct
            let card = Card {
                id: card_id,
                winning_numbers,
                numbers: selected_numbers,
                            
            };
            cards.originals.insert(card_id, card);
        }

        cards
    }

    pub fn process_cards(&mut self) {
        // first get all card ids
        let mut cards_to_process = self.originals.keys().cloned().collect::<Vec<u32>>();

        while !cards_to_process.is_empty() {
            let mut won_cards = Vec::new();

            for card_id in cards_to_process.iter() {
                self.cards.push(card_id.clone());
                let card = self.originals.get(card_id).unwrap();
                let score = card.score();

                for add_id in card_id+1..card_id+score+1 {
                    if self.originals.contains_key(&add_id) {
                        won_cards.push(add_id.clone());
                    }
                }
                
            }

            cards_to_process = won_cards.clone();
        }
    }

}


fn main() {
    // test data
    // let mut cards = Cards::new(TEST_DATA);

    // read data from file
    let input_text = fs::read_to_string(DATA_PATH)
        .expect("Something went wrong reading the file");
    let mut cards = Cards::new(&input_text);

    // print cards
    // for (id, card) in cards.originals {
    //     println!("{}", card);
    // }

    cards.process_cards();

    // print number of cards
    println!("Number of cards: {}", cards.cards.len());
}

