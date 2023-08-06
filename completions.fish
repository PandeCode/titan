
complete -c titan --keep-order --no-files
complete -c titan -s v -l verbose -d "Increase# verbosity, and can be used multiple times"
complete -c titan -s j -l json-schema -d "Generate json schema"
complete -c titan -s c -l config -d "  [default: \$HOME/.titan.toml]"
complete -c titan -s h -l help -d "Print help"
complete -c titan -s V -l version -d "Print version"

complete -c titan -n "__fish_seen_subcommand_from ifconfig" -a "ip ua lang encoding mime forwarded all"
complete -c titan -n "__fish_seen_subcommand_from test" -a "world not"
complete -c titan -n "__fish_seen_subcommand_from test not" -a "not"
complete -c titan -n "__fish_seen_subcommand_from test not not" -a "not"
complete -c titan -n "__fish_seen_subcommand_from test not not not" -a "not"
complete -c titan -n "__fish_seen_subcommand_from update" -a "all pacman paru yarn pnpm python"
complete -c titan -n "__fish_seen_subcommand_from rust" -a "build run"
complete -c titan -n "__fish_seen_subcommand_from system" -a "cpu"
complete -c titan -n "__fish_seen_subcommand_from system cpu" -a "hogs"
complete -c titan -n "__fish_seen_subcommand_from music" -a "play play_next"

set -l all_commands ifconfig localip test update rust config system music
complete -c titan -n "not __fish_seen_subcommand_from $all_commands" -a "$all_commands"

