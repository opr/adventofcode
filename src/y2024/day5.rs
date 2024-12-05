use std::collections::{HashMap, HashSet};
use itertools::Itertools;
use crate::input_reader::input_reader::read_input_for_day;

pub fn task() {
    let input = read_input_for_day(2024, 5, false );

    let start = std::time::Instant::now();
    println!("Part 1: {} in {:?}", part1( input.clone() ), start.elapsed());

    let start = std::time::Instant::now();
    println!("Part 2: {} in {:?}", part2( input.clone() ), start.elapsed());
}

fn parse_input( input: String ) -> ( Vec<Vec<usize>>, HashMap<usize,HashSet<usize>> ) {
    let ( ordering_rules_raw, updates_raw ): (&str, &str) = input.split_once("\n\n" ).unwrap();
    
    let ordering_rules: Vec<(usize, usize)> = ordering_rules_raw
        .split( '\n' )
        .map( |rule| {
            let pages = rule
                .split_once('|')
                .unwrap();
            ( pages.0.parse().unwrap(), pages.1.parse().unwrap() )
        } )
        .collect();
    let updates: Vec<Vec<usize>> = updates_raw
        .split( '\n' )
        .map( | line | line.split(',' )
            .map( | page | page.parse().unwrap() ).collect() )
        .collect();

    let page_map = ordering_rules
        .iter()
        .fold( HashMap::<usize,HashSet<usize>>::new(), | mut map, val | {
            let ( before, after ) = val;
            let mut must_come_before = map
                .get( before )
                .or( Some( &HashSet::new() ) )
                .unwrap()
                .clone();

            must_come_before.insert( *after );
            map.insert( *before, must_come_before );
            map
        } );

    ( updates, page_map )
}

fn part1( input: String ) -> usize {
    let ( updates, page_map ) = parse_input( input );
    let empty_set = HashSet::<usize>::new();
    let mut total = 0;
    for update in updates {
        let mut valid = true;
        for ( index, page ) in update.iter().enumerate() {
            if ! valid {
                continue;
            }
            let pages_before: HashSet<&usize> = update.iter().take( index ).collect();
            let must_come_before = page_map.get( page ).unwrap_or( &empty_set );
            valid = ! must_come_before.iter().any( | page | pages_before.contains( page ) );
        }
        if valid {
            let middle_page = update.get( update.len() / 2 ).unwrap();
            total += middle_page;
        }
    }
    total
}

fn part2( input: String ) -> usize {
    let ( updates, page_map ) = parse_input( input );
    let empty_set = HashSet::<usize>::new();
    let mut total = 0;
    let invalid_updates: Vec<Vec<usize>> = updates.iter().filter_map( | update | {
        let mut valid = true;
        for ( index, page ) in update.clone().iter().enumerate() {
            if ! valid {
                continue;
            }
            let pages_before: HashSet<&usize> = update.iter().take( index ).collect();
            let must_come_before = page_map.get( page ).unwrap_or( &empty_set );
            valid = ! must_come_before.iter().any( | page | pages_before.contains( page ) );
        }
        match valid {
            true => None,
            false => Some( update.clone() )
        }
    } ).collect();

    let mut total = 0;

    for update in invalid_updates.clone().iter_mut() {
        let mut new_update = Vec::<usize>::new();
        for page in update {
            let must_come_before = page_map.get( page ).unwrap_or( &empty_set );
            let pages_before: HashSet<&usize> = new_update.iter().collect();
            let ( insert_at, _ ) = new_update.iter().find_position( | page | must_come_before.contains( page ) ).unwrap_or( ( pages_before.len(), &0 ) );
            new_update.insert( insert_at, page.clone() );
        }
        total += new_update.get( new_update.len() / 2 ).unwrap();
    }
    total
}