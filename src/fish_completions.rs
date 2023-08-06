fn get_completions(parent: &String, command: &crate::config::Command) -> Option<String> {
    let mut ret = String::new();
    let commands = command
        .children_names()
        .unwrap_or(vec![])
        .join(" ")
        .replace('"', "");

    if !commands.is_empty() {
        ret += format!( r#"complete -c titan -n "__fish_seen_subcommand_from {parent}" -a "{commands}"
"#,)
        .as_str();
    }

    if let Some(children) = &command.children {
        if children.len() != 0 {
            for child in children {
                for (key, value) in child {
                    if let Some(child_completions) = {
                        let new_parent = format!("{parent} {key}");
                        get_completions(&new_parent, &value)
                    } {
                        ret += child_completions.as_str();
                    }
                }
            }
        }
    }

    return Some(ret);
}

pub fn fish_completions(config: &crate::config::Config) -> String {
    let mut ret = r#"
complete -c titan --keep-order --no-files
complete -c titan -s v -l verbose -d "Increase# verbosity, and can be used multiple times"
complete -c titan -s j -l json-schema -d "Generate json schema"
complete -c titan -s c -l config -d "  [default: \$HOME/.titan.toml]"
complete -c titan -s h -l help -d "Print help"
complete -c titan -s V -l version -d "Print version"

"#
    .to_owned();

    let mut keys: Vec<String> = vec![];

    for command in &config.commands {
        for (key, value) in command {
            keys.push(key.to_string());

            if let Some(child_completions) = get_completions(&key, &value) {
                ret += child_completions.as_str();
            }
        }
    }
    let kj = keys.join(" ");

    if !kj.is_empty() {
        ret += format!(
            r#"
set -l all_commands {kj}
complete -c titan -n "not __fish_seen_subcommand_from $all_commands" -a "$all_commands"
"#,
        )
        .as_str();
    }

    ret
}
