use std::collections::{HashMap, HashSet, VecDeque};
use log::debug;
use crate::input_reader::input_reader::read_input_for_day;

#[derive(Clone,Debug)]
struct LensBox {
    lenses: VecDeque<Lens>,
    id: usize,
}

#[derive(Clone,Debug)]
struct Lens {
    label: String,
    focal_length: usize,
}

pub fn task() {
    let start = std::time::Instant::now();
    let mut input = read_input_for_day( 2023, 15, false );
    let lines: String = input.replace("\n", "" );
    let inputs = lines.split( ',' ).map( | i | i.to_string() ).collect::<Vec<String>>();
    let part_1_answer = inputs.iter().fold( 0usize, | acc, input | acc + hash( input ) );
    println!( "Day 15 task 1: {}", part_1_answer );
    eprintln!("{:?}", start.elapsed() );

    let mut boxes = (0..256).map( | id | LensBox { id, lenses: VecDeque::new(), } ).collect::<Vec<LensBox>>();

    for input in inputs {

        let last_char = input.chars().last().unwrap();
        match last_char {
            '-' => {
                let label = input.replace("-", ""  );
                let box_number = hash( &label.to_string() );
                let found_lens = boxes[ box_number ].lenses.iter().position( | lens | lens.label == label );
                if let Some( lens_index ) = found_lens {
                    boxes[box_number].lenses.remove( lens_index );
                }
            }

            _ => {
                // Plus operation
                let ( label, number ) = input.split_once("=" ).unwrap();
                let box_number = hash( &label.to_string() );
                let focal_length = number.parse::<usize>().unwrap();

                let found_lens = boxes[ box_number ].lenses.iter().position( | lens | lens.label == label );
                let additional_lens = Lens { focal_length, label: label.to_string() };
                if let Some( lens_index ) = found_lens {
                    boxes[box_number].lenses.remove( lens_index );
                    boxes[box_number].lenses.insert( lens_index, additional_lens );
                    continue;
                }

                boxes[ box_number ].lenses.push_back( additional_lens );
            }
        }
    }


    let part_2_answer = boxes.iter().fold( 0usize, | acc, lens_box | {
        if lens_box.lenses.len() == 0 {
            return acc;
        }

        acc + lens_box.lenses.iter().enumerate().fold( 0, | acc, ( lens_index, lens ) | {
            acc + ( 1 + lens_box.id ) * ( lens_index + 1 ) * lens.focal_length
        } )
    } );


    println!( "Day 15 task 2: {}", part_2_answer );
    eprintln!("{:?}", start.elapsed() );

}

fn hash( input: &String ) -> usize {
    let mut current = 0usize;
    input.chars().fold( current, |mut acc, char | {
        let value = char as usize;
        acc += value;
        acc = acc * 17;
        acc = acc % 256;
        return acc;
    } )
}