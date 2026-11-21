use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub port: u16,
    pub public_base_url: String,
    pub database_path: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            port: 8080,
            public_base_url: "http://localhost:8080".to_string(),
            database_path: "./urls.sqlite3".to_string(),
        }
    }
}

impl Config {
    pub fn from_env() -> Self {
        let defaults = Self::default();
        Self {
            port: env::var("PORT")
                .ok()
                .and_then(|value| value.parse().ok())
                .unwrap_or(defaults.port),
            public_base_url: env::var("PUBLIC_BASE_URL").unwrap_or(defaults.public_base_url),
            database_path: env::var("DATABASE_PATH").unwrap_or(defaults.database_path),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use super::*;

    static ENV_LOCK: Mutex<()> = Mutex::new(());

    fn clear_env() {
        env::remove_var("PORT");
        env::remove_var("PUBLIC_BASE_URL");
        env::remove_var("DATABASE_PATH");
    }

    #[test]
    fn defaults_are_used_when_variables_are_unset() {
        let _guard = ENV_LOCK.lock().unwrap();
        clear_env();
        let config = Config::from_env();
        assert_eq!(config.port, 8080);
        assert_eq!(config.public_base_url, "http://localhost:8080");
        assert_eq!(config.database_path, "./urls.sqlite3");
        clear_env();
    }

    #[test]
    fn environment_variables_override_defaults() {
        let _guard = ENV_LOCK.lock().unwrap();
        clear_env();
        env::set_var("PORT", "9090");
        env::set_var("PUBLIC_BASE_URL", "https://sho.rt/");
        env::set_var("DATABASE_PATH", "/tmp/custom/urls.sqlite3");
        let config = Config::from_env();
        assert_eq!(config.port, 9090);
        assert_eq!(config.public_base_url, "https://sho.rt/");
        assert_eq!(config.database_path, "/tmp/custom/urls.sqlite3");
        clear_env();
    }
}
