# Put this file in ~/.config/fish/completions/ 

complete -c mkml -s h -l help -d "Print help information"

complete -c mkml -n __fish_use_subcommand -s V -l version -d "Print version"
complete -c mkml -n __fish_use_subcommand -a clone -d "Replicate an existing project, which should contain a `mkml.json` files"
complete -c mkml -n __fish_use_subcommand -a init -d "Create a new starter project for a website"
complete -f -c mkml -n __fish_use_subcommand -a update -d "Auto-update mkml"

complete -c mkml -n "__fish_seen_subcommand_from clone" -s o -l overwrite -d "Allow files that exists to be overwritten"
complete -c mkml -n "__fish_seen_subcommand_from clone" -s s -l skip-exist -d "Skip files that exists"

complete -c mkml -n "__fish_seen_subcommand_from init" -s e -l exclude -d "Don't include CSS or JS to be in the project"
complete -c mkml -n "__fish_seen_subcommand_from init" -s m -l minimal -d "Create only a basic HTML template file"

complete -c mkml -n "__fish_seen_subcommand_from update" -s l -l list -d "Display all available mkml builds"
complete -c mkml -n "__fish_seen_subcommand_from update" -s v -l version -d "Choose a custom version of mkml to install instead of the latest"
complete -c mkml -n "__fish_seen_subcommand_from update" -s o -l omit-download-bar -d "Displays the download/progress bar"
