use std::collections::{HashMap, VecDeque};
use std::process::id;
use crate::input_reader::input_reader::read_input_for_day;
use crate::y2023::day20::FlipFlopState::{Off, On};
use crate::y2023::day20::ModuleType::{Broadcaster, Conjunction, FlipFlop};
use crate::y2023::day20::PulseType::{Hi, Lo};

#[derive(Clone,Debug)]
struct Module {
    id: String,
    module_type: ModuleType,
    inputs: HashMap<String, PulseType>,
    outputs: Vec<String>,
    flip_flop: FlipFlopState,
    last_received_pulse: PulseType,
    number_of_presses_to_emit_high: usize,
}

impl Module {
    fn are_all_inputs_high( &self ) -> bool {
        self.inputs.iter().all( | (_, pulse_type) | match pulse_type { Hi => true, _ => false } )
    }
}
#[derive(Clone,Debug)]
struct Pulse {
    origin: String,
    pulse_type: PulseType,
}

#[derive(Clone, Debug)]
enum ModuleType {
    Broadcaster,
    FlipFlop,
    Conjunction
}
#[derive(Clone, Debug)]
enum FlipFlopState {
    On,
    Off
}
#[derive(Clone, Debug)]
enum PulseType {
    Hi,
    Lo
}

pub fn task() {
    let input = read_input_for_day(2023, 20, false);
    run_task( input );
}

fn run_task( input: String ) -> usize {
    let mut modules: HashMap<String, Module> = HashMap::from_iter( input.split("\n" ).map( |module | {
        let input_parts = module.split(" -> ").collect::<Vec<&str>>();
        let mut id = input_parts[0].to_string();
        let outputs = input_parts[1].split( ", " ).map( | s | s.to_string() ).collect::<Vec<String>>();
        if id == "broadcaster" {
            return (id.clone(), Module {
                last_received_pulse: Lo,
                id: id.clone(),
                module_type: Broadcaster,
                inputs: HashMap::new(),
                outputs,
                number_of_presses_to_emit_high: 0,
                flip_flop: FlipFlopState::Off
            } );
        }

        // Derive module type
        let module_type = match id.chars().nth(0).unwrap() {
            '%' => FlipFlop,
            '&' => Conjunction,
            _ => panic!("unexpected input"),
        };
        id = id.chars().skip(1).collect::<String>();

        ( id.clone(), Module {
            last_received_pulse: Lo,
            id: id.clone(),
            module_type,
            inputs: HashMap::new(),
            outputs,
            number_of_presses_to_emit_high: 0,
            flip_flop: FlipFlopState::Off
        } )

    } ) );
    for ( id, module ) in modules.clone() {
        for o in &module.outputs {
            if let Some( found ) = modules.get_mut( o.as_str() ) {
                found.inputs.insert(id.to_string(), Lo);
            }
        }
    }
    println!(
        "ddf"
    );
    let mut queue: VecDeque<(String, Pulse)> = VecDeque::new();
    let mut pulses: Vec<Pulse> = Vec::new();

    let state = get_state( &modules );
    let mut cycle = 0;
    loop {
        cycle = cycle + 1;
        queue.push_back(("broadcaster".to_string(), Pulse { origin: "button".to_string(), pulse_type: Lo }));
        process_queue(&mut queue, &mut modules, &mut pulses, cycle );

        let kh = modules.get( "kh" ).unwrap();
        if kh.are_all_inputs_high() {
            panic!("djdj")
        }

        while queue.len() > 0 {
            //println!("pushing button {}", cycle);
            //println!("Items in queue are:{:?}", queue.iter().map(|(q_id, _)| q_id).collect::<Vec<_>>());
            process_queue(&mut queue, &mut modules, &mut pulses, cycle);
            // if get_state(&modules) == state {
            //     let (_, last_pulse) = queue.pop_back().unwrap();
            //     pulses.push(last_pulse);
            //     println!("States matched");
            //     break;
            // }

            // let pv = modules.get( "pv" ).unwrap();
            // let qh = modules.get( "qh" ).unwrap();
            // let xm = modules.get( "xm" ).unwrap();
            // let hz = modules.get( "hz" ).unwrap();
            //
            // if pv.are_all_inputs_high() && qh.are_all_inputs_high() && xm.are_all_inputs_high() && hz.are_all_inputs_high() {
            //     println!("cycles {}", cycle);
            //     panic!()
            // }


        }
    }

    let pulse_counts = pulses.iter().fold(
        ( 0, 0 ),
        | ( lo, hi ), pulse |
            match pulse.pulse_type {
                Lo => ( lo + 1, hi ), Hi => ( lo, hi + 1 )
            } );

    println!("{:?}", modules);
    let answer = pulse_counts.1 * pulse_counts.0;
    println!("answer: {}", answer);
    return 0;
}

