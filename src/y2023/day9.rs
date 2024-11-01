use std::collections::{HashMap, HashSet};
use crate::input_reader::input_reader::read_input_for_day;

pub fn task() {
    let start = std::time::Instant::now();
    let mut input = read_input_for_day( 2023, 9, false );
    let raw_lines: Vec<&str> = input.split("\n").collect();
    let lines = raw_lines
        .iter()
        .map( | line | line.split( " " )
            .map( | number | number.parse::<isize>().unwrap() )
            .collect::<Vec<isize>>() )
        .collect::<Vec<Vec<isize>>>();

    let mut history_maps: Vec<Vec<Vec<isize>>> = vec![];

    // Process each line into its differences.
    for line in &lines {
        let mut new_lines: Vec<Vec<isize>> = vec![ line.to_vec() ];

        // Process until the last line has only 0s.
        loop {
            let last_line = new_lines.last_mut().unwrap();
            if is_complete( &last_line ) {
                break;
            }
            let next_entry = process_line( &last_line );
            new_lines.push( next_entry );
        }

        let mut new_histories = new_lines.to_owned();
        new_histories.reverse();

        for index in 0..new_histories.len() - 1 {
            // We have to take ownership of the list of new histories in each loop or it complains about borrowing mutably and immutably.
            let previous_histories = new_histories.to_owned();

            // The last line of values i.e the one we're using to get the value for the current line. e.g. line 2 is used to get the value for line 3 (the list is reversed)
            let previous_history = previous_histories.get( index ).unwrap();
            let mut next_history = new_histories.get_mut( index + 1 ).unwrap();

            // Take the first and last values of the line, they will be used to calculate the new first and last values of the next line.
            let first_value = previous_history.first().unwrap();
            let last_value = previous_history.last().unwrap();

            next_history.push( next_history.last().unwrap() + last_value);
            next_history.insert( 0, next_history.first().unwrap() - first_value);
        }

        history_maps.push( new_histories );
    }

    let answer = history_maps.iter().fold((0, 0 ), |( part_1, part_2), map | {
        ( map.last().unwrap().last().unwrap() + part_1, map.last().unwrap().first().unwrap() + part_2 )
    } );

    println!("Day 9 task 1: {}", answer.0 );
    println!("Day 9 task 2: {}", answer.1 );
    eprintln!("{:?}", start.elapsed() );
}

// Checks to see if a list is composed of entirely 0s.
fn is_complete( list: &[isize] ) -> bool {
    list.iter().all( | l | *l == 0isize )
}

// Takes a list and processes it into the next list,the next list is a list of differences from the first list.
fn process_line( list: &[isize] ) -> Vec<isize> {
    list.windows( 2 ).map( | item | {
     return item[1] - item[0]
    } ).collect::<Vec<isize>>()
}