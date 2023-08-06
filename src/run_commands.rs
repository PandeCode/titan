use execute::shell;
use miette::{IntoDiagnostic, Result};

use crate::config::{Command, CommandType, Config};
use crate::errors::{CommandNotRunnable, SubCommandNotFound};

use execute::Execute;

enum ReturnCode {
    ReturnCode(i32),
    VecStatusCode(Vec<i32>),
}

impl Default for ReturnCode {
    fn default() -> Self {
        Self::ReturnCode(1)
    }
}

async fn cmd_wrapper(command: &str) -> i32 {
    if let Some(exit_code) = shell(command)
        .execute_output()
        .into_diagnostic()
        .unwrap()
        .status
        .code()
    {
        log::info!("Run command '{command}'");
        exit_code
    } else {
        log::error!("Failed to run command '{command}'");
        -1
    }
}

fn was_successful(success: ReturnCode) -> bool {
    match success {
        ReturnCode::ReturnCode(success) => success == 0,
        ReturnCode::VecStatusCode(success) => (|| {
            for val in success {
                if val != 0 {
                    return false;
                }
            }
            true
        })(),
    }
}

macro_rules! run_command {
    ($command: expr, $strict: expr) => {
        run_command($command, $strict)
    };
    ($command: expr) => {
        run_command($command, None)
    };
}
async fn run_command(command: &CommandType, strict: Option<bool>) -> ReturnCode {
    match command {
        CommandType::String(command) => ReturnCode::ReturnCode(cmd_wrapper(&command).await),
        CommandType::VecString(command) => {
            let mut return_codes: Vec<i32> = Vec::new();
            for cmd in command {
                let return_code = cmd_wrapper(cmd.as_str()).await;

                if let Some(strict) = strict {
                    if strict {
                        if return_code != 0 {
                            return ReturnCode::VecStatusCode(return_codes);
                        }
                    }
                }

                return_codes.push(return_code)
            }
            ReturnCode::VecStatusCode(return_codes)
        }
    }
}

pub(crate) async fn run_commands(subcommands: &[String], config: Config) -> Result<()> {
    // INFO: 218 subcommands.len() > 0
    let first_subcommand = subcommands.first().unwrap();
    let mut current_command: Option<&Command> = None;

    for commands in &config.commands {
        for (key, value) in commands {
            if key == first_subcommand {
                current_command = Some(value);
                log::debug!("Inital Subcommand: '{:?}'", value)
            }
        }
    }

    if current_command.is_none() {
        Err(SubCommandNotFound {})?
    }

    let mut current_command = current_command.unwrap();

    for subcommand in subcommands[1..].iter() {
        if let Some(command) = current_command.find_subcommand(subcommand.to_owned()) {
            current_command = command;
            log::debug!("Added Subcommand: '{:?}'", current_command)
        } else {
            Err(SubCommandNotFound {})?
        }
    }

    let mut success = Default::default();
    if let Some(cmd) = &current_command.cmd {
        success = run_command!(&cmd).await;
    } else {
        if let Some(alias) = &current_command.alias {
            success = run_command!(&CommandType::String(alias.to_owned())).await;
        } else {
            Err(CommandNotRunnable::with_command(current_command))?
        }
    };

    if was_successful(success) {
        log::debug!("All commands successful");

        if let Some(onsuccess) = current_command.onsuccess.as_ref() {
            run_command!(onsuccess).await;
            log::debug!("Run success command");
        }
    } else {
        log::debug!("Some commands failed");

        if let Some(onfailure) = current_command.onfailure.as_ref() {
            run_command!(onfailure).await;
            log::debug!("Run failure command");
        }
    };

    Ok(())
}
