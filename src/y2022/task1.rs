pub mod task1 {
    pub fn task() {
        let input = crate::input_reader::input_reader::read_input_for_day( 2022,1, false );

        let elves: Vec<&str> = input.split("\n\n").collect();

        let foods: Vec<Vec<i32>> = elves.iter().map( | item | {
          let individual_foods: Vec<i32> = item.split("\n").map( |food_str| {
              food_str.parse::<i32>().unwrap()
          } ).collect();
           return individual_foods;
        } ).collect();

        let mut food_totals: Vec<i32> = foods.iter().map( | elf_food | {
            return elf_food.iter().copied().reduce( | a, b | a + b).unwrap();
        } ).collect();

        food_totals.sort_by( | a, b | b.cmp(a));
        println!("Day 1 task 1 answer: {}", food_totals[0]);
        println!("Day 1 task 2 answer: {}", food_totals.iter().take(3).sum::<i32>())
    }
}