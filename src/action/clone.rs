use super::{fail, success};
use crate::args::{Action, App, CloneProject};

use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};
use serde_json::error::Result;

use std::io::{Error, ErrorKind};
use std::path::PathBuf;
use std::{
    fs,
    path::{Path, MAIN_SEPARATOR as SEP},
};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct Context {
    name: String,
    description: Option<String>,
    authors: Option<Vec<String>>,
    #[serde(skip)]
    path: String,
}

impl Context {
    fn with_path(&mut self, path: String) -> &mut Self {
        let mut new = self;
        new.path = path;
        new
    }

    fn with_name(&mut self, name: String) -> &mut Self {
        let mut new = self;
        new.name = name;
        new
    }

    fn with_description(&mut self, description: Option<String>) -> &mut Self {
        let mut new = self;
        new.description = description;
        new
    }

    fn with_authors(&mut self, authors: Option<Vec<String>>) -> &mut Self {
        let mut new = self;
        new.authors = authors;
        new
    }

    fn deserialize_from_path(&self) -> std::io::Result<Context> {
        // let file = std::fs::read_to_string(&self.path).expect("Failed to read file");
        // let metadata = fs::metadata(&self.path).unwrap();
        // if metadata.is_dir() {}
        // let json: Context = serde_json::from_str(&file)?;

        let mut context = self;

        let md = match fs::metadata(&context.path) {
            Ok(md) => md,
            Err(err) => return Err(err),
        };

        if md.is_dir() {
            log::info!("Cloning project: got directory");
            let mut found = false;
            for entry in fs::read_dir(&self.path)?.into_iter() {
                let entry = entry?;

                if entry.file_name() == "mkml.json" {
                    found = true;

                    let mut json: Context =
                        match serde_json::from_str(&fs::read_to_string(&entry.path())?) {
                            Ok(ctx) => ctx,
                            Err(error) => {
                                log::error!("Failed to deserialize: {error}");
                                return Err(Error::new(
                                    ErrorKind::Other,
                                    "Failed to deserialize JSON file",
                                ));
                            }
                        };

                    json.path = context.path.clone();

                    return Ok(json);
                }
            }

            if !found {
                return Err(Error::new(
                    ErrorKind::NotFound,
                    format!("In `{}`: a `mkml.json` could not be found", self.path),
                ));
            }
        }

        if md.is_file() {
            log::info!("Cloning project: got file");
            let file = fs::read_to_string(&self.path)?;

            let json: Context = match serde_json::from_str(&file) {
                Ok(ctx) => ctx,
                Err(error) => {
                    log::error!("Failed to deserialize: {error}");
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "Failed to deserialize JSON file",
                    ));
                }
            };
        }

        Ok(context.clone())
    }
}

pub fn clone_project(args: &App) {
    match &args.action {
        Action::Clone(project) => match clone(project) {
            Ok(ctx) => success(&format!("cloned project: {}", ctx.name)),
            Err(err) => fail(&format!("{err}")),
        },
        _ => unreachable!(),
    }
}

fn clone(project: &CloneProject) -> std::io::Result<Context> {
    let mut ctx = Context::default()
        .with_path(project.path.clone())
        .deserialize_from_path()?;

    ctx.path = project.path.clone();

    log::info!(
        "Config: overwrite: {}, skip_exist: {}",
        project.overwrite,
        project.skip_exist
    );

    let mut options = fs_extra::dir::CopyOptions::new();
    options.overwrite = project.overwrite;
    options.skip_exist = project.skip_exist;
    options.copy_inside = true;

    log::info!("Cloning project: `{}`", ctx.name);

    if let Some(desc) = &ctx.description {
        log::info!("Project's description: \"{desc}\"");
    }

    if let Some(authors) = &ctx.authors {
        log::info!("Project's authors: {authors:?}");
    }

    let path = fs::metadata(&ctx.path)?;

    let renamed = Path::new(&project.rename);

    if !project.overwrite && renamed.try_exists()? {
        return Err(Error::new(
            ErrorKind::AlreadyExists,
            "cannot clone with new name as it already exists. Use -o to overwrite",
        ));
    }

    log::info!("ctx.path: {}", ctx.path);
    if path.is_dir() {
        match fs_extra::copy_items(
            &get_dir_contents(&ctx.path)?,
            std::env::current_dir()?.join(&project.rename),
            &options,
        ) {
            Ok(_) => {}
            Err(err) => return Err(Error::new(ErrorKind::Other, err)),
        }
    } else if path.is_file() {
        let mut dir = ctx.path.clone();
        let actual_dir = dir.trim_end_matches("mkml.json");

        match fs_extra::copy_items(
            &get_dir_contents(actual_dir)?,
            std::env::current_dir()?.join(&project.rename),
            &options,
        ) {
            Ok(_) => log::info!(
                "Creating clone in: {}",
                std::env::current_dir()?
                    .join(&project.rename)
                    .to_str()
                    .unwrap()
            ),
            Err(err) => return Err(Error::new(ErrorKind::Other, err)),
        }
    }

    ctx.name = project.rename.clone();

    Ok(ctx)
}

fn get_dir_contents<T: AsRef<Path>>(path: T) -> std::io::Result<Vec<PathBuf>> {
    Ok(path
        .as_ref()
        .read_dir()?
        .flat_map(|entry| entry.ok())
        .flat_map(|entry| Some(entry.path()))
        .collect())
}
