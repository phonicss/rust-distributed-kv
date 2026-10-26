pub enum Command {
    Set { key: String, value: String },
    Get(String),
    Delete(String),
}

impl Command {
    pub fn parse(input: &str) -> Result<Command, String> {
        let mut parts = input.splitn(3, ' ');
        let command = parts.next();
        let key = parts.next();
        let value = parts.next();

        match (command, key, value) {
            (Some("GET"), Some(key), None) => Ok(Command::Get(key.to_string())),
            (Some("SET"), Some(key), Some(value)) => Ok(Command::Set {
                key: key.to_string(),
                value: value.to_string(),
            }),
            (Some("DELETE"), Some(key), None) => Ok(Command::Delete(key.to_string())),
            _ => Err("Invalid command".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Command;

    #[test]
    fn parse_get_command() {
        match Command::parse("GET name") {
            Ok(command) => match command {
                Command::Get(key) => {
                    assert_eq!(key, "name");
                }
                _ => {
                    panic!("Unexpected command");
                }
            },
            Err(error) => {
                panic!("Unexpected error: {}", error);
            }
        }
    }

    #[test]
    fn parse_set_command() {
        match Command::parse("SET name Ivan Portnov") {
            Ok(command) => match command {
                Command::Set { key, value } => {
                    assert_eq!(key, "name");
                    assert_eq!(value, "Ivan Portnov")
                }
                _ => {
                    panic!("Unexpected command");
                }
            },
            Err(error) => {
                panic!("Unexpected error: {}", error);
            }
        }
    }

    #[test]
    fn parse_delete_command() {
        match Command::parse("DELETE name") {
            Ok(command) => match command {
                Command::Delete(key) => {
                    assert_eq!(key, "name");
                }
                _ => {
                    panic!("Unexpected command");
                }
            },
            Err(error) => {
                panic!("Unexpected error: {}", error);
            }
        }
    }

    #[test]
    fn parse_invalid_command() {
        match Command::parse("HELLO name") {
            Ok(_) => {
                panic!("Expected parsing error");
            }
            Err(error) => {
                assert_eq!(error, "Invalid command");
            }
        }
    }
}
