use std::collections::{HashMap, HashSet};
use crate::input_reader::input_reader::read_input_for_day;

#[derive(Clone,Debug)]
struct Hand {
    cards: Vec<usize>,
    bid: usize,
    rank_type: usize,
    rank_desc: String,
}

#[cfg(test)]
mod tests {
    use crate::y2023::day7::get_card_rank;

    #[test]
    fn card_ranking_works_correctly() {
        let cards = [

            ( [ 2, 2, 1, 3, 3 ], 3 ),
            ( [ 3, 3, 3, 4, 2 ], 4 ),

            // Multiple 5 of a kind configs
            ( [ 1, 1, 1, 1, 1 ], 1 ),
            ( [ 1, 1, 1, 1, 2 ], 1 ),
            ( [ 1, 1, 1, 2, 2 ], 1 ),
            ( [ 1, 1, 2, 2, 2 ], 1 ),
            ( [ 1, 2, 2, 2, 2 ], 1 ),
            ( [ 2, 2, 2, 2, 2 ], 1 ),

            // Multiple 4 of a kind configs
            ( [ 1, 1, 1, 2, 3 ], 2 ),
            ( [ 1, 1, 1, 2, 3 ], 2 ),
            ( [ 1, 1, 2, 2, 3 ], 2 ),
            ( [ 1, 2, 2, 2, 3 ], 2 ),
            ( [ 2, 2, 2, 2, 3 ], 2 ),

            // Multiple full house configs
            ( [ 2, 2, 2, 3, 3 ], 3 ),
            ( [ 2, 2, 3, 3, 3 ], 3 ),
            ( [ 2, 2, 1, 3, 3 ], 3 ),

            // Multiple three of a kind configs
            ( [ 3, 3, 3, 4, 2 ], 4 ),
            ( [ 3, 1, 1, 9, 8 ], 4 ),
            ( [ 1, 3, 3, 4, 9 ], 4 ),

            // Only one possible two pair configs
            ( [ 3, 3, 4, 4, 5 ], 5 ),

            // Multiple 1 pair configs
            ( [ 3, 1, 4, 5, 6 ], 6 ),
            ( [ 3, 3, 4, 5, 6 ], 6 ),

            // One high card config
            ( [ 2, 3, 4, 5, 6 ], 7 )
        ];

        for ( card, expected) in cards {
            assert_eq!( get_card_rank( card.to_vec() ), expected );
        }
    }
}

fn get_card_rank( cards: Vec<usize> ) -> usize {

    // Five of a kind:  Rank 1
    // Four of a kind:  Rank 2
    // Full house:      Rank 3
    // Three of a kind: Rank 4
    // Two pair:        Rank 5
    // One pair:        Rank 6
    // High card:       Rank 7

    let card_set = cards.iter().cloned().collect::<HashSet<usize>>();
    let card_set_without_jokers = cards.iter().cloned().filter( | c | *c != 1usize ).collect::<HashSet<usize>>();
    let mut ordered_cards = cards.iter().cloned().filter( | c | *c != 1usize ).collect::<Vec<usize>>();
    ordered_cards.sort();
    let mut ordered_cards_with_jokers = cards.iter().cloned().collect::<Vec<usize>>();
    ordered_cards_with_jokers.sort();

    let joker_count = cards.iter().cloned().filter( | c | *c == 1usize ).collect::<Vec<usize>>().len();

    let is_high_card = card_set.len() == 5 && joker_count == 0;
    if is_high_card {
        return 7;
    }

    let is_five_of_a_kind = joker_count == 5 || ordered_cards.windows(5 - joker_count ).any( | window | window.iter().all( | w | w == &window[0] ) );
    if is_five_of_a_kind {
        return 1;
    }

    let is_four_of_a_kind = joker_count == 4 || ordered_cards.windows(4 - joker_count ).any( | window | window.iter().all( | w | w == &window[0] ) );
    if is_four_of_a_kind {
        return 2;
    }

    let is_three_of_a_kind = ordered_cards.windows(3 - joker_count ).any( | window | window.iter().all( | w | w == &window[0] ) );
    let mut is_full_house = false;

    for card in &card_set_without_jokers {
        if is_full_house {
            continue;
        }
        // Replace all jokers with current card then do the logic from day 1.
        let mut replaced = ordered_cards_with_jokers.clone().iter().map( | c | {
            if *c == 1usize {
                return *card;
            }
            return *c;
        } ).collect::<Vec<usize>>();
        replaced.sort();

        let new_card_set = replaced.iter().cloned().collect::<HashSet<usize>>();

        is_full_house = new_card_set.len() == 2 && ( replaced.windows(3).any( | window | window.iter().all( | w | w == &window[0] ) ) && replaced.windows(2).any( | window | window.iter().all( | w | w == &window[0] ) ) );
    }

    if is_full_house {
        return 3;
    }

    if is_three_of_a_kind {
        return 4;
    }

    let is_two_pair = joker_count == 0 && card_set.len() == 3;
    if is_two_pair {
        return 5;
    }

    let is_one_pair = ordered_cards.windows(2 - joker_count ).any( | window | window.iter().all( | w | w == &window[0] ) );
    if is_one_pair {
        return 6;
    }

    panic!("What combo is possibly not caught?!");
}
impl Hand {
    fn update_rank_type( &mut self ) {

        let new_rank = get_card_rank( self.cards.clone() );
        self.rank_type = new_rank;

    }
}


#[derive(Clone,Debug)]
struct Card {
    value: usize
}

pub fn task() {
    let start = std::time::Instant::now();
    let mut input = read_input_for_day(2023, 7, false );
    let lines: Vec<&str> = input.split("\n").collect();

    let mut face_map: HashMap<char, usize> = HashMap::new();
    face_map.insert( 'T', 10 );
    face_map.insert( 'J', 1 );
    face_map.insert( 'Q', 12 );
    face_map.insert( 'K', 13 );
    face_map.insert(  'A', 14 );

    let mut hands: Vec<Hand> = lines.iter().map( |line| {
        let ( raw_cards, raw_bid ) = line.split_once( " " ).unwrap();
        let bid = raw_bid.parse::<usize>().unwrap();
        let mut cards: Vec<usize> = raw_cards
            .chars()
            .map( |raw| {
                let parse_attempt = raw.to_string().parse::<usize>();
                match parse_attempt {
                    Ok( card_value) => {
                        card_value
                    }
                    Err(_) => {
                        *( face_map.get( &raw ).unwrap() )
                    }
                }
            } )
            .collect();

        let g  =1;
        return Hand {
            cards,
            bid,
            rank_type: 0,
            rank_desc: "".to_string(),
        };
    } ).collect();

    for hand in hands.iter_mut() {
        hand.update_rank_type();
    }

    hands.sort_by( |first, second| {
        if first.rank_type != second.rank_type {
            return second.rank_type.cmp( &first.rank_type );
        }
        if first.rank_type == second.rank_type {
            let mut card_index = 0;
            while card_index < 5 {
                if first.cards[card_index] != second.cards[card_index] {
                    return first.cards[card_index].cmp( &second.cards[card_index ] );
                }
                card_index += 1;
            }
        }
        panic!( "Should have sorted before reaching here, duplicate card sets found." );
    } );

    let answer = hands.iter().enumerate().fold( 0, | acc, ( index, hand ) | { acc + hand.bid * ( index+1 ) } );

    println!( "Day 7 task 2: {}", answer );
    eprintln!("{:?}", start.elapsed() );
}
