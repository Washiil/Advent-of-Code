use std::{collections::HashSet, cmp::Ordering};

advent_of_code::solution!(7);

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
    pub fn from_hand(hand: &Vec<u32>, wilds: bool) -> HandType {
        let mut pairs = vec![0; 5];
        let mut cp = hand.clone();

        if wilds {
            let number_of_wilds = hand.clone().iter().filter(|&n| *n == 1).count();

            cp.retain(|&n| n != 1);

            let unique: HashSet<u32> = HashSet::from_iter(cp.clone().into_iter());

            for u in &unique {
                let x = cp.len();
                cp.retain(|c| c != u);
                let y = cp.len();
    
                pairs[x - y - 1] += 1;
            }

            if let Some(index) = pairs.iter().rposition(|&value| value != 0) {
                pairs[index] -= 1;
                pairs[index + number_of_wilds] += 1;
            }
            // Shift right most non zero value to the right (wilds) amount of indecies.
        }
        else {
            let unique: HashSet<u32> = HashSet::from_iter(cp.clone().into_iter());
            for u in &unique {
                let x = cp.len();
                cp.retain(|c| c != u);
                let y = cp.len();

                pairs[x - y - 1] += 1;
            }
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
        // println!("A: {:?}\nB: {:?}", self.cards, other.cards);
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

pub fn part_one(input: &str) -> Option<u32> {
    let raw_hands = input.lines();
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

        let hand_type = HandType::from_hand(&cards, false);

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
    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let raw_hands = input.lines();
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
                    'J' => 1,
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

        let hand_type = HandType::from_hand(&cards, true);

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
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn temp_test() {
        let result = HandType::from_hand(&vec![1, 8, 13, 4, 10], true);
        assert_eq!(result, HandType::OnePair)
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
