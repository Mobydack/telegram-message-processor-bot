/// Get environment variable with scope.
///
/// ```rust
/// let var = get_env_with_scope!("BAR", "FOO"); // Provide env variable APP_FOO_BAR
/// ```
#[macro_export]
macro_rules! get_env_with_scope {
    ($scope: literal, $env_variable_name: expr) => {{
        let normalized_env_name = format!("APP_{}_{}", $scope, $env_variable_name);

        match std::env::var(normalized_env_name.clone()) {
            Ok(value) => value,
            Err(e) => panic!("${} is not set ({})", normalized_env_name, e),
        }
    }};
}

pub(super) use get_env_with_scope;
