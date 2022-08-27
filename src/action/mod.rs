mod init;

pub use init::initialize_project;

use io::Write;
use std::fs;
use std::io;

/// A macro that takes in a variadic amount of arguments
/// and creates files for each argument. This macro allows
/// you to add content inside of the file by adding =>,
/// forwarded by a string.
/// 
/// # Example
/// ```
/// fn create_files() -> std::io::Result<()> {
///    use crate::files_create;
/// 
///    files_create!(
///     // # NAME     # CONTENT
///     "test.txt" => "Hello, World",
///     "inner.html" => ""
///    ); 
/// }
/// ```
#[macro_export]
macro_rules! files_create {
    () => {};

    ($($fname:expr $(=> $content:expr)?),*) => {
        $(
            {
                create_file($fname, $($content)*)?
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
///     dir_create!(
///         "directory",
///         "top/subdirectory/",
///         "upper/sub/bottom/inner"
///     );
/// }
/// ```
#[macro_export]
macro_rules! dir_create {
    () => {};

    ($($dir_name:expr),*) => {
        $(
            {
                std::fs::DirBuilder::new()
                    .recursive(true)
                    .create($dir_name)?
            }
        )*
    };
}

pub fn create_file(name: &str, contents: &str) -> io::Result<()> {
    let mut file = fs::File::create(name)?;

    write!(file, "{contents}")?;
    file.flush()
}
