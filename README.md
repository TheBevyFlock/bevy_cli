# Bevy CLI

A prototype [Bevy] CLI tool intended to streamline common tasks when working on projects. Please see the [initial scope document] and [original issue] for history and motivation.

- [**Documentation**](https://thebevyflock.github.io/bevy_cli/)
- [**Repository**](https://github.com/TheBevyFlock/bevy_cli)
- [**Issue Tracker**](https://github.com/TheBevyFlock/bevy_cli/issues)

If you need assistance or want to help, reach out to the [`bevy_cli` working group channel] in the [Bevy Discord].

[Bevy]: https://bevyengine.org
[initial scope document]: https://hackmd.io/cCHAfbtaSviU_MDnbNHKxg
[original issue]: https://github.com/bevyengine/bevy/issues/436
[`bevy_cli` working group channel]: https://discord.com/channels/691052431525675048/1278871953721262090
[Bevy Discord]: https://discord.gg/bevy

## Installation

<!-- Please keep this section synchronized with the `mdbook` docs. -->

At this point, the CLI is not published on <https://crates.io> and needs to be installed via Git:

```sh
cargo install --git https://github.com/TheBevyFlock/bevy_cli --locked bevy_cli
```

The **linter** is not included with the CLI, and will need to be installed separately if you wish to use it. Please refer to its [installation page](https://thebevyflock.github.io/bevy_cli/api/bevy_lint) for more information!

## Quick Start

<!-- Please keep this section synchronized with the `mdbook` docs. -->

With the following steps, you can create a new 2D app with Bevy and run it in your browser:

1. Create a new Bevy app using [the 2D template](https://github.com/TheBevyFlock/bevy_new_2d):

    ```sh
    bevy new -t=2d my_bevy_app
    ```

2. Navigate into the folder:

   ```sh
   cd my_bevy_app
   ```

3. Check the code quality with the linter:

    ```sh
    bevy lint
    ```

4. Run the app in the browser:

    ```sh
    bevy run web --open
    ```

## License

The Bevy CLI is licensed under either of

- Apache License, Version 2.0 ([`LICENSE-APACHE`](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([`LICENSE-MIT`](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contributing

Please see [`CONTRIBUTING.md`](CONTRIBUTING.md) for more information!

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
