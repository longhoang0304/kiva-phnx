use std::collections::VecDeque;
use std::error::Error;

use crate::command_processors::parsers::cores::{CommandParseError, CommandParser};
use crate::exe_engine::cores::{Command, CommandParameter};

pub struct GetDel;

impl CommandParser for GetDel {
    fn parse(tokens: VecDeque<String>) -> Result<Command, Box<dyn Error>> {
        let tk_len = tokens.len() as u16;
        if tk_len != 1 {
            let err = Box::new(CommandParseError::WrongNumberOfArguments(1, tk_len));
            return Err(err);
        }

        let key = tokens.front().unwrap().clone();
        let parameters = VecDeque::from([CommandParameter::String(key)]);

        Ok(Command::new(
            GetDel::name(),
            Some(parameters),
        ))
    }

    fn name() -> &'static str {
        "GETDEL"
    }
}
