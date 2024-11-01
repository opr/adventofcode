use crate::input_reader::input_reader::read_input_for_day;

#[derive(Clone,Debug)]
struct Pipe {
    pipe_type: char,
    x: usize,
    y: usize,
    distance_from_start: usize,
    visited: bool,
    inside: bool,
    is_junk: bool,
}

impl Pipe {
    fn set_visited( &mut self ) {
        self.visited = true;
    }

    fn set_inside( &mut self ) {
        self.inside = true;
    }
    fn set_distance_from_start( &mut self, distance: usize ) {
        self.distance_from_start = distance;
    }
}

impl Default for Pipe {
    fn default() -> Self {
        Pipe {
            pipe_type: '.',
            x: 0,
            y: 0,
            distance_from_start: 0,
            visited: false,
            inside: false,
            is_junk: false,
        }
    }
}

pub fn task() {
    let start = std::time::Instant::now();
    let mut input = read_input_for_day( 2023, 10, false );
    let lines: Vec<&str> = input.split("\n").collect();

    let grid_width = lines[0].len();

    let mut pipe_map = lines.iter().enumerate().flat_map( | ( y, line ) | {
        line.chars().enumerate().map( | ( x, pipe ) | {
            Pipe { x, y, pipe_type: pipe, ..Default::default() }
        } ).collect::<Vec<Pipe>>()
    } ).collect::<Vec<Pipe>>();

    let pipe_types = "F|-LJ7";

    let start_pipe = pipe_map.iter().find( | pipe | pipe.pipe_type == 'S' ).unwrap();
    let start_index = get_pipe_index_at_coords( start_pipe.x, start_pipe.y, grid_width );

    let mut starting_pipe_type = ' ';

    // Calculate first pipe type by looking at left and right
    for pipe_type in pipe_types.chars() {
        match pipe_type {
            'F' => {
                // Cannot have this pipe type on the right or bottom rows.
                if pipe_map[ start_index ].x == grid_width - 1 || pipe_map[ start_index ].y == lines.len() - 1 {
                    continue;
                }
                // Only -, 7, and J can connect to this from right.
                // Only |, L, and J can connect to this from below.
                let valid_pipes_right = "-7J";
                let valid_pipes_below = "|LJ";
                let ( pipe_right_index, pipe_below_index ) = ( get_pipe_index_at_coords( pipe_map[ start_index ].x + 1, pipe_map[ start_index ].y, grid_width ), get_pipe_index_at_coords( pipe_map[ start_index ].x, pipe_map[ start_index ].y+1, grid_width ) );

                let is_valid = neighbour_pipes_contain( pipe_right_index, pipe_below_index, valid_pipes_right, valid_pipes_below, &pipe_map );
                if is_valid {
                    starting_pipe_type = pipe_type;
                    break;
                }
            }
            '|' => {
                // Cannot have this pipe type on the top or bottom rows.
                if pipe_map[ start_index ].y == 0 || pipe_map[ start_index ].y == lines.len() - 1 {
                    continue;
                }

                // Only |, F, and 7 can connect to this from above.
                // Only |, L, and J can connect to this from below.
                let valid_pipes_right = "|F7";
                let valid_pipes_below = "|LJ";
                let ( pipe_above_index, pipe_below_index ) = ( get_pipe_index_at_coords( pipe_map[ start_index ].x, pipe_map[ start_index ].y - 1, grid_width ), get_pipe_index_at_coords( pipe_map[ start_index ].x, pipe_map[ start_index ].y + 1, grid_width ) );

                let is_valid = neighbour_pipes_contain( pipe_above_index, pipe_below_index, valid_pipes_right, valid_pipes_below, &pipe_map );
                if is_valid {
                    starting_pipe_type = pipe_type;
                    break;
                }
            }
            '-' => {
                // Cannot have this pipe type on the left or right columns.
                if pipe_map[ start_index ].x == 0 || pipe_map[ start_index ].x == grid_width - 1 {
                    continue;
                }

                // Only 7, J, and - can connect to this from right.
                // Only F, L, and - can connect to this from left.
                let valid_pipes_right = "7J-";
                let valid_pipes_left = "FL-";
                let (pipe_left_index, pipe_right_index) = (get_pipe_index_at_coords(pipe_map[ start_index ].x - 1, pipe_map[ start_index ].y, grid_width ), get_pipe_index_at_coords(pipe_map[ start_index ].x + 1, pipe_map[ start_index ].y, grid_width ) );

                let is_valid = neighbour_pipes_contain(pipe_left_index, pipe_right_index, valid_pipes_left, valid_pipes_right, &pipe_map );
                if is_valid {
                    starting_pipe_type = pipe_type;
                    break;
                }
            }
            'L' => {
                // Cannot have this pipe type on the top or right columns.
                if pipe_map[ start_index ].y == 0 || pipe_map[ start_index ].x == grid_width - 1 {
                    continue;
                }

                // Only 7, J, and - can connect to this from right.
                // Only F, 7, and | can connect to this from above.
                let valid_pipes_right = "7J-";
                let valid_pipes_above = "F|7";
                let (pipe_above_index, pipe_right_index) = (get_pipe_index_at_coords(pipe_map[ start_index ].x, pipe_map[ start_index ].y - 1, grid_width ), get_pipe_index_at_coords(pipe_map[ start_index ].x + 1, pipe_map[ start_index ].y, grid_width ) );

                let is_valid = neighbour_pipes_contain(pipe_above_index, pipe_right_index, valid_pipes_above, valid_pipes_right, &pipe_map );
                if is_valid {
                    starting_pipe_type = pipe_type;
                    break;
                }
            }
            'J' => {
                // Cannot have this pipe type on the top or left columns.
                if pipe_map[ start_index ].y == 0 || pipe_map[ start_index ].x == 0 {
                    continue;
                }

                // Only F, L, and - can connect to this from left.
                // Only F, 7, and | can connect to this from above.
                let valid_pipes_left = "7L-";
                let valid_pipes_above = "F|7";
                let ( pipe_above_index, pipe_left_index ) = (get_pipe_index_at_coords(pipe_map[ start_index ].x, pipe_map[ start_index ].y - 1, grid_width ), get_pipe_index_at_coords(pipe_map[ start_index ].x - 1, pipe_map[ start_index ].y, grid_width ) );

                let is_valid = neighbour_pipes_contain(pipe_left_index, pipe_above_index, valid_pipes_left, valid_pipes_above, &pipe_map );
                if is_valid {
                    starting_pipe_type = pipe_type;
                    break;
                }
            }
            '7' => {
                // Cannot have this pipe type on the bottom or left columns.
                if pipe_map[ start_index ].y == lines.len() - 1 || pipe_map[ start_index ].x == 0 {
                    break;
                }

                // Only F, L, and - can connect to this from left.
                // Only L, J, and | can connect to this from below.
                let valid_pipes_left = "FL-";
                let valid_pipes_below = "LJ|";
                let (pipe_left_index, pipe_below_index) = (get_pipe_index_at_coords(pipe_map[ start_index ].x - 1, pipe_map[ start_index ].y, grid_width ), get_pipe_index_at_coords(pipe_map[ start_index ].x, pipe_map[ start_index ].y + 1, grid_width ) );

                let is_valid = neighbour_pipes_contain(pipe_below_index, pipe_left_index, valid_pipes_below, valid_pipes_left, &pipe_map );
                if is_valid {
                    starting_pipe_type = pipe_type;
                    break;
                }
            }
            _ => {
                panic!( "Added a pipe type without handling it." );
            }
        }
    }

    // Update pipe map to set starting pipe's type.
    pipe_map[start_index].pipe_type = starting_pipe_type;

    // Visit each pipe in the pipeline recursively. The depth will increment for each pipe visited.
    // Each pipe will have a "distance_from_start" property, but that is actually steps taken from the beginning following the path.
    // The last pipe before the start will have a "distance_from_start" equal to the length of the loop. It doesn't matter though, we only need to know the midpoint.
    let depth = visit_connected_pipes( start_index, &mut pipe_map, grid_width, start_index, 0 );
    let midpoint = ( depth + 1 ) / 2;

    // Remove junk pipes.
    pipe_map.iter_mut().for_each( | p | {
        if ! p.visited && pipe_types.contains( p.pipe_type ) {
            p.pipe_type = '.';
            p.is_junk = true;
        }
    } );

    // For each . type see if it passed an odd number of pipes
    let cloned_pipe_map = pipe_map.clone();

    // Grab rows of pipes using chunks.
    for pipe_chunk in cloned_pipe_map.chunks( grid_width ) {
        if pipe_chunk[0].y == lines.len() - 1 || pipe_chunk[0].y == 0 {
            continue;
        }

        let ground_points: Vec<&Pipe> = pipe_chunk.iter().filter( | p | p.pipe_type == '.' ).collect();
        for i in 0.. ground_points.len() {
            let mut ground_points_clone = &mut vec![];
            ground_points.clone_into(ground_points_clone);
            let mut pipe = &mut ground_points_clone[ i ];
            if pipe.pipe_type != '.' {
                continue;
            }

            // Find only pipes above this point.
            let pipes_above = cloned_pipe_map
                .iter()
                .filter( | x_pipe | x_pipe.x == pipe.x )
                .take_while( | x_pipe | x_pipe.y != pipe.y )
                .collect::<Vec<&Pipe>>();

            let mut fs = 0isize;
            let mut ls = 0isize;
            let mut js = 0isize;
            let mut sevens = 0isize;
            let mut crossings = 0isize;

            for pipe_above in &pipes_above {

                match pipe_above.pipe_type {
                    '7' => {
                        sevens+=1;
                    }
                    'L' => {
                        ls += 1;
                    }
                    'J' => {
                        js += 1;
                    }
                    'F' => {
                        fs += 1;
                    }
                    '|' => {

                    }
                    '-' => {
                        crossings += 1;
                    }
                    '.' => {

                    }
                    _ => {

                    }
                }
            }

            // If the number of Js crossed does not match the 7s crossed or the Fs crossed doesn't match Ls crossed you made a crossing.
            if fs.abs_diff( ls ) %2 != 0 || js.abs_diff( sevens ) %2 != 0 {
                crossings += 1;
            }

            if crossings %2 != 0 {
                let index = get_pipe_index_at_coords( pipe.x, pipe.y, grid_width );
                pipe_map[ index ].set_inside();
                continue;
            }
        }
    }

    let areas_inside = pipe_map.iter().fold( 0usize, | acc, pipe | {
        if pipe.inside {
            return acc + 1;
        }
        acc
    } );

    // print_pipeline( &pipe_map, grid_width );
    // println!();

    println!( "Day 10 task 1: {}", midpoint );
    println!( "Day 10 task 2: {}", areas_inside );
    eprintln!("{:?}", start.elapsed() );
}

