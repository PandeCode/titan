mod command_line;
mod config;
mod errors;
mod fish_completions;
mod print_config;
mod run_commands;
mod tests;

use miette::{IntoDiagnostic, Result};

use run_commands::run_commands;
use tokio::fs::read_to_string;

use crate::config::Config;
use crate::errors::NotEnoughArgs;
use crate::fish_completions::fish_completions;
use crate::print_config::print_config;

#[tokio::main]
async fn main() -> Result<()> {
    let opts = crate::command_line::parse();

    if opts.json_schema {
        println!("{}", Config::get_schema());
        return Ok(());
    }

    let config_contents = read_to_string(&opts.config).await.into_diagnostic()?;

    let parsed_config = Config::new(&config_contents);

    if opts.print_config {
        let _ = print_config(&parsed_config);
        return Ok(());
    }

    if opts.fish_completions {
        println!("{}", fish_completions(&parsed_config));
        return Ok(());
    }

    if opts.args.len() == 0 {
        Err(NotEnoughArgs {})?
    }

    run_commands(&opts.args, parsed_config).await
}
