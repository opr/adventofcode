use crate::grid_utils::grid_utils::{
    get_coords_at_index, get_index_at_coords, print_map, GridPoint, GridPrintable,
};
use crate::input_reader::input_reader::read_input_for_day;
use log::debug;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug)]
struct GridSquare {
    x: usize,
    y: usize,
    is_dug: bool,
    is_full: bool,
    is_corner: bool,
    corner_type: char,
    edge_type: char,
    is_part_of_trench: bool,
}
impl GridPoint for GridSquare {
    fn get_x(&self) -> usize {
        return self.x;
    }

    fn set_x(&mut self, new_value: usize) {
        self.x = new_value;
    }

    fn get_y(&self) -> usize {
        return self.y;
    }

    fn set_y(&mut self, new_value: usize) {
        self.y = new_value;
    }
}

impl GridPrintable for GridSquare {
    fn get_print(&self) -> String {
        if self.edge_type != ' ' && self.corner_type == ' ' {
            return self.edge_type.to_string();
        }
        if self.corner_type != ' ' {
            return self.corner_type.to_string();
        }
        if self.is_corner {
            return self.corner_type.to_string();
        }
        if self.is_full {
            return "*".to_string();
        }
        if self.is_dug {
            return "X".to_string();
        }
        return ".".to_string();
    }
}

