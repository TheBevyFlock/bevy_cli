/// Configure the arguments for an external CLI command.
///
/// Can either be disabled, enabled with default arguments, or enabled with custom arguments.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExternalCliArgs {
    /// Disable the external command if `false`, use default args if `true`.
    Enabled(bool),
    /// Enable and use the provided arguments.
    Args(Vec<String>),
}

impl ExternalCliArgs {
    /// Parse the arguments from a list.
    ///
    /// `true` and `false` are treated special, to enable or disable the command.
    pub fn from_raw_args(args: Vec<String>) -> Self {
        let mut cur_args = Vec::<String>::new();

        for arg in args {
            match arg.to_lowercase().as_str() {
                "true" => return Self::Enabled(true),
                "false" => return Self::Enabled(false),
                _ => cur_args.push(arg),
            }
        }

        if cur_args.is_empty() {
            Self::Enabled(false)
        } else {
            Self::Args(cur_args)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_true() {
        let args = vec!["true".to_string()];
        let parsed = ExternalCliArgs::from_raw_args(args);
        assert!(matches!(parsed, ExternalCliArgs::Enabled(true)));
    }

    #[test]
    fn should_parse_false() {
        let args = vec!["false".to_string()];
        let parsed = ExternalCliArgs::from_raw_args(args);
        assert!(matches!(parsed, ExternalCliArgs::Enabled(false)));
    }

    #[test]
    fn should_parse_args() {
        let args = vec!["arg1".to_string(), "arg2".to_string()];
        let parsed = ExternalCliArgs::from_raw_args(args);
        assert!(matches!(
            parsed,
            ExternalCliArgs::Args(ref args) if args == &["arg1".to_string(), "arg2".to_string()]
        ));
    }
}
