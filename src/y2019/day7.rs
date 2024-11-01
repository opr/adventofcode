use std::process::exit;
use crate::input_reader::input_reader::read_input_for_day;
use crate::y2019::intcode_computer::IntcodeComputer;

pub fn task1() {
    let input = read_input_for_day( 2019, 7, false );

    let mut amplifiers = (0..5).into_iter().map( |i| IntcodeComputer {
        program: input.clone(),
        ..Default::default()
    } ).collect::<Vec<IntcodeComputer>>();
    let mut output = 0;

    for a in 0..=4  {
        for b in 0..=4 {
            if b == a {
                continue;
            }
            for c in 0..=4 {
                if c == b || c == a {
                    continue;
                }
                for d in 0..=4 {
                    if d == c || d == b || d == a {
                        continue;
                    }
                    for e in 0..=4 {
                        if e == d || e == c || e == b || e == a {
                            continue;
                        }
                        println!("Trying combo: {} {} {} {} {}", a, b, c, d, e );
                        amplifiers.iter_mut().for_each(|amp| amp.reset_computer() );

                        run_amps_with_inputs(a, b, c, d, e, &mut amplifiers, false );
                        let last_res = amplifiers[4].get_last_output();
                        if last_res > output {
                            output = last_res;
                        }
                        println!("{last_res}");
                    }
                }
            }
        }
    }

    println!("Day 7 task 1: {}", output);
}
pub fn task2() {
    let input = read_input_for_day( 2019, 7, false );

    let mut amplifiers = (0..5).into_iter().map( |i| IntcodeComputer {
        program: input.clone(),
        return_on_output: true,
        ..Default::default()
    } ).collect::<Vec<IntcodeComputer>>();
    let mut output = 0;

    amplifiers.iter_mut().for_each(|amp| amp.parse() );
    let gg = run_amps_until_last_halts( 9,8,7,6,5, &mut amplifiers, true );
    // println!("{gg}");
    // exit(0);

    for a in 5..=9  {
        for b in 5..=9 {
            if b == a {
                continue;
            }
            for c in 5..=9 {
                if c == b || c == a {
                    continue;
                }
                for d in 5..=9 {
                    if d == c || d == b || d == a {
                        continue;
                    }
                    for e in 5..=9 {
                        if e == d || e == c || e == b || e == a {
                            continue;
                        }
                        println!("Trying combo: {} {} {} {} {}", a, b, c, d, e );
                        //amplifiers.iter_mut().for_each(|amp| amp.reset_computer() );
                        amplifiers.iter_mut().for_each(|amp| amp.reset_computer() );
                        let last_res = run_amps_until_last_halts( a,b,c,d,e, &mut amplifiers, true );
                        if last_res > output {
                            output = last_res;
                        }
                        println!("{last_res}");
                    }
                }
            }
        }
    }

    println!("Day 7 task 2: {}", output);
}

pub fn task() {
    //task1();
    task2();
}
pub fn run_amps_until_last_halts( a: isize, b:isize, c:isize, d:isize, e:isize, amps: &mut Vec<IntcodeComputer>, skip_init: bool ) -> isize {
    amps[0].inputs = vec![a];
    amps[1].inputs = vec![b];
    amps[2].inputs = vec![c];
    amps[3].inputs = vec![d];
    amps[4].inputs = vec![e];

    let mut last_amp_halted = amps[4].current_instruction == 99;
    let mut last_output = 0;
    let mut loops = 0;
    while ! last_amp_halted {
        loops += 1;
        for amp in 0..5 {
            amps[amp].inputs.push(last_output);
            let amp_run_result = amps[amp].run(&[], skip_init);
            if ! amps[amp].is_halted {
                last_output = amp_run_result;
            }
            println!("Last output was {}", last_output);
            last_amp_halted = amp == 4 && amps[amp].is_halted;
        }
    }
    let g =1 ;
    return last_output;
}
pub fn run_amps_with_inputs(a: isize, b:isize, c:isize, d:isize, e:isize, amps: &mut Vec<IntcodeComputer>, skip_init: bool ) {
    amps[0].inputs = vec![a, 0];
    amps[1].inputs = vec![b];
    amps[2].inputs = vec![c];
    amps[3].inputs = vec![d];
    amps[4].inputs = vec![e];

    let signals = [a,b,c,d,e];
    let mut last_amp_halted = amps[4].current_instruction == 99;
    let mut next_inputs: Vec<usize> = vec![];
    let mut ran = false;

    for amp in 0..5 {
        let mut input_value = 0;
        if amp != 0 && ! ran {
            input_value = amps[amp - 1].get_last_output()
        }
        if ! ran {
            amps[amp].inputs = vec![signals[amp], input_value];
        } else {
            amps[amp].inputs.push( input_value );
        }
        amps[amp].run(&[], skip_init);
        last_amp_halted = amp == 4 && amps[amp].parsed_program[amps[amp].current_instruction] == 99;
    }
}