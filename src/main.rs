#![feature(iterator_try_reduce)]
mod command_line;
mod config;
mod errors;
mod run_commands;
mod tests;

use miette::{IntoDiagnostic, Result};

use run_commands::run_commands;
use tokio::fs::read_to_string;

use crate::config::Config;
use crate::errors::NotEnoughArgs;

#[tokio::main]
async fn main() -> Result<()> {
    let opts = crate::command_line::parse();

    if opts.json_schema {
        println!("{}", Config::get_schema());
        return Ok(());
    }

    if opts.args.len() == 0 {
        Err(NotEnoughArgs {})?
    }

    let config_contents = read_to_string(&opts.config).await.into_diagnostic()?;

    let parsed_config = Config::new(&config_contents);

    run_commands(&opts.args, parsed_config).await
}
