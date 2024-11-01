use std::collections::{HashMap, HashSet, VecDeque};
use log::debug;
use crate::grid_utils::grid_utils::{get_index_at_coords, GridPoint, GridPrintable, print_map};
use crate::input_reader::input_reader::read_input_for_day;

#[derive(Clone,Debug)]
struct GridSquare {
    mirror: MirrorType,
    x: usize,
    y: usize,
    energized: bool,
    energized_directions: HashSet<char>
}

impl GridPrintable for GridSquare {
    fn get_print( &self ) -> String {

        if self.energized {
            return "#".to_string()
        }
        return '.'.to_string();

        return match self.mirror {
            MirrorType::HSplitter => {
                "-".to_string()
            }
            MirrorType::VSplitter => {
                "|".to_string()
            }
            MirrorType::Backslash => {
                "\\".to_string()
            }
            MirrorType::ForwardSlash => {
                "/".to_string()
            }
            MirrorType::None => {
                ".".to_string()
            }
        }
    }
}
impl GridPoint for GridSquare {
    fn get_x( &self ) -> usize {
        return self.x;
    }

    fn set_x(&mut self, new_value: usize) {
        self.x = new_value;
    }

    fn get_y( &self ) -> usize {
        return self.y;
    }

    fn set_y(&mut self, new_value: usize) {
        self.y = new_value;
    }
}
#[derive(Clone,Debug, Eq, PartialEq)]
enum MirrorType {
    HSplitter,
    VSplitter,
    Backslash,
    ForwardSlash,
    None,
}
#[derive(Clone,Debug)]
struct Mirror {
    mirror_type: MirrorType,
}



pub fn task() {
    let start = std::time::Instant::now();
    let test = false;
    let mut input = read_input_for_day( 2023, 16, test );
    let lines: Vec<&str> = input.split("\n").collect();
    let grid_width = lines[0].len();
    let grid_height = lines.len();

    let mut mirror_map = lines.iter().enumerate().flat_map( | ( y, line ) | {
        line.chars().enumerate().map( | ( x, square ) | {
            let mut mirror = MirrorType::None;
            match square {
                '/' => {
                    mirror = MirrorType::ForwardSlash;
                },
                '\\' => {
                    mirror = MirrorType::Backslash;
                }
                '-' => {
                    mirror = MirrorType::HSplitter;
                }
                '|' => {
                    mirror = MirrorType::VSplitter;
                }
                _ => {
                    MirrorType::None;
                }
            }
            GridSquare{ x, y, energized: false, mirror, energized_directions: HashSet::new() }
        } ).collect::<Vec<GridSquare>>()
    } ).collect::<Vec<GridSquare>>();

    let mut energized_tile_count = 0;
    let answer = 0;

    // Do the corners first
    run_beam( &mut mirror_map, ( 0, 0 ), 'E', grid_width );
    let count = count_tiles( &mirror_map );
    let answer = count;
    if count > energized_tile_count {
        energized_tile_count = count;
    }
    reset_grid( &mut mirror_map );

    run_beam( &mut mirror_map, ( 0, 0 ), 'S', grid_width );
    let count = count_tiles( &mirror_map );
    if count > energized_tile_count {
        energized_tile_count = count;
    }
    reset_grid( &mut mirror_map );

    run_beam( &mut mirror_map, ( grid_width - 1, 0 ), 'S', grid_width );
    let count = count_tiles( &mirror_map );
    if count > energized_tile_count {
        energized_tile_count = count;
    }
    reset_grid( &mut mirror_map );

    run_beam( &mut mirror_map, ( grid_width - 1, 0 ), 'W', grid_width );
    let count = count_tiles( &mirror_map );
    if count > energized_tile_count {
        energized_tile_count = count;
    }
    reset_grid( &mut mirror_map );

    run_beam( &mut mirror_map, ( grid_width - 1, grid_height - 1 ), 'N', grid_width );
    let count = count_tiles( &mirror_map );
    if count > energized_tile_count {
        energized_tile_count = count;
    }
    reset_grid( &mut mirror_map );

    run_beam( &mut mirror_map, ( grid_width - 1, grid_height - 1 ), 'W', grid_width );
    let count = count_tiles( &mirror_map );
    if count > energized_tile_count {
        energized_tile_count = count;
    }
    reset_grid( &mut mirror_map );

    run_beam( &mut mirror_map, ( 0, grid_height - 1 ), 'E', grid_width );
    let count = count_tiles( &mirror_map );
    if count > energized_tile_count {
        energized_tile_count = count;
    }
    reset_grid( &mut mirror_map );

    run_beam( &mut mirror_map, ( 0, grid_height - 1 ), 'N', grid_width );
    let count = count_tiles( &mirror_map );
    if count > energized_tile_count {
        energized_tile_count = count;
    }
    reset_grid( &mut mirror_map );


    for y in 1..grid_height {
        run_beam( &mut mirror_map, ( 0, y ), 'E', grid_width );
        let count = count_tiles( &mirror_map );
        if count > energized_tile_count {
            energized_tile_count = count;
        }
        reset_grid( &mut mirror_map );


        run_beam( &mut mirror_map, ( grid_width-1, y ), 'W', grid_width );
        let count = count_tiles( &mirror_map );
        if count > energized_tile_count {
            energized_tile_count = count;
        }
        reset_grid( &mut mirror_map );
    }

    for x in 1..grid_width {
        run_beam( &mut mirror_map, ( x, 0 ), 'S', grid_width );
        let count = count_tiles( &mirror_map );
        if count > energized_tile_count {
            energized_tile_count = count;
        }
        reset_grid( &mut mirror_map );


        run_beam( &mut mirror_map, ( x, grid_height - 1 ), 'N', grid_width );
        let count = count_tiles( &mirror_map );
        if count > energized_tile_count {
            energized_tile_count = count;
        }
        reset_grid( &mut mirror_map );
    }

    let answer2 = energized_tile_count;

    println!( "Day 16 task 1: {}", answer );
    println!( "Day 16 task 2: {}", answer2 );
    eprintln!("{:?}", start.elapsed() );
}

