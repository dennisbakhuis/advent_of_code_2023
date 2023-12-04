use std::fs;
use std::fmt;
use regex::Regex;

const TEST_DATA: &str = "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";
const BASE_SCORE: u32 = 1;
const DATA_PATH: &str = "../../data/04_scratchcards.txt";


struct Card {
    id: u32,
    winning_numbers: Vec<u32>,
    numbers: Vec<u32>,
}

impl Card {
    pub fn score(&self) -> u32 {
        // amount of winning numbers on the card
        let have_winning_numbers = self.numbers.iter()
            .filter(|&x| self.winning_numbers.contains(x))
            .count() as u32;
       
        if have_winning_numbers == 0 {
            return 0;
        }
        
        let score = BASE_SCORE * 2u32.pow(have_winning_numbers  - 1);
        // println!("winning numbers: {}, score: {}", have_winning_numbers, score);
        score
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
    cards: Vec<Card>,
}

impl Cards {
    pub fn new(input_text: &str) -> Cards {
        let mut cards = Cards {
            cards: Vec::new(),
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
            println!("numbers: {:?}", numbers);

            let card_id = numbers[0].clone();
            let winning_numbers = numbers[1..11].to_vec();
            let selected_numbers = numbers[11..36].to_vec();

            println!("card_id: {}", card_id);
            println!("winning_numbers: {:?}", winning_numbers);
            println!("numbers: {:?}", selected_numbers);


            // fill in the card struct
            let card = Card {
                id: card_id,
                winning_numbers,
                numbers: selected_numbers,
                            
            };
            cards.cards.push(card);
        }

        cards
    }

    pub fn collect_scores(&self) -> Vec<u32> {
        let mut scores: Vec<u32> = Vec::new();
        for card in &self.cards {
            scores.push(card.score());
        }
        scores
    }

}


fn main() {
    // test data
    // let cards = Cards::new(TEST_DATA);

    // read data from file
    let input_text = fs::read_to_string(DATA_PATH)
        .expect("Something went wrong reading the file");

    // select first 10 rows
    // let input_text = input_text.lines()
    //     .take(10)
    //     .collect::<Vec<&str>>()
    //     .join("\n");

    // select only line 4 and 5
    // let input_text = input_text.lines()
    //     .skip(3)
    //     .take(1)
    //     .collect::<Vec<&str>>()
    //     .join("\n");

    let cards = Cards::new(&input_text);

    let scores = cards.collect_scores();
    let sum_of_scores: u32 = scores.iter().sum();

    // print cards
    for card in &cards.cards {
        println!("{}", card);
    }

    // print number of cards
    println!("Number of cards: {}", cards.cards.len());

    // print sum of scores
    println!("Sum of scores: {}", sum_of_scores);
}

