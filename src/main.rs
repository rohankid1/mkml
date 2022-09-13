#![allow(warnings)]

use clap::Parser;

mod action;
mod args;

use args::Action;

fn main() {
    // initialize the logger
    pretty_env_logger::init();

    let mut args = std::env::args().collect::<Vec<_>>();

    profile_time::debug_time! {

                if !args.contains(&"-D".to_owned()) {

                println!("If you are using this tool normally, you should be running the release mode;
        the debug mode (which you are running currently) is meant for developers to debug their application. 
        See https://github.com/rohankid1/mkml/ on how to get the release mode which is more optimised.");

                println!("If you are debugging or a developer and want to remove this message, run the application again
                but with -D");

            }

        let search = args.binary_search(&"-D".to_owned());

        if search.is_ok() {
            args.remove(search.unwrap());
        }
    }

    // parse the command line flags and others
    let args = args::App::parse_from(args);

    // TODO: make it able to change the log level at
    // runtime. Example: the flag -ll changes the log
    // level to info at runtime, before anything else
    // runs.
    // match args.log {
    //     1 => env::set_var("RUST_LOG", "debug"),
    //     2 => env::set_var("RUST_LOG", "info"),
    //     3 => env::set_var("RUST_LOG", "warning"),
    //     4 => env::set_var("RUST_LOG", "error"),
    //     0 | _ => {}
    // }

    log::info!("Started");

    // currently, there is only one action.
    // There will be more actions in future
    // releases
    match &args.action {
        Action::Init(_) => action::initialize_project(&args),
        Action::Clone(_) => action::clone_project(&args),
    }
}
