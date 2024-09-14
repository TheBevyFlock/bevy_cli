#[derive(Debug, Clone)]

/// A helper to make passing arguments to [`Command`](std::process::Command) more convenient.
pub struct ArgBuilder(Vec<String>);

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
    /// # use bevy_cli::external_cli::arg_builder::ArgBuilder;
    /// ArgBuilder::new().arg("--release");
    /// ```
    pub fn arg<A>(mut self, arg: A) -> Self
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
    /// # use bevy_cli::external_cli::arg_builder::ArgBuilder;
    /// ArgBuilder::new().add_with_value("--bin", "bevy");
    /// ```
    pub fn add_with_value<A, V>(self, arg: A, value: V) -> Self
    where
        A: Into<String>,
        V: Into<String>,
    {
        self.arg(arg).arg(value)
    }

    /// Add a boolean flag with the given name, if `value` is `true`.
    ///
    /// # Example
    ///
    /// ```
    /// # use bevy_cli::external_cli::arg_builder::ArgBuilder;
    /// let is_release = true;
    /// ArgBuilder::new().add_flag_if("--release", is_release);
    /// ```
    pub fn add_flag_if<N>(self, name: N, value: bool) -> Self
    where
        N: Into<String>,
    {
        if value {
            self.arg(name)
        } else {
            self
        }
    }

    /// Add an argument with an optional value.
    ///
    /// # Example
    ///
    /// ```
    /// # use bevy_cli::external_cli::arg_builder::ArgBuilder;
    /// let maybe_name = Some("bevy");
    /// ArgBuilder::new().add_opt_value("--bin", &maybe_name);
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

    /// Add an argument with multiple values.
    pub fn add_value_list<N, V>(self, name: N, value_list: Vec<V>) -> Self
    where
        N: Into<String>,
        V: Into<String>,
    {
        let values: Vec<String> = value_list.into_iter().map(|val| val.into()).collect();
        self.add_with_value(name, values.join(","))
    }

    /// Add all arguments from the other builder to this one.
    pub fn append(mut self, mut other: ArgBuilder) -> Self {
        self.0.append(&mut other.0);
        self
    }
}

impl Default for ArgBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl IntoIterator for ArgBuilder {
    type Item = <Vec<String> as IntoIterator>::Item;
    type IntoIter = <Vec<String> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
