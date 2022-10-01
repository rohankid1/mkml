use super::{
    super::{create_dirs, files},
    create_file, fail, success,
};
use crate::args::{Action, App, Exclude, InitProject};
use std::fs::create_dir;
use std::io::Write;
use std::path::MAIN_SEPARATOR as SEP;

const HTML_CONTENTS: &str = r##"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta http-equiv="X-UA-Compatible" content="IE=edge">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <link rel="stylesheet" href="css/style.css">
    <script src="js/index.js" />
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

fn generate_default_json_config_file(name: &str) -> String {
    format!(
        r#"{{
    "name": "{name}",
    "description": "",
    "authors": [""]
}}"#
    )
}

pub fn initialize_project(args: &App) {
    match &args.action {
        Action::Init(project) => match init(project) {
            Ok(()) => {}
            Err(err) => {
                log::error!("{err:?}");
                fail(&format!("{err}"));
            }
        },
        _ => unreachable!(),
    }
}

fn init(project: &InitProject) -> std::io::Result<()> {
    let mut name = project.name.clone().unwrap_or_default();
    name = name.split_whitespace().collect::<String>().to_owned();
    let is_empty = name.is_empty();
    let exclude = match project.exclude.clone() {
        Some(exclude) => exclude,
        None => Exclude::default(), // Exclude::None,
    };

    if name.ends_with(SEP) {
        name = name.strip_suffix(SEP).unwrap().to_owned();
    }

    if is_empty {
        create_file("mkml.json", &generate_default_json_config_file(&name))?;
        name = ".".to_owned();
    } else {
        create_dirs!("{name}");

        let json = generate_default_json_config_file(&name);

        files!(
            {"{name}/mkml.json" => "{json}"}
        );
    }

    if project.minimal {
        if is_empty {
            match create_file("index.html", HTML_CONTENTS) {
                Ok(_) => return Ok(()),
                Err(err) => return Err(err),
            }
        } else {
            match create_file(&format!("{name}/index.html"), HTML_CONTENTS) {
                Ok(_) => {
                    success("created minimal project");
                    return Ok(());
                }
                Err(err) => return Err(err),
            }
        }
    }

    match exclude {
        Exclude::JS | Exclude::Javascript => {
            log::info!("Excluding JS directory. Creating in directory {name}");

            create_dirs!("{name}/css");

            files!(
                {"{name}/css/style.css" => "{CSS_CONTENTS}"},
                {"{name}/index.html" => "{HTML_CONTENTS}"}
            );
        }
        Exclude::CSS => {
            log::info!("Excluding CSS directory. Creating in directory {name}");

            create_dirs!("{name}/js");

            files!(
                {"{name}/js/index.js" => "{JS_CONTENTS}"},
                {"{name}/index.html" => "{HTML_CONTENTS}"}
            );
        }
        Exclude::None => {
            log::info!("No exclusion. Creating in directory {name}");

            create_dirs!("{name}/css", "{name}/js");

            files!(
                {"{name}/css/style.css" => "{CSS_CONTENTS}"},
                {"{name}/js/index.js" => "{JS_CONTENTS}"},
                {"{name}/index.html" => "{HTML_CONTENTS}"}
            );
            // }
        }
    }

    if is_empty {
        success("created project");
    } else {
        success(&format!("created `{name}`"));
    }

    Ok(())
}
