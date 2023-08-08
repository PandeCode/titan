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

#[derive(Error, Debug, Diagnostic)]
#[error("No config file found")]
#[diagnostic(help(
    r#"Specifiy at least one config file or a default:
  - ~/.config/titan/config.toml
  - ~/.titan.toml
  - ~/titan.toml
  - $PWD/.titan.toml
  - $PWD/titan.toml"#
))]
pub struct NoConfigCouldBeFound {}

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
        let children = format!("{}", command.children_names().unwrap_or(vec![]).join(","));

        CommandNotRunnable {
            help: Some(format!(
                "Command neither specifies an 'alias' or 'cmd' required to be runnable.
            Or check parameters if you meant to access children [{}]",
                children
            )),
        }
    }
}
