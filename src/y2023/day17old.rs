use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::env::current_dir;
use std::fmt::format;
use std::hash::{Hash, Hasher};
use std::thread::current;
use log::debug;
use crate::grid_utils::grid_utils::{get_index_at_coords, GridPoint, GridPrintable, print_map};
use crate::input_reader::input_reader::read_input_for_day;
use priority_queue::PriorityQueue;

#[derive(Clone,Debug,Hash,Eq, PartialEq)]
struct MinHeapNode {
    node: GridSquare,
    direction: char,
    priority: usize,
    distance: usize,
}

impl Ord for MinHeapNode {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.priority.cmp(&self.priority)
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for MinHeapNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.cmp(self))
    }
}

#[derive(Clone,Debug,Hash,Eq, PartialEq)]
struct Successor {
    parent: GridSquare,
    direction: char,
    cost: usize,
}

#[derive(Clone,Debug,Hash,Eq, PartialEq, Ord, PartialOrd)]
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
        return ".".to_string();//self.heat_loss.to_string();
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

pub fn task() {
    let start = std::time::Instant::now();
    let mut input = read_input_for_day( 2023, 17, false );
    let lines: Vec<&str> = input.split("\n").collect();
    let grid_height = lines.len();
    let grid_width = lines[0].len();

    let mut heat_map = lines.iter().enumerate().flat_map( | ( y, line ) | {
        line.chars().enumerate().map( | ( x, square ) | {
            GridSquare{ x, y, heat_loss: square.to_string().parse::<usize>().unwrap(), id: y * grid_width + x, visited: false }
        } ).collect::<Vec<GridSquare>>()
    } ).collect::<Vec<GridSquare>>();

    let answer = 1;
    let goal = ( grid_width - 1, grid_height - 1 );

    let mut open_set: BinaryHeap<MinHeapNode> = BinaryHeap::new();
    let mut came_from: HashMap< GridSquare, ( GridSquare, char ) > = HashMap::new();
    let mut last_directions: Vec<char> = vec![];

    open_set.push( MinHeapNode{
        node: heat_map[0].clone(),
        priority: 0,
        direction: ' ',
        distance: 0,
    } );

    let mut g_scores: HashMap<GridSquare, usize> = HashMap::new();
    g_scores.insert( heat_map[0].clone(), 0 );
    let mut f_scores: HashMap<GridSquare, usize> = HashMap::new();
    f_scores.insert( heat_map[0].clone(), 0 + h( ( heat_map[0].x, heat_map[0].y ), goal ) );

    let mut last_directions: Vec<char> = vec![];

    while let Some(current_node) = open_set.pop() {
        let current = current_node.node;
        if current_node.direction != ' ' {
            last_directions.push(current_node.direction);
        }
        let n = get_neighbours( &came_from, &last_directions, &current, &heat_map, grid_width, grid_height );
        debug!("");

        for ( neighbor, direction ) in n {
            if let Some( found ) = came_from.iter().find( | c | c.0.id == neighbor.id ) {
                last_directions.pop();
                continue;
            }

            if direction == 'S' && current_node.direction == 'N' {
                continue;
            }
            if direction == 'N' && current_node.direction == 'S' {
                continue;
            }
            if direction == 'E' && current_node.direction == 'W' {
                continue;
            }
            if direction == 'W' && current_node.direction == 'E' {
                continue;
            }

            let tentative_g = g_scores.get( &current ).unwrap_or( &usize::MAX ) + neighbor.heat_loss;
            let mut cloned_iter = open_set.iter().cloned().find( | set_member | set_member.node.id == neighbor.id );
            let open_set_find = cloned_iter.iter().collect::<Vec<&MinHeapNode>>();

            if open_set_find.len() == 0 || tentative_g < *g_scores.get(&neighbor).unwrap_or( &usize::MAX ) {
                g_scores.insert(neighbor.clone(), tentative_g);
                f_scores.insert(neighbor.clone(), tentative_g + h( ( neighbor.x, neighbor.y ), goal ) );
                came_from.insert( neighbor.clone(), ( current.clone(), direction ) );
                // Add the neighbor and direction of travel to the open set

                let same_dir = direction == current_node.direction;
                // println!("Moving from {},{} to {},{} - {}", current.x, current.y, neighbor.x, neighbor.y, same_dir );
                let mut distance = 0;
                distance += if same_dir { current_node.distance + 1 } else { 0 };
                if distance == 2 && same_dir {
                    continue;
                }
                debug!("");

                open_set.push(MinHeapNode {
                    node: neighbor.clone(),
                    direction,
                    distance,
                    priority: f_scores[&neighbor],
                });
            }
        }
        debug!("nb processed");

        if current.x == goal.0 && current.y == goal.1 {
            let mut current = &current.clone();
            while let Some( x ) = came_from.iter().find( | s| *s.0 == current.clone() ) {
                heat_map[ get_index_at_coords( current.x, current.y, grid_width ) ].visited = true;
                current = &came_from.get( &current.clone() ).unwrap().0;
                if current.x == 0 && current.y == 0 {
                    break;
                }
            }
            break
        }
    }

    print_map( &heat_map, grid_width );
    println!( "Day 17 task 1: {}", answer );
    eprintln!("{:?}", start.elapsed() );
}

fn get_neighbours( came_from: &HashMap< GridSquare, ( GridSquare, char ) >, last_directions: &Vec<char>, current: &GridSquare, grid: &Vec<GridSquare>, grid_width: usize, grid_height: usize ) -> Vec< ( GridSquare, char ) > {
    let mut banned_direction = 'W';
    let ( x, y ) = ( current.x, current.y );

    if let Some(third_node) = came_from.get( current ) {
        if let Some( second_node ) = came_from.get( &third_node.0 ) {
            if let Some( first_node) = came_from.get(&second_node.0) {
                if first_node == second_node && second_node == third_node {
                    banned_direction = 'W';//first_node.1;
                }
            }
        }
    }

    let mut neighbours = vec![];

    // Check neighbours east
    if x + 1 < grid_width && banned_direction != 'E' {
        neighbours.push( ( grid[ get_index_at_coords( x + 1, y, grid_width ) ].clone(), 'E' ) );
    }

    // Check neighbours south
    if y + 1 < grid_height && banned_direction != 'S' {
        neighbours.push( ( grid[ get_index_at_coords( x, y + 1, grid_width ) ].clone(), 'S' ) );
    }

    // Check neighbours north
    if y > 0 && banned_direction != 'N' {
        neighbours.push( ( grid[ get_index_at_coords( x, y - 1, grid_width ) ].clone(), 'N' ) );
    }

    // Check neighbours west
    if x > 0 && banned_direction != 'W' {
        neighbours.push( ( grid[ get_index_at_coords( x - 1, y, grid_width ) ].clone(), 'W' ) );
    }

    neighbours.iter().cloned().filter_map( | n | {
        if let Some( neighbour ) = came_from.get( &n.0 ) {
            return None;
        }
        return Some( n );
    } ).collect::<Vec<(GridSquare, char)>>()
}

fn g( origin: ( usize, usize ), current: ( usize, usize ) ) -> usize {
    return 0;
}

fn h( origin: ( usize, usize ), goal: ( usize, usize ) ) -> usize {
    ( origin.0.abs_diff( goal.0 ) + origin.1.abs_diff( goal.1 ) ) * 3
}
#[cfg(test)]
mod tests {
    // use crate::y2023::day17::function_to_test;

    #[test]
    fn code_works_correctly() {

    }
}