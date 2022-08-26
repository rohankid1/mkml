use crate::args::{Action, App, Exclude, InitProject};
use owo_colors::OwoColorize;

use super::create_file;
use std::{fs::create_dir, path::MAIN_SEPARATOR};

const HTML_CONTENTS: &str = r##"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta http-equiv="X-UA-Compatible" content="IE=edge">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
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

const JS_CONTENTS: &str = r##"console.log('Hello, World!');
"##;

pub fn initialize_project(args: &App) {
    match &args.action {
        Action::Init(str) => match init(str) {
            Ok(()) => {}
            Err(err) => {
                log::error!("{err}");
                log::debug!("Error debug: {err:?}");
                println!("{}: {err}", "Error".red())
            }
        },
        // more actions soon...
    }
}

fn init(project: &InitProject) -> std::io::Result<()> {
    let name = project.name.clone().unwrap_or_default();
    let exclude = match project.exclude.clone() {
        Some(exclude) => exclude,
        None => Exclude::default(), // Exclude::None,
    };

    if project.minimal {
        if name.is_empty() {
            match create_file("index.html", HTML_CONTENTS) {
                Ok(_) => return Ok(()),
                Err(err) => return Err(err),
            }
        } else {
            create_dir(&name)?;
            match create_file(&format!("{name}/index.html"), HTML_CONTENTS) {
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
                create_file(&format!("css{MAIN_SEPARATOR}style.css"), CSS_CONTENTS)?;
                create_file("index.html", HTML_CONTENTS)?;
            }
            Exclude::CSS => {
                log::info!("Excluding CSS directory. Creating in current directory");
                create_dir("js")?;
                create_file(&format!("js{MAIN_SEPARATOR}index.js"), JS_CONTENTS)?;
                create_file("index.html", HTML_CONTENTS)?
            }
            Exclude::None => {
                log::info!("Full. Creating in current directory");
                create_dir("js")?;
                create_file(&format!("js{MAIN_SEPARATOR}index.js"), JS_CONTENTS)?;
                create_dir("css")?;
                create_file(&format!("css{MAIN_SEPARATOR}style.css"), CSS_CONTENTS)?;
                create_file("index.html", HTML_CONTENTS)?
            }
        }
    } else {
        create_dir(&name)?;
        match exclude {
            Exclude::JS | Exclude::Javascript => {
                log::info!("Excluding JS directory. Creating in directory {name}");
                create_dir(&format!("{name}{MAIN_SEPARATOR}css"))?;
                create_file(
                    &format!("{name}{MAIN_SEPARATOR}css{MAIN_SEPARATOR}style.css"),
                    CSS_CONTENTS,
                )?;
                create_file(&format!("{name}{MAIN_SEPARATOR}index.html"), HTML_CONTENTS)?;
            }
            Exclude::CSS => {
                log::info!("Excluding CSS directory. Creating in directory {name}");
                create_dir(&format!("{name}{MAIN_SEPARATOR}js"))?;
                create_file(
                    &format!("{name}{MAIN_SEPARATOR}js{MAIN_SEPARATOR}index.js"),
                    JS_CONTENTS,
                )?;
                create_file(&format!("{name}{MAIN_SEPARATOR}index.html"), HTML_CONTENTS)?;
            }
            Exclude::None => {
                log::info!("Full. Creating in directory {name}");
                create_dir(&format!("{name}{MAIN_SEPARATOR}js"))?;
                create_dir(&format!("{name}{MAIN_SEPARATOR}css"))?;
                create_file(
                    &format!("{name}{MAIN_SEPARATOR}css{MAIN_SEPARATOR}style.css"),
                    CSS_CONTENTS,
                )?;
                create_file(
                    &format!("{name}{MAIN_SEPARATOR}js{MAIN_SEPARATOR}index.js"),
                    JS_CONTENTS,
                )?;
                create_file(&format!("{name}{MAIN_SEPARATOR}index.html"), HTML_CONTENTS)?;
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
