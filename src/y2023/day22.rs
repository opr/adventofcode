use std::cmp::{max, min};
use std::path::Component::ParentDir;
use crate::input_reader::input_reader::read_input_for_day;
use plotly::{Plot, Scatter,Scatter3D};
use plotly::common::Mode;

pub fn task() {
    let input = read_input_for_day(2023, 22, true );
    run_task_1( input );
}

#[derive(Clone,Debug)]
struct Brick {
    x_start: usize,
    y_start: usize,
    z_start: usize,
    x_end: usize,
    y_end: usize,
    z_end: usize,
    id: usize,
    supports: Vec<usize>,
    supported_by: Vec<usize>,
    name: String,
}
fn move_down(brick: &mut Brick) {
    if brick.z_start > 1 {
        brick.z_start -= 1;
        brick.z_end -= 1;
    }
}

fn move_up(brick: &mut Brick) {
    brick.z_start += 1;
    brick.z_end += 1;
}

fn run_task_1( input: String ) -> usize {
    let mut brick_descriptions = input.split("\n").collect::<Vec<&str>>();
    brick_descriptions.sort_by( |a, b| {
        let a_split = a.replace('~', "," );
        let b_split = b.replace('~', "," );
        
        let a_coords = a_split.split(',').collect::<Vec<&str>>();
        let b_coords = b_split.split(',').collect::<Vec<&str>>();
        
        min( a_coords.iter().nth(2).unwrap().parse::<usize>().unwrap(), a_coords.iter().nth(5).unwrap().parse::<usize>().unwrap()).cmp(min(&b_coords.iter().nth(2).unwrap().parse::<usize>().unwrap(), &b_coords.iter().nth(5).unwrap().parse::<usize>().unwrap()))
    });
    let mut bricks: Vec<Brick> = Vec::new();
    for (i, brick) in brick_descriptions.iter().enumerate() {
        let xyz_start = brick.split("~").nth(0).unwrap();
        let xyz_end = brick.split("~").nth(1).unwrap();
        let parsed_start = xyz_start.split(',').map(|i| i.parse::<usize>().unwrap()).collect::<Vec<usize>>();
        let parsed_end = xyz_end.split(',').map(|i| i.parse::<usize>().unwrap()).collect::<Vec<usize>>();
        let (x_start, y_start, z_start) = (parsed_start[0], parsed_start[1], parsed_start[2]);
        let (x_end, y_end, z_end) = (parsed_end[0], parsed_end[1], parsed_end[2]);
        let chars = "ABCDEFGHIJKLMNOPQRSTVWXYZabcdefghijklmnopqrstuvwxyz1234567890!@£$%^&*()";
        let mut crafted_brick = Brick {
            id: i,
            name: chars.chars().nth(min(i, 10)).unwrap().to_string(),
            x_end,
            y_end,
            z_end,
            x_start,
            y_start,
            z_start,
            supports: Vec::new(),
            supported_by: Vec::new(),
        };
        bricks.push(crafted_brick);
    }
    bricks.iter_mut()
        .fold(
            (Vec::<Brick>::new(), 1),
            |(mut result, max_z_reached), mut brick| {
                // position just above max_z_reached
                let diff = brick.z_end - brick.z_start;
                brick.z_start = max_z_reached + 1;
                brick.z_end = brick.z_start + diff;

                loop {
                    let mut can_move_down = true;

                    move_down(&mut brick);

                    for i in (0..result.len()).rev() {
                        if result[i].z_end < brick.z_start {
                            continue;
                        }

                        let collision = ! space_below(&brick, &result[i]);

                        if collision {
                            can_move_down = false;
                            let len = result.len();
                            result[i].supports.push(len);
                            brick.supported_by.push(i);
                        }
                    }

                    if !can_move_down {
                        // cannot move any further down (collided with some brick)
                        // revert and break
                        move_up(&mut brick);
                        break;
                    }

                    // reached bottom
                    if brick.z_start == 1 {
                        break;
                    }
                }

                let new_max_z_reached = max_z_reached.max(brick.z_end);

                result.push(brick.clone());

                (result, new_max_z_reached)
            },
        );
    
    
    let cloned_bricks = bricks.clone();
    for brick in bricks.iter_mut() {
        brick.supports = cloned_bricks.iter().filter_map( |b| { if b.supported_by.contains( &brick.id ) { return Some(b.id) } None } ).collect();
    }
    
    let answer = bricks.iter().fold( 0, |acc, b| {
        if b.supports.len() == 0 {
            return acc + 1;
        }
        
        // find bricks that are supported by this brick
        let bricks_supported_by_this = bricks.iter().cloned().filter( | sb | {
            sb.supported_by.contains( &b.id )
        } ).collect::<Vec<Brick>>();
        
        let bricks_with_other_support = bricks_supported_by_this.iter().cloned().filter(|sb| {
            sb.supported_by.len() > 1
        }).collect::<Vec<Brick>>();
        
        if bricks_with_other_support.iter().count() >= 1 {
            return acc + 1;
        }
        acc
    });
    println!("Part 1: {}", answer);

    {
        let mut plot = Plot::new();
        for brick in bricks.clone() {
            let trace = Scatter3D::new(vec![brick.x_start, brick.x_end], vec![brick.y_start, brick.y_end], vec![brick.z_start, brick.z_end]).name(brick.name);
            plot.add_trace(trace);
        }
        plot.write_html("out.html");
    }
    answer
}

