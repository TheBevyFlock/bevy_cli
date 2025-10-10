# Minimal Footprint

The Bevy CLI is not only used by people in the terminal, but also as a tool in CI workflows.
This means that it will be compiled or downloaded very frequently.

Hence, we have a responsibility to keep compile times and binary size as low as we can.

## Features

To account for different use-cases of the CLI and different environments where it can be used,
we need to make heavy use of Cargo features.

For example, not everyone wants to develop a web app with Bevy,
so they should be able to compile out everything related to the web functionality.
This can shave of a lot of dependencies and additional code for these use-cases.

Similarly, some environments (such as Nix OS) don't have tools like `rustup` available.
The packages for these distributions should be able to compile out any checks or usages related to `rustup` to avoid any overhead.

## Dependencies

When adding a new dependency, please _disable default features_ and only enable the features that we really need.
This can drastically reduce the footprint of the CLI and reduces both compile times and binary size.

Similarly, when a dependency is only used in a specific context,
make it optional and only include it when the corresponding feature is enabled.
