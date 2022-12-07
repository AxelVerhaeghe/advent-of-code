use std::collections::HashMap;

use crate::{
    directory::Directory,
    file::File,
    instruction::{Instruction, Type},
};

pub struct FileSystem {
    directories: Vec<Directory>,
}

impl FileSystem {
    pub fn new() -> Self {
        FileSystem {
            directories: Vec::new(),
        }
    }

    pub fn process_scan(&mut self, logs: Vec<String>) {
        let nb_lines = logs.len();

        let mut current_dir_index = self.add_directory(Directory::new_root());

        let mut i = 1;
        while i < nb_lines {
            let line = logs.get(i).unwrap();
            if self.is_instruction(line) {
                let instruction = Instruction::new(line);
                match instruction.command_type() {
                    Type::NAVIGATE => {
                        current_dir_index = self.handle_navigate(instruction, current_dir_index)
                    }
                    Type::LIST => {}
                }
            } else if self.is_directory_line(line) {
                let dir = Directory::new_from_string(line, current_dir_index);
                self.add_directory(dir);
            } else {
                self.add_file(current_dir_index, File::new(line));
            }
            i += 1;
        }
    }

    fn is_instruction(&self, line: &str) -> bool {
        line.starts_with("$")
    }

    fn is_directory_line(&self, line: &str) -> bool {
        line.starts_with("dir")
    }

    fn handle_navigate(&self, instruction: Instruction, current_dir_index: usize) -> usize {
        let dir_name = instruction.argument().unwrap();
        if dir_name == ".." {
            self.get_parent_for_index(current_dir_index)
        } else {
            self.get_index_for_name(dir_name)
        }
    }

    pub fn add_directory(&mut self, directory: Directory) -> usize {
        let index = self
            .directories
            .iter()
            .position(|dir| dir.name() == directory.name());

        if index.is_none() {
            self.directories.push(directory);
            self.directories.len() - 1
        } else {
            index.unwrap()
        }
    }

    pub fn add_file(&mut self, dir_index: usize, file: File) {
        let dir = self.directories.get_mut(dir_index).unwrap();
        dir.add_file(file);
    }

    pub fn get_index_for_name(&self, name: &str) -> usize {
        self.directories
            .iter()
            .position(|dir| dir.name() == name)
            .unwrap()
    }

    pub fn get_parent_for_index(&self, current_index: usize) -> usize {
        let current_dir = self.directories.get(current_index).unwrap();
        if current_dir.is_root() {
            println!("Already at root, returning same index");
            current_index
        } else {
            current_dir.parent_index().unwrap()
        }
    }

    pub fn get_total_dir_size(&self, index: usize) -> i32 {
        let current = self.directories.get(index).unwrap();
        let children: i32 = self
            .directories
            .iter()
            .filter(|dir| dir.parent_index() == Some(index))
            .map(|child| child.size())
            .sum();
        children + current.size()
    }

    pub fn get_all_total_sizes(&self) -> HashMap<&str, i32> {
        let mut result: HashMap<&str, i32> = HashMap::new();
        for i in 0..self.directories.len() {
            let dir = self.directories.get(i).unwrap();
            let size = self.get_total_dir_size(i);
            result.insert(dir.name(), size);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_directory_returns_index() {
        let mut fs = FileSystem::new();
        let dir = Directory::new_root();

        let result = fs.add_directory(dir);

        assert_eq!(result, 0);
    }
}
