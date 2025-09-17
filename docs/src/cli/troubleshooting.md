# Troubleshooting

## The `--help` flag

All CLI commands support the `--help` and `-h` flags, which provides a reference to available options, subcommands, and arguments:

```sh
bevy --help

# Prints a shorter, summarized version of the help screen.
bevy -h

# Prints web-specific options.
bevy build web --help
```

## View debug logs

If you encounter issues or don't understand what the CLI is doing, try adding the `--verbose` flag. Every command that the CLI executes will be logged, making it easy to understand what's going on!

Internally, the CLI uses [`tracing`](https://crates.io/crates/tracing) for logging. You can control what level of logs are displayed using the `BEVY_LOG` environmental variable:

```sh
# Equivalent to `bevy build --verbose`.
BEVY_LOG=debug bevy build
```

The supported values for `BEVY_LOG` are:

- `error`
- `warn`
- `info` (default)
- `debug` (enabled with `--verbose`)
- `trace`
