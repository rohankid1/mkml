use clap::{clap_derive::ArgEnum, Args, Parser, Subcommand};

#[derive(Parser, Debug, Clone)]
#[clap(author, version, about)]
pub struct App {
    /// Set the logging level:
    /// 0: No logging,
    /// 1: Debug,
    /// 2: Info,
    /// 3: Warning,
    /// 4: Error
    #[clap(short, long, action = clap::ArgAction::Count)]
    pub log: u8,
    /// What to do - e.g. initialize
    #[clap(subcommand)]
    pub action: Action,
}

#[derive(Debug, Subcommand, Clone)]
pub enum Action {
    /// Create a HTML project
    Init(InitProject),
}

#[derive(Debug, Args, Clone)]
pub struct InitProject {
    /// The name of the project
    #[clap(short, long)]
    pub name: Option<String>,

    /// Don't include a certain language to be added
    #[clap(short, long, arg_enum)]
    pub exclude: Option<Exclude>,

    /// Creates a basic HTML template file in a directory (if the --name or -n flag is provided)
    #[clap(short, long)]
    pub minimal: bool,
}

#[derive(Debug, Clone, ArgEnum, Default)]
pub enum Exclude {
    JS,
    Javascript,
    CSS,
    #[default]
    None,
}
