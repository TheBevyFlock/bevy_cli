use std::{collections::HashSet, ffi::OsString};

use clap::CommandFactory;

/// Expand a user-defined Cargo alias in `argv`, so that it can be parsed by Clap.
pub fn expand<C: CommandFactory>(argv: Vec<OsString>) -> Vec<OsString> {
    let Some(index) = subcommand_index(&argv) else {
        return argv;
    };

    let Some(name) = argv[index].to_str() else {
        return argv;
    };

    if builtin_subcommands::<C>().contains(name) {
        return argv;
    }

    let Ok(config) = cargo_config2::Config::load() else {
        return argv;
    };

    let Some(alias) = config.alias.get(name) else {
        return argv;
    };

    let mut expanded = argv[..index].to_vec();
    expanded.extend(alias.list.iter().map(OsString::from));
    expanded.extend_from_slice(&argv[index + 1..]);
    expanded
}

fn builtin_subcommands<C: CommandFactory>() -> HashSet<String> {
    let mut command = C::command();
    command.build();

    command
        .get_subcommands()
        .flat_map(|subcommand| {
            std::iter::once(subcommand.get_name().to_owned())
                .chain(subcommand.get_all_aliases().map(str::to_owned))
        })
        .collect()
}

fn subcommand_index(argv: &[OsString]) -> Option<usize> {
    for (index, arg) in argv.iter().enumerate().skip(1) {
        let arg = arg.as_encoded_bytes();

        if arg == b"--" {
            return None;
        }

        if !arg.starts_with(b"-") {
            return Some(index);
        }
    }

    None
}