fn count_tiles(grid_squares: &Vec<GridSquare>) -> usize {
    grid_squares.iter().fold( 0, | acc, square | {
        if square.energized {
            return acc + 1;
        }
        return acc;
    } )
}

fn reset_grid( grid_squares: &mut Vec<GridSquare> ) {
    grid_squares.iter_mut().for_each( | square | {
        square.energized = false;
        square.energized_directions = HashSet::new();
    } );
}

struct BeamConfig {
    origin: (usize, usize),
    direction: char,
}
fn run_beam( grid_squares: &mut Vec<GridSquare>, origin: ( usize, usize ), direction: char, grid_width: usize ) {

    let mut current_square_index = get_index_at_coords(origin.0, origin.1, grid_width);

    loop {

        // Energize current square and set its energized direction, this will prevent loops
        grid_squares[current_square_index].energized = true;
        if !grid_squares[current_square_index].energized_directions.insert(direction) {
            // Square has already been visited from this direction
            return;
        }


        let cloned_grid_square = grid_squares[current_square_index].clone();
        let current_mirror_type = cloned_grid_square.mirror;

        match current_mirror_type {
            MirrorType::Backslash => {
                match direction {
                    'N' => {
                        // Next direction will be W
                        // Check if we're not exceeding grid width to the left
                        if cloned_grid_square.x != 0 {
                            run_beam( grid_squares, (cloned_grid_square.x - 1, cloned_grid_square.y), 'W', grid_width );
                        }
                        return;
                    }
                    'E' => {
                        // Next direction will be S
                        // Check if we're not exceeding grid height
                        if cloned_grid_square.y + 1 >= grid_squares.len() / grid_width {
                            return;
                        }
                        run_beam( grid_squares, (cloned_grid_square.x, cloned_grid_square.y + 1 ), 'S', grid_width );
                        return;
                    }
                    'S' => {
                        // Next direction will be E
                        // Check if we're not exceeding grid width to the right
                        if cloned_grid_square.x + 1 >= grid_width {
                            return;
                        }
                        run_beam( grid_squares, (cloned_grid_square.x + 1, cloned_grid_square.y), 'E', grid_width );
                        return;
                    }
                    'W' => {
                        // Next direction will be N
                        // Check if we're not exceeding grid height to the top
                        if cloned_grid_square.y == 0 {
                            return;
                        }
                        run_beam( grid_squares, (cloned_grid_square.x, cloned_grid_square.y - 1), 'N', grid_width );
                        return;
                    }
                    _ => {}
                }
            }
            MirrorType::ForwardSlash => {
                match direction {
                    'N' => {
                        // Next direction will be E
                        // Check if we're not exceeding grid width to the right
                        if cloned_grid_square.x + 1 >= grid_width {
                            return;
                        }
                        run_beam( grid_squares, (cloned_grid_square.x + 1, cloned_grid_square.y), 'E', grid_width );
                        return;
                    }
                    'E' => {
                        // Next direction will be N
                        // Check if we're not exceeding grid height to the top
                        if cloned_grid_square.y == 0 {
                            return;
                        }
                        run_beam( grid_squares, (cloned_grid_square.x, cloned_grid_square.y - 1), 'N', grid_width );
                        return;
                    }
                    'S' => {
                        // Next direction will be W
                        // Check if we're not exceeding grid width to the left
                        if cloned_grid_square.x == 0 {
                            return;
                        }
                        run_beam( grid_squares, (cloned_grid_square.x - 1, cloned_grid_square.y), 'W', grid_width );
                        return;
                    }
                    'W' => {
                        // Next direction will be S
                        // Check if we're not exceeding grid height
                        if cloned_grid_square.y + 1 >= grid_squares.len() / grid_width {
                            return;
                        }
                        run_beam( grid_squares, (cloned_grid_square.x, cloned_grid_square.y + 1), 'S', grid_width );
                        return;
                    }
                    _ => {}
                }
            },
            MirrorType::HSplitter => {
                match direction {
                    'N' | 'S' => {
                        if grid_squares[current_square_index].x != grid_width - 1 {
                            run_beam( grid_squares, (cloned_grid_square.x + 1, cloned_grid_square.y ), 'E', grid_width );
                        }
                        if grid_squares[current_square_index].x != 0 {
                            run_beam( grid_squares, ( cloned_grid_square.x - 1, cloned_grid_square.y ), 'W', grid_width );
                        }
                        return;
                    }
                    'E'  => {
                        if grid_squares[current_square_index].x == grid_width - 1 {
                            return;
                        }
                        current_square_index += 1;
                        continue;
                    }
                    'W'  => {
                        if grid_squares[current_square_index].x == 0 {
                            return;
                        }
                        current_square_index -= 1;
                        continue;
                    }
                    _ => {}
                }
            }
            MirrorType::VSplitter => {
                match direction {
                    'E' | 'W' => {
                        if grid_squares[current_square_index].y + 1 < grid_squares.len() / grid_width {
                            run_beam( grid_squares, (cloned_grid_square.x, cloned_grid_square.y + 1), 'S', grid_width );
                        }
                        if grid_squares[current_square_index].y > 0 {
                            run_beam( grid_squares, ( cloned_grid_square.x, cloned_grid_square.y - 1 ), 'N', grid_width );
                        }
                        return;
                    }
                    'S'  => {
                        if grid_squares[current_square_index].y + 1 < grid_squares.len() / grid_width {
                            current_square_index += grid_width;
                            continue;
                        }
                    }
                    'N'  => {
                        if grid_squares[current_square_index].y > 0 {
                            current_square_index -= grid_width;
                            continue;
                        }
                    }
                    _ => {}
                }
            }
            MirrorType::None => {
                match direction {
                    'N' => {
                        if grid_squares[current_square_index].y > 0 {
                            current_square_index -= grid_width;
                            continue;
                        }
                    }
                    'E'  => {
                        if grid_squares[current_square_index].x == grid_width - 1 {
                            return;
                        }
                        current_square_index += 1;
                        continue;
                    }
                    'W'  => {
                        if grid_squares[current_square_index].x == 0 {
                            return;
                        }
                        current_square_index -= 1;
                        continue;
                    }
                    'S'  => {
                        if grid_squares[current_square_index].y + 1 < grid_squares.len() / grid_width {
                            current_square_index += grid_width;
                            continue;
                        }
                    }
                    _ => {

                    }
                }
            }
            _ => {}
        }
    }
}