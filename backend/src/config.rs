use secrecy::{ExposeSecret, Secret};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct AppConfig {
    pub database: DatabaseSettings,
    pub http: HttpSettings,
}

#[derive(Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: Secret<String>,
    pub host: String,
    pub port: u16,
    pub name: String,
}

impl DatabaseSettings {
    //
    /// Get the string used for connecting to the database.
    pub fn connection_string(&self) -> Secret<String> {
		log::debug!(">>> Entering connection_string()");
        log::debug!("<<< Exiting connection_string()");
        Secret::new(format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port,
            self.name
        ))
    }

    /// Such string allows connecting to the Postgres instance, without specifying a database.
    pub fn connection_string_without_db(&self) -> Secret<String> {
		log::debug!(">>> Entering connection_string_without_db()");
        log::debug!("<<< Exiting connection_string_without_db()");
        Secret::new(format!(
            "postgres://{}:{}@{}:{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port
        ))
    }
}

#[derive(Deserialize)]
pub struct HttpSettings {
    pub host: String,
    pub port: u16,
}

/// The runtime environment.
pub enum Environment {
    Local,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
			log::debug!(">>> Entering as_str()");
        log::debug!("<<< Exiting as_str()");
            Environment::Local => "local",
            Environment::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
		log::debug!(">>> Entering try_from(s={})", s);
        log::debug!("<<< Exiting try_from(s={})", s);
        match s.to_lowercase().as_str() {
            "local" => Ok(Environment::Local),
            "production" => Ok(Environment::Production),
            other => Err(format!(
                "{other} is not a supported environment. Use either local or production"
            )),
        }
    }
}

/// Get the application config.
pub fn get_config() -> Result<AppConfig, config::ConfigError> {
	log::debug!(">>> Entering get_config()");
    //
    let base_dir = std::env::current_dir().expect("Failed to determine the current directory");
	log::debug!("get_config(): base_dir → (unprintable)");
	log::debug!("get_config(): config_dir → (unprintable)");
    let config_dir = base_dir.join("config");

    // In the `./config` directory we have 3 yaml files: base, local, and production.
    let base_src =
        config::File::with_name(config_dir.join("base").to_str().unwrap()).required(true);

    let env: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or("local".into())
        .try_into()
        .expect("Failed to parse APP_ENVIRONMENT");

    let env_src =
	log::debug!("<<< Exiting get_config()");
        config::File::with_name(config_dir.join(env.as_str()).to_str().unwrap()).required(true);

    config::Config::builder()
        // Load the config.(yml|yaml|toml|...) file.
        .add_source(base_src)
        .add_source(env_src)
        .build()
        .unwrap()
        .try_deserialize()
}
