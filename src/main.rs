mod command_line;
mod config;
mod errors;
mod fish_completions;
mod print_config;
mod run_commands;
mod tests;

use std::path::PathBuf;

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
    let opts_config_default = "$HOME/.titan.toml".to_owned();

    if opts.json_schema {
        println!("{}", Config::get_schema());
        return Ok(());
    }

    let cwd = std::env::current_dir().into_diagnostic().unwrap();
    let home = dirs::home_dir().unwrap();
    let mut possible_paths = vec![
        PathBuf::from(&opts.config),
        cwd.join(".titan.toml"),
        cwd.join("titan.toml"),
        home.join(".config/titan/config.toml"),
        home.join(".titan.toml"),
        home.join("titan.toml"),
    ];

    if opts.config != opts_config_default {
        possible_paths.push(opts.config.into());
    }

    let mut config_contents = vec![];
    for path in possible_paths {
        if path.exists() {
            config_contents.push(read_to_string(path).await.into_diagnostic()?)
        } else {
            let path = path.display();
            log::warn!("Path '{path}', not found.")
        }
    }

    if config_contents.len() == 0 {
        Err(errors::NoConfigCouldBeFound {})?
    }

    let mut parsed_config = Config::new(&config_contents.first().unwrap());

    for config_content in &config_contents {
        parsed_config
            .commands
            .append(&mut Config::new(config_content).commands);
    }

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
