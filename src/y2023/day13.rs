use std::collections::{HashMap, HashSet};
use log::debug;
use crate::grid_utils::grid_utils::GridPoint;
use crate::input_reader::input_reader::read_input_for_day;

#[derive(Clone,Debug)]
struct MapPoint {
    x: usize,
    y: usize,
    has_rock: bool,
    original_coords: String,
}

impl GridPoint for MapPoint {
    fn get_x( &self ) -> usize {
        return self.x;
    }

    fn get_y( &self ) -> usize {
        return self.y;
    }

    fn set_x(&mut self, new_value: usize) {
        self.x = new_value;
    }
    fn set_y(&mut self, new_value: usize) {
        self.y = new_value;
    }
}


pub fn task() {
    let start = std::time::Instant::now();
    let mut input = read_input_for_day(2023, 13, false );
    let groups: Vec<&str> = input.split("\n\n").collect();

    let mut total = 0;

    for group in &groups {
        let lines: Vec<&str> = group.split("\n").collect();
        let grid_width = lines[0].len();
        let grid_height = lines.len();

        let mut map = lines.iter().enumerate().flat_map(|(y, line)| {
            line.chars().enumerate().map(|(x, char)| MapPoint { x, y, has_rock: char == '#', original_coords: format!("{},{}", x, y) }).collect::<Vec<MapPoint>>()
        }).collect::<Vec<MapPoint>>();

        let mut midpoint = grid_width / 2;

        // Scan from the left edge until the midpoint.
        if let Some( left_los ) = find_los( &map, midpoint, grid_height ) {
            total += left_los;
            continue;
        }

        // Flip the map over and repeat.
        mirror_map( &mut map, grid_width );

        // Scan from the left (really the right) edge until the midpoint.
        if let Some( right_los ) = find_los( &map, midpoint, grid_height ) {
            total += grid_width - right_los;
            continue;
        }

        rotate_map( &mut map, grid_height );

        // New midpoint since the map rotated and width changed.
        midpoint = grid_height / 2;

        // Scan from the left (really the top) edge until the midpoint.
        if let Some( left_los ) = find_los( &map, midpoint, grid_width ) {
            total += ( left_los * 100 );
            continue;
        }

        // Flip the rotated map.
        mirror_map( &mut map, grid_height );

        // Scan from the left (really the bottom) edge until the midpoint.
        if let Some( right_los ) = find_los( &map, midpoint, grid_width ) {
            total += ( grid_height - right_los ) * 100;
            continue;
        }
    }

    let answer = total;
    println!("Day 13 task 1: {}", answer);
    eprintln!("{:?}", start.elapsed());
}

fn find_los( map: &Vec<MapPoint>, midpoint: usize, grid_height: usize ) -> Option<usize> {

    // Check every line of symmetry from 1 until the midpoint.
    for v_line in 1..=midpoint {
        let mut rows_before: Vec<String> = vec![];
        let mut rows_after: Vec<String> = vec![];

        // Get the characters from each row into a Vec. E.g. if search width is 2, then each string will be 2 long. 2 from the left of the midpoint 2 from the right.
        for row in 0..grid_height {
            let chars_before: String = map.iter().filter_map( | point | {
                if point.x < v_line && point.y == row {
                    if point.has_rock {
                        return Some('#');
                    }
                    return Some('.');
                }
                return None;
            } ).into_iter().collect();
            rows_before.push( chars_before.clone() );

            let chars_after: String = map.iter().filter_map( | point | {
                if point.x >= v_line && point.x < v_line + v_line && point.y == row {
                    if point.has_rock {
                        return Some('#');
                    }
                    return Some('.');
                }
                return None;
            } ).into_iter().rev().collect();
            rows_after.push( chars_after.clone() );
        }

        if rows_equal( &rows_before, &rows_after ) {
            return Some( v_line );
        }
    }
    return None;
}

// Mirrors the map, puts elements on the right onto the left and vice/versa.
fn mirror_map( map: &mut Vec<MapPoint>, grid_width: usize ) {
    map.iter_mut().for_each( |mut m| m.x = m.x.abs_diff( grid_width - 1 ) );
    map.sort_by( | a, b | a.y.cmp( &b.y ).then_with(|| a.x.cmp(&b.x) ) );
}

// Rotates the map 90 degrees. Actually there is a bug here where it comes out rotated and flipped, but didn't have brainpower to figure out why, and it also doesn't matter.
fn rotate_map( map: &mut Vec<MapPoint>, new_grid_width: usize ) {

    map.iter_mut().for_each( |mut m| {
        let new_x = ( m.y + new_grid_width ) % new_grid_width;//.abs_diff( new_grid_width );
        let new_y = m.x;
        m.y = new_y;
        m.x = new_x;
    } );
    map.sort_by( | a, b | a.y.cmp( &b.y ).then_with(|| a.x.cmp(&b.x) ) );
}

fn rows_equal( left: &Vec<String>, right: &Vec<String> ) -> bool {
    let mut diff = 0;
    for i in 0..left.len() {
        let left_chars = &left[i].chars().collect::<Vec<char>>();
        let right_chars = &right[i].chars().collect::<Vec<char>>();

        for index in 0..left_chars.len() {
            if left_chars[index] == right_chars[index] {
                continue;
            }
            diff += 1;
        }
    }

    // Change to 0 for part 1, or use simpler .all below.
    return diff == 1;
    //left.iter().enumerate().all( | ( index, row ) | *row == right[index] )
}

fn print_map(map: &Vec<MapPoint>, grid_width: usize ) {
    for i in 0..map.len() {
        if i % grid_width == 0 {
            println!()
        }
        if map[i].has_rock == true {
            print!("#");
            continue;
        }

        print!(".");
    }
    println!();
}