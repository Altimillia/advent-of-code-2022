use std::{path::{Path, PathBuf}};

pub fn part_one(input: String) -> usize { 
    let fs = FileSystem { root: Directory { name: "/".to_string(), files: Vec::new(), directories: Vec::new() }};
    let commands = parse_commands(input); 
    let filled_system = process_commands(commands, fs);
    return filled_system.root.get_child_directories_at_or_below_size(100000).into_iter().map(|dir| dir.get_directory_size()).sum();
}

pub fn part_two(input: String) -> usize {
    const TOTAL_SIZE:usize = 70000000;
    const UPDATE_SIZE:usize = 30000000;

    let fs = FileSystem { root: Directory { name: "/".to_string(), files: Vec::new(), directories: Vec::new() }};
    let filled_system = process_commands(parse_commands(input), fs);

    let unusued_space = TOTAL_SIZE - filled_system.root.get_directory_size();
    let space_needed_for_update = UPDATE_SIZE - unusued_space;

    let mut viable_directories = filled_system.root.get_child_directories_at_or_above_size(space_needed_for_update);
    viable_directories.sort_by(|a,b| a.get_directory_size().cmp(&b.get_directory_size()));

    let lowest = viable_directories.first().unwrap().get_directory_size();

    return lowest;
}

fn parse_commands(input: String) -> Vec<Commands> {
    return input.lines().map(|command| {
        let terms:Vec<&str> = command.split_whitespace().collect();
        let split = terms.as_slice();
        if split[0].starts_with("$") {
            if split[1].starts_with("cd") && split[2].starts_with("..") {
                return Commands::ChangeDirectoryUp;
            }
            if split[1].starts_with("cd") {
                return Commands::ChangeDirectory(split[2].to_string());
            }
            if split[1].starts_with("ls") {
                return Commands::List;
            }
        }
        if split[0].starts_with("dir") {
            return Commands::AddDirectory(split[1].to_string());
        }
        return Commands::AddFile(split[1].to_string(), split[0].to_string().parse::<usize>().unwrap())
    }).collect();
}

fn process_commands(commands:Vec<Commands>, mut fs:FileSystem) -> FileSystem { 
    let mut path = PathBuf::from("/");

    commands.into_iter().for_each(|command| {
        match command {
            Commands::ChangeDirectory(dir_name) => path.push(dir_name),
            Commands::ChangeDirectoryUp =>  { path.pop(); return ();},
            Commands::List => { return () },
            Commands::AddFile(file_name, size) => fs.add_file_to_directory_at_path(path.as_path(), File { size: size, name: file_name }),
            Commands::AddDirectory(dir_name) => fs.add_directory_to_directory_at_path(path.as_path(), dir_name)
        }
    });

    return fs;
}

enum Commands {
    ChangeDirectory(String),
    ChangeDirectoryUp,
    List,
    AddFile(String, usize),
    AddDirectory(String)
}

struct FileSystem {
    root: Directory
}

impl FileSystem {

    pub fn add_directory_to_directory_at_path(&mut self, directory_path: &Path, name: String) { 
        let mut path_stack:Vec<&str> = directory_path.components().map(|x| x.as_os_str().to_str().unwrap()).collect();

        FileSystem::add_directory_to_directory_recursive(&mut self.root, &mut path_stack, name);
    }

    pub fn add_file_to_directory_at_path(&mut self, directory_path: &Path, child_file: File) { 
        let mut path_stack:Vec<&str> = directory_path.components().map(|x| x.as_os_str().to_str().unwrap()).collect();

        FileSystem::add_file_to_directory_recursive(&mut self.root, &mut path_stack, child_file);
    }

    fn add_file_to_directory_recursive(directory: &mut Directory, path_stack: &mut Vec<&str>, child_file: File) {
        if path_stack.len() > 1 {
            path_stack.remove(0);
            FileSystem::add_file_to_directory_recursive(directory.get_child_directory_mut(path_stack.first().unwrap().to_string()), path_stack, child_file);
        }
        else {
            directory.add_child_file(File { name: child_file.name.to_string(), size: child_file.size });
        }
    }

    
    fn add_directory_to_directory_recursive(directory: &mut Directory, path_stack: &mut Vec<&str>, name: String) {
        if path_stack.len() > 1 {
            path_stack.remove(0);
            FileSystem::add_directory_to_directory_recursive(directory.get_child_directory_mut(path_stack.first().unwrap().to_string()), path_stack, name);
        }
        else {
            directory.add_child_directory(Directory { files: Vec::new(), directories: Vec::new(), name: name });
        }
    }

    pub fn get_directory_at_path(&self, directory_path: &Path) -> &Directory {
        let mut current_directory = &self.root;

        directory_path
            .components()
            .for_each(|f| {
                let directory_name =  f.as_os_str().to_str().unwrap();
                if directory_name != r#"/"# {
                    current_directory.directories.iter().for_each(|dir| {
                        if directory_name == dir.name {
                            current_directory = dir;
                        }
                    });
                }

        });

