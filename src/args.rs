use clap::{Parser, Subcommand, ValueEnum, Args, clap_derive::ArgEnum};

#[derive(Parser, Debug, Clone)]
#[clap(author, version, about)]
pub struct App {
    /// Display logs based on the level
    #[clap(value_enum)]
    pub log: Option<Level>,
    /// What to do - e.g. initialize
    #[clap(subcommand)]
    pub action: Action,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum Level {
    Debug,
    Info,
    Warning,
    Error,
}

#[derive(Debug, Subcommand, Clone)]
pub enum Action {
    /// Create a HTML project
    Init(InitProject),
}

#[derive(Debug, Args, Clone)]
pub struct InitProject {
    #[clap(short, long)]
    pub name: Option<String>,
    #[clap(short, long, arg_enum)]
    pub exclude: Option<Exclude>
}

#[derive(Debug, Clone, ArgEnum, Default)]
pub enum Exclude {
    JS,
    CSS,
    #[default]
    None
}