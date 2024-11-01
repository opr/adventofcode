use std::collections::{HashMap, HashSet};
use std::io;
use std::io::Write;
use std::ops::Add;
use num_integer;
use crate::input_reader::input_reader::read_input_for_day;

#[derive(Clone,Debug)]
struct SpringPoint {
    has_spring: bool,
    unknown: bool,
}

pub fn task() {
    let start = std::time::Instant::now();
    let mut input = read_input_for_day( 2023, 12, false );
    let lines: Vec<&str> = input.split("\n").collect();

    let springs_1 = create_spring_points_from_string( &"#.#.###".to_string() );
    //let g  = is_valid_spring_config( &springs_1, vec![ 1, 1, 3 ] );
    let r  = 1;

    let mut iter = 0;
    for line in &lines {
        // if iter != 1 {
        //     iter+=1;
        //     continue;
        // }
        // iter += 1;
        let ( _springs, _raw_group_sizes ) = line.split_once( " " ).unwrap();
        let springs = vec![ _springs, _springs, _springs, _springs, _springs ].join("?");
        let raw_group_sizes = vec![ _raw_group_sizes, _raw_group_sizes, _raw_group_sizes, _raw_group_sizes, _raw_group_sizes ].join(",");
        let groups = raw_group_sizes.split(",").collect::<Vec<&str>>().iter().map( | size | size.parse::<usize>().unwrap() ).collect::<Vec<usize>>();
        //let spring_points = create_spring_points_from_string( &springs.to_string() );
        let group_sum = groups.iter().fold( 0, | acc, group | acc + group );
        let group_indices = groups.iter().enumerate().map( | (i, _) | i * 2 ).collect::<Vec<usize>>();

        let mut cache: &mut HashMap<String, usize> = &mut HashMap::new();
        iter += shuffle_groups_below_2( &springs.to_string(), &groups, ( 0, 0, 0 ), cache );
        // let n = 0;
        //let unique_configs = shuffle_groups_below_2( 0, springs.len(), &groups, &group_indices, &springs.to_string() );
        // let valid_for_row = unique_configs.iter().fold( 0, | acc, config | {
        //     let spring_points = &( create_spring_points_from_string( config ) );
        //     if is_valid_spring_config( spring_points, &groups, springs.to_string() ) {
        //         println!("{}", config);
        //         return acc+1;
        //     }
        //     return acc;
        // } );
        // iter+=valid_for_row;
        // //println!("{}: {}", iter);
        // let g = 1;
    }

    let answer = iter;

    println!( "Day 12 task 1: {}", answer );
    eprintln!("{:?}", start.elapsed() );
}

//     private function find_possible_combinations( string $springs, array $sizes, array $state, array &$cache = [] ): int {
fn shuffle_groups_below_2( original_map: &String, groups: &Vec<usize>, state: (usize, usize, usize ), cache: &mut HashMap<String, usize> ) -> usize {
    //shuffle_groups_below_2( original_map, groups, state, cache );
    let key = format!("{}:{}:{}", state.0, state.1, state.2 );
    if cache.contains_key( key.as_str() ) {
        return *cache.get( key.as_str() ).unwrap();
    }

    let mut position = state.0;
    let mut index = state.1;
    let mut length = state.2;

    if position == original_map.len() {
        if index == groups.len() - 1 && length == groups[ index ] {
            index+=1;
            length = 0;
        }
        if index == groups.len() && length == 0 {
            return 1;
        }
        return 0;
    }

    let mut arrangements = 0;

    /*
           // If the current spring is operational or unknown, explore the next state
        if ( str_contains( '.?', $springs[ $position ] ) ) {
            // If the current group length is zero, continue exploring without incrementing the group index
            if ( $length == 0 ) {
                $next_state   = [ $position + 1, $index, 0 ];
                $arrangements += $this->find_possible_combinations( $springs, $sizes, $next_state, $cache );
            } elseif ( $index < count( $sizes ) && $sizes[ $index ] == $length ) {
                // The current group has reached the correct length, move to the next group
                $next_state   = [ $position + 1, $index + 1, 0 ];
                $arrangements += $this->find_possible_combinations( $springs, $sizes, $next_state, $cache );
            }
        }
     */

    let mut next_state = (0,0,0);
    if *original_map.chars().collect::<Vec<char>>().get( position ).unwrap() == '.' || *original_map.chars().collect::<Vec<char>>().get( position ).unwrap() == '?' {
        if length == 0 {
            next_state = ( position + 1, index, 0 );
            arrangements += shuffle_groups_below_2( original_map, groups, next_state, cache );
        } else if index < groups.len() && groups[index] == length {
            next_state = ( position + 1, index+1, 0 );
            arrangements += shuffle_groups_below_2( original_map, groups, next_state, cache );
        }
    }

    /*

        // If the current spring is damaged or unknown, extend the current group's length
        if ( str_contains( '#?', $springs[ $position ] ) ) {
            $next_state   = [ $position + 1, $index, $length + 1 ];
            $arrangements += $this->find_possible_combinations( $springs, $sizes, $next_state, $cache );
        }
     */

    if *original_map.chars().collect::<Vec<char>>().get( position ).unwrap() == '#' || *original_map.chars().collect::<Vec<char>>().get( position ).unwrap() == '?' {
        next_state = ( position + 1, index, length + 1 );
        arrangements += shuffle_groups_below_2( original_map, groups, next_state, cache );
    }
    cache.insert( key, arrangements );
    return arrangements;
}

