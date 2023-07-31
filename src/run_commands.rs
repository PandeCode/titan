use miette::Result;

use crate::config::{Command, CommandType, Config};
use crate::errors::{CommandNotRunnable, SubCommandNotFound};

async fn cmd_wrapper(command: &str) -> u8 {
    log::info!("Running command: '{}'", command);
    0
}

enum ReturnCode {
    ReturnCode(u8),
    VecStatusCode(Vec<u8>),
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
async fn run_command(command: Option<&CommandType>, strict: Option<bool>) -> Option<ReturnCode> {
    match command {
        Some(command) => Some({
            match command {
                CommandType::String(command) => ReturnCode::ReturnCode(cmd_wrapper(&command).await),
                CommandType::VecString(command) => {
                    let mut return_codes: Vec<u8> = Vec::new();
                    for cmd in command {
                        let return_code = cmd_wrapper(cmd.as_str()).await;

                        if let Some(strict) = strict {
                            if strict {
                                if return_code != 0 {
                                    return Some(ReturnCode::VecStatusCode(return_codes));
                                }
                            }
                        }

                        return_codes.push(return_code)
                    }
                    ReturnCode::VecStatusCode(return_codes)
                }
            }
        }),
        None => None,
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

    if current_command.alias.is_none() {
        if current_command.cmd.is_none() {
            // Err(CommandNotRunnable::default())?
            Err(CommandNotRunnable::with_command(current_command))?
        }
    }

    match run_command!(current_command.cmd.as_ref()).await {
        Some(success) => {
            if was_successful(success) {
                log::debug!("All commands successful");

                if current_command.onsuccess.is_some() {
                    run_command!(current_command.onsuccess.as_ref()).await;
                    log::debug!("Run success command");
                }

                run_command!(current_command.onsuccess.as_ref()).await;
            } else {
                log::debug!("Some commands failed");

                if current_command.onfailure.is_some() {
                    run_command!(current_command.onfailure.as_ref()).await;
                    log::debug!("Run failure command");
                }
            }
        }
        None => log::debug!("this was none, {:#?}", current_command),
    }

    Ok(())
}
