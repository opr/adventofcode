use crate::input_reader::input_reader::read_input_for_day;

struct Game {
    id: i32,
    blue_count: i32,
    red_count: i32,
    green_count: i32,
}
pub fn task() {
    let mut input = read_input_for_day(2023, 2, false);
    let mut lines: Vec<_> = input.split("\n").collect();
    let colours = [("blue", 14), ("green", 13), ("red",12) ];

    let games_total = lines.iter().cloned().filter_map( |l| {
        let (game, rounds) = l.split_once(": " ).unwrap();
        let id = game.replace("Game ", "");
        let rounds: Vec<_> = rounds.split("; ").map(|r| r.split(", ").collect::<Vec<_>>() ).collect();

        let game_valid = rounds.iter().all( |round| {
            round.iter().all( | cubes | {
                colours.iter().cloned().all( |(colour, limit)| {
                    if cubes.contains(colour ) {
                        let (colour_count, _) = cubes.split_once( " " ).unwrap();
                        return colour_count.parse::<i32>().unwrap_or( 0 ) <= limit
                    }
                    true
                } )
            } )
        } );

        if game_valid {
            return Some( id.parse::<i32>().unwrap_or(0) );
        }
        return None;
    } ).fold(0, |a, b| a+b);


    let pow = lines.iter().cloned().map( |l| {
        let (game, rounds) = l.split_once(": " ).unwrap();
        let id = game.replace("Game ", "");
        let rounds: Vec<_> = rounds.split("; ").map(|r| r.split(", ").collect::<Vec<_>>() ).collect();

        let mut blue_max = 0;
        let mut green_max = 0;
        let mut red_max = 0;
        rounds.iter().for_each( |round| {
            round.iter().for_each( | cubes | {
                colours.iter().cloned().for_each( |(colour, limit)| {
                    if cubes.contains(colour ) {
                        let (colour_count, _) = cubes.split_once( " " ).unwrap();
                        let colour_num = colour_count.parse::<i32>().unwrap_or( 0 );
                        match colour {
                            "blue" => {
                                if colour_num >= blue_max {
                                    blue_max = colour_num;
                                }
                            }
                            "red" => {
                                if colour_num >= red_max {
                                    red_max = colour_num;
                                }
                            }
                            "green" => {
                                if colour_num >= green_max {
                                    green_max = colour_num;
                                }
                            }
                            &_ => {

                            }
                        }
                    }
                } );
            } );
        } );
        blue_max * red_max * green_max
    } ).fold(0, |a,b| a+b);

    println!("Day 2 task 1: {}", games_total);
    println!("Day 2 task 2: {}", pow);
}