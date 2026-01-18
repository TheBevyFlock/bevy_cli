#[derive(Debug, Default)]
pub struct FastCompilesConfig {
    toolchain: RustToolchain,
    scope: ConfigScope,
}

#[derive(Debug, Default)]
pub enum RustToolchain {
    #[default]
    Nightly,
    Beta,
    Stable,
}

pub enum ConfigScope {
    #[default]
    User,
    Workspace,
}
