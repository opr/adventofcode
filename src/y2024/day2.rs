use std::cmp::Ordering::{Greater, Less};
use itertools::Itertools;
use crate::input_reader::input_reader::read_input_for_day;

pub fn task() {
    let start = std::time::Instant::now();
    let input = read_input_for_day(2024, 2, false );
    let reports = input.split('\n').map(|s| s.split(" ").map( | i | i.parse::<usize>().unwrap() ).collect_vec()).collect_vec();
    part1( reports.clone() );
    part2( reports.clone() );
}

fn part1( reports: Vec<Vec<usize>> ) {
    let start = std::time::Instant::now();
    let safe_reports = reports.iter().filter( | levels | check_report_safety( levels, 0, 1, 0 ) );
    println!("Part 1: {} in {:?}", safe_reports.count(), start.elapsed());
}
fn part2( reports: Vec<Vec<usize>> ) {
    let start = std::time::Instant::now();
    let safe_reports = reports.iter().filter( | levels | {
        if ! check_report_safety( levels, 0, 1, 1 ) {
            if ! check_report_safety( levels, 1, 2, 0 ) {
                return check_report_safety(levels, 0, 2, 0 );
            }
        }
        true
    } ).collect_vec();
    println!("Part 2: {} in {:?}", safe_reports.len(), start.elapsed());
}

fn check_report_safety( levels : &Vec<usize>, first_level_index: usize, second_level_index: usize, allowed_unsafe_levels: usize ) -> bool {
    let first_level = levels.iter().nth( first_level_index ).unwrap();
    let second_level = levels.iter().nth( second_level_index ).unwrap();

    if first_level == second_level || first_level.abs_diff( *second_level ) > 3 {
        return false;
    }

    let direction = match first_level > second_level {
        true => Less,
        false => Greater,
    };

    let mut last_compared = second_level;
    let mut unsafe_levels = 0;

    for level in levels.iter().skip(second_level_index + 1 ) {
        if unsafe_levels > 1 {
            return false;
        }
        if level == last_compared || level.abs_diff( *last_compared ) > 3 || level.cmp( &last_compared ) != direction {
            unsafe_levels += 1;
            continue;
        }
        last_compared = level;
    }
    unsafe_levels <= allowed_unsafe_levels
}