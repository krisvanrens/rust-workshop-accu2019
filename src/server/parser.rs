#[derive(Debug, PartialEq)]
pub enum Command {
    Pub(Vec<String>),
    Get(u32)
}


#[derive(Debug, PartialEq)]
pub enum ParseError {
    InvalidCommand,
    NoArguments,
    TooManyArguments,
    InvalidArgument
}


pub fn parse_error_to_string(error: ParseError) -> String {
    match error {
        ParseError::InvalidCommand   => "Invalid command",
        ParseError::NoArguments      => "No arguments given",
        ParseError::TooManyArguments => "Too many arguments given",
        ParseError::InvalidArgument  => "Invalid argument"
    }.to_string()
}


pub fn parse(input: &str) -> Result<Command, ParseError> {
    let mut input_split: Vec<&str> = input.split(' ').collect();

    input_split.retain(|x|!x.is_empty());

    match input_split.len() {
        0 => return Err(ParseError::InvalidCommand),
        1 => match input_split[0] {
            "PUB"|"GET" => return Err(ParseError::NoArguments),
            _ =>  return Err(ParseError::InvalidCommand)
        },
        _ => {}
    }

    match input_split[0] {
        "PUB" => {
            Ok(Command::Pub(input_split[1..].to_owned().into_iter().map(|x|x.into()).collect()))
        },
        "GET" => {
            if input_split.len() > 2 {
                return Err(ParseError::TooManyArguments)
            }

            Ok(Command::Get(input_split[1].parse().map_err(|_| ParseError::InvalidArgument)?))
        },
        _ => Err(ParseError::InvalidCommand)
    }
}


#[test]
fn test_general() {
    assert_eq!(parse("").is_ok(), false);
    assert_eq!(parse("").err().unwrap(), ParseError::InvalidCommand);

    assert_eq!(parse("PuB").is_ok(), false);
    assert_eq!(parse("PuB").err().unwrap(), ParseError::InvalidCommand);
    assert_eq!(parse("GEt").is_ok(), false);
    assert_eq!(parse("GEt").err().unwrap(), ParseError::InvalidCommand);
}


#[test]
fn test_pub() {
    assert_eq!(parse("PUB").is_ok(), false);
    assert_eq!(parse("PUB").err().unwrap(), ParseError::NoArguments);

    assert_eq!(parse("PUB one").is_ok(), true);
    assert_eq!(parse("PUB one").ok().unwrap(), Command::Pub(vec!["one".to_string()]));
    assert_eq!(parse("PUB one").ok().unwrap(),Command::Pub(vec!["one".to_string()]));
    assert_eq!(parse("PUB one two").is_ok(), true);
    assert_eq!(parse("PUB one two").ok().unwrap(), Command::Pub(vec!["one".to_string(), "two".to_string()]));
    assert_eq!(parse("PUB 1 two three").is_ok(), true);
    assert_eq!(parse("PUB 1 two three").ok().unwrap(), Command::Pub(vec!["1".to_string(), "two".to_string(), "three".to_string()]));
}


#[test]
fn test_get() {
    assert_eq!(parse("GET").is_ok(), false);
    assert_eq!(parse("GET").err().unwrap(), ParseError::NoArguments);

    assert_eq!(parse("GET 1 2").is_ok(), false);
    assert_eq!(parse("GET 1 2").err().unwrap(), ParseError::TooManyArguments);

    assert_eq!(parse("GET one").is_ok(), false);
    assert_eq!(parse("GET one").err().unwrap(), ParseError::InvalidArgument);

    assert_eq!(parse("GET 1").is_ok(), true);
    assert_eq!(parse("GET 1").ok().unwrap(), Command::Get(1));

    assert_eq!(parse("GET 3").is_ok(), true);
    assert_eq!(parse("GET 3").ok().unwrap(), Command::Get(3));
}
