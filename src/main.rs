use clap::Parser;
use std::env;

mod action;
mod args;

use args::Action;

fn main() {
    pretty_env_logger::init();

    let args = args::App::parse();

    // TODO: get it to actually log
    // when on a certain level.
    // match args.log {
    //     1 => env::set_var("RUST_LOG", "debug"),
    //     2 => env::set_var("RUST_LOG", "info"),
    //     3 => env::set_var("RUST_LOG", "warning"),
    //     4 => env::set_var("RUST_LOG", "error"),
    //     0 | _ => {}
    // }

    log::info!("Started");

    match &args.action {
        Action::Init(_) => action::initialize_project(&args),
    }
}
