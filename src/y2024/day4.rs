use itertools::Itertools;
use regex::Regex;
use crate::grid_utils::grid_utils::get_coords_at_index;
use crate::input_reader::input_reader::read_input_for_day;

pub fn task() {
    let input = read_input_for_day(2024, 4, false );
    let start = std::time::Instant::now();
    println!("Part 1: {} in {:?}", part1( input.clone() ), start.elapsed());

    let start = std::time::Instant::now();
    println!("Part 2: {} in {:?}", part2( input.clone() ), start.elapsed());
}

fn part1( input: String ) -> usize {

    let width = input.find( '\n' ).unwrap_or(input.len());
    let lines = input.split('\n').collect::<Vec<&str>>();
    let height = lines.len();
    let mut finds = 0;
    let mut grid = vec![ vec!['\0'; width]; height ];
    for ( y, line ) in lines.iter().enumerate() {
        for ( x, char ) in line.chars().enumerate() {
            grid[y][x] = char.clone();
        }
    }

    for ( y, line ) in grid.iter().enumerate() {
        for ( x, char ) in line.iter().enumerate() {
            if *char != 'X' { continue }
            // Check left.
            if x >= 3 {
                let test: String = line.get( x - 3..x ).unwrap().iter().collect();
                if test == "SAM" { finds += 1 }
            }

            // Check right.
            if x < width - 3 {
                let test: String = line.get( x + 1..x + 4 ).unwrap().iter().collect();
                if test == "MAS" { finds += 1 }
            }

            // Check up.
            if y >= 3 {
                let test: String = format!( "{}{}{}", grid[y - 1][x], grid[y - 2][x], grid[y - 3][x] );
                if test == "MAS" { finds += 1 }
            }

            // Check down.
            if y < height - 3 {
                let test: String = format!( "{}{}{}", grid[y + 1][x], grid[y + 2][x], grid[y + 3][x] );
                if test == "MAS" { finds += 1 }
            }

            // Check up-left.
            if y >= 3 && x >= 3 {
                let test: String = format!( "{}{}{}", grid[y - 1][x - 1], grid[y - 2][x - 2], grid[y - 3][x - 3] );
                if test == "MAS" { finds += 1 }
            }

            // Check up-right.
            if y >= 3 && x < width - 3 {
                let test: String = format!( "{}{}{}", grid[y - 1][x + 1], grid[y - 2][x + 2], grid[y - 3][x + 3] );
                if test == "MAS" { finds += 1 }
            }

            // Check down-left.
            if y < height - 3 && x >= 3 {
                let test: String = format!( "{}{}{}", grid[y + 1][x - 1], grid[y + 2][x - 2], grid[y + 3][x - 3] );
                if test == "MAS" { finds += 1 }
            }

            // Check down-right.
            if y < height - 3 && x < width - 3 {
                let test: String = format!( "{}{}{}", grid[y + 1][x + 1], grid[y + 2][x + 2], grid[y + 3][x + 3] );
                if test == "MAS" { finds += 1 }
            }
        }
    }
    finds
}

fn part2( input: String ) -> usize {

    let width = input.find( '\n' ).unwrap_or(input.len());
    let lines = input.split('\n').collect::<Vec<&str>>();
    let height = lines.len();
    let mut finds = 0;
    let mut grid = vec![ vec!['\0'; width]; height ];
    for ( y, line ) in lines.iter().enumerate() {
        for ( x, char ) in line.chars().enumerate() {
            grid[y][x] = char.clone();
        }
    }

    for ( y, line ) in grid.iter().enumerate() {
        for ( x, char) in line.iter().enumerate() {
            if ( y == 0 || y == height - 1 ) || ( x == 0 || x == width - 1 ) { continue }
            if *char != 'A' { continue }
            // Chars going top to bottom left to right
            let up_left = grid[y - 1][x - 1];
            let down_right = grid[y + 1][x + 1];
            // Chars going top to bottom right to left
            let up_right = grid[y - 1][x + 1];
            let down_left = grid[y + 1][x - 1];
            if ((up_left == 'S' && down_right == 'M') || (up_left == 'M' && down_right == 'S')) && ((up_right == 'S' && down_left == 'M') || (up_right == 'M' && down_left == 'S')) {
                finds += 1;
            }
        }
    }
    finds
}

#[cfg(test)]
mod tests {
    use crate::y2024::day4::part1;
    use crate::y2024::day4::part2;

    #[test]
    fn test_part1_horizontal () {
        assert_eq!( part1( "SAMXMAS\nXXXXXXX\nXXXXXXX\nXXXXXXX".to_string() ), 2 );
    }

    #[test]
    fn test_part1_vertical () {
        //     0 1 2 3 4 5 6
        //     -------------
        // 0 | S A M X M A S
        // 1 | A A M M M A S
        // 2 | M A M A M A S
        // 3 | X A M S M A X
        assert_eq!( part1( "SAMXMAS\nAAMMMAS\nMAMAMAS\nXAMSMAX".to_string() ), 4 );

    }

    #[test]
    fn test_part1_diagonal () {
        //     0 1 2 3 4 5 6 7 8
        //     -----------------
        // 0 | X M M S S S S X S
        // 1 | X M A M S S M A M
        // 2 | X M A M S A M M M
        // 3 | X M M S S X S S X
        assert_eq!( part1( "XMMSSSSXS\nXMAMSSMAM\nXMAMSAMMM\nXMMSSXSSX".to_string() ), 4 );

    }
    #[test]
    fn test_part1_all () {
        assert_eq!( part1( "MMMSXXMASM\nMSAMXMSMSA\nAMXSXMAAMM\nMSAMASMSMX\nXMASAMXAMM\nXXAMMXXAMA\nSMSMSASXSS\nSAXAMASAAA\nMAMMMXMMMM\nMXMXAXMASX".to_string() ), 18 );

    }

    #[test]
    fn test_part2_small () {
        assert_eq!( part2( "MXS\nXAX\nMAS".to_string() ), 1 );
    }
    #[test]
    fn test_part2_full () {
        //     0 1 2 3 4 5 6 7 8 9
        //     --------------------
        // 0 | M M M S X X M A S M
        // 1 | M S A M X M S M S A
        // 2 | A M X S X M A A M M
        // 3 | M S A M A S M S M X
        // 4 | X M A S A M X A M M
        // 5 | X X A M M X X A M A
        // 6 | S M S M S A S X S S
        // 7 | S A X A M A S A A A
        // 8 | M A M M M X M M M M
        // 9 | M X M X A X M A S X
        assert_eq!( part2( "MMMSXXMASM\nMSAMXMSMSA\nAMXSXMAAMM\nMSAMASMSMX\nXMASAMXAMM\nXXAMMXXAMA\nSMSMSASXSS\nSAXAMASAAA\nMAMMMXMMMM\nMXMXAXMASX".to_string() ), 9 );
    }
}