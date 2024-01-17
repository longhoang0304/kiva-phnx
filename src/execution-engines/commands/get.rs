use std::error::Error;

use crate::exe_engine::cores::{Command, CommandExecutor, CommandResult};
use crate::storage::cores::Storage;

pub struct Get;

impl CommandExecutor for Get {
    fn execute(storage: &mut Box<dyn Storage>, cmd: &Command) -> Result<CommandResult, Box<dyn Error>> {
        let parameters = cmd.parameters.as_ref().unwrap();
        let mut params_iter = parameters.iter();
        let key: String = params_iter.next().unwrap().clone().try_into()?;

        let entry = storage.read(&key)?;
        let data = entry.get_data();

        Ok(CommandResult::from(data))
    }
}
