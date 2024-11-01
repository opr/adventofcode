use std::collections::{HashMap, HashSet};
use std::ffi::c_long;
use std::fmt::format;
use std::iter::Map;
use log::debug;
use crate::grid_utils::grid_utils::{get_index_at_coords, GridPoint};
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
}

pub fn task() {
    let start = std::time::Instant::now();
    let mut input = read_input_for_day( 2023, 13, false );
    let groups: Vec<&str> = input.split("\n\n").collect();

    let mut total = 0;

    let bp = 1;
    for group in &groups {
        let lines: Vec<&str> = group.split("\n").collect();
        let grid_width = lines[0].len();

        let map = lines.iter().enumerate().flat_map( | ( y, line ) | {
            line.chars().enumerate().map( | ( x, char ) | MapPoint{ x, y, has_rock: char == '#', original_coords: format!("{},{}", x,y) } ).collect::<Vec<MapPoint>>()
        } ).collect::<Vec<MapPoint>>();

        print_map( &map, grid_width );

        let v_los_found = find_los( &map, grid_width );

        if v_los_found != 0 {
            total += v_los_found + 1;
        }
        if v_los_found == 0 || true {
            let mut rotated_map: Vec<MapPoint> = vec![];
            let b = rotate_90_degrees( &map, lines.len(), grid_width );
            // for m in 0..map.len() {
            //     println!("Moving {},{} to {},{} it has a rock {}",
            //              map[m].x,
            //              map[m].y,
            //              lines.len() - 1 - map[m].y,
            //              map[m].x,
            //              map[m].has_rock );
            //     rotated_map.push( MapPoint{
            //         x: lines.len() - 1 - map[m].y,
            //         y: map[m].x,
            //         has_rock: map[m].has_rock
            //     } );
            // }

            debug!("");
            // let mut sorted_map: Vec<MapPoint> = vec![];
            // for x in 0..grid_width {
            //     for y in 0..lines.len() {
            //         debug!("");
            //         sorted_map.push( b.iter().find( | m | m.x == y && m.y == x ).unwrap().clone() )
            //     }
            // }

            print_map( &b, lines.len() );
            let h_los_found = find_los(&b, lines.len() );
            if h_los_found == 0 {
                continue;
                panic!("no los found in either direction!");
            }
            println!("Actual horizontal los was {}", h_los_found);
            total += (h_los_found + 1) * 100;
            debug!("");
        }
        debug!("");
    }

    let answer = total;

    println!( "Day 13 task 1: {}", answer );
    eprintln!("{:?}", start.elapsed() );
}


/*
def rotate_grid_90_clockwise(grid, width, height):
    rotated_grid = [0] * (width * height)

    for x in range(width):
        for y in range(height):
            # Calculate the new index in the rotated grid
            new_x = y
            new_y = width - x - 1
            new_index = new_y * height + new_x

            # Copy the value to the rotated grid
            rotated_grid[new_index] = grid[y * width + x]

    return rotated_grid

 */
fn rotate_90_degrees(grid: &Vec<MapPoint>, rows: usize, cols: usize) -> Vec<MapPoint> {
    // Transpose the grid and swap element positions
    let mut rotated_grid = vec![grid[0].clone(); rows * cols];
    for x in 0..cols {
        for y in 0..rows {
            let new_x = y;
            let new_y = cols - x - 1;
            //println!("{},{} is moving into {},{}", x,y,new_x,new_y);
            rotated_grid[get_index_at_coords(new_x, new_y, rows)] = grid[get_index_at_coords(x, y, cols)].clone();
            rotated_grid[get_index_at_coords(new_x, new_y, rows)].x  = new_x;
            rotated_grid[get_index_at_coords(new_x, new_y, rows)].y  = new_y;
        }
    }

    rotated_grid
}

fn find_los( map: &Vec<MapPoint>, grid_width: usize ) -> usize {

    let h_midpoint = grid_width / 2;

    let mut v_largest_group = 0;
    let mut v_los = 0;
    // Check vertical symmetry
    for los in 0..grid_width - 1 {
        let is_before = los <= h_midpoint - 1;
        let mut search_width = ( grid_width - los - 1 );
        if is_before {
            search_width = 1 + los;
        }

        if ( los + 1 ) - search_width != 0 && ( los + 1 ) + search_width != grid_width {
            continue;
        }

        let before = ( ( los + 1 ) - search_width..=los ).collect::<Vec<usize>>();
        let mut after = ( ( los + 1 )..=( los + search_width ) ).collect::<Vec<usize>>();
        after.reverse();


        if is_before && before[0] != 0 {
            continue;
        }

        if ! is_before && after[0] != grid_width - 1 {
            continue;
        }

        //println!("Checking line of symmetry between {} and {}, is it before mid {}. Search width is {} (on either side of line) {}/{}", los, los+1, is_before, search_width, before.iter().map(|b| b.to_string() ).collect::<Vec<String>>().join(""), &after.iter().map(| a| a.to_string() ).collect::<Vec<String>>().join("") );

        let mut comparisons = 0;
        loop {
            if comparisons == before.len() - 1 {
                break;
            }

            let rocks_before = map.iter().filter( | point | point.x == before[comparisons] ).collect::<Vec<&MapPoint>>();
            let rocks_after = map.iter().filter( | point | point.x == after[comparisons] ).collect::<Vec<&MapPoint>>();

            // all rocks before match all rocks after
            let all_rocks_valid = rocks_before.iter().enumerate().all( | ( i, p ) | {
                p.has_rock == rocks_after[i].has_rock
            } );

            if ! all_rocks_valid {
                break;
            }

            // println!("Checking the rocks:");
            // println!("before: {}", rocks_before.iter().map( | r | { if r.has_rock { return "#".to_string() } return ".".to_string() } ).collect::<Vec<String>>().join("" ) );
            // println!("after:  {}", rocks_after.iter().map( | r | { if r.has_rock { return "#".to_string() } return ".".to_string() } ).collect::<Vec<String>>().join("" ) );

            if los > v_los {
                v_largest_group = before.len();
                v_los = los;
            }
            let g = 0;
            comparisons += 1;
        }
        let n = 0;
    }

    if v_los != 0 {
        println!("Vertical symmetry found at {}", v_los + 1 );
       // total += v_los;
    }

    if v_los == 0 {
        println!("No vertical found, need to rotate and retry");
    }

    // let mut rotated_map: Vec<MapPoint> = vec![];
    // for m in 0..map.len() {
    //     //println!("Moving {},{} to {},{}", map[m].x, map[m].y, ( lines.len() - 1 ) - map[m].y, grid_width - 1 - map[m].x );
    //     rotated_map.push( MapPoint{ x: ( lines.len() - 1 ) - map[m].y, y: grid_width - 1 - map[m].x, has_rock: map[m].has_rock } );
    // }
    return v_los;
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

#[cfg(test)]
mod tests {
    // use crate::y2023::day13::function_to_test;

    #[test]
    fn code_works_correctly() {

    }
}