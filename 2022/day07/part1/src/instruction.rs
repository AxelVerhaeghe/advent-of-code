#[derive(PartialEq, Eq, Debug)]
pub struct Instruction {
    command_type: Type,
    argument: Option<String>,
}

impl Instruction {
    pub fn new(input_line: &str) -> Self {
        if !input_line.starts_with("$") {
            panic!("Input [{}] is not a command.", input_line);
        }
        let split_string: Vec<&str> = input_line.split(" ").collect();
        let unparsed_type = split_string.get(1).unwrap();
        let command_type = Type::from_string(*unparsed_type);

        match command_type {
            Type::LIST => Instruction {
                command_type,
                argument: None,
            },
            Type::NAVIGATE => {
                let argument = split_string
                    .get(2)
                    .expect("Expected argument for command of type [CD] but found none.");
                Instruction {
                    command_type,
                    argument: Option::Some(String::from(*argument)),
                }
            }
        }
    }

    pub fn command_type(&self) -> &Type {
        &self.command_type
    }

    pub fn argument(&self) -> Option<&String> {
        self.argument.as_ref()
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum Type {
    NAVIGATE,
    LIST,
}

impl Type {
    fn from_string(input_string: &str) -> Self {
        match input_string {
            "cd" => Type::NAVIGATE,
            "ls" => Type::LIST,
            _ => panic!("Input command [{}] is not a known type.", input_string),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Input [dir a] is not a command.")]
    fn new_when_not_a_command_panics() {
        Instruction::new("dir a");
    }

    #[test]
    fn new_when_cd_uses_type_cd() {
        let result = Instruction::new("$ cd /");
        assert_eq!(result.command_type, Type::NAVIGATE);
    }

    #[test]
    fn new_when_ls_uses_type_ls() {
        let result = Instruction::new("$ ls");
        assert_eq!(result.command_type, Type::LIST);
    }

    #[test]
    #[should_panic(expected = "Input command [pwd] is not a known type.")]
    fn new_when_unknown_type_panics() {
        Instruction::new("$ pwd");
    }

    #[test]
    #[should_panic(expected = "Expected argument for command of type [CD] but found none.")]
    fn new_when_cd_expect_argument() {
        Instruction::new("$ cd");
    }

    #[test]
    fn new_when_cd_parses_argument() {
        let result = Instruction::new("$ cd /");
        assert!(result.argument.is_some());
        assert_eq!(result.argument.unwrap(), "/");
    }

    #[test]
    fn new_when_ls_dont_parse_arguments() {
        let result = Instruction::new("$ ls someArgument");
        assert!(result.argument.is_none());
    }
}
