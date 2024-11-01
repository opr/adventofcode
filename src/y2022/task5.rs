pub mod task5 {

    pub struct MoveInstruction {
        from: usize,
        to: usize,
        amount: usize
    }

    impl MoveInstruction {
        fn new( from: usize, to: usize, amount: usize ) -> MoveInstruction {
            MoveInstruction {from, to, amount}
        }
    }
    pub fn task() {
        let input = crate::input_reader::input_reader::read_input_for_day( 2022,5, false );
        let input_split: Vec<&str> = input.split("\n\n").collect();

        let instructions: Vec<&str> = input_split[1].split("\n").collect();
        let crates: Vec<&str> = input_split[0].split("\n").collect::<Vec<&str>>().iter().rev().skip(1).rev().cloned().collect();
        let cols: u32 = input_split[0].split("\n").collect::<Vec<&str>>().iter().rev().take(1).cloned().collect::<Vec<&str>>()[0].chars().rev().collect::<Vec<char>>()[0].to_digit(10).unwrap();

        let mut crate_list: Vec<Vec<char>> = (0..cols).fold( Vec::new(), | mut n, i | {
            n.push( get_crates_for_col( ((4 * i) + 1)  as usize, &crates ) );
            return n;
        } );

        let mut num_instructions: Vec<MoveInstruction> = Vec::new();

        for instruction in instructions {
            let numbers: String = instruction.replace( "move ", "" ).replace(" from ", " " ).replace(" to ", " ");
            let individual_instructions: Vec<usize> = numbers.split(" ").map( |c| {
                return c.parse::<usize>().unwrap();
            }).collect::<Vec<usize>>();
            num_instructions.push(MoveInstruction::new( individual_instructions[1], individual_instructions[2], individual_instructions[0]) );
            crate_list = process_instruction( MoveInstruction::new( individual_instructions[1], individual_instructions[2], individual_instructions[0]), &crate_list ).clone();
        }

        let answer: String = crate_list.iter().map( | col | col.get(0).cloned().unwrap() ).collect::<Vec<char>>().iter().collect();
        println!("Day 5 task 1 answer: {}", answer)
    }

    pub fn process_instruction( i: MoveInstruction, crate_list: &Vec<Vec<char>> ) -> (Vec<Vec<char>>) {
        let mut new_crate_list = crate_list.clone();
        let mut from_col = crate_list.get(i.from-1).cloned().unwrap();
        let mut to_col = crate_list.get(i.to-1).cloned().unwrap();
        let mut taken: Vec<char> = from_col.iter().take( i.amount ).cloned().collect::<Vec<char>>();
        from_col = from_col.iter().skip( i.amount ).cloned().collect();
        new_crate_list[i.from-1] = from_col;
        let mut temp_to: Vec<char> = taken.to_owned();
      //  temp_to.reverse();
        temp_to.extend( to_col );
        to_col = temp_to;
        new_crate_list[i.to-1] = to_col;
        let b = 1;
        return new_crate_list;
    }

    pub fn get_crates_for_col( col: usize, crates: &Vec<&str> ) -> Vec<char> {
        return crates.iter().map( | c | {
            let chars: Vec< char > = c.chars().collect();
            if chars.len() <= col || chars[col] == ' ' {
                // No crate at first col, skip
                return ' ';
            }
            return chars[col];
        }).filter( | &x| x != ' ').collect();
    }
}