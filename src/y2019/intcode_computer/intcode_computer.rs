pub struct IntcodeComputer {
    pub(crate) program: String,
    pub(crate) parsed_program: Vec<isize>,
    pub(crate) current_instruction: usize,
    pub(crate) inputs: Vec<isize>,
    pub(crate) outputs: Vec<isize>,
    pub(crate) inputs_consumed: usize,
    pub(crate) return_on_output: bool,
    pub(crate) is_halted: bool
}

impl Default for IntcodeComputer {
    fn default() -> Self {
        IntcodeComputer {
            inputs_consumed: 0,
            program: "".to_string(),
            current_instruction: 0,
            inputs: vec![],
            outputs: vec![],
            parsed_program: vec![],
            return_on_output: false,
            is_halted: false,
        }
    }
}

impl IntcodeComputer {

    pub(crate) fn get_last_output( &self ) -> isize {
        if self.outputs.iter().len() == 0 {
            return 0;
        }
        return *self.outputs.iter().rev().collect::<Vec<_>>()[0];
    }

    pub(crate) fn print_computer( &self ) {
        println!("{}", self.parsed_program.iter().cloned().map(|x| x.to_string() ).collect::<Vec<_>>().join(",") );
    }

    fn resolve_value( &self, pos: &isize, mode: &isize ) -> isize {
        // Mode might change here so lets handle if it does, for now we just resolve pointer.
        match mode {
            0 => {
                return self.parsed_program[ *pos as usize ];
            }
            1 => {
                return *pos;
            }
            _ => { panic!("unrecognised mode {}", mode)}
        }
    }

    pub(crate) fn reset_computer(&mut self) {
        self.current_instruction = 0;
        self.parse();
        self.inputs = vec![];
        self.inputs_consumed = 0;
        self.outputs = vec![];
        self.is_halted = false;
    }

    pub(crate) fn parse(&mut self) {
        let codes = self.program.split(",").map(|c| c.parse::<isize>().unwrap() ).collect::<Vec<isize>>();
        self.parsed_program = codes;
    }

    pub(crate) fn run( &mut self, input_map: &[( isize , isize)], skip_init: bool ) -> isize {
        if ! skip_init {
            self.current_instruction = 0;
            self.parse();
        }
        input_map.iter().cloned().enumerate().for_each(|(index, (position, value))| self.parsed_program[position as usize] = value);
        let length = self.parsed_program.len();
        let mut last_instruction = 0;

        while ! self.is_halted && self.current_instruction < length {
            let instruction = &self.parsed_program[self.current_instruction];
            let mut opcode = instruction % 10;

            if *instruction == 99isize {
                opcode = 99;
            }

            last_instruction = opcode;

            let opcode_modes = [ (instruction / 100 ) % 10, (instruction / 1000) % 10, (instruction / 10000) % 10 ];

            match opcode {

                // Add / Multiply
                1 | 2 => {
                    let slice = &self.parsed_program[self.current_instruction + 1..=self.current_instruction + 3];
                    let [_, _, raw_op3 ] = slice else { panic!("Opcodes went wrong") };
                    let resolved: Vec<isize> = slice.iter().enumerate().map(|(index, code)| self.resolve_value(code, &opcode_modes[index])).collect();
                    let mut result = 0;
                    if opcode == 1isize {
                        result = resolved[0] + resolved[1];
                    }

                    if opcode == 2isize {
                        result = resolved[0] * resolved[1];
                    }

                    let op3 = *raw_op3;
                    self.parsed_program[op3 as usize] = result;
                    self.set_pointer(self.current_instruction + 4);

                    continue;
                }

                // Input
                3 => {
                    let param = self.parsed_program[self.current_instruction + 1];
                    let input = self.inputs[self.inputs_consumed];
                    self.inputs_consumed+=1;
                    self.parsed_program[param as usize] = input;
                    self.set_pointer(self.current_instruction + 2);
                    continue;
                }

                // Output
                4 => {
                    let param = self.resolve_value(&((self.current_instruction + 1) as isize), &(opcode_modes[0])) as usize;
                    self.outputs.push(self.parsed_program[param]);
                    self.set_pointer(self.current_instruction + 2);
                    if self.return_on_output {
                        return self.parsed_program[param];
                    }
                    continue;
                }

                // Jump if true / false
                5 | 6 => {
                    let slice = &self.parsed_program[self.current_instruction + 1..=self.current_instruction + 2];
                    // let [_, raw_op2 ] = slice else { panic!("Opcodes went wrong")};
                    let resolved: Vec<isize> = slice.iter().enumerate().map(|(index, code)| self.resolve_value(code, &opcode_modes[index])).collect();

                    if opcode == 5 && resolved[0] > 0 {
                        self.set_pointer( resolved[1] as usize );
                        continue;
                    }
                    if opcode == 6 && resolved[0] == 0 {
                        self.set_pointer( resolved[1] as usize );
                        continue;
                    }
                    self.set_pointer(self.current_instruction + 3);
                    continue;
                }
                // Less than / Equal
                7 | 8 => {
                    let slice = &self.parsed_program[self.current_instruction + 1..=self.current_instruction + 3];
                    // let [_, raw_op2 ] = slice else { panic!("Opcodes went wrong")};
                    let resolved: Vec<isize> = slice.iter().enumerate().map(|(index, code)| self.resolve_value(code, &opcode_modes[index])).collect();
                    let insert_into = slice[2] as usize;
                    self.parsed_program[insert_into] = 0;
                    if opcode == 7 && resolved[0] < resolved[1] {
                        self.parsed_program[insert_into] = 1;
                    }
                    if opcode == 8 && resolved[0] == resolved[1] {
                        self.parsed_program[insert_into] = 1;
                    }
                    self.set_pointer(self.current_instruction + 4);
                    continue;
                }

                99 => {
                    self.is_halted = true;
                    continue;
                }

                _ => {
                    println!("unresolved opcode {}", opcode);
                    self.set_pointer( self.current_instruction + 1 );
                    continue;
                }
            }
        }
        return -1;//self.parsed_program[0];
    }

    fn set_pointer( &mut self, new_index: usize ) {
        self.current_instruction = new_index;
    }
}