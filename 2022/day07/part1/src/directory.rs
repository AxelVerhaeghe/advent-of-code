use crate::file::File;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Directory {
    name: String,
    files: Vec<File>,
    parent_index: Option<usize>,
}

impl Directory {
    pub fn new_root() -> Self {
        Directory {
            name: String::from("/"),
            files: Vec::new(),
            parent_index: None,
        }
    }

    pub fn new_from_string(input_string: &str, parent_index: usize) -> Self {
        let split_line: Vec<&str> = input_string.split(" ").collect();
        let name = split_line.get(1).expect("Could not find name in input");
        Directory {
            name: String::from(*name),
            files: Vec::new(),
            parent_index: Some(parent_index),
        }
    }

    pub fn add_file(&mut self, file: File) {
        self.files.push(file);
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn parent_index(&self) -> Option<usize> {
        self.parent_index
    }

    pub fn size(&self) -> i32 {
        let mut result = 0;
        for file in &self.files {
            result += file.size();
        }
        result
    }

    pub fn is_root(&self) -> bool {
        self.name == "/"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_root_adds_name() {
        let result = Directory::new_root();

        assert_eq!(result.name, "/");
    }
}
