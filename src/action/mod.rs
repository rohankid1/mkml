mod init;

pub use init::initialize_project;

use io::Write;
use std::io;
use std::fs;

pub fn create_file(name: &str, contents: &str) -> io::Result<()> {
    // if fs::metadata(name)?.is_file() { // then the file must already exists
    //     log::warn!("File already exists: `{name}`");
    //     return Ok(());
    // }

    let mut file = fs::File::create(name)?;

    write!(file, "{contents}")?;
    file.flush()
}

#[inline]
pub fn create_directory(name: &str) -> io::Result<()> {
    // if fs::metadata(name)?.is_dir() { // then the directory must already exists 
    //     log::warn!("Directory already exists: `{name}`");
    //     return Ok(());
    // }

    fs::create_dir(name)
}