mod clone;
mod init;

pub use clone::clone_project;
pub use init::initialize_project;

use io::Write;
use owo_colors::OwoColorize;
use std::{fs, io};

/// A macro that takes in a variadic amount of arguments
/// and creates files for each argument. This macro allows
/// you to add content inside of the file by adding =>,
/// forwarded by a string.
///
/// # Example
/// ```
/// fn create_two_files() -> std::io::Result<()> {
///    use crate::files_create;
///
///    create_files!(
///     // # NAME     # CONTENT
///     "test.txt" => "Hello, World",
///     "inner.html" => ""
///    )?;
/// }
/// ```
#[macro_export]
macro_rules! create_files {
    () => {};

    ($($fname:expr $(=> $content:expr)?),*) => {
        $(
                create_file($fname, $($content)*)?;
        )*
    };
}

#[macro_export]
macro_rules! files {
    ($({ $arg:tt => $cnt:expr $(; $fmt:expr),* }),*) => {
        $(
            {
                let mut file = std::fs::File::create(format!($arg $(, $fmt)*))?;

                core::write!(file, $cnt)?;
                file.flush()?;
            }
        )*
    };
}

/// This macro, similar to [`files_create`], allows
/// you to create any amount of directories. It is
/// recursive which means it can create parent folders
/// if they do not exist.
///
/// # Example
/// ```
/// fn create_directories() -> std::io::Result<()> {
///     use crate::dir_create;
///
///     let formatted_text = "upper/inner/inner/inner/end";
///
///     create_dirs!(
///         "directory",
///         "top/subdirectory/",
///         "upper/sub/bottom/inner",
///        "{formatted_text}"
///     )?;
/// }
/// ```
#[macro_export]
macro_rules! create_dirs {
    () => {};

   ($($arg:tt),*) => {
        $(
            {
                std::fs::create_dir_all(&format!($arg))?
            }
        )*
    };
}

pub fn create_file(name: &str, contents: &str) -> io::Result<()> {
    let mut file = fs::File::create(name)?;

    write!(file, "{contents}")?;
    file.flush()
}

pub fn success(msg: &str) {
    println!("{}: {msg}", "Success".green());
}

pub fn fail(msg: &str) {
    eprintln!("{}: {msg}", "Error".red());
}
