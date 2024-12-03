use std::cmp::Ordering::{Greater, Less};
use itertools::Itertools;
use regex::Regex;
use crate::input_reader::input_reader::read_input_for_day;

pub fn task() {
    let start = std::time::Instant::now();
    let input = read_input_for_day(2024, 3, false );
    
    println!("Part 1: {} in {:?}", part1( input ), start.elapsed());
}

fn part1( input: String ) -> usize {
    let re = Regex::new( r"mul\((\d+),(\d+)\)" ).unwrap();
    let muls = re.captures_iter( &input );
    muls.fold( 0, | acc, mul | {
        let first = mul.get( 1 ).unwrap().as_str().parse::<usize>().unwrap();
        let second = mul.get( 2 ).unwrap().as_str().parse::<usize>().unwrap();
        acc + ( second * first  )
    } )
}