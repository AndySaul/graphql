use env_logger::Env;

/// Initialises log settings
///
/// Defaults to "info" if ```GRAPHQL_APP_LOG_LEVEL``` not set
pub fn init() {
    env_logger::init_from_env(
        Env::default()
            .filter_or("GRAPHQL_APP_LOG_LEVEL", "info")
            .write_style_or("GRAPHQL_APP_LOG_STYLE", "never"),
    );
}
