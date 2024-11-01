use std::collections::{HashMap, HashSet};
use crate::input_reader::input_reader::read_input_for_day;

#[derive(Clone,Debug)]
struct Galaxy {
    x: usize,
    y: usize,
    id: usize,
}

pub fn task2() {
    let start = std::time::Instant::now();
    let mut input = read_input_for_day( 2023, 11, false );
    let unexpanded_grid_width = input.split_once( "\n" ).unwrap().0.len();
    let mut lines: Vec<String> = input.split("\n").map( | l | l.to_string() ).collect();

    let expand_by = 1000000;
    let ( row_ranges, col_ranges ) = find_expansion_ranges( input, unexpanded_grid_width );

    let galaxies = lines.join("").chars().enumerate().filter_map( | ( index, point ) | {
        if point != '#' {
            return None
        }
        let ( x,y ) = get_galaxy_coords_at_index( index, unexpanded_grid_width );
        let expansions_left = col_ranges.iter().filter( | r | r < &&x ).collect::<Vec<&usize>>();
        let expansions_down = row_ranges.iter().filter( | c | c < &&y ).collect::<Vec<&usize>>();
        let b = 0;
        Some( Galaxy{ id: index, x: x + ( ( expand_by - 1 ) * expansions_left.len() ), y: y + ( ( expand_by - 1 ) * expansions_down.len() ) } )
    } ).collect::<Vec<Galaxy>>();

    let mut distances = 0;
    for galaxy in &galaxies {
        for next_galaxy in &galaxies {
            if galaxy.id >= next_galaxy.id {
                continue;
            }

            // Manhattan distance gets the shortest distance.
            distances += find_manhattan_distance( &galaxy, &next_galaxy );
        }
    }

    let b = 1;
    println!( "Day 11 task 2: {}", distances );
    eprintln!("{:?}", start.elapsed() );
}

fn find_expansion_ranges( input: String, grid_width: usize ) -> ( Vec<usize>, Vec<usize> ) {
    let mut lines: Vec<String> = input.split("\n").map( | l | l.to_string() ).collect();

    let mut row_ranges = vec![];
    let mut col_ranges = vec![];

    let rows_without_galaxies = lines.iter().enumerate().filter_map( | ( index, row ) | {
        let chars = row.chars().collect::<Vec<char>>();
        if chars.contains( &'#' ) {
            return None;
        }
        Some( index )
    } ).collect::<Vec<usize>>();

    let mut row_ranges_added = 0;
    for row in rows_without_galaxies {
        let start = row;
        row_ranges.push( start );
        row_ranges_added += 1;
    }

    let cols_without_galaxies = (0..grid_width).filter_map( | col | {
        let input_string = input.replace( "\n", "" ).chars().collect::<Vec<char>>();

        for y in 0..lines.len() {
            let index = get_galaxy_index_at_coords( col, y, grid_width );
            if index > input_string.len() {
                continue;
            }
            if input_string[ index ] == '#' {
                return None;
            }
        }
        return Some( col );
    } ).collect::<Vec<usize>>();

    let mut col_ranges_added = 0;
    for col in cols_without_galaxies {
        let start = col + col_ranges_added;
        col_ranges.push( col );
        col_ranges_added += 1;
    }
    return ( row_ranges, col_ranges );
}

fn find_manhattan_distance( g1: &Galaxy, g2: &Galaxy ) -> usize {
    return g1.y.abs_diff( g2.y ) + g1.x.abs_diff( g2.x );
}
fn get_galaxy_index_at_coords(x: usize, y: usize, grid_width: usize ) -> usize {
    return y * grid_width + x;
}
fn get_galaxy_coords_at_index( index: usize, grid_width: usize ) -> ( usize, usize ) {
    let y = index / grid_width;
    let x = index % grid_width;
    ( x, y )
}

fn print_space_map( lines: &Vec<String>, galaxies: &Vec<Galaxy> ) {
    lines.iter().for_each( | line | println!( "{line}" ) );
    println!()
}

#[cfg(test)]
mod tests {
    use crate::y2023::day11::get_galaxy_coords_at_index;

    #[test]
    fn get_galaxy_coords_at_index_works_correctly() {
        assert_eq!( get_galaxy_coords_at_index( 2, 5 ), ( 2, 0 ) );
        assert_eq!( get_galaxy_coords_at_index( 0, 5 ), ( 0, 0 ) );
        assert_eq!( get_galaxy_coords_at_index( 5, 5 ), ( 0, 1 ) );
        assert_eq!( get_galaxy_coords_at_index( 11, 5 ), ( 1, 2 ) );
        assert_eq!( get_galaxy_coords_at_index( 11, 5 ), ( 1, 2 ) );
    }
}