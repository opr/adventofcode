
    use std::collections::VecDeque;
    #[derive(Debug, Clone)]
    struct FS<'a> {
        root_dir: Dir<'a>,
        cwd: String,
    }

    impl <'a> FS <'a> {

        fn find_dir( &mut self, name: &'a str ) -> Option<&mut Dir<'a>> {

            if name == "/" {
                return Some(&mut self.root_dir);
            }
            let mut queue: VecDeque<&mut Dir<'a>> = VecDeque::new();
            queue.push_back( &mut self.root_dir );

            while let Some(node) = queue.pop_front() {
                let dirs = &mut node.dirs;
                for child in dirs.iter_mut() {
                    if child.path == name {
                        return Some(child);
                    }
                    queue.push_back( child );
                }
            }
            return None;
        }
        fn ensure_root_has_slash(&mut self) {
            if self.cwd.len() == 0 || self.cwd.chars().collect::<Vec<_>>()[0] != '/' {
                self.cwd.insert(0, '/');
            }
        }

        fn change_dir(&mut self, new_dir: &str ) {
            let mut parts = self.cwd.split("/").collect::<Vec<&str>>();
            if new_dir == ".." {
                self.cwd = parts.iter().take( parts.len() - 1 ).cloned().collect::<Vec<&str>>().join("/");
                self.ensure_root_has_slash();
                return;
            }
            parts.push(new_dir);
            self.cwd = parts.clone().iter().cloned().filter(|x| x != &"").collect::<Vec<_>>().join("/");
            self.ensure_root_has_slash();
        }
        fn new() -> Self {
            let root_dir = Dir::new("/".to_string() );
            FS {
                cwd: "/".to_string(),
                root_dir: Dir::new("/".to_string() )
            }
        }
    }

    #[derive(Debug, Clone)]
    struct Dir<'a> {
        path: String,
        files: Vec<File<'a>>,
        dirs: Vec<Self>
    }
    impl<'a> Dir<'a> {
        fn new( path: String ) -> Self {
            Dir {
                path,
                files: Vec::new(),
                dirs: Vec::new()
            }
        }
    }

    #[derive(Debug, Clone)]
    struct File<'a> {
        name: &'a str,
        size: i32,
    }

    impl<'a> File <'a> {
        fn new ( name: &'a str, size: i32 ) -> Self {
            File {
                name,
                size,
            }
        }
    }
    pub fn task() {
        let mut file_system = FS::new();
        let input = crate::input_reader::input_reader::read_input_for_day( 2022, 7, true );
        let output_lines: Vec<&str> = input.split("\n").collect();
        let mut line_pointer = 0;
       // let fs = &mut file_system;
        while line_pointer < output_lines.len() {
            let command_and_lines: Vec<&str> = output_lines.iter()
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
                file_system.change_dir(next_dir.as_str() );
                line_pointer += command_and_lines.len();
            }

            if &command[0..2] == "ls" {
                // process next lines to add dirs and files
                let l = 23;
                for dir_member in command_and_lines.iter().cloned().skip(1).collect::<Vec<&str>>() {
                  if &dir_member[0..3] == "dir" {
                      let dir_name = (&dir_member[4..dir_member.len()].to_string()).clone();
                      let mut name = &file_system.cwd;
                       name.push_str( dir_name.as_str() );
                       let new_dir = Dir::new(  format!("{}", name)  );
                       file_system.root_dir.dirs.push( new_dir );
                      continue;
                  }
                  // Currently a file then
                  let file_info = dir_member.split(" ").collect::<Vec<&str>>();
                  let size = file_info[0];
                  let name = file_info[1];
                  let new_file = File::new( name, size.parse::<i32>().unwrap() );
                    let cwd = &file_system.cwd;
                    let p = file_system.find_dir( &file_system.cwd );
                }
            }

            let b =1;
            line_pointer += command_and_lines.len();
        }

        let b =1;
    }