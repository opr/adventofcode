use crate::input_reader::input_reader::read_input_for_day;

struct Part {
    value: i32,
    start_index: i32,
    end_index: i32,
    row: i32,
    is_valid: bool,
}

struct Gear {
    index: i32,
    row: i32,
    is_valid: bool,
}

fn contains_symbol( symbols: &str ) -> bool {
    symbols.chars().any( |s | s != '.' && ! s.is_alphanumeric() )
}
pub fn task() {
    let mut input = read_input_for_day(2023, 3, false );
    let lines: Vec<_> = input.split("\n").collect();

    let mut line_count = 0;
    let mut parts: Vec<Part> = vec![];
    let mut gears: Vec<Gear> = vec![];
    for line in &lines {
        let chars = line.chars().collect::<Vec<char>>();

        let mut num_index = 0;
        let mut num_found = false;
        let mut current_char = 0;
        let mut last_index = 0;
        let mut number: Vec<char>  = vec![];

        for c in &chars {

            if c == &'*' {
                gears.push(Gear {
                    row: line_count,
                    index: current_char,
                    is_valid: false
                } );
            }

            let is_numeric = c.is_numeric();
            if is_numeric {
                if ! num_found {
                    num_found = true;
                    num_index = current_char;
                }
                number.push(*c);
            }

            if (! is_numeric || (current_char+1) as usize == chars.len()) && num_found {
                last_index = current_char;
                let parsed_number = number.iter().collect::<String>();
                parts.push( Part {
                   value: parsed_number.parse::<i32>().unwrap(),
                    start_index: num_index as i32,
                    end_index: (last_index-1) as i32,
                    row: line_count,
                    is_valid: false
                } );
                last_index = 0;
                num_index = 0;
                num_found = false;
                number = vec![];
            }

            let g = 1;

            current_char+=1;
        }
        line_count+=1;
    }

    let g =7;
    let new_lines = lines;

    for mut part in &mut parts {
        let low_bounds = [ part.row- 1 , part.start_index- 1 ];
        let high_bounds = [ part.row+ 1 , part.end_index+ 1 ];

        let current_row = new_lines.get( part.row as usize ).unwrap();
        let highest_index = current_row.len().min( high_bounds[1] as usize );
        let lowest_index = (0 as isize).max( low_bounds[1] as isize ) as usize;

        if low_bounds[0] >= 0 {
            let top_row = new_lines.get( low_bounds[0] as usize ).unwrap();
            let positions = top_row.get( lowest_index ..= highest_index ).unwrap();
            if contains_symbol( positions ) {
                part.is_valid = true;
                continue;
            }
        }

        let current_compare = current_row.get( lowest_index..=highest_index ).unwrap();
        if contains_symbol( current_compare ) {
            part.is_valid = true;
            continue;
        }

        if (high_bounds[0] as usize) < new_lines.len() {
            let bottom_row = new_lines.get( high_bounds[0] as usize ).unwrap();
            let positions = bottom_row.get( lowest_index ..= highest_index ).unwrap();
            if contains_symbol( positions ) {
                part.is_valid = true;
                continue;
            }
        }
        //println!("Eligible positions for {}, symbols for given number are: {}", number.value, low_bounds.iter().cloned().map( |g| g.to_string() ).collect::<Vec<_>>().join(", ") );
    }

    let valid_gears: Vec<_> = gears.iter().filter_map(|g| {
        let parts_above_below = parts.iter().filter( |p | {
            if p.row != g.row - 1 && p.row != g.row + 1 {
                return false;
            }
            return ( p.start_index-1..=p.end_index+1).contains( &g.index );
        } ).collect::<Vec<&Part>>();

        let parts_row = parts.iter().filter( |p|  {
            if p.row != g.row {
                return false;
            }
            return p.start_index-1 == g.index || p.end_index+1 == g.index;
        } ).collect::<Vec<&Part>>();
        if parts_above_below.len() + parts_row.len() == 2 {
            let mut joined_parts: Vec<&Part> = vec![];
            joined_parts.extend( parts_above_below.iter().cloned() );
            joined_parts.extend( parts_row.iter().cloned() );
            let power = joined_parts.iter().fold(1, |a,b| a * b.value );
            return Some(power);
        }
        return None;
    } ).collect();

    let g = 1;

    let valid_parts = &parts.iter().filter(|p| p.is_valid).fold(0, |a, b| a + b.value );
    let valid_gear_total = valid_gears.iter().cloned().fold(0, |a,b| a+b);
    //let g  =1;
    println!("Day 3 task 1: {}", valid_parts);
    println!("Day 3 task 2: {}", valid_gear_total);
}