use std::{collections::{HashMap, HashSet}, vec};
use std::cmp::Ordering;

use crate::{runner::Runner, utility::read_lines};

pub struct Day07;

// Define a custom enum to represent the type of a hand
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandType {
    pub fn from_hand(hand: &Vec<u32>) -> HandType {
        let mut cp = hand.clone();
        let unique: HashSet<u32> = HashSet::from_iter(hand.clone().into_iter());

        let mut pairs = vec![0; 5];

        for u in &unique {
            let x = cp.len();
            cp.retain(|c| c != u);
            let y = cp.len();

            pairs[x - y - 1] += 1;
        }

        if pairs[4] == 1 {
            return HandType::FiveOfAKind
        }
        else if pairs[3] == 1 {
            return HandType::FourOfAKind
        }
        else if pairs[2] == 1 {
            if pairs[1] == 1 {
                return HandType::FullHouse
            }
            return HandType::ThreeOfAKind
        }
        else if pairs[1] == 2 {
            return HandType::TwoPair
        }
        else if pairs[1] == 1 {
            return HandType::OnePair
        }
        return HandType::HighCard;
    }
}

// Define a struct to represent a hand
#[derive(Debug)]
struct Hand {
    cards: Vec<u32>,
    hand_type: HandType,
    bid: u32,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        // First, compare by hand type
        let hand_type_ordering = self.hand_type.cmp(&other.hand_type);

        // If hand types are the same, compare by the first element in the vector
        if hand_type_ordering == Ordering::Equal {
            // Compare each element of the hand vector
            for (a, b) in self.cards.iter().zip(other.cards.iter()) {
                let element_ordering = b.cmp(a);
                if element_ordering != Ordering::Equal {
                    return element_ordering;
                }
            }

            // If all elements are equal, compare by bid
            self.bid.cmp(&other.bid)
        } else {
            hand_type_ordering
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards && self.hand_type == other.hand_type && self.bid == other.bid
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Hand {}

impl Runner for Day07 {
    fn part_one(&self) -> u32 {
        let raw_hands = read_lines("./input/day07.txt");
        let mut hands: Vec<Hand> = Vec::new();

        for hand in raw_hands {
            let parts: Vec<&str> = hand.split(" ").collect();
            let cards: Vec<u32> = parts[0]
                .chars()
                .map(|c| {
                    match c {
                        'A' => 14,
                        'K' => 13,
                        'Q' => 12,
                        'J' => 11,
                        'T' => 10,
                        '9' => 9,
                        '8' => 8,
                        '7' => 7,
                        '6' => 6,
                        '5' => 5,
                        '4' => 4,
                        '3' => 3,
                        '2' => 2,
                        _ =>{ 
                            panic!("Error parsing file!")
                        }
                    }
                })
                .collect();
            

            let bid = parts[1].parse().unwrap();

            let hand_type = HandType::from_hand(&cards);
            
            hands.push(Hand{
                cards,
                hand_type,
                bid,
            })
        }

        hands.sort();
        hands.reverse();

        let mut total = 0;
        for i in 0..hands.len() {
            total += (i + 1) as u32 * hands[i].bid;
        }
        total
    }

    fn part_two(&self) -> u32 {
        0
    }
}