        return &current_directory;
    }
}


struct Directory {
    files: Vec<File>,
    directories: Vec<Directory>,
    name: String
}

impl Directory {
    pub fn get_directory_size(&self) -> usize {
        let files_size = self.files.iter().map(|x| x.size).sum::<usize>();
        let directories_size = self.directories.iter().map(|x| x.get_directory_size()).sum::<usize>();
        return files_size + directories_size;
    }

    pub fn get_child_directories_at_or_below_size(&self, size:usize) -> Vec<&Directory> {
        let mut directories:Vec<&Directory> = self.directories.iter().filter(|f| f.get_directory_size() < size).collect();
        let mut child_directory_recursive:Vec<&Directory> = self.directories.iter().map(|dir| dir.get_child_directories_at_or_below_size(size)).flatten().collect();
        directories.append(&mut child_directory_recursive);

        return directories;
    }

    pub fn get_child_directories_at_or_above_size(&self, size:usize) -> Vec<&Directory> {
        let mut directories:Vec<&Directory> = self.directories.iter().filter(|f| f.get_directory_size() >= size).collect();
        let mut child_directory_recursive:Vec<&Directory> = self.directories.iter().map(|dir| dir.get_child_directories_at_or_above_size(size)).flatten().collect();
        directories.append(&mut child_directory_recursive);

        return directories;
    }

    pub fn add_child_directory(&mut self, directory: Directory) {
        self.directories.push(directory);
    }

    pub fn add_child_file(&mut self, file: File) { 
        self.files.push(file);
    }

    pub fn get_child_directory_mut(&mut self, name: String) -> &mut Directory { 
        return self.directories.iter_mut().find(|p| p.name == name).unwrap();
    }

    pub fn get_child_directory(&mut self, name: String) -> &Directory { 
        return self.directories.first().unwrap();

    }
}

struct File {
    size: usize,
    name: String
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::solutions::day07::{FileSystem, Directory};

    use super::File;
    
    #[test]
    fn file_system_can_have_multiple_directories_off_root() {
        let mut file_system = FileSystem { root: Directory { files: Vec::new(), directories: Vec::new(), name: "/".to_string() } };

        file_system.root.add_child_directory(Directory { files: Vec::new(), directories: Vec::new(), name: "abc".to_string()} );
        file_system.root.add_child_directory(Directory { files: Vec::new(), directories: Vec::new(), name: "xkc".to_string()} );

        assert_eq!(file_system.root.directories.len(), 2);
    }

    #[test]
    fn files_can_be_added_to_directory() {
        let mut file_system = FileSystem { root: Directory { files: Vec::new(), directories: Vec::new(), name: "/".to_string() } };
        
        file_system.root.add_child_file(File { name: "phone.exe".to_string(), size: 299011 });

        assert_eq!(file_system.root.files.len(), 1);
    }

    #[test]
    fn files_can_be_summed_in_directory() {
        let mut file_system = FileSystem { root: Directory { files: Vec::new(), directories: Vec::new(), name: "/".to_string() } };
        const FILE_SIZE:usize = 299011;

        file_system.root.add_child_file(File { name: "phone.exe".to_string(), size: FILE_SIZE });

        assert_eq!(file_system.root.get_directory_size(), FILE_SIZE);
    }

    #[test]
    fn directory_can_be_found_by_path() {
        let mut file_system = FileSystem { root: Directory { files: Vec::new(), directories: Vec::new(), name: "/".to_string() } };

        file_system.root.add_child_directory(Directory { files: Vec::new(), directories: vec![Directory { files: Vec::new(), directories: Vec::new(), name: "8789".to_string() }], name: "abc".to_string()} );
        file_system.root.add_child_directory(Directory { files: Vec::new(), directories: Vec::new(), name: "xkc".to_string()} );

        let dir = file_system.get_directory_at_path(Path::new("/abc/8789"));
        assert_eq!(dir.name, "8789");
    }

    #[test]
    fn file_system_can_add_file_to_directory() {
        let mut file_system = FileSystem { root: Directory { files: Vec::new(), directories: Vec::new(), name: "/".to_string() } };
        const FILE_SIZE:usize = 299011;

        file_system.root.add_child_directory(Directory { files: Vec::new(), directories: vec![Directory { files: Vec::new(), directories: Vec::new(), name: "8789".to_string() }], name: "abc".to_string()} );
        file_system.root.add_child_directory(Directory { files: Vec::new(), directories:  vec![Directory { files: Vec::new(), directories: Vec::new(), name: "777".to_string() }], name: "xkc".to_string()} );

        let path = Path::new("/xkc/777");
        file_system.add_file_to_directory_at_path(path, File { name: "christmas_spirit.exe".to_string(),size: FILE_SIZE});

        assert_eq!(file_system.root.get_directory_size(), FILE_SIZE);
        
        let directory = file_system.get_directory_at_path(path);

        assert_eq!(directory.get_directory_size(), FILE_SIZE);
    }

}