#[derive(Debug, PartialEq)]
pub enum Command<'a> {
    Set { key: &'a str, value: &'a str },
    Get { key: &'a str },
    Del { key: &'a str },
}

impl<'a> Command<'a> {
    fn from_parts(parts: Vec<&str>) -> Result<Command, CommandError> {
        let command = parts.get(0).map(|s| s.as_ref()).unwrap_or("");

        match command {
            "SET" => {
                if parts.len() < 3 {
                    return Err(CommandError::MissingArguments);
                }
                Ok(Command::Set {
                    key: parts[1],
                    value: parts[2],
                })
            }

            "GET" => {
                if parts.len() < 2 {
                    return Err(CommandError::MissingArguments);
                }
                Ok(Command::Get { key: parts[1] })
            }

            "DEL" => {
                if parts.len() < 2 {
                    return Err(CommandError::MissingArguments);
                }
                Ok(Command::Del { key: parts[1] })
            }

            _ => Err(CommandError::UnknownCommand),
        }
    }
}

#[derive(Debug, PartialEq)]
enum CommandError {
    UnknownCommand,
    MissingArguments,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_command_from_parts() {
        let parts = vec!["SET", "mykey", "myvalue"];
        let command = Command::from_parts(parts);
        assert_eq!(
            Ok(Command::Set {
                key: "mykey",
                value: "myvalue"
            }),
            command
        );
    }

    #[test]
    fn test_get_command_from_parts() {
        let parts = vec!["GET", "mykey"];
        let command = Command::from_parts(parts);
        assert_eq!(Ok(Command::Get { key: "mykey" }), command);
    }

    #[test]
    fn test_del_command_from_parts() {
        let parts = vec!["DEL", "mykey"];
        let command = Command::from_parts(parts);
        assert_eq!(Ok(Command::Del { key: "mykey" }), command);
    }
}
