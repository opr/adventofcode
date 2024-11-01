pub mod task4 {
    use std::collections::{HashSet};
    use std::hash::Hash;

    pub fn task() {
        let input = crate::input_reader::input_reader::read_input_for_day( 2022,4, false );
        let pairs: Vec<&str> = input.split("\n").collect();
        let mut duplicate_elves = 0;

        for pair in pairs {
            let elves: Vec<&str> = pair.split(",").collect();
            let range1: Vec<i32> = elves.iter().nth(0).unwrap().split("-").map(|x| x.parse::<i32>().unwrap()).collect();
            let range2: Vec<i32> = elves.iter().nth(1).unwrap().split("-").map(|x| x.parse::<i32>().unwrap()).collect();

            let mut set1: HashSet<i32> = HashSet::new();
            let mut set2: HashSet<i32> = HashSet::new();

            for i in range1[0]..=range1[1] {
                set1.insert( i );
            }
            for i in range2[0]..=range2[1] {
                set2.insert( i );
            }
            if set1.is_subset( &set2 ) || set2.is_subset( &set1 ) {
                duplicate_elves += 1;
            }
        }

        println!("Day 4 task 1: {}", duplicate_elves);
    }
    pub fn task_2() {
        let input = crate::input_reader::input_reader::read_input_for_day( 2022,4, false );
        let pairs: Vec<&str> = input.split("\n").collect();
        let mut duplicate_elves = 0;

        for pair in pairs {
            let elves: Vec<&str> = pair.split(",").collect();
            let range1: Vec<i32> = elves.iter().nth(0).unwrap().split("-").map(|x| x.parse::<i32>().unwrap()).collect();
            let range2: Vec<i32> = elves.iter().nth(1).unwrap().split("-").map(|x| x.parse::<i32>().unwrap()).collect();

            let mut set1: HashSet<i32> = HashSet::new();
            let mut set2: HashSet<i32> = HashSet::new();

            for i in range1[0]..=range1[1] {
                set1.insert( i );
            }
            for i in range2[0]..=range2[1] {
                set2.insert( i );
            }
            if set1.intersection( &set2 ).collect::<Vec<_>>().len() > 0 {
                duplicate_elves += 1;
            }
        }

        println!("Day 4 task 2: {}", duplicate_elves);
    }
}