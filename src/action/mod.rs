mod init;

pub use init::initialize_project;

use io::Write;
use std::fs;
use std::io;

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
