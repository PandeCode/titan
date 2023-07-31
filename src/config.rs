use schemars::{schema_for, JsonSchema};
use std::collections::HashMap;

use miette::IntoDiagnostic;

use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum CommandType {
    String(String),
    VecString(Vec<String>),
}

#[derive(Debug, Default, PartialEq, Deserialize, JsonSchema)]
pub struct Command {
    pub children: Option<Vec<HashMap<String, Command>>>,

    pub alias: Option<String>,

    pub cmd: Option<CommandType>,

    pub refer: Option<CommandType>,

    pub run_async: Option<bool>,
    pub shell: Option<bool>,
    pub strict: Option<bool>, // Break if any fail

    pub onfailure: Option<CommandType>,
    pub onsuccess: Option<CommandType>,
}

impl Command {
    pub fn find_subcommand(self: &Self, subcommand: String) -> Option<&Command> {
        match &self.children {
            Some(children) => {
                for child in children {
                    for (key, value) in child {
                        if *key == subcommand {
                            return Some(value);
                        }
                    }
                }
                None
            }
            None => None,
        }
    }
}

#[derive(Debug, PartialEq, Deserialize, JsonSchema)]
pub struct Config {
    pub commands: Vec<HashMap<String, Command>>,
}

impl Config {
    pub fn new(config_contents: &str) -> Self {
        Self::parse_config(config_contents)
    }

    fn parse_config(config_contents: &str) -> Self {
        serde_yaml::from_str(config_contents)
            .into_diagnostic()
            .expect("Unable to parse config file\n")
    }

    pub fn get_schema() -> String {
        serde_json::to_string_pretty(&schema_for!(Self))
            .into_diagnostic()
            .expect("Failed to generate schema for Config")
    }
}
