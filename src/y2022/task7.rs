#[derive(Debug,Clone)]
struct Node {
    name: String,
    dirs: Vec<Node>,
    files: Vec<File>,
    size: i32,
}

impl Node {

    fn dirs_to_vec( self ) -> Vec<i32> {
        let mut vec: Vec<i32> = vec![ self.size ];

        for dir in &self.dirs {
            vec.append( &mut dir.clone().dirs_to_vec() )
        }
        return vec;
    }

    fn print_tree ( &self, depth: usize ) {
        println!("{:indent$}- {} (dir, size={})", "", self.name, self.size, indent = depth * 2 );

        for dir in &self.dirs {
            dir.print_tree( depth + 1 );
        }

        for file in &self.files {
            println!("{:indent$}- {} (file, size={})", "", file.name, file.size, indent = ( depth + 1 ) * 2 )
        }
    }

    fn update_size ( &mut self, depth: usize ) -> i32 {
        let mut size = 0i32;

        for dir in &mut self.dirs {
            dir.update_size( depth + 1 );
        }
        size += self.files.iter().fold(0, |a, file | a + file.size );
        size += self.dirs.iter().fold(0, |a, dir | a + dir.size );
        self.size = size;
        return size;
    }

    fn find_dirs_less_than_100k_total( &self ) -> i32 {
        let mut size = 0i32;

        println!("Checking {}, to see if its less than 100k", self.name );

        if self.size <= 100000 {
            size += self.size;
        }

        for dir in &self.dirs {
            size += dir.find_dirs_less_than_100k_total();
        }
        println!("In {} the size is: {}", self.name, size );
        return size;
    }

    fn find_dir_by_path( &mut self, path: Vec<&str> ) -> Option<&mut Node> {
        if path.len() == 0 {
            return Some( self );
        }
        else {
            let next_dir_name = path[0];
            for dir in &mut self.dirs {
                if dir.name == next_dir_name {
                    return dir.find_dir_by_path( path.iter().skip( 1 ).cloned().collect() )
                }
            }
        }
        return None;
    }
}

#[derive(Debug,Clone)]
struct File {
    size: i32,
    name: String,
}

pub fn task() {

    let mut root_dir = Node {
        files: vec![],
        dirs: vec![],
        name: "/".to_string(),
        size: 0,
    };

    let input = crate::input_reader::input_reader::read_input_for_day( 2022,7, false );
    let output_lines: Vec<&str> = input.split("\n").collect();
    let mut line_pointer = 0;
    let mut cwd: String = String::from("");

    while line_pointer < output_lines.len() {
        let mut command_and_lines: Vec<&str> = output_lines.iter()
            .cloned()
            .skip(line_pointer)
            .enumerate()
            .peekable()
            .take_while( |&(index, line)|
                {
                    index == 0 || line.chars().collect::<Vec<_>>()[0] != '$'
                }
            )
            .map(|(index,value)|value)
            .collect::<Vec<&str>>();

        let command = command_and_lines[0].replace("$ ", "");
        if &command[0..2] == "cd" {
            let cloned_command = command.to_string();
            let next_dir = cloned_command[3..command.len() ].to_string();
            cwd = change_dir( &cwd, &next_dir );
            line_pointer += command_and_lines.len();
            continue;
        }

        if &command[0..2] == "ls" {
            let dir_members = command_and_lines.iter().skip(1).cloned().collect::<Vec<&str>>();
            let mut parent_dir = root_dir.find_dir_by_path( cwd.split("/").filter(|x| *x != "" ).collect::<Vec<&str>>() ).unwrap();//find_dir( &cwd, &mut root_dir );

            for line in dir_members {
                if &line[0..3] == "dir" {
                    add_dir( &mut parent_dir, Node {
                        dirs: vec![],
                        files: vec![],
                        name: line[4..line.len()].to_string(),
                        size: 0
                    } );
                    continue;
                }
                let (file_size, file_name) = line.split_once(" ").unwrap();
                add_file(&mut parent_dir, File {
                    name: file_name.to_string(),
                    size: file_size.parse::<i32>().unwrap()
                } );
            }

        }
        line_pointer += command_and_lines.len();
    }

    root_dir.update_size( 0 );
    root_dir.print_tree( 0 );
    println!("{}", root_dir.find_dirs_less_than_100k_total() );

    let total_disk_space = 70000000;
    let free_space = total_disk_space - root_dir.size;
    let needed_space = 30000000 - free_space;
    let mut sizes = root_dir.dirs_to_vec();
    sizes = sizes.iter().cloned().filter( |x| *x >= needed_space ).collect();
    sizes.sort();
    println!( "Day 7 task 2: {}", sizes[0] );
}

fn add_dir(node: &mut Node, value: Node ) {
    node.dirs.push( value );
}
fn add_file( node: &mut Node, value: File ) {
    node.files.push( value );
}

fn find_dir<'a>( cwd: &String, mut root_dir: &'a mut Node ) -> &'a mut Node {
    let mut split_cwd = cwd.split("/").filter(|x| *x != "" ).collect::<Vec<&str>>();
    if split_cwd.len() == 0  || cwd == "/" {
        return root_dir;
    }
   let mut found = &mut root_dir.dirs.iter().cloned().find( |m| m.name == split_cwd[0] ).unwrap();

   split_cwd = split_cwd.iter().cloned().skip(1).collect::<Vec<&str>>();

    let h = 1;
   // return find_dir( &split_cwd.join("/"), found );

    return root_dir;
}

fn change_dir ( cwd: &String, new_dir: &String ) -> String {
    let mut split_cwd = cwd.split("/").filter(|x| *x != "" ).collect::<Vec<&str>>();
    if new_dir == "/" {
        return "/".to_string()
    }

    if new_dir == ".." {
        if split_cwd.len() == 0 {
            return "/".to_string()
        }
        split_cwd.pop();
        split_cwd.insert(0, "" );

        if split_cwd.len() == 1 {
            return "/".to_string()
        }

        return split_cwd.join("/");
    }
    split_cwd.insert(0, "" );
    split_cwd.push( new_dir );
    return split_cwd.join("/");
}