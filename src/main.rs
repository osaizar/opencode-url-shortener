use std::path::Path;
use std::str::FromStr;
use std::sync::Arc;

use axum::routing::{get, post};
use axum::Router;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};

use url_shortener::code::Base62Generator;
use url_shortener::config::Config;
use url_shortener::handlers::{self, AppState};
use url_shortener::store::Store;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::from_env();

    if let Some(parent) = Path::new(&config.database_path).parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent)?;
        }
    }

    let connection = format!("sqlite://{}", config.database_path);
    let options = SqliteConnectOptions::from_str(&connection)?.create_if_missing(true);
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    let state = AppState {
        store: Store::new(pool),
        public_base_url: config.public_base_url,
        code_generator: Arc::new(Base62Generator),
    };

    let app = Router::new()
        .route("/api/v1/urls", post(handlers::create_url))
        .route("/:code", get(handlers::redirect))
        .route("/health", get(handlers::health))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(("0.0.0.0", config.port)).await?;
    println!("listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    Ok(())
}
