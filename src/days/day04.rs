use core::num;
use std::{collections::HashMap, vec};
use std::collections::HashSet;

use crate::{runner::Runner, utility::read_lines};

pub struct Day04;

struct Card {
    winning_numbers: Vec<i32>,
    numbers: Vec<i32>
}

impl Card {
    pub fn get_winners(&self) -> Vec<i32> {
        self.numbers.iter().filter(|&num| self.winning_numbers.contains(num)).cloned().collect()
    }

    pub fn new(winning_numbers: Vec<i32>, numbers: Vec<i32>) -> Card {
        Card {
            winning_numbers,
            numbers
        }
    }
}

impl Runner for Day04 {
    fn part_one(&self) -> u32 {
        let mut total = 0;
        let lines = read_lines("./input/day04.txt");
        for line in lines {
            let mut points = 0;

            let data: Vec<&str> = line.split(": ").collect();
            let nums = data[1].replace("  ", " ");

            let nums: Vec<&str> = nums
                .split(" | ")
                .collect();
            
            let winners: Vec<&str> = nums[0]
                .split(" ")
                .collect();

            let numbers: Vec<&str> = nums[1]
                .split(" ")
                .collect();

            for num in &numbers {
                if winners.contains(num) {
                    if points == 0 {
                        points = 1;
                    }
                    else {
                        points *= 2;
                    }
                }
            }
            total += points;
        }
        return total;
    }

    fn part_two(&self) -> u32 {
        let mut cards: Vec<Card> = Vec::new();
        let lines = read_lines("./input/day04.txt");

        for line in lines {
            let card: Vec<&str> = line.split(": ").collect();

            let nums = card[1].replace("  ", " ");

            let nums: Vec<&str> = nums
                .split(" | ")
                .collect();
            
            let winners: Vec<&str> = nums[0]
                .split(" ")
                .collect();

            let numbers: Vec<&str> = nums[1]
                .split(" ")
                .collect();

            let winners = winners.iter().filter_map(|f| f.parse().ok()).collect();
            let numbers = numbers.iter().filter_map(|f| f.parse().ok()).collect();
            
            cards.push(Card::new(winners, numbers));
        }

        let mut card_counts: Vec<u32> = vec![1; cards.len()];

        for i in 0..cards.len() {
            let card: &Card = &cards[i];
            let wins = card.get_winners();

            for j in i + 1..=i + wins.len() {
                card_counts[j] += card_counts[i]
            }
        }
        return card_counts.iter().sum();
    }
}