fn shuffle_groups_below(mut depth: usize, line_len: usize, groups: &Vec<usize>, group_indices: &Vec<usize>, original_map: &String ) -> HashSet<String> {
    if depth == groups.len() {
        return HashSet::new();
    }
    let mut valid_counts = 0;
    let mut valid_set: HashSet<String> = HashSet::new();
    let current_token_size = groups[depth];
    let mut group_indices = group_indices.clone();
    let sum_of_groups_before = groups.iter().take( depth ).fold(0usize, | acc, group | acc + group );
    let sum_of_groups_after = groups.iter().skip( depth + 1 ).fold(0usize, | acc, group | acc + group );
    let first_possible_index = sum_of_groups_before + depth;
    let last_possible_index = ( line_len - ( sum_of_groups_after + groups.len() - ( depth ) ) );
    let g = 0;

    for i in first_possible_index..=last_possible_index {
        group_indices[ depth ] = i;

        valid_set = valid_set.union( &(shuffle_groups_below( depth + 1 , line_len, &groups, &group_indices, original_map ) ) ).cloned().collect();

        let mut groups_printed = 0;
        let mut char = 0;
        let mut buf = "".to_string();
        while char < line_len {
            if groups_printed < group_indices.len() && group_indices[ groups_printed ] == char {
                buf.push_str(&*"#".repeat(groups[groups_printed] ) );
                char += groups[ groups_printed ];
                groups_printed += 1;
                continue;
            }
            char += 1;
            buf.push('.' );
        }
        //let points = &( create_spring_points_from_string( buf.to_string() ) );

        println!("{}", buf.as_str() );
        //let x = is_valid_spring_config( points, groups, original_map.to_string() );
        valid_set.insert( buf );
    }
    //println!("{}------------------------------", depth);
    return valid_set;
}

fn create_spring_points_from_string(springs: &String) -> Vec<SpringPoint> {
    springs.chars().map( | spring_point | SpringPoint { has_spring: spring_point == '#', unknown: spring_point == '?' } ).collect::<Vec<SpringPoint>>().to_vec()
}

fn is_valid_spring_config( spring_points: &Vec<SpringPoint>, groups: &Vec<usize>, original_map: String ) -> bool {
    let mut rev_groups = groups.clone();
    rev_groups.reverse();

    if spring_points.len() != original_map.len() {
        return false;
    }

    let chars = original_map.chars();

    let scount = spring_points.iter().fold( 0usize, | acc, c | {
        if c.has_spring {
            return acc + 1;
        }
        acc
    } );
    let group_sum = groups.iter().fold(0usize, | acc, group | acc + group );
    if scount != group_sum {
        let b = 0;
        return false;
    }

    for ( i, c ) in original_map.chars().collect::<Vec<char>>().iter().enumerate() {
        // if is # and spring point is not spring, return false immediately.
        if *c == '#' && ! spring_points[i].has_spring {
            return false;
        }

        if *c == '.' && spring_points[i].has_spring {
            return false;
        }
    }

    let mut last_point_processed = 0;
    let mut complete_spring_groups: Vec<Vec<&SpringPoint>> = vec![];
    loop {
        if last_point_processed == spring_points.len() || rev_groups.len() == 0 {
            break;
        }
        let r = 1;
        let points = spring_points
            .iter()
            .skip(last_point_processed)
            .take_while( | spring | spring.has_spring && ! spring.unknown )
            .collect::<Vec<&SpringPoint>>();

        if points.len() > 0 {
            if points.len() != *(rev_groups.last().unwrap()) {
                return false;
            }
            rev_groups.pop();
            complete_spring_groups.push(points.clone());
        }
        last_point_processed += 1 + points.len();
        let g = 1;
    }

    // Valid so far, let's see if all # in original string are present in new map
    return true;
}


#[cfg(test)]
mod tests {
    // use crate::y2023::day12::function_to_test;

    use crate::y2023::day12::{ is_valid_spring_config, create_spring_points_from_string };

    #[test]
    fn spring_config_checks_correctly() {
        assert!( is_valid_spring_config( &( create_spring_points_from_string( &"#.#.###".to_string() ) ), &vec![ 1, 1, 3 ], "#??????".to_string() ) );
        assert_eq!( is_valid_spring_config( &( create_spring_points_from_string( &"#.#.###".to_string() ) ), &vec![ 1, 1, 3 ], "?#?????".to_string() ), false );
        assert!( is_valid_spring_config( &( create_spring_points_from_string( &"#.#.###.....".to_string() ) ), &vec![ 1, 1, 3 ], "????????????".to_string() ) );
        assert_eq!( is_valid_spring_config( &( create_spring_points_from_string( &".##.###".to_string() ) ), &vec![ 1, 1, 3 ], "???????".to_string() ), false );
    }
}