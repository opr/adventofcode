use crate::grid_utils::grid_utils::{get_index_at_coords, print_map, GridPoint, GridPrintable};
use crate::input_reader::input_reader::read_input_for_day;
use log::debug;
use pathfinding::prelude::astar;

#[derive(Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
struct GridSquare {
    x: usize,
    y: usize,
    id: usize,
    heat_loss: usize,
    visited: bool,
}
impl GridPrintable for GridSquare {
    fn get_print(&self) -> String {
        if self.visited {
            return "#".to_string();
        }
        return ".".to_string();
    }
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

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct State {
    position: (usize, usize),
    direction: (isize, isize),
    distance: usize,
}

pub fn task() {
    let start_time = std::time::Instant::now();
    let input = read_input_for_day(2023, 17, false);
    let lines: Vec<&str> = input.split("\n").collect();
    let grid_height = lines.len();
    let grid_width = lines[0].len();

    let mut heat_map = lines
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, square)| GridSquare {
                    x,
                    y,
                    heat_loss: square.to_string().parse::<usize>().unwrap(),
                    id: y * grid_width + x,
                    visited: false,
                })
                .collect::<Vec<GridSquare>>()
        })
        .collect::<Vec<GridSquare>>();
    let end = (grid_width - 1, grid_height - 1);

    let start = State {
        position: (0, 0),
        direction: (0, 0),
        distance: 0,
    };
    const P1MIN: usize = 1;
    const P1MAX: usize = 3;

    let p1_path = astar(
        &start,
        |state| match state.distance > 0 || state.direction.0 == 0 && state.direction.1 == 0 {
            true => compute_neighbouring_successors::<P1MIN, P1MAX>(
                state,
                &heat_map,
                grid_width,
                grid_height,
            ),
            false => compute_next_successor(state, &heat_map, grid_width, grid_height),
        },
        |state| end.0.abs_diff(state.position.0) + end.1.abs_diff(state.position.1),
        |state| state.position == end && state.distance >= P1MIN,
    );

    const MIN: usize = 4;
    const MAX: usize = 10;

    let path = astar(
        &start,
        |state| match state.distance > 0 || state.direction.0 == 0 && state.direction.1 == 0 {
            true => compute_neighbouring_successors::<MIN, MAX>(
                state,
                &heat_map,
                grid_width,
                grid_height,
            ),
            false => compute_next_successor(state, &heat_map, grid_width, grid_height),
        },
        |state| end.0.abs_diff(state.position.0) + end.1.abs_diff(state.position.1),
        |state| state.position == end && state.distance >= MIN,
    );

    // Purely for visualisation.
    if let Some(ref p) = &path {
        for x in &p.0 {
            heat_map[get_index_at_coords(x.position.0, x.position.1, grid_width)].visited = true;
        }
    }

    print_map(&heat_map, grid_width);

    let answer1 = p1_path.unwrap().1;
    let answer2 = path.unwrap().1;

    println!("Day 17 task 1: {}", answer1);
    println!("Day 17 task 2: {}", answer2);
    eprintln!("{:?}", start_time.elapsed());
}

fn compute_next_successor(
    state: &State,
    grid: &Vec<GridSquare>,
    grid_width: usize,
    grid_height: usize,
) -> Vec<(State, usize)> {
    let (x, y) = state.position;
    let mut next_pos = (0, 0);
    match state.direction {
        // North.
        (0, -1) => {
            if y == 0 {
                return Vec::with_capacity(0);
            }
            next_pos = (x, y - 1);
        }
        // East.
        (1, 0) => {
            if x == grid_width - 1 {
                return Vec::with_capacity(0);
            }
            next_pos = (x + 1, y);
        }
        // South.
        (0, 1) => {
            if y == grid_height - 1 {
                return Vec::with_capacity(0);
            }
            next_pos = (x, y + 1);
        }
        // West.
        (-1, 0) => {
            if x == 0 {
                return Vec::with_capacity(0);
            }
            next_pos = (x - 1, y);
        }
        _ => {}
    }
    let new_state = State {
        position: next_pos,
        direction: state.direction,
        distance: state.distance + 1,
    };
    vec![(
        new_state,
        grid[get_index_at_coords(next_pos.0, next_pos.1, grid_width)].heat_loss,
    )]
}

fn compute_neighbouring_successors<const MIN: usize, const MAX: usize>(
    state: &State,
    grid: &Vec<GridSquare>,
    grid_width: usize,
    grid_height: usize,
) -> Vec<(State, usize)> {
    let mut neighbours = vec![];
    let (x, y) = state.position;
    let mut mandatory_direction = (0, 0);
    if state.distance < MIN {
        mandatory_direction = state.direction;
    }

    // Check neighbours east
    if x + 1 < grid_width {
        let distance = match state.direction == (1, 0) {
            true => state.distance + 1,
            false => 1,
        };
        if state.direction != (-1, 0) {
            neighbours.push((
                State {
                    position: (x + 1, y),
                    direction: (1, 0),
                    distance,
                },
                (1, 0),
                grid[get_index_at_coords(x + 1, y, grid_width)].heat_loss,
            ));
        }
    }

    // Check neighbours south
    if y + 1 < grid_height {
        let distance = match state.direction == (0, 1) {
            true => state.distance + 1,
            false => 1,
        };
        if state.direction != (0, -1) {
            neighbours.push((
                State {
                    position: (x, y + 1),
                    direction: (0, 1),
                    distance,
                },
                (0, 1),
                grid[get_index_at_coords(x, y + 1, grid_width)].heat_loss,
            ));
        }
    }

    // Check neighbours north
    if y > 0 {
        let distance = match state.direction == (0, -1) {
            true => state.distance + 1,
            false => 1,
        };
        if state.direction != (0, 1) {
            neighbours.push((
                State {
                    position: (x, y - 1),
                    direction: (0, -1),
                    distance,
                },
                (0, -1),
                grid[get_index_at_coords(x, y - 1, grid_width)].heat_loss,
            ));
        }
    }

    // Check neighbours west
    if x > 0 {
        let distance = match state.direction == (-1, 0) {
            true => state.distance + 1,
            false => 1,
        };
        if state.direction != (1, 0) {
            neighbours.push((
                State {
                    position: (x - 1, y),
                    direction: (-1, 0),
                    distance,
                },
                (-1, 0),
                grid[get_index_at_coords(x - 1, y, grid_width)].heat_loss,
            ));
        }
    }

    neighbours
        .iter()
        .cloned()
        .filter(|(state, _, _)| match mandatory_direction == (0, 0) {
            true => true,
            false => state.direction == mandatory_direction,
        })
        .filter_map(|(state, _, heat_loss)| match state.distance <= MAX {
            true => Some((state, heat_loss)),
            false => None,
        })
        .collect()
}
