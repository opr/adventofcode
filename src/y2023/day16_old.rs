use std::collections::{HashMap, HashSet};
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
    let mut input = read_input_for_day( 2023, 16, false );
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

    run_beam( &mut mirror_map, ( 0, 0 ), 'S', grid_width );
    print_map( &mirror_map, grid_width );

    let energized_tiles = mirror_map.iter().fold( 0, | acc, square | {
        if square.energized {
            return acc + 1;
        }
        return acc;
    } );

    let answer = energized_tiles;

    println!( "Day 16 task 1: {}", answer );
    eprintln!("{:?}", start.elapsed() );
}

fn run_beam( grid_squares: &mut Vec<GridSquare>, origin: ( usize, usize ), direction: char, grid_width: usize ) {
    let ( x, y ) = origin;
    let mut current_index = get_index_at_coords( x, y, grid_width );
    let last_indices_on_lines = ( 0..grid_squares.len() / grid_width ).map( | i | ( i * grid_width + grid_width ) - 1 ).collect::<Vec<usize>>();
    loop {
        // Energize current square and move on.
        grid_squares[ current_index ].energized = true;
        if ! grid_squares[ current_index ].energized_directions.insert( direction.clone() ) {
            return;
        }

        let mut next_char_index = current_index;
        match direction {
            'N' => {
                if current_index < grid_width {
                    println!("Beam went out of bounds.");
                    break;
                }
                next_char_index = current_index - grid_width;
            }
            'S' => {
                if current_index + grid_width >= grid_squares.len() {
                    println!("Beam went out of bounds.");
                    break;
                }
                next_char_index = current_index + grid_width;
            }
            'E' => {
                if last_indices_on_lines.contains( &( current_index ) ) {
                    println!("Beam went out of bounds.");
                    break;
                }
                next_char_index = current_index + 1;
            }
            'W' => {
                if current_index == 0 || last_indices_on_lines.contains( &( current_index - 1 ) ) {
                    println!("Beam went out of bounds.");
                    break;
                }
                next_char_index = current_index - 1;
            }
            _ => {}
        }
        current_index = next_char_index;

        let next_square = &grid_squares[ current_index ].clone();

        // Already visited, no point continuing.
        if next_square.energized && next_square.energized_directions.contains( &direction ) || next_square.energized_directions.len() >= 4 {
            break;
        }

        //print_map( &grid_squares, grid_width );
        let next_origin = ( 0, 0 );
        match next_square.mirror {
            MirrorType::None => {
                continue;
            },
            MirrorType::VSplitter => {
                match direction {
                    'W' | 'E' => {
                        run_beam( grid_squares, ( next_square.x, next_square.y ), 'N', grid_width );
                        run_beam( grid_squares, ( next_square.x, next_square.y ), 'S', grid_width );
                        break;
                    }
                    'N' | 'S' => {
                        continue;
                    }
                    _ => {}
                }
            }
            MirrorType::HSplitter => {
                match direction {
                    'N' | 'S' => {
                        run_beam( grid_squares, ( next_square.x, next_square.y ), 'E', grid_width );
                        run_beam( grid_squares, ( next_square.x, next_square.y ), 'W', grid_width );
                        break;
                    }
                    'E' | 'W' => {
                        continue;
                    }
                    _ => {}
                }
            }
            MirrorType::ForwardSlash => {
                match direction {
                    'E' => {
                        run_beam( grid_squares, ( next_square.x, next_square.y ), 'N', grid_width );
                        break;
                    }
                    'W' => {
                        run_beam( grid_squares, ( next_square.x, next_square.y ), 'S', grid_width );
                        break;
                    }
                    'S' => {
                        run_beam( grid_squares, ( next_square.x, next_square.y ), 'W', grid_width );
                        break;
                    }
                    'N' => {
                        run_beam( grid_squares, ( next_square.x, next_square.y ), 'E', grid_width );
                        break;
                    }
                    _ => {}
                }
            }
            MirrorType::Backslash => {
                match direction {
                    'E' => {
                        run_beam( grid_squares, ( next_square.x, next_square.y ), 'S', grid_width );
                        break;
                    }
                    'W' => {
                        run_beam( grid_squares, ( next_square.x, next_square.y ), 'N', grid_width );
                        break;
                    }
                    'S' => {
                        run_beam( grid_squares, ( next_square.x, next_square.y ), 'E', grid_width );
                        break;
                    }
                    'N' => {
                        run_beam( grid_squares, ( next_square.x, next_square.y ), 'W', grid_width );
                        break;
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}