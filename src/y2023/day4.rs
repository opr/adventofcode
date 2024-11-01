use std::collections::HashSet;
use crate::input_reader::input_reader::read_input_for_day;

#[derive(Clone)]
struct Card {
    id: usize,
    winning_numbers: Vec<usize>,
    my_numbers: Vec<usize>,
    score: usize,
    matches: usize,
    is_copy: bool,
    copies: usize,
}

impl Card {
    fn calculate_score(&mut self) -> usize {
        let set1: HashSet<usize> = self.winning_numbers.iter().cloned().collect();
        let set2: HashSet<usize> = self.my_numbers.iter().cloned().collect();
        let union: Vec<_> = set1.union( &set2 ).cloned().collect();
        let diff = (set1.len() + set2.len()) - union.len();
        if diff == 0 {
            return 0;
        }
        self.matches = diff;
        let calc = 2i32.pow( (diff - 1) as u32 ) as usize;
        self.score = calc;
        return calc;
    }
}

fn parse_card( card: &&str, is_copy: bool ) -> Card {
    {
        let mut card_m = card.to_string();
        card_m = card_m.replace("Card ", "").replace("  ", " ");
        let ( card_id, card_numbers ) = card_m.split_once(": ").unwrap();
        let parsed_card_id = card_id.replace(" ", "").parse::<usize>().unwrap_or( 0 );
        let (u_winning_numbers, u_my_numbers) = card_numbers.split_once(" | " ).unwrap();
        let winning_numbers = u_winning_numbers.split(" ").map( |n | n.parse::<usize>().unwrap() ).collect::<Vec<usize>>();
        let my_numbers = u_my_numbers.split(" ").map( |n | n.parse::<usize>().unwrap() ).collect::<Vec<usize>>();

        let mut card = Card {
            my_numbers,
            winning_numbers,
            id: parsed_card_id,
            matches: 0,
            score: 0,
            is_copy,
            copies: 0,
        };

        let score = &card.calculate_score();

        println!("Card {} score is: {}", parsed_card_id, score );
        return card;
    }
}

fn get_original_card_id_index( id: usize, cards: &Vec<Card> ) -> usize {
    let (index, _): ( usize, &Card ) = cards.iter().enumerate().find( |(index,card)| card.id == id && ! card.is_copy ).unwrap();
    index
}

pub fn task() {
    let mut input = read_input_for_day(2023, 4, false );
    let lines: Vec<_> = input.split("\n").collect();

    let mut cards: Vec<Card> = lines.iter().map(|card| parse_card( card, false )  ).collect();

    for index in 0..cards.len() {
        let mut card = &cards[index];
        let existing_copies = card.copies;

        if card.matches == 0 {
            continue;
        }
        let card_from = card.id;
        let card_to = (card.id + card.matches) - 1;
        for mod_index in card_from..=card_to {
            cards[mod_index].copies += 1+existing_copies;
            let b = 1;
        }
    }

    let total_score = &cards.iter().fold(0, |a,b| a + b.score );
    let total = &cards.iter().fold(0, |a,b| a + b.copies+1 );
    println!("Day 4 task 1: {}", total_score );
    println!("Day 4 task 2: {}", total );

    let g =1;
}