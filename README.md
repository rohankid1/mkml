# mkml

[![Tests](https://github.com/rohankid1/mkml/actions/workflows/tests.yml/badge.svg)](https://github.com/rohankid1/mkml/actions/workflows/tests.yml)
![Crates.io](https://img.shields.io/crates/v/mkml)
![Crates.io](https://img.shields.io/crates/d/mkml)


mkml is a simple but cross-platform command written in Rust to initialize a simple HTML website project with organisation in mind.

This project is still in development, but it is usable. However, there are still stuff
that have yet to be complete; for example, selecting a log level flag will not do anything yet.

# Usage

```
USAGE:
    mkml [OPTIONS] <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -l, --log        Set the logging level: 0: No logging, 1: Debug, 2: Info, 3: Warning, 4: Error
    -V, --version    Print version information

SUBCOMMANDS:
    init    Create a HTML project
    clone    Clone an existing project
    update  Fetch and install the latest version if possible
    help    Print this message or the help of the given subcommand(s)
```

# Updating
From 0.2.0 and above, you can auto update mkml by simply running `mkml update`.

If you want to install a different version instead of the latest, run `mkml update -v <NUM>.<NUM>.<NUM>`.
You can also print out available builds by running `mkml update -l`.

# Installing
If you just want the latest executable binary, you can install them without
having to build anything from the [release page](https://github.com/rohankid1/mkml/releases); however, the page may not always contain the
latest release so it's better to build from source.

If you're a Rust developer, you can simply run the command to install it:
```
cargo install mkml
```

# Building from source
## Prerequisites 
* Rustc
* Cargo

* Minimum Rust version is 1.63 (haven't tested it on older versions)

Rustc and Cargo can be installed from [here](https://www.rust-lang.org/tools/install)

To build from source, simply clone this repository.
You can clone the repository by running the following command: `git clone https://github.com/rohankid1/mkml.git`

Alternatively, you can download the zip file containing the source code by clicking on the button labelled
 "Code". Then click on `Download ZIP`.

Once done, `cd` into the folder. It's time for building!

For developers wanting to add their own features, you can edit the source code directly and run cargo r
(short for cargo run) to test it in dev mode. The debug binary will be located in `target/debug/`.

For users who just want to use the program: run `cargo build --release`. Since this project is not a large project,
it will not take too long compile.

Once it has finished compiling, the executable should have been created in `target/release/`.

To run it, you have to launch it through the terminal.

Also, for quick access, you should copy or move the binary to `/usr/local/bin` - that way you can just
run `mkml` and not something like: `Downloads/mkml/target/release/mkml`. You can simply do this by running
`sudo cp path/to/mkml /usr/local/bin` or mv instead of cp if you want to move it.