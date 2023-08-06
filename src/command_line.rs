use clap::Parser;
use env_logger::Env;

#[derive(Parser)]
#[clap(version, about, long_about = None)]
pub struct Options {
    /// Increase verbosity, and can be used multiple times
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,

    #[arg(short, long, default_value_t = false)]
    pub json_schema: bool,

    #[arg(short, long, default_value_t = false)]
    pub fish_completions: bool,

    #[arg(short, long, default_value_t = false)]
    pub print_config: bool,

    #[arg()]
    pub args: Vec<String>,

    #[arg(long, short, default_value_t=format!("{}{}", dirs::home_dir().unwrap().display(), "/.titan.toml"))]
    pub config: String,
}

pub fn parse() -> Options {
    let opts = Options::parse();

    let debug_level = match opts.verbose {
        0 => "info",
        1 => "debug",
        _ => "trace",
    };
    env_logger::Builder::from_env(Env::default().default_filter_or(debug_level)).init();

    opts
}
