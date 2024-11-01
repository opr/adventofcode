pub mod input_reader {

    use std::fs;
    pub fn read_input_for_day( year: i32, day: i32, test: bool ) -> String {
        let mut suffix = "";
        if test {
            suffix = "_test"
        }
        let mut file_path = format!("inputs/{}/{}{}.txt", year, day, suffix );
        println!("{}", file_path);
        let contents = fs::read_to_string(file_path)
            .expect("Should have been able to read the file");

        return contents;
    }
}