fn get_state( modules: &HashMap<String, Module> ) -> String {
    modules.iter().map( | ( id, m ) | {
        let mut module_state = id.to_string();;
        module_state.push_str( match m.module_type { Broadcaster => "broadcaster", FlipFlop => "flipflop", Conjunction => "conjunction" }  );
        let input_state = m.inputs.iter().map( | ( input_id, pulse_type ) | {
            let mut result = "{".to_string();
            result.push_str( input_id.clone().as_str() );
            result.push_str( match pulse_type { Lo => "lo", Hi => "hi" } );
            result.push_str( "}," );
            result
        } ).collect::<Vec<String>>();
        module_state.push('[');
        module_state.push_str( match m.flip_flop {
            Off => "off",
            On => "on"
        } );
        module_state.push(']');
        module_state.push('<');
        module_state.push_str( match m.last_received_pulse {
            Lo => "lo",
            Hi => "hi"
        } );
        module_state.push('>');
        module_state.push_str( input_state.concat().as_str() );
        module_state.push( ',' );

        module_state
    }).collect::<Vec<String>>().concat()
}

fn process_queue( queue: &mut VecDeque<(String, Pulse)>, modules: &mut HashMap<String, Module>, pulses: &mut Vec<Pulse>, number_of_presses: usize ) {
    let (module_id, pulse) = queue.pop_front().unwrap();
    pulses.push( pulse.clone() );
    let module_opt = modules.get_mut(&module_id);
    if module_opt.is_none() {
        return;
    }
    let mut module = module_opt.unwrap().clone();

    let outputs: Vec<String> = module.outputs.clone();

    let mut next_pulse = Pulse{origin: module_id.clone(), pulse_type: Lo};

    match &module.module_type {
        Conjunction => {
            let cloned_pulse = pulse.clone();
            module.inputs.insert(cloned_pulse.origin, cloned_pulse.pulse_type);
            let all_inputs_high = module.are_all_inputs_high();
            if all_inputs_high {
                next_pulse.pulse_type = Lo;
            } else {
                if module.number_of_presses_to_emit_high == 0 {
                    module.number_of_presses_to_emit_high = number_of_presses;
                }
                next_pulse.pulse_type = Hi;
            }
        },
        FlipFlop => {
            match pulse.pulse_type {
                Hi => {
                    return;
                },
                Lo => {
                    match module.flip_flop {
                        On => {
                            next_pulse.pulse_type = Lo;
                            module.flip_flop = Off
                        },
                        Off => {
                            next_pulse.pulse_type = Hi;
                            module.flip_flop = On
                        },
                    }
                }
            }
        },
        Broadcast => {},
    }

    if outputs.len() == 0 {
        let pt = match next_pulse.pulse_type {
            Lo => "lo",
            Hi => "hi",
        };
        //println!("adding {} pulse from {} to no output", pt, module_id.clone());
        pulses.push(next_pulse.clone());
    }

    modules.insert(module_id.clone(), module);
    for o in &outputs {
        let pt = match next_pulse.pulse_type {
            Lo => "lo",
            Hi => "hi",
        };
        if outputs.len() == 1 && pt == "lo" && o == "rx" {
            println!("found it lol");
            panic!("found it!")
        }
        //println!("adding {} pulse from {} to {}", pt, module_id, o);
        queue.push_back((o.clone(), next_pulse.clone()));
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(run_task("broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a".to_string()),32000000);
    }
}
