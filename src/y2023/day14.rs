use std::collections::{HashMap, HashSet};
use log::debug;
use crate::grid_utils::grid_utils::{get_index_at_coords, GridPoint, GridPrintable, print_map};
use crate::input_reader::input_reader::read_input_for_day;

#[derive(Clone,Debug)]
struct Square {
    has_rock: bool,
    x: usize,
    y: usize,
    is_round: bool,
}

impl GridPoint for Square {
    fn get_x( &self ) -> usize {
        return self.x;
    }

    fn set_x(&mut self, new_value: usize) {
        todo!()
    }

    fn get_y( &self ) -> usize {
        return self.y;
    }

    fn set_y(&mut self, new_value: usize) {
        todo!()
    }
}

impl GridPrintable for Square {
    fn get_print(&self) -> String {
        if self.is_round {
            return "O".to_string();
        }
        if self.has_rock {
            return "#".to_string();
        }
        return ".".to_string();
    }
}

pub fn task() {
    let start = std::time::Instant::now();
    let mut input = read_input_for_day( 2023, 14, false );
    let lines: Vec<&str> = input.split("\n").collect();
    let grid_width = lines[0].len();
    let grid_height = lines.len();

    let mut map = lines.iter().enumerate().flat_map( | ( y, line ) | {
        line.chars().collect::<Vec<char>>().iter().enumerate().map( | ( x, space ) | Square{ x: x, y: y, has_rock: *space == 'O' || *space == '#', is_round: *space == 'O' } ).collect::<Vec<Square>>()
    } ).collect::<Vec<Square>>();

    let mut rocks_moved = 1;

    loop {
        if rocks_moved == 0 {
            break;
        }
        let mut shift_count = 0;

        // Move all rocks up
        for i in 0..map.len() {
            if rock_can_move_up( &map, map[i].x, map[i].y, grid_width ) {
                shift_count += 1;
                map[i].has_rock = false;
                map[i].is_round = false;
                map[i-grid_width].has_rock = true;
                map[i-grid_width].is_round = true;
            }
        }
        rocks_moved = shift_count;
    }

    let mut total_force = 0;
    for y in 0..grid_height {
        let rock_count = map.iter().filter( | r | r.is_round && r.y == y ).collect::<Vec<&Square>>().len();
        total_force += ( grid_height - y ) * rock_count;
    }

    let answer = total_force;

    println!( "Day 14 task 1: {}", answer );
    eprintln!("{:?}", start.elapsed() );
}

fn rock_can_move_up( map: &Vec<Square>, x: usize, y: usize, grid_width: usize ) -> bool {
    if y == 0 {
        return false;
    }
    let rock = &map[ get_index_at_coords( x, y, grid_width ) ];
    if ! rock.is_round || ! rock.has_rock {
        return false;
    }

    return ! &map[ get_index_at_coords( x, y-1, grid_width ) ].has_rock;
}