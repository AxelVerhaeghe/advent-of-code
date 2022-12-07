#[derive(Debug, PartialEq, Eq, Clone)]
pub struct File {
    size: i32,
    name: String,
}

impl File {
    pub fn new(input_line: &str) -> Self {
        let split_line: Vec<&str> = input_line.split(" ").collect();
        let size_unparsed = split_line.get(0).unwrap();
        let size = size_unparsed
            .parse::<i32>()
            .expect("input was not a valid size");

        let name = split_line.get(1).expect("Could not find name in input");
        File {
            size,
            name: String::from(*name),
        }
    }

    pub fn size(&self) -> i32 {
        self.size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_parses_size() {
        let result = File::new("14848514 b.txt");
        assert_eq!(result.size, 14848514);
    }

    #[test]
    fn new_parses_name() {
        let result = File::new("14848514 b.txt");
        assert_eq!(result.name, "b.txt");
    }
}
