use crate::args::{Action, App, Exclude, InitProject};
use owo_colors::OwoColorize;

use super::create_file;
use crate::{create_dirs, create_files};
use std::{fs::create_dir, path::MAIN_SEPARATOR as SEP, f32::consts::E};

const HTML_CONTENTS: &str = r##"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta http-equiv="X-UA-Compatible" content="IE=edge">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <link rel="stylesheet" href="css/style.css">
    <title>Document</title>
</head>
<body>
    
</body>
</html>
"##;

const CSS_CONTENTS: &str = r##"* {
    padding: 0;
    margin: 0;
    box-sizing: border-box;
}
"##;

const JS_CONTENTS: &str = "console.log('Hello, World!');";

pub fn initialize_project(args: &App) {
    match &args.action {
        Action::Init(project) => match init(project) {
            Ok(()) => {}
            Err(err) => {
                log::error!("{err}");
                log::debug!("Error debug: {err:?}");
                println!("{}: {err}", "Error".red())
            }
        },
        Action::Clone(clone) => {}
        // more actions soon...
    }
}

fn init(project: &InitProject) -> std::io::Result<()> {
    let name = project.name.clone().unwrap_or_default();
    let exclude = match project.exclude.clone() {
        Some(exclude) => exclude,
        None => Exclude::default(), // Exclude::None,
    };

    if name.is_empty() {
        create_file(".mkml", "")?;
    } else {
        create_file(&format!("{name}{SEP}.mkml"), "")?;
    }

    if project.minimal {
        if name.is_empty() {
            match create_file("index.html", HTML_CONTENTS) {
                Ok(_) => return Ok(()),
                Err(err) => return Err(err),
            }
        } else {
            create_dir(&name)?;
            match create_file(&format!("{name}{SEP}index.html"), HTML_CONTENTS) {
                Ok(_) => {
                    println!("{}: created minimal project", "Success".green());
                    return Ok(());
                }
                Err(err) => return Err(err),
            }
        }
    }

    if name.is_empty() {
        match exclude {
            Exclude::JS | Exclude::Javascript => {
                log::info!("Excluding JS directory. Creating in current directory");
                create_dir("css")?;

                create_files!(
                    &format!("css{SEP}style.css") => CSS_CONTENTS,
                    "index.html" => HTML_CONTENTS
                );
            }
            Exclude::CSS => {
                log::info!("Excluding CSS directory. Creating in current directory");
                create_dir("js")?;

                create_files!(
                    &format!("js{SEP}index.js") => JS_CONTENTS,
                    "index.html" => HTML_CONTENTS
                );
            }
            Exclude::None => {
                log::info!("Full. Creating in current directory");

                create_files!(
                    &format!("css{SEP}style.css") => CSS_CONTENTS,
                    &format!("js{SEP}index.js") => JS_CONTENTS,
                    "index.html" => HTML_CONTENTS
                );

                create_dirs!("js", "css");
            }
        }
    } else {
        create_dir(&name)?;
        match exclude {
            Exclude::JS | Exclude::Javascript => {
                log::info!("Excluding JS directory. Creating in directory {name}");
                create_dir(&format!("{name}{SEP}css"))?;

                create_files!(
                    &format!("{name}{SEP}css{SEP}style.css") => CSS_CONTENTS,
                    &format!("{name}{SEP}index.html") => HTML_CONTENTS
                );
            }
            Exclude::CSS => {
                log::info!("Excluding CSS directory. Creating in directory {name}");
                create_dir(&format!("{name}{SEP}js"))?;

                create_files!(
                    &format!("{name}{SEP}js{SEP}index.js") => JS_CONTENTS,
                    &format!("{name}{SEP}index.html") => HTML_CONTENTS
                );
            }
            Exclude::None => {
                log::info!("Full. Creating in directory {name}");

                create_dirs!(
                    format!("{name}{SEP}js"),
                    format!("{name}{SEP}css")
                );

                create_files!(
                    &format!("{name}{SEP}css{SEP}style.css") => CSS_CONTENTS,
                    &format!("{name}{SEP}js{SEP}index.js") => JS_CONTENTS,
                    &format!("{name}{SEP}index.html") => HTML_CONTENTS
                );
            }
        }
    }

    if !name.is_empty() {
        println!("{}: created `{name}`", "Success".green());
    } else {
        println!("{}: created project", "Success".green());
    }

    Ok(())
}
