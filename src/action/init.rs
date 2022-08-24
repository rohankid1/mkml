use crate::args::{Action, App, Exclude, InitProject};
use owo_colors::OwoColorize;

use super::{create_directory, create_file};
use std::path::MAIN_SEPARATOR;

const HTML_CONTENTS: &str = r##"
<!DOCTYPE html>
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

const CSS_CONTENTS: &str = r##"
* {
    padding: 0;
    margin: 0;
    box-sizing: border-box;
}
"##;

const JS_CONTENTS: &str = r##"
console.log('Hello, World!');
"##;

#[inline]
pub fn initialize_project(args: &App) {
    match &args.action {
        Action::Init(str) => match init(str) {
            Ok(()) => (),
            Err(err) => {
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

    if name.is_empty() {
        match exclude {
            Exclude::JS => {
                log::info!("Excluding JS directory");
                create_directory("css")?;
                create_file(&format!("css{MAIN_SEPARATOR}style.css"), CSS_CONTENTS)?;
                create_file("index.html", HTML_CONTENTS)?;
            }
            Exclude::CSS => {
                log::info!("Excluding CSS directory");
                create_directory("js")?;
                create_file(&format!("js{MAIN_SEPARATOR}index.js"), JS_CONTENTS)?;
                create_file("index.html", HTML_CONTENTS)?
            }
            Exclude::None => {
                log::info!("No exclusion");
                create_directory("js")?;
                create_file(&format!("js{MAIN_SEPARATOR}index.js"), JS_CONTENTS)?;
                create_directory("css")?;
                create_file(&format!("css{MAIN_SEPARATOR}style.css"), CSS_CONTENTS)?;
                create_file("index.html", HTML_CONTENTS)?
            }
        }
    } else {
        create_directory(&name)?;
        match exclude {
            Exclude::JS => {
                log::info!("Excluding JS directory");
                create_directory(&format!("{name}{MAIN_SEPARATOR}css"))?;
                create_file(
                    &format!("{name}{MAIN_SEPARATOR}css{MAIN_SEPARATOR}style.css"),
                    CSS_CONTENTS,
                )?;
                create_file(&format!("{name}{MAIN_SEPARATOR}index.html"), HTML_CONTENTS)?;
            }
            Exclude::CSS => {
                log::info!("Excluding CSS directory");
                create_directory(&format!("{name}{MAIN_SEPARATOR}js"))?;
                create_file(
                    &format!("{name}{MAIN_SEPARATOR}js{MAIN_SEPARATOR}index.js"),
                    CSS_CONTENTS,
                )?;
                create_file(&format!("{name}{MAIN_SEPARATOR}index.html"), HTML_CONTENTS)?;
            }
            Exclude::None => {
                log::info!("No exclusion");
                create_directory(&format!("{name}{MAIN_SEPARATOR}js"))?;
                create_directory(&format!("{name}{MAIN_SEPARATOR}css"))?;
                create_file(
                    &format!("{name}{MAIN_SEPARATOR}css{MAIN_SEPARATOR}style.css"),
                    CSS_CONTENTS,
                )?;
                create_file(
                    &format!("{name}{MAIN_SEPARATOR}js{MAIN_SEPARATOR}index.js"),
                    CSS_CONTENTS,
                )?;
                create_file(&format!("{name}{MAIN_SEPARATOR}index.html"), HTML_CONTENTS)?;
            }
        }
    }

    Ok(())
}
