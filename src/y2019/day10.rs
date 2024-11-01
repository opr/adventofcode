use std::collections::{HashMap, HashSet};
use crate::input_reader::input_reader::read_input_for_day;
use crate::grid_utils::grid_utils::{find_angle, find_manhattan_distance, get_coords_at_index, GridPoint};

#[derive(Clone,Debug)]
struct Asteroid {
    x: usize,
    y: usize,
    angle: String,
}

impl GridPoint for Asteroid {
    fn get_x( &self ) -> usize {
        return self.x;
    }

    fn set_x(&mut self, new_value: usize) {
        self.x = new_value;
    }
    fn set_y(&mut self, new_value: usize) {
        self.y = new_value;
    }

    fn get_y( &self ) -> usize {
        return self.y;
    }
}
pub fn task() {
    let start = std::time::Instant::now();
    let mut input = read_input_for_day( 2019, 10, false );
    let lines: Vec<&str> = input.split("\n").collect();
    let grid_width = lines[0].len();

    let mut asteroids = input.replace("\n", "" ).chars().collect::<Vec<char>>().iter().enumerate().filter_map( |(index, point) | {
        if *point != '#' {
            return None;
        }
        let ( x, y ) = get_coords_at_index( index, grid_width );
        return Some( Asteroid{ x, y, angle: "".to_string() } );
    } ).collect::<Vec<Asteroid>>();
    let mut visible_asteroids = 0;

    for asteroid in &asteroids {
        let mut angle_set: HashSet<String> = HashSet::new();
        for mut other_asteroid in &asteroids {
            if other_asteroid.y == asteroid.y && other_asteroid.x == asteroid.x {
                continue;
            }
            let angle = find_angle( asteroid, other_asteroid );
            let r = 0;
            angle_set.insert( format!("{:.2}", angle ) );
            //other_asteroid.angle = format!("{:.2}", angle );
            if angle_set.len() > visible_asteroids {
                visible_asteroids = angle_set.len();
            }
        }
    }
    //print_grid( &asteroids, grid_width, lines.len() );

    let answer = 1;

    println!( "Day 10 task 1: {}", visible_asteroids );
    eprintln!("{:?}", start.elapsed() );
}

fn print_grid( asteroids: &Vec<Asteroid>, grid_width:usize, grid_height: usize ) {
    for y in 0..grid_height {
        for x in 0..grid_width {
            print!("\t");
            if let Some( asteroid_found ) = asteroids.iter().find( | asteroid | asteroid.x == x && asteroid.y == y ) {
                print!("{}\t", asteroid_found.angle);
                continue;
            }
            print!(".");
            print!("\t");
        }
        println!()
    }
}

#[cfg(test)]
mod tests {
    // use crate::y2019::day10::function_to_test;

    #[test]
    fn code_works_correctly() {

    }
}