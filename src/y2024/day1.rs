use std::collections::HashMap;
use itertools::Itertools;
use crate::input_reader::input_reader::read_input_for_day;

pub fn task() {
    let start = std::time::Instant::now();
    let input = read_input_for_day( 2024, 1, false  );
    let left  = input.split('\n').map(|s| s.split("   ").nth(0).unwrap().parse::<usize>().unwrap()).sorted().collect::<Vec<usize>>();
    let right = input.split('\n').map(|s| s.split("   ").nth(1).unwrap().parse::<usize>().unwrap()).sorted().collect::<Vec<usize>>();

    let answer = left.iter().enumerate().fold( 0, | acc, ( index, item ) | {
        acc + usize::abs_diff( *item, *right.iter().nth( index ).unwrap() )
    } );

    println!("Part 1 {answer} in {:?}", start.elapsed());
    let frequencies = generate_frequency_map(&right);
    let answer =
        left.iter().fold( 0, | acc, item | acc + ( frequencies.get( item ).unwrap_or(&0) * item ) );

    println!("Part 2 {answer} in {:?}", start.elapsed());
}
fn generate_frequency_map(input : &Vec<usize>) -> HashMap<usize,usize> {
    input
        .iter()
        .copied()
        .fold(HashMap::new(), |mut map, val|{
            map.entry(val)
                .and_modify(|frq|*frq+=1)
                .or_insert(1);
            map
        })
}