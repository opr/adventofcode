pub mod task3 {
    use std::collections::{HashMap, HashSet};

    pub fn get_lookup () -> HashMap<char, i32> {
        let mut lookup = HashMap::new();

        for (index, letter) in ('a'..='z').enumerate() {
            lookup.insert(letter, (index+1) as i32);
        }
        for (index, letter) in ('A'..='Z').enumerate() {
            lookup.insert(letter, (index+27) as i32);
        }
        lookup
    }

    pub fn task() {
        let input = crate::input_reader::input_reader::read_input_for_day(2022, 3, false );
        let rucksacks: Vec<&str> = input.split("\n").collect();
        let mut total = 0;

        let lookup = get_lookup();


        for sack in rucksacks {
            total += process_rucksack(sack, lookup.clone() );
        }
        println!("Day 3 task 1: {}", total)
    }

    pub fn task_2() {
        let input = crate::input_reader::input_reader::read_input_for_day(2022, 3, false );
        let rucksacks: Vec<&str> = input.split("\n").collect();

        let lookup = get_lookup();

        let group_count = (rucksacks.len() / 3 ) as i32;
        let mut total = 0;

        for group in 0..group_count {
            let current_group: Vec<&str> = rucksacks.iter().cloned().skip( (group * 3) as usize ).take(3 ).collect();
            let set1: HashSet<char> = current_group.iter().nth(0).unwrap().chars().collect();
            let set2: HashSet<char> = current_group.iter().nth(1).unwrap().chars().collect();
            let set3: HashSet<char> = current_group.iter().nth(2).unwrap().chars().collect();

            let intersection1_result: HashSet<_> = set1.intersection(&set2).cloned().collect();
            let final_intersection: HashSet<_> = intersection1_result.intersection( &set3 ).cloned().collect();
            let common_group = final_intersection.iter().nth(0).unwrap();

            total += lookup.get(common_group).cloned().unwrap();
        }
        println!("Day 3 task 2 {}", total);

    }
    pub fn process_rucksack(rucksack: &str, lookup: HashMap<char, i32> ) -> i32 {
        if rucksack == "" {
            return 0;
        }

        let mut parts = rucksack.split_at( rucksack.len()/2 );
        let set1: HashSet<_> = parts.0.to_string().chars().collect();
        let set2: HashSet<_> = parts.1.to_string().chars().collect();

        let intersection: &char = set1.intersection( &set2 ).nth(0).unwrap();

        return lookup.get(&intersection).cloned().unwrap();
    }
}