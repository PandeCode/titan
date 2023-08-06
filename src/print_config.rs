use miette::{IntoDiagnostic, Result};
use ptree::print_config::StaticIndentChars;
use ptree::{print_tree_with, PrintConfig, Style, TreeBuilder};

macro_rules! loop_children {
    ($tree: expr, $children: expr) => {
        for child in $children {
            for (k, v) in child {
                $tree.begin_child(k.to_string());
                get_children(v, $tree);
                $tree.end_child();
            }
        }
    };
}

fn get_children(command: &crate::config::Command, tree: &mut TreeBuilder) {
    if let Some(children) = &command.children {
        loop_children!(tree, children);
    }
}

pub fn print_config(config: &crate::config::Config) -> Result<()> {
    let mut tree = TreeBuilder::new("tree".to_string());

    for hm in &config.commands {
        for (key, val) in hm {
            tree.begin_child(key.to_string());
            if let Some(children) = &val.children {
                loop_children!(&mut tree, children);
            } else {
                tree.add_empty_child(key.to_string());
            }
            tree.end_child();
        }
    }

    let tree = tree.build();

    let config: PrintConfig = {
        let mut config = PrintConfig::from_env();
        config.branch = Style {
            // foreground: Some(Color::Red),
            // background: Some(Color::Yellow),
            dimmed: true,
            ..Style::default()
        };
        config.leaf = Style {
            bold: true,
            ..Style::default()
        };
        config.characters = StaticIndentChars {
            down_and_right: "├",
            down: "│",
            turn_right: "╰",
            right: "─",
            empty: " ",
        }
        .into();

        config.indent = 4;
        config
    };

    print_tree_with(&tree, &config).into_diagnostic()?;

    Ok(())
}
