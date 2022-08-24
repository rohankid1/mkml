#![allow(warnings)] // for now

use args::Level;
use clap::Parser;

mod action;
mod args;

fn main() {
    pretty_env_logger::init();

    log::info!("Started");

    let args = args::App::parse();

    match &args.log {
        Some(log) => match log {
            Level::Debug => {}
            Level::Error => {}
            Level::Info => {}
            Level::Warning => {}
        },
        None => {}
    }

    match &args.action {
        args::Action::Init(dir) => action::initialize_project(&args),
    }

    log::info!("End");
}
