
use miette::Diagnostic;
use thiserror::Error;

use crate::config::Command;

#[derive(Error, Debug, Diagnostic)]
#[error("Not enough arguments")]
#[diagnostic(help("Specifiy at least one argument"))]
pub struct NotEnoughArgs {}

#[derive(Error, Debug, Diagnostic)]
#[error("Subcommand not found")]
#[diagnostic(help("Check your parameters and config file"))]
pub struct SubCommandNotFound {}

#[derive(Error, Debug, Diagnostic)]
#[error("Command not runnable")]
#[diagnostic()]
pub struct CommandNotRunnable {
    #[help]
    help: Option<String>,
}

impl Default for CommandNotRunnable {
    fn default() -> Self {
        CommandNotRunnable {
            help: Some(
                "Command specifiy an 'alias' or 'cmd'. 
Check parameters if you meant to access children"
                    .to_owned(),
            ),
        }
    }
}

impl CommandNotRunnable {
    pub fn with_command(command: &Command) -> Self {
        let keys: Vec<&String> = command
            .children
            .as_ref()
            .unwrap()
            .iter()
            .flat_map(|map| map.keys())
            .collect();

        let keys: Vec<String> = keys.iter().map(|key| format!("{:?}", key)).collect();

        let children = format!("{}", keys.join(","));

        CommandNotRunnable {
            help: Some(format!(
                "Command neither specifies an 'alias' or 'cmd' required to be runnable.
            Or check parameters if you meant to access children [{}]",
                children
            )),
        }
    }
}
