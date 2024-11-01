use crate::input_reader::input_reader::read_input_for_day;
use crate::y2019::intcode_computer::IntcodeComputer;

pub fn task() {
    let input = read_input_for_day( 2019, 2, false );
    let mut computer = IntcodeComputer {
        program: input.clone(),
        ..Default::default()
    };
    computer.parse();
    println!("Day 2 task 1: {}", computer.run(&[(1isize,12isize), (2isize, 2isize)], false ), );

    for o in 0..=99 {
        for t in 0..=99 {
            let r = computer.run( &[ ( 1isize, o as isize) , (2isize, t as isize) ], false );
            if r == 19690720 {
                println!("Day 2 task 2: 100 * {o} + {t} = {}", (100 * o) + t );
                return;
            }
        }
    }

    let g = 1;

}