pub fn task() {
    let start = std::time::Instant::now();
    let mut input = read_input_for_day(2023, 18, false);
    let lines: Vec<&str> = input.split("\n").collect();

    let instructions: Vec<_> = lines
        .iter()
        .map(|line| {
            let string = line.split(" ").collect::<Vec<&str>>();
            let dir = string[0];
            let len = string[1].parse::<usize>().unwrap();
            let hex = string[2].replace("(", "").replace(")", "").replace("#", "");
            (dir, len, hex)
        })
        .collect();

    let (grid_width, grid_height) = (500, 500);

    let mut current = (250, 250);

    let mut grid: Vec<GridSquare> = vec![];

    for y in 0..grid_height {
        for x in 0..grid_width {
            grid.push(GridSquare {
                x,
                corner_type: ' ',
                y,
                is_dug: false,
                is_full: false,
                is_corner: false,
                edge_type: ' ',
                is_part_of_trench: false,
            })
        }
    }

    for (index, (dir, len, color)) in instructions.iter().cloned().enumerate() {
        grid[get_index_at_coords(current.0, current.1, grid_width)].is_corner = true;
        grid[get_index_at_coords(current.0, current.1, grid_width)].is_dug = true;
        let next_end_coords = match dir {
            "U" => (current.0, current.1 - len),
            "D" => (current.0, current.1 + len),
            "L" => (current.0 - len, current.1),
            "R" => (current.0 + len, current.1),
            _ => {
                panic!("Direction not valid")
            }
        };
        grid[get_index_at_coords(next_end_coords.0, next_end_coords.1, grid_width)].is_corner =
            true;
        grid[get_index_at_coords(next_end_coords.0, next_end_coords.1, grid_width)].is_dug = true;

        let mut next_index = index + 1;
        if index == instructions.len() - 1 {
            next_index = 0;
        }

        let corner_type = match dir {
            "U" => match instructions[next_index].0 {
                "R" => 'F',
                "L" => '7',
                _ => ' ',
            },
            "D" => match instructions[next_index].0 {
                "R" => 'L',
                "L" => 'J',
                _ => ' ',
            },
            "L" => match instructions[next_index].0 {
                "U" => 'L',
                "D" => 'F',
                _ => ' ',
            },
            "R" => match instructions[next_index].0 {
                "U" => 'J',
                "D" => '7',
                _ => ' ',
            },
            _ => {
                panic!("Direction not valid")
            }
        };

        grid[get_index_at_coords(next_end_coords.0, next_end_coords.1, grid_width)].corner_type =
            corner_type;

        match dir {
            "U" => {
                for y in next_end_coords.1..current.1 {
                    grid[get_index_at_coords(next_end_coords.0, y, grid_width)].is_dug = true;
                    grid[get_index_at_coords(next_end_coords.0, y, grid_width)].edge_type = '|';
                }
            }
            "D" => {
                for y in current.1..next_end_coords.1 {
                    grid[get_index_at_coords(next_end_coords.0, y, grid_width)].is_dug = true;
                    grid[get_index_at_coords(next_end_coords.0, y, grid_width)].edge_type = '|';
                }
            }
            "L" => {
                for x in next_end_coords.0..current.0 {
                    grid[get_index_at_coords(x, current.1, grid_width)].is_dug = true;
                    grid[get_index_at_coords(x, current.1, grid_width)].edge_type = '-';
                }
            }
            "R" => {
                for x in current.0..next_end_coords.0 {
                    grid[get_index_at_coords(x, current.1, grid_width)].is_dug = true;
                    grid[get_index_at_coords(x, current.1, grid_width)].edge_type = '-';
                }
            }
            _ => {}
        }
        current = next_end_coords;
    }

    let lowest_x = grid.iter().fold(usize::MAX, |acc, square| {
        match square.x < acc && square.is_dug {
            true => return square.x,
            false => return acc,
        }
    });
    let highest_x = grid
        .iter()
        .fold(0, |acc, square| match square.x > acc && square.is_dug {
            true => return square.x,
            false => return acc,
        });
    let highest_y = grid
        .iter()
        .fold(0, |acc, square| match square.y > acc && square.is_dug {
            true => return square.y,
            false => return acc,
        });
    let lowest_y = grid.iter().fold(usize::MAX, |acc, square| {
        return match square.y < acc && square.is_dug {
            true => square.y,
            false => acc,
        };
    });

    let mut width = (highest_x - lowest_x) + 1;
    let mut height = (highest_y - lowest_y) + 1;

    let mut new_small_grid: Vec<GridSquare> = vec![];
    for y in lowest_y..=highest_y {
        for x in lowest_x..=highest_x {
            let mut new_grid_square = grid[get_index_at_coords(x, y, grid_width)].clone();
            new_grid_square.x = new_grid_square.x - lowest_x;
            new_grid_square.y = new_grid_square.y - lowest_y;
            new_small_grid.push(new_grid_square);
        }
    }

    let cloned_grid = new_small_grid.clone();
    for square in new_small_grid.iter_mut() {
        if square.is_dug {
            continue;
        }
        let mut corners_crossed_before = 0;
        let mut corners_crossed_after = 0;
        let mut edges_crossed = 0;

        let mut fs_crossed = 0usize;
        let mut sevens_crossed = 0usize;
        let mut js_crossed = 0usize;
        let mut ls_crossed = 0usize;
        let mut crossings = 0;

        for new_x in square.x..width {
            let s = &cloned_grid[get_index_at_coords(new_x, square.y, width)];
            match s.corner_type {
                ' ' => {}
                'J' => js_crossed += 1,
                'F' => fs_crossed += 1,
                'L' => ls_crossed += 1,
                '7' => sevens_crossed += 1,
                _ => {}
            }
            if !s.is_corner && s.edge_type == '|' {
                crossings += 1;
            }
        }

        // If the number of Js crossed does not match the 7s crossed or the Fs crossed doesn't match Ls crossed you made a crossing.
        if ls_crossed.abs_diff(js_crossed) % 2 != 0 || fs_crossed.abs_diff(sevens_crossed) % 2 != 0
        {
            crossings += 1;
        }

        if crossings % 2 == 1 || edges_crossed % 2 != 0 {
            square.is_dug = true;
            square.is_full = true;
        }
    }
    print_map(&new_small_grid, width);

    let answer = new_small_grid
        .iter()
        .fold(0, |acc, square| match square.is_dug {
            true => acc + 1,
            false => acc,
        });
    println!("Day 18 task 1: {}", answer);
    eprintln!("{:?}", start.elapsed());

    // Task 2
    // Orient coords positively
    let mut new_instructions = instructions.clone();

    let mut current_x = 0;
    let mut current_y = 0;
    let mut total_lengths = 2;
    let mut coords = new_instructions
        .iter()
        .map(|(_, _, color)| {
            let split = color.split_at(5);
            let len = isize::from_str_radix(split.0, 16).unwrap();
            total_lengths += len;
            let dir = match split.1 {
                "0" => "R",
                "1" => "D",
                "2" => "L",
                "3" => "U",
                _ => {
                    panic!("direction invalid")
                }
            };
            match dir {
                "R" => {
                    current_x += len;
                    return (current_x, current_y);
                }
                "L" => {
                    current_x -= len;
                    return (current_x, current_y);
                }
                "U" => {
                    current_y -= len;
                    return (current_x, current_y);
                }
                "D" => {
                    current_y += len;
                    return (current_x, current_y);
                }
                _ => {
                    panic!("")
                }
            }
        })
        .collect::<Vec<(isize, isize)>>();

    coords.reverse();

    let n = coords.len();
    let mut total = 0;

    for i in 0..n {
        let j = (i + 1) % n;
        total += (coords[i].0 * coords[j].1) - (coords[j].0 * coords[i].1);
    }

    total = (total.abs() + total_lengths) / 2;

    println!("Day 18 task 2: {}", total);
    eprintln!("{:?}", start.elapsed());
}

#[cfg(test)]
mod tests {
    // use crate::y2023::day18::function_to_test;

    #[test]
    fn code_works_correctly() {}
}