// For debugging the map, showing distances from start.
fn print_depths( pipes: Vec<Pipe>, grid_width: usize ) {
    for i in 0..pipes.len() {
        if i % grid_width == 0 {
            println!()
        }

        if pipes[i].pipe_type == '.' {
            print!(".");
            continue;
        }
        print!("{}", pipes[i].distance_from_start);
    }
    println!();
}

// For debugging the map, showing the pipeline
fn print_pipeline( pipes: &Vec<Pipe>, grid_width: usize ) {
    for i in 0..pipes.len() {
        if i % grid_width == 0 {
            println!()
        }
        if pipes[i].pipe_type == '.' && pipes[i].inside {
            print!("I");
            continue;
        }

        if pipes[i].pipe_type == '.' && ! pipes[i].inside {
            print!("O");
            continue;
        }

        print!("{}", pipes[i].pipe_type);
    }
    println!();
}

fn visit_connected_pipes( pipe_index: usize, pipes: &mut Vec<Pipe>, grid_width: usize, starting_pipe_index: usize, mut depth: usize ) -> usize {
    let mut pipe = &mut pipes[ pipe_index ];

    if pipe.visited {
        // Decrement depth because this was an invalid step.
        return depth - 1;
    }

    // Set this pipe as visited and update its distance from start. That is how deep we are in the loop.
    pipe.set_visited();
    pipe.set_distance_from_start( depth );

    let mut next_pipe_1 = 0;
    let mut next_pipe_2 = 0;
    match pipe.pipe_type {
        'F' => {
            next_pipe_1 = get_pipe_index_at_coords( pipe.x + 1, pipe.y, grid_width );
            next_pipe_2 = get_pipe_index_at_coords( pipe.x, pipe.y + 1, grid_width );
        }
        '|' => {
            next_pipe_1 = get_pipe_index_at_coords( pipe.x, pipe.y - 1, grid_width );
            next_pipe_2 = get_pipe_index_at_coords( pipe.x, pipe.y + 1, grid_width );
        }
        '-' => {
            next_pipe_1 = get_pipe_index_at_coords( pipe.x - 1, pipe.y, grid_width );
            next_pipe_2 = get_pipe_index_at_coords( pipe.x + 1, pipe.y, grid_width );
        }
        'L' => {
            next_pipe_1 = get_pipe_index_at_coords( pipe.x, pipe.y - 1, grid_width );
            next_pipe_2 = get_pipe_index_at_coords( pipe.x + 1, pipe.y, grid_width );
        }
        'J' => {
            next_pipe_1 = get_pipe_index_at_coords( pipe.x, pipe.y - 1, grid_width );
            next_pipe_2 = get_pipe_index_at_coords( pipe.x - 1, pipe.y, grid_width );
        }
        '7' => {
            next_pipe_1 = get_pipe_index_at_coords( pipe.x - 1, pipe.y, grid_width );
            next_pipe_2 = get_pipe_index_at_coords( pipe.x, pipe.y + 1, grid_width );
        }
        _ => {}
    }

    // Because we don't keep track of the previous pipe we have to visit the "next" and "previous" pipes.
    // The function returns if the pipe is already visited though.
    depth = visit_connected_pipes( next_pipe_1, pipes, grid_width, starting_pipe_index, depth + 1 );
    depth = visit_connected_pipes( next_pipe_2, pipes, grid_width, starting_pipe_index, depth + 1 );
    return depth;
}

// Abstraction to check if the neighbouring pipe indexes contain any of the specified characters.
fn neighbour_pipes_contain( pipe_1: usize, pipe_2: usize, valid_pipes_1: &str, valid_pipes_2: &str, pipes: &Vec<Pipe> ) -> bool {
    return valid_pipes_1.contains( pipes[ pipe_1 ].pipe_type ) && valid_pipes_2.contains( pipes[ pipe_2 ].pipe_type );
}

// Makes reasoning with X/Y coords easier.
fn get_pipe_index_at_coords(x: usize, y: usize, grid_width: usize ) -> usize {
    return y * grid_width + x;
}

#[cfg(test)]
mod tests {
    use std::ptr;
    use crate::y2023::day10::{get_pipe_index_at_coords, Pipe};

    #[test]
    fn get_pipe_at_returns_correct_pipe() {
        let mut test_pipes: Vec<Pipe> = vec![];
        for y in 0..10 {
            for x in 0..10 {
                test_pipes.push( Pipe{ x, y, pipe_type: 'F', ..Default::default() } )
            }
        }

        let maybe_found_pipe = get_pipe_index_at_coords( 5, 2, test_pipes.len() );
        assert_eq!( maybe_found_pipe, 25 );
    }
}