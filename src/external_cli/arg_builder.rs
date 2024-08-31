#[derive(Debug, Clone)]

/// A helper to make passing arguments to [`std::process::Command`] more convenient.
pub(crate) struct ArgBuilder(Vec<String>);

impl ArgBuilder {
    /// Create a new builder for command arguments.
    pub fn new() -> Self {
        Self(Vec::new())
    }

    /// Add a singular argument.
    ///
    /// # Example
    ///
    /// ```
    /// # use crate::external_cli::arg_builder::ArgBuilder;
    /// ArgBuilder::new().add("--release");
    /// ```
    pub fn add<A>(mut self, arg: A) -> Self
    where
        A: Into<String>,
    {
        self.0.push(arg.into());
        self
    }

    /// Add an argument with a value.
    ///
    /// # Example
    ///
    /// ```
    /// # use crate::external_cli::arg_builder::ArgBuilder;
    /// ArgBuilder::new().add_with_value("--bin", "bevy");
    /// ```
    pub fn add_with_value<A, V>(self, arg: A, value: V) -> Self
    where
        A: Into<String>,
        V: Into<String>,
    {
        self.add(arg).add(value)
    }

    /// Add a boolean flag with the given name, if `value` is `true`.
    ///
    /// # Example
    ///
    /// ```
    /// # use crate::external_cli::arg_builder::ArgBuilder;
    /// let is_release = true;
    /// ArgBuilder::new().add_flag("--release", is_release);
    /// ```
    pub fn add_flag_if<N>(self, name: N, value: bool) -> Self
    where
        N: Into<String>,
    {
        if value {
            self.add(name)
        } else {
            self
        }
    }

    /// Add an argument with an optional value.
    ///
    /// # Example
    ///
    /// ```
    /// # use crate::external_cli::arg_builder::ArgBuilder;
    /// let maybe_name = Some("bevy");
    /// ArgBuilder::new().add_opt_value("--bin", maybe_name);
    /// ```
    pub fn add_opt_value<N, V>(self, name: N, value: &Option<V>) -> Self
    where
        N: Into<String>,
        V: Into<String> + Clone,
    {
        if let Some(value) = value {
            self.add_with_value::<N, V>(name, value.clone())
        } else {
            self
        }
    }
}

impl IntoIterator for ArgBuilder {
    type Item = <Vec<String> as IntoIterator>::Item;
    type IntoIter = <Vec<String> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}