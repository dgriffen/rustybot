use crate::database::Db;
use failure::Error;
use irc::client::prelude::*;
use std::collections::HashMap;

pub struct Command<'a> {
    pub source_nick: &'a str,
    pub command_str: &'a str,
    pub arguments: Vec<&'a str>,
}

impl<'a> Command<'a> {
    pub fn try_parse(source_nick: &'a str, message: &'a str) -> Option<Command<'a>> {
        if !message.starts_with('~') {
            return None;
        }

        let mut parts = message.split(' ');
        let command = parts.next()?;
        Some(Command {
            source_nick: source_nick,
            command_str: command,
            arguments: parts.collect(),
        })
    }
}

pub struct Handler {
    db: Db,
    commands:
        HashMap<&'static str, fn(Command, &crate::config::Config, &Db) -> Result<String, Error>>,
    default: Option<fn(Command, &crate::config::Config, &Db) -> Result<String, Error>>,
}

impl Handler {
    pub fn new(db: Db) -> Self {
        Handler {
            db,
            commands: HashMap::new(),
            default: None,
        }
    }

    pub fn register(
        &mut self,
        label: &'static str,
        handler: fn(Command, &crate::config::Config, &Db) -> Result<String, Error>,
    ) {
        self.commands.insert(label, handler);
    }

    pub fn register_default(
        &mut self,
        handler: fn(Command, &crate::config::Config, &Db) -> Result<String, Error>,
    ) {
        self.default = Some(handler);
    }

    pub fn handle(
        &self,
        command: Command,
        config: &crate::config::Config,
    ) -> Result<String, Error> {
        if self.commands.contains_key(command.command_str) {
            self.commands[command.command_str](command, config, &self.db)
        } else if let Some(default) = self.default {
            default(command, config, &self.db)
        } else {
            Ok(format!("command '{}' not found", command.command_str))
        }
    }
}