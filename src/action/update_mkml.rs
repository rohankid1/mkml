use super::Error;
use owo_colors::{AnsiColors, OwoColorize};
use regex::Regex;
use self_update::backends::github::{ReleaseList, Update};
use self_update::cargo_crate_version;

const CURRENT_VERSION: &str = cargo_crate_version!();

pub fn update(args: &crate::args::UpdateFlags) {
    match update_cli(args) {
        Ok(()) => {}
        Err(err) => {
            // log::error!("Error: {err:?}");
            eprintln!("{}: {err}", "ERROR".red());
        }
    }
}

fn update_cli(args: &crate::args::UpdateFlags) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(version) = args.version.clone() {
        return install_custom_version(args, version);
    }

    if args.list {
        return list_builds();
    }

    let status = Update::configure()
        .repo_owner("rohankid1")
        .repo_name("mkml")
        .bin_name("mkml")
        .current_version(CURRENT_VERSION)
        .show_download_progress(args.download_bar)
        .build()?
        .update()?;

    if status.uptodate() {
        println!("{}", "mkml is already up to date!".green());
    }

    Ok(())
}

fn install_custom_version(
    args: &crate::args::UpdateFlags,
    version: String,
) -> Result<(), Box<dyn std::error::Error>> {
    if !right_version_format(&version) {
        return Err(Box::new(Error(
            "Incorrect version string usage.\nUSAGE: <NUM>.<NUM>.<NUM>",
        )));
    }

    let status = Update::configure()
        .repo_owner("rohankid1")
        .repo_name("mkml")
        .bin_name("mkml")
        .current_version(CURRENT_VERSION)
        .target_version_tag(&version)
        .show_download_progress(args.download_bar)
        .build()?
        .update()?;

    if status.uptodate() {
        println!("{}", "mkml is already up to date!".green());
    }

    Ok(())
}

fn list_builds() -> Result<(), Box<dyn std::error::Error>> {
    let list = ReleaseList::configure()
        .repo_owner("rohankid1")
        .repo_name("mkml")
        .build()?
        .fetch()?;

    for build in list {
        let version = if build.version == CURRENT_VERSION {
            build.version.color(AnsiColors::Green)
        } else {
            build.version.color(AnsiColors::Default)
        };

        println!("{} ({version}) -> Publish date: {}", build.name, build.date);
    }

    Ok(())
}

fn right_version_format(str: &str) -> bool {
    let re = Regex::new(r"^(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)(?:-((?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(?:\+([0-9a-zA-Z-]+(?:\.[0-9a-zA-Z-]+)*))?$").unwrap();
    re.is_match(str)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn right_version() {
        let version = "0.1.0";
        let version2 = "1.1.1";
        let version3 = "1.10.9";

        assert!(right_version_format(version));
        assert!(right_version_format(version2));
        assert!(right_version_format(version3));
    }

    #[test]
    fn bad_version() {
        let version = "a.b.c";
        let version2 = "x.5.2";
        let version3 = "1.x.42";
        let version4 = "1.3.5.6.39.43";

        assert!(!right_version_format(version));
        assert!(!right_version_format(version2));
        assert!(!right_version_format(version3));
        assert!(!right_version_format(version4));
    }
}
