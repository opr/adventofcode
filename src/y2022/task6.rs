pub mod task6 {
    use std::collections::HashSet;

    pub fn task() {
        let input = crate::input_reader::input_reader::read_input_for_day(2022, 6, false );

        let chars = input.chars().collect::<Vec<char>>();
        let answer = find_start_marker( 4, chars.clone() );
        let answer_2 = find_start_marker( 14, chars.clone() );
        println!("Day 6 task 1: {}", answer );
        println!("Day 6 task 2: {}", answer_2 );
    }

    pub fn find_start_marker( packet_len: usize, char_vec: Vec<char> ) -> usize {
        for i in 0..char_vec.len() {
            let next_four = char_vec.iter().skip(i ).take( packet_len ).cloned().collect::<Vec<char>>();
            let mut char_hash: HashSet<char> = HashSet::new();
            char_hash.extend( next_four );
            if char_hash.len() != packet_len {
                continue;
            }
            return i+packet_len;
        }
        return 1;
    }
}