fn support_below( brick: Brick, bricks_in: &Vec<Brick> ) -> Vec<usize> {
    let x_square2d = brick.x_start..brick.x_end+1;
    let y_square2d = brick.y_start..brick.y_end+1;
    let mut bricks = bricks_in.clone();
    // bricks.sort_by( | b1, b2| {
    //     let highest_z1    = max( b1.z_end, b1.z_start);
    //     let highest_z2    = max( b2.z_end, b2.z_start);
    //     highest_z1.cmp(&highest_z2)
    // } );
    let bb = bricks.iter().filter( |b| {
        //println!("checking brick {} to see if its max z ({}) is 1 less than the test brick's ({}) min z ({})", b.name, max(b.z_start, b.z_end), brick.name, min( brick.z_start, b.z_end ));
        max(b.z_start, b.z_end) == min( brick.z_start, brick.z_end ) - 1
    }
    ).collect::<Vec<&Brick>>();
    let olbb = bb.iter().filter_map( | b | {
        if b.id == brick.id {
            return None;
        }
        let overlap = ! space_below( &brick, b );
        if overlap {
            return Some(b);
        }
        None
    } ).collect::<Vec<&&Brick>>();
    olbb.iter().map(|b| b.id).collect()
}

fn point_in_range(p: usize, range: (usize, usize)) -> bool {
    p >= range.0 && p <= range.1
}

fn space_below(brick_a: &Brick, brick_b: &Brick) -> bool {
    !  ( (point_in_range(brick_a.z_start, (brick_b.z_start, brick_b.z_end))
        || point_in_range(brick_a.z_end, (brick_b.z_start, brick_b.z_end))
        || point_in_range(brick_b.z_start, (brick_a.z_start, brick_a.z_end))
        || point_in_range(brick_b.z_end, (brick_a.z_start, brick_a.z_end))) &&
    (point_in_range(brick_a.x_start, (brick_b.x_start, brick_b.x_end))
        || point_in_range(brick_a.x_end, (brick_b.x_start, brick_b.x_end))
        || point_in_range(brick_b.x_start, (brick_a.x_start, brick_a.x_end))
        || point_in_range(brick_b.x_end, (brick_a.x_start, brick_a.x_end)))
        && (point_in_range(brick_a.y_start, (brick_b.y_start, brick_b.y_end))
        || point_in_range(brick_a.y_end, (brick_b.y_start, brick_b.y_end))
        || point_in_range(brick_b.y_start, (brick_a.y_start, brick_a.y_end))
        || point_in_range(brick_b.y_end, (brick_a.y_start, brick_a.y_end))) )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = r"
1,0,1~1,2,1
0,2,3~2,2,3
2,0,5~2,2,5
0,0,4~0,2,4
0,1,6~2,1,6
0,0,2~2,0,2
1,1,8~1,1,9
        "
            .trim().to_string();

        assert_eq!(run_task_1(input), 5);
    }

    #[test]
    fn test_part_1_1() {
        let input = r"
0,0,1~0,0,1
0,0,3~0,0,3
        "
            .trim().to_string();

        assert_eq!(run_task_1(input), 1);
    }

    #[test]
    fn test_part_1_2() {
        let input = r"
0,0,1~0,0,1
0,1,3~0,1,3
        "
            .trim().to_string();

        assert_eq!(run_task_1(input), 2);
    }

    #[test]
    fn test_part_1_3() {
        let input = r"
0,0,1~0,0,10
0,0,300~0,1,300
        "
            .trim().to_string();

        assert_eq!(run_task_1(input), 1);
    }

    #[test]
    fn test_part_1_4() {
        let input = r"
0,0,1~0,0,10
0,0,300~0,1,300
1,0,50~1,0,50
        "
            .trim().to_string();

        assert_eq!(run_task_1(input), 2);
    }

    #[test]
    fn test_part_1_5() {
        let input = r"
0,0,1~0,0,10
0,1,1~0,1,10
0,0,300~0,1,300
        "
            .trim().to_string();

        assert_eq!(run_task_1(input), 3);
    }
    #[test]
    fn test_part_1_6() {
        let input = r"
0,0,1~0,0,1
1,1,1~1,1,1
0,0,2~0,1,2
0,1,3~1,1,3
        "
            .trim().to_string();

        assert_eq!(run_task_1(input), 2);
    }
}
