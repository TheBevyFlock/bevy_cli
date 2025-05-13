# Scaffolding

The `bevy new` command lets you easily scaffold a new Bevy project using a custom template or a [minimal template provided by Bevy](https://github.com/TheBevyFlock/bevy_new_minimal). Templates are just GitHub repositories and can be specified with the `-t` flag.

## Usage

If the template is omitted, the [default minimal template](https://github.com/TheBevyFlock/bevy_new_minimal) will be chosen.

```sh
bevy new my-project
```

Other built-in templates from [TheBevyFlock](https://github.com/TheBevyFlock) will be usable via its shortcut form i.e. `-t 2d` will use the template [bevy_new_2d](https://github.com/TheBevyFlock/bevy_new_2d).

```sh
bevy new my-project -t 2d
```

To use a specific template, provide the full GitHub URL:

```sh
bevy new my-project -t https://github.com/TheBevyFlock/bevy_new_2d.git
```
