pub mod task2 {
    use std::collections::HashMap;
    pub struct MapList {
        points_map: HashMap<String, i32>,
        win_map: HashMap<String, String>,
        loss_map: HashMap<String, String>,
        draw_map: HashMap<String, String>
    }

    pub fn get_maps() -> MapList {
        let mut points_map = HashMap::new();
        points_map.insert("X".to_string(), 1 );
        points_map.insert("Y".to_string(), 2 );
        points_map.insert("Z".to_string(), 3 );

        let mut win_map: HashMap<String,String> = HashMap::new();
        win_map.insert("X".to_string(), "C".to_string() );
        win_map.insert("Y".to_string(), "A".to_string() );
        win_map.insert("Z".to_string(), "B".to_string() );

        let mut loss_map: HashMap<String,String> = HashMap::new();
        loss_map.insert("X".to_string(), "B".to_string() );
        loss_map.insert("Y".to_string(), "C".to_string() );
        loss_map.insert("Z".to_string(), "A".to_string() );

        let mut draw_map: HashMap<String,String> = HashMap::new();
        draw_map.insert("A".to_string(), "X".to_string() );
        draw_map.insert("B".to_string(), "Y".to_string() );
        draw_map.insert("C".to_string(), "Z".to_string() );

        return MapList { points_map, win_map, loss_map, draw_map };
    }
    pub fn task_1() {
        let input = crate::input_reader::input_reader::read_input_for_day( 2022,2, false );

        let MapList { points_map, win_map, loss_map, draw_map} = get_maps();

        let games: Vec<&str> = input.split("\n").collect();
        let mut total = 0;

        for game in games {
            let enemy = game.chars().nth(0).unwrap().to_string();
            let me = game.chars().nth(2).unwrap().to_string();

            total += resolve_game(&me, &enemy);
        }

        println!("Day 2 task 1: {}", total);
    }

    pub fn resolve_game( me: &str, enemy: &str ) -> i32 {
        let MapList { points_map, win_map, loss_map, draw_map} = get_maps();
        let mut my_points = points_map.get(me ).unwrap().to_owned();

        let is_win = win_map.get( me ).unwrap() == &enemy;
        let is_lose = loss_map.get( me ).unwrap() == &enemy;

        if is_win {
            return my_points + 6;
        }

        if is_lose {
            return my_points;
        }

        return my_points + 3;
    }

    pub fn task_2() {
        let input = crate::input_reader::input_reader::read_input_for_day( 2022,2, false   );

        let MapList { points_map, win_map, loss_map, draw_map} = get_maps();

        let games: Vec<&str> = input.split("\n").collect();
        let mut total = 0;

        for game in games {
            let enemy = game.chars().nth(0).unwrap().to_string();
            let me = game.chars().nth(2).unwrap().to_string();

            let should_win = me == "Z";
            let should_lose = me == "X";
            let should_draw = !should_lose && !should_win;
            let mut my_value = "";

            if should_win {
                my_value = win_map.iter().find(|entry| { return entry.1 == &enemy } ).unwrap().0;
            }

            if should_lose {
                my_value = loss_map.iter().find(|entry| { return entry.1 == &enemy } ).unwrap().0;
            }

            if should_draw {
                my_value = draw_map.iter().find(|entry| { return entry.0 == &enemy } ).unwrap().1;
            }

            total += resolve_game( my_value, &enemy );
        }

        println!("Day 2 task 2: {}", total);
    }
}