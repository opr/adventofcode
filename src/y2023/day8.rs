use std::collections::{HashMap, HashSet};
use std::iter::Map;
use crate::input_reader::input_reader::read_input_for_day;

#[derive(Clone,Debug)]
struct MapEntry {
    id: usize,
    left: usize,
    right: usize,
}

pub fn task() {
    let start = std::time::Instant::now();
    let mut input = read_input_for_day( 2023, 8, false );
    let lines: Vec<&str> = input.split("\n").collect();

    let instructions = lines[0];

    let mut locations = HashMap::<&str, usize>::new();

    let maps = lines[ 2..lines.len() ].to_vec();

    // Build location map first.
    for ( index, map ) in maps.iter().enumerate() {

        let ( location, _ ) = map.split_once( " = " ).unwrap();
        locations.insert( location, index );
        let h = 1;
    }

    // Build a list of MapEntry where the node id and it's left-right values are stored.
    let map_entries = maps.iter().enumerate().map( | ( index, map ) | {
        let ( location, destination ) = map.split_once( " = " ).unwrap();

        let replaced = destination.replace( "(", "" ).replace( ")", "" );
        let ( left, right ) = replaced.split_once( ", " ).unwrap();

        return MapEntry {
            id: *locations.get( location ).unwrap(),
            left: *locations.get( left ).unwrap(),
            right: *locations.get( right) .unwrap(),
        };

        let h = 1;
    } ).collect::<Vec<MapEntry>>();

    let start_id = locations.get( "AAA" ).unwrap();
    let end_id = locations.get( "ZZZ" ).unwrap();

    let answer = follow_path( instructions, *start_id, *end_id, map_entries );

    println!( "Day 8 task 1: {}", answer );
    eprintln!( "{:?}", start.elapsed() );
}

fn follow_path( instructions: &str, start_id: usize, end_id: usize, map_entries: Vec<MapEntry> ) -> usize {
    let mut current_id = start_id;
    let mut instruction_pointer = 0;
    let instructions = instructions.chars().collect::<Vec<char>>();

    while current_id != end_id {
        let instruction = instructions[ instruction_pointer % instructions.len() ];

        if instruction == 'L' {
            current_id = map_entries.get( current_id ).unwrap().left
        }
        if instruction == 'R' {
            current_id = map_entries.get( current_id ).unwrap().right
        }
        instruction_pointer += 1;
    }
    return instruction_pointer;
}