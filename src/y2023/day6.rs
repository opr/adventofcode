use crate::input_reader::input_reader::read_input_for_day;

#[derive(Clone)]
struct Race {
    time: usize,
    distance: usize,
}

pub fn task() {
    let mut input = read_input_for_day(2023, 6, false );
    let lines: Vec<_> = input.split("\n").collect();

    let times: Vec<_> = lines[0].replace( "Time: ", "" ).split(" " ).filter_map( |x| extract_values(x) ).collect();
    let distances: Vec<_> = lines[1].replace( "Distance: ", "").split(" " ).filter_map( |x| extract_values(x) ).collect();

    let races: Vec<Race> = times.iter().enumerate().map( | (index, time) | Race { time: *time, distance: distances[index] } ).collect();

    for ( race_id, race ) in races.iter().enumerate() {
        let time_midpoint = race.time / 2;

        let mut higher_bound = time_midpoint;
        let mut lower_bound = time_midpoint;

        // Go up
        for time in time_midpoint..race.time {
            let new_distance = calculate_distance( time, race.time );
            if new_distance > race.distance {
                higher_bound = time;
                continue;
            }
            break;
        }

        // Go down
        for time in (0..time_midpoint).rev() {
            let new_distance = calculate_distance( time, race.time );
            if new_distance > race.distance {
                lower_bound = time;
                continue;
            }
            break;
        }

        println!("Race: {} ways to win {}", race_id, ( higher_bound - lower_bound ) + 1);

    }
}

fn calculate_distance( hold: usize, race_time: usize ) -> usize {
    let remaining = race_time - hold;
    remaining * hold
}

fn extract_values ( value: &str ) -> Option< usize > {
        if value != " " && value != "" {
            return Some(value.to_string().parse::<usize>().unwrap())
        }
        return None;
}