use std::path::Component::ParentDir;
use crate::grid_utils::grid_utils::{find_manhattan_distance, get_coords_at_index, get_index_at_coords, get_neighbor_indices, print_map, GridPoint, GridPrintable};
use crate::input_reader::input_reader::read_input_for_day;


#[derive(Clone,Debug)]
struct Square {
    has_rock: bool,
    x: usize,
    y: usize,
    index: usize,
    start_point: bool,
    visited: bool,
    checked_at_step: u64,
    distance_from_start: usize,
    manhattan_distance: usize,
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
        if self.has_rock {
            return "#".to_string();
        }
        if self.visited {
            return "V".to_string();
        }
        if self.start_point {
            return "S".to_string();
        }
        if ! self.has_rock && self.manhattan_distance <= 64 && self.manhattan_distance % 2 == 0 {
            return "0".to_string();
        }
        
        // if ! self.has_rock && self.manhattan_distance <= 64 && self.manhattan_distance % 2 == 1 {
        //     return "1".to_string();
        // }
        // if ! self.has_rock && self.manhattan_distance > 64 && self.manhattan_distance % 2 == 0 && self.y > 65 && self.x > 65 {
        //     return "b".to_string();
        // }
        // if ! self.has_rock && self.manhattan_distance > 64 && self.manhattan_distance % 2 == 1 && self.y > 65 && self.x > 65{
        //     return "d".to_string();
        // }
        // if ! self.has_rock && self.manhattan_distance > 64 && self.manhattan_distance % 2 == 1 && self.y <= 65 {
        //     return "t".to_string();
        // }
        //let mut a = self.manhattan_distance.to_string();
        //a.push(' ');
        //return a;
        //if self.manhattan_distance == 130 { return self.manhattan_distance.to_string(); } else { return ".".to_string(); }

        '.'.to_string()
    }
}

pub fn task() {
    let input = read_input_for_day(2023, 21, false );
    //run_task_1( input );
    run_task_1_alt( input );
}

fn get_valid_squares( squares: &Vec<Square>, even_parity: bool ) -> usize {
    let parity_check = match even_parity {
        true => 0,
        _ => 1,
    };
    squares.iter().fold( 0, |acc,square| {
        if square.index % 2 == parity_check && ! square.has_rock {
            return acc + 1;
        }
        acc
    } )
}

fn run_task_1_alt( input: String ) {
    let ( mut squares, grid_width, grid_height, starting_index ) = prepare_grid( input.clone() );
    let cs = squares.clone();
    let target_depth = f64::floor((grid_width / 2) as f64 ) as usize ;

    print_map( &squares, grid_width );
    let answer = squares.iter().fold( 0, | acc, sq | {
        if ! sq.has_rock && sq.manhattan_distance < target_depth && sq.manhattan_distance % 2 == 0 {
            return acc + 1;
        }
        acc
    });
    println!("Part 1 Answer: {}", answer);
    //return;
    // Mark neighbour distances
    for n in 0..target_depth * 4 {
        let mut squares_to_check: Vec<usize> = Vec::new();
        if n == 0 {
            squares_to_check.push( starting_index );
        } else {
            let indices: Vec<usize> = squares.iter().enumerate().filter(|(i, sq)| sq.distance_from_start == n ).map( | (i,_) | i ).collect();
            squares_to_check = [ squares_to_check, indices.clone() ].concat();
        }
        for stc in squares_to_check {
            let (left, right, up, down) = get_neighbor_indices( stc, grid_width );
            if left >= 0 && ! squares[left as usize].visited && ! squares[left as usize].has_rock {
                squares[left as usize].distance_from_start = n + 1;
                squares[left as usize].visited = true;
            }
            if right < squares.len() as isize && ! squares[right as usize].visited && ! squares[right as usize].has_rock {
                squares[right as usize].distance_from_start = n + 1;
                squares[right as usize].visited = true;
            }
            if up >= 0 && ! squares[up as usize].visited && ! squares[up as usize].has_rock {
                squares[up as usize].distance_from_start = n + 1;
                squares[up as usize].visited = true;
            }
            if down < squares.len() as isize && ! squares[down as usize].visited && ! squares[down as usize].has_rock {
                squares[down as usize].distance_from_start = n+1;
                squares[down as usize].visited = true;
            }
        }
    }
    print_map( &squares, grid_width );
    let answer = squares.iter().fold( 0, | acc, square | match square.distance_from_start % 2 == 0 && square.manhattan_distance <= 64 && square.visited {
        true => acc + 1,
        false => acc,
    } );
    println!( "Result of Part 1 is {}", answer );
    
    let visited_count = squares.iter().filter( |sq| sq.visited).count();
    println!("Visited count = {}", visited_count);

    let even_parity_full = squares.iter().enumerate().fold( 0, | acc, ( i , sq ) | {
        if i % 2 == 0 && sq.visited {
            return acc + 1;
        }
        acc
    } );
    let odd_parity_full = squares.iter().enumerate().fold( 0, | acc, ( i , sq ) | {
        if i % 2 == 1 && sq.visited {
            return acc + 1;
        }
        acc
    } );
    let even_corners = squares.iter().enumerate().fold( 0, | acc, ( i , sq ) | {
        if i % 2 == 0 && sq.visited && sq.distance_from_start > 65 {
            return acc + 1;
        }
        acc
    } );
    let odd_corners = squares.iter().enumerate().fold( 0, | acc, ( i , sq ) | {
        if i % 2 == 1 && sq.visited && sq.distance_from_start > 65 {
            return acc + 1;
        }
        acc
    } );
    let dte = grid_width/2;
    if dte != 65 { panic!("dte wrong, got {}", dte); }
    let n = ( 26501365 - dte ) / grid_width;
    if n != 202300 { panic!("n calc wrong, got {}", n); }

    let p2 = ((n+1)*(n+1)) * odd_parity_full + (n*n) * even_parity_full - (n+1) * odd_corners + n * even_corners;


    println!("Even parity: {}, odd parity: {}", even_parity_full, odd_parity_full);
    println!( "odd corners: {}, even corners: {}", odd_corners, even_corners);
    println!("Part 2 answer: {}", p2);
}

fn prepare_grid( input: String ) -> ( Vec<Square>, usize, usize, usize ) {
    let branches : Vec<usize> = Vec::new();
    let rows: Vec<String> = input.split("\n").map(|r| r.to_string() ).collect::<Vec<String>>();
    let grid_width = rows[0].len();
    let grid_height = rows.len();
    let mut squares: Vec<Square> = Vec::new();
    let mut square_counter = 0;
    let mut starting_index = 0;
    let mut y = 0;

    for row in rows {
        let mut x = 0;
        for sq in row.chars() {
            squares.push( Square {
                has_rock: sq == '#',
                x,
                index: square_counter,
                y,
                visited: sq == 'S',
                start_point: sq == 'S',
                checked_at_step: 0,
                distance_from_start: 0,
                manhattan_distance: 0,
            } );
            x += 1;
            if sq == 'S' { starting_index = square_counter }
            square_counter += 1;
        }
        y += 1;
    }
    let start_square = squares[starting_index].clone();
    squares.iter_mut().for_each(|sq| sq.manhattan_distance += find_manhattan_distance( start_square.clone(), sq.clone() ));
    ( squares, grid_width, grid_height, starting_index )
}

fn run_task_1( input: String ) {
    let ( mut squares, grid_width, grid_height, starting_index ) = prepare_grid( input.clone() );
    print_map( &squares, grid_width );
    let ff = get_coords_at_index( starting_index, grid_width );

    // Check step bottom left
    record_step( 0, starting_index, &mut squares, grid_width, starting_index );

    print_map( &squares, grid_width );

    let answer = squares.iter().fold( 0, | acc, square | {
        if square.visited && square.distance_from_start % 2 == 0 {
            return acc + 1;
        }
        return acc;
    } );
    println!("Part 1 answer: {}", answer);
}

fn record_step( depth: usize, index: usize, squares: &mut Vec< Square >, grid_width : usize, starting_index : usize ) {
    let target_depth = f64::floor((grid_width / 2) as f64 ) as usize + 1;
    if let Some( square ) = squares.clone().get( index ) {
        if square.has_rock {
            return;
        }
        let dist = find_manhattan_distance( squares[starting_index].clone(), square.clone() );
        if depth > target_depth {
            return;
        }

        squares[index].visited = true;
        squares[index].distance_from_start = depth;

        if square.get_x() > 0 {
            let left = get_index_at_coords(square.get_x() - 1, square.get_y(), grid_width);
            if ! squares[ left ].visited || squares[ left ].distance_from_start > depth {
                record_step(depth + 1, left, squares, grid_width, starting_index);
            }
        }

        if square.get_y() > 0 {
            let up = get_index_at_coords(square.get_x(), square.get_y() - 1, grid_width);
            if ! squares[ up ].visited || squares[ up ].distance_from_start > depth {
                record_step(depth + 1, up, squares, grid_width, starting_index);
            }
        }

        let right = get_index_at_coords(square.get_x() + 1, square.get_y(), grid_width);
        if ! squares[ right ].visited || squares[ right ].distance_from_start > depth {
            record_step(depth + 1, right, squares, grid_width, starting_index);
        }

        let down = get_index_at_coords(square.get_x(), square.get_y() + 1, grid_width);
        if ! squares[ down ].visited || squares[ down ].distance_from_start > depth {
            record_step(depth + 1, down, squares, grid_width, starting_index);
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        let ( squares, _, _, _ ) = prepare_grid(".........\
        .........\
        .........\
        .........\
        .........".to_string());
        assert_eq!(get_valid_squares( &squares, true ), 23);
        assert_eq!(get_valid_squares( &squares, false ), 22);
    }
}
