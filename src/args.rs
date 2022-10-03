use clap::{clap_derive::ValueEnum, Args, Parser, Subcommand};

#[derive(Parser, Debug, Clone)]
#[clap(version)]
pub struct App {
    /// Set the logging level:
    /// 0: No logging,
    /// 1: Debug,
    /// 2: Info,
    /// 3: Warning,
    /// 4: Error
    #[clap(short, long, action = clap::ArgAction::Count)]
    pub log: u8,
    /// Action to run - i.e. initialize or clone
    #[clap(subcommand)]
    pub action: Action,

    #[cfg(debug_assertions)]
    #[clap(skip)]
    pub debug: bool,
}

#[derive(Debug, Subcommand, Clone)]
pub enum Action {
    /// Create a HTML project
    Init(InitProject),
    /// Clone an existing project
    Clone(CloneProject),
    /// Fetch and install the latest version if possible
    Update(UpdateFlags),
}

#[derive(Debug, Args, Clone)]
pub struct InitProject {
    /// The name of the project
    pub name: Option<String>,
    /// Don't include a certain language to be added
    #[clap(short, long, value_enum)]
    pub exclude: Option<Exclude>,

    /// Creates a basic HTML template file in a directory, if the --name or -n flag is provided
    #[clap(short, long)]
    pub minimal: bool,
}

#[derive(Debug, Args, Clone)]
pub struct CloneProject {
    /// The path to the project to clone.
    pub path: String,
    /// Rename the cloned project.
    /// This is required because
    /// cloned directories cannot
    /// have the exact name.
    pub rename: String,
    /// Allow files that exist to be overwritten
    #[clap(short, long)]
    pub overwrite: bool,
    /// Skip files that already exist
    #[clap(short, long)]
    pub skip_exist: bool,
}

#[derive(Debug, Args, Clone)]
pub struct UpdateFlags {
    /// Allows installation for a different version, instead of the latest.
    /// For example, 0.1.0
    #[clap(short, long)]
    pub version: Option<String>,
    /// Don't display the download/progress bar
    #[clap(short, long)]
    pub omit_download_bar: bool,
    /// Displays all available builds
    #[clap(short, long)]
    pub list: bool,
}

#[derive(Debug, Clone, ValueEnum, Default)]
pub enum Exclude {
    JS,
    Javascript,
    CSS,
    #[default]
    None,
}
