pub mod application_settings;
pub mod database_settings;
pub mod redis_settings;

pub use application_settings::ApplicationSettings;
pub use database_settings::DatabaseSettings;
pub use redis_settings::RedisSettings;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Settings {
    pub application: application_settings::ApplicationSettings,
    pub database:    database_settings::DatabaseSettings,
    pub redis:       redis_settings::RedisSettings
}

impl Settings {
    pub fn new(path: &str) -> Result<Settings, config::ConfigError> {
        let base_path = std::env::current_dir().expect("Failed to determine the current directory");
        let configuration_directory = base_path.join(path);

        let environment: Environment = std::env::var("APP_ENVIRONMENT")
            .unwrap_or_else(|_| "local".into())
            .try_into()
            .expect("Failed to parse APP_ENVIRONMENT.");

        let environment_filename = format!("{}.yaml", environment.as_str());

        let settings = config::Config::builder()
            .add_source(config::File::from(
                configuration_directory.join("base.yaml")
            ))
            .add_source(config::File::from(
                configuration_directory.join(environment_filename)
            ))
            .add_source(
                config::Environment::with_prefix("APP")
                    .prefix_separator("_")
                    .separator("__")
            )
            .build()?;

        settings.try_deserialize::<Settings>()
    }
}

pub enum Environment {
    Local,
    Production
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Production => "production"
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            other => Err(format!(
                "{other} is not a supported environment. Use either `local` or `production`."
            ))
        }
    }
}
