# Scaffolding

The `bevy new` command lets you easily scaffold a new Bevy project using a custom template or [the default minimal template, `bevy_new_minimal`](https://github.com/TheBevyFlock/bevy_new_minimal). Internally, `bevy new` relies on [`cargo generate`](https://github.com/cargo-generate/cargo-generate) and will ask to install it for you if you don't already have it. Templates are just GitHub repositories and can be specified with the `--template` / `-t` flag.

## Usage

If the template is omitted, [`bevy_new_minimal`](https://github.com/TheBevyFlock/bevy_new_minimal) will be chosen:

```sh
bevy new my-project
```

Other built-in templates from [TheBevyFlock](https://github.com/TheBevyFlock) will be usable via its shortcut form. For example, `-t 2d` will use the template [`bevy_new_2d`](https://github.com/TheBevyFlock/bevy_new_2d):

```sh
bevy new -t 2d my-project
```

To use any other template on Github, provide the full URL:

```sh
bevy new my-project --template https://github.com/TheBevyFlock/bevy_new_2d
```

To pass additional arguments to `cargo-generate`, put them after `--`:

```sh
# Don't prompt the user and use the default values for variables.
bevy new -t 2d my-project -- --silent
```
