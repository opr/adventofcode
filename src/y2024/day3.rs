use regex::Regex;
use crate::input_reader::input_reader::read_input_for_day;

pub fn task() {
    let input = read_input_for_day(2024, 3, false );
    let start = std::time::Instant::now();
    println!("Part 1: {} in {:?}", part1( input.clone() ), start.elapsed());

    let start = std::time::Instant::now();
    println!("Part 2: {} in {:?}", part2( input.clone() ), start.elapsed());
}

fn part1( input: String ) -> usize {
    let re = Regex::new( r"mul\((\d+),(\d+)\)" ).unwrap();
    let muls = re.captures_iter( &input );
    muls.fold( 0, | acc, mul | {
        let first = mul.get( 1 ).unwrap().as_str().parse::<usize>().unwrap();
        let second = mul.get( 2 ).unwrap().as_str().parse::<usize>().unwrap();
        acc + ( second * first  )
    } )
}

fn part2( input: String ) -> usize {
    let re = Regex::new( r"don't\(\)|do\(\)|mul\((\d+),(\d+)\)" ).unwrap();
    let muls = re.captures_iter( &input );
    let mut doing = true;
    muls.fold( 0, | acc, mul | {
        let full_match = mul.get( 0 ).unwrap().as_str();
        if mul.get( 0 ).unwrap().as_str() == "do()" {
            doing = true;
            return acc;
        }
        if ! doing || mul.get( 0 ).unwrap().as_str() == "don't()" {
            doing = false;
            return acc;
        }
        let first = mul.get( 1 ).unwrap().as_str().parse::<usize>().unwrap();
        let second = mul.get( 2 ).unwrap().as_str().parse::<usize>().unwrap();

        match doing {
            true => acc + ( first * second ),
            false => acc
        }
    } )
}

#[cfg(test)]
mod tests {
    use crate::y2024::day3::part1;
    use crate::y2024::day3::part2;

    #[test]
    fn test_part1 () {
        assert_eq!( part1( "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))".to_string() ), 161 );
    }

    #[test]
    fn test_part2 () {
        assert_eq!( part2( "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))".to_string() ), 48 );
    }
}