use crate::input_reader::input_reader::read_input_for_day;

pub fn task() {
    let mut input = read_input_for_day( 2023, 1, false  );

    let replace = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    // replace.iter().enumerate().for_each( |(index, value)| {
    //     input = input.replace(value, &*(index + 1).to_string()) ;
    // } );

    let mut lines: Vec<_> = input.split("\n").collect();

    let string_lines: Vec<_> = lines.iter().cloned().map( | l | {
        let mut s = l.to_string();
        let mut lowest_index = s.len();
        let mut lowest_index_replace_from = "";
        let mut lowest_index_replace_with = 0;
        let mut highest_index = 0;
        let mut highest_index_replace_from = "";
        let mut highest_index_replace_with = 0;
        let mut iter_count = 0;
        let mut low_chars_before = 0;

        let mut high_found = false;
        let mut low_found = false;

        for replacement in replace.iter().cloned() {
            if let Some( index ) = l.find( replacement ) {
                if index <= lowest_index {
                    low_found = true;
                    lowest_index = index;
                    lowest_index_replace_with = iter_count + 1;
                }
            }
            iter_count += 1;
        }
        if low_found {
            s.insert( lowest_index, ('0' as i32 + lowest_index_replace_with ) as u8 as char)
        }

        iter_count = 0;
        for replacement in replace.iter().cloned() {
            if let Some(last_index) = s.rfind(replacement) {
                if last_index >= highest_index {
                    high_found = true;
                    highest_index = last_index;
                    highest_index_replace_with = iter_count + 1;
                }
            }
            iter_count += 1;
        }
        if high_found {
            s.insert( highest_index, ('0' as i32 + highest_index_replace_with ) as u8 as char );
        }
        s
        //return tmp.to_string().replace_range(lowestIndex..lowestIndexReplace.len(), lowestIndexReplace);
    } ).collect();
    let mut a = 0;
    let b: Vec<_>= string_lines.iter().cloned().map( |l| {
        let chars   = l.chars().find(|p| p.is_numeric()).unwrap() as i32 - ('0' as i32);
        let chars_b = l.chars().rev().find( |p| p.is_numeric() ).unwrap() as i32 - ('0' as i32);
        println!("{}, {} {} {}", l, chars, chars_b, lines[a]);
        a+=1;
        return vec![chars,chars_b];
    }).collect();

    let answer = b.iter().fold( 0, |a, b | a + ( b[0] * 10) + b[1] );


    println!("Day 1 task 1: {}", answer)


}