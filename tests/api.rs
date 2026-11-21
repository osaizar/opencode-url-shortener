use std::str::FromStr;
use std::sync::Arc;

use axum::body::{to_bytes, Body};
use axum::http::{header, Request, StatusCode};
use axum::routing::{get, post};
use axum::Router;
use serde_json::{json, Value};
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool, SqlitePoolOptions};
use tower::ServiceExt;

use url_shortener::code::{CodeGenerator, SequenceGenerator};
use url_shortener::handlers::{self, AppState};
use url_shortener::store::Store;

struct TestContext {
    app: Router,
    pool: SqlitePool,
    store: Store,
    _dir: tempfile::TempDir,
}

async fn setup_with_base(generator: Arc<dyn CodeGenerator>, base_url: &str) -> TestContext {
    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("urls.sqlite3");
    let connection = format!("sqlite://{}", db_path.display());
    let options = SqliteConnectOptions::from_str(&connection)
        .unwrap()
        .create_if_missing(true);
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect_with(options)
        .await
        .unwrap();
    sqlx::migrate!("./migrations").run(&pool).await.unwrap();

    let store = Store::new(pool.clone());
    let state = AppState {
        store: store.clone(),
        public_base_url: base_url.to_string(),
        code_generator: generator,
    };
    let app = Router::new()
        .route("/api/v1/urls", post(handlers::create_url))
        .route("/:code", get(handlers::redirect))
        .route("/health", get(handlers::health))
        .with_state(state);

    TestContext {
        app,
        pool,
        store,
        _dir: dir,
    }
}

async fn setup(generator: Arc<dyn CodeGenerator>) -> TestContext {
    setup_with_base(generator, "http://localhost:8080").await
}

async fn request(
    app: &Router,
    method: &str,
    uri: &str,
    body: Option<&str>,
) -> (StatusCode, Value, Option<String>) {
    let mut builder = Request::builder().method(method).uri(uri);
    if body.is_some() {
        builder = builder.header("content-type", "application/json");
    }
    let response = app
        .clone()
        .oneshot(
            builder
                .body(Body::from(body.unwrap_or("").to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    let status = response.status();
    let location = response
        .headers()
        .get(header::LOCATION)
        .map(|value| value.to_str().unwrap().to_string());
    let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let value = serde_json::from_slice(&bytes).unwrap_or(Value::Null);
    (status, value, location)
}

async fn post_json(app: &Router, body: &str) -> (StatusCode, Value, Option<String>) {
    request(app, "POST", "/api/v1/urls", Some(body)).await
}

#[tokio::test]
async fn creates_a_short_url_and_redirects_to_the_exact_target() {
    let ctx = setup(Arc::new(SequenceGenerator::new(
        vec!["aB3xY7z".to_string()],
    )))
    .await;
    let target = "https://example.com/path?q=1#frag";

    let (status, body, _) = post_json(&ctx.app, &json!({ "url": target }).to_string()).await;
    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(body["code"], "aB3xY7z");
    assert_eq!(body["short_url"], "http://localhost:8080/aB3xY7z");
    assert_eq!(body.as_object().unwrap().len(), 2);

    let (status, _, location) = request(&ctx.app, "GET", "/aB3xY7z", None).await;
    assert_eq!(status, StatusCode::FOUND);
    assert_eq!(location.as_deref(), Some(target));
}

#[tokio::test]
async fn repeated_submission_gets_a_distinct_code() {
    let ctx = setup(Arc::new(SequenceGenerator::new(vec![
        "aaaaaaa".to_string(),
        "bbbbbbb".to_string(),
    ])))
    .await;
    let body = json!({ "url": "https://example.com/same" }).to_string();

    let (first_status, first, _) = post_json(&ctx.app, &body).await;
    let (second_status, second, _) = post_json(&ctx.app, &body).await;
    assert_eq!(first_status, StatusCode::CREATED);
    assert_eq!(second_status, StatusCode::CREATED);
    assert_ne!(first["code"], second["code"]);

    for code in [
        first["code"].as_str().unwrap(),
        second["code"].as_str().unwrap(),
    ] {
        let (_, _, location) = request(&ctx.app, "GET", &format!("/{code}"), None).await;
        assert_eq!(location.as_deref(), Some("https://example.com/same"));
    }
}

#[tokio::test]
async fn collision_retries_with_the_next_code() {
    let ctx = setup(Arc::new(SequenceGenerator::new(vec![
        "aaaaaaa".to_string(),
        "ccccccc".to_string(),
    ])))
    .await;
    assert!(ctx
        .store
        .insert("aaaaaaa", "https://example.com/existing")
        .await
        .unwrap());

    let (status, body, _) = post_json(
        &ctx.app,
        &json!({ "url": "https://example.com/new" }).to_string(),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(body["code"], "ccccccc");
    assert_eq!(
        ctx.store.find("aaaaaaa").await.unwrap().as_deref(),
        Some("https://example.com/existing")
    );
}

#[tokio::test]
async fn invalid_urls_are_rejected_with_the_exact_error() {
    for input in ["ftp://example.com", "example.com", "https://", ""] {
        let ctx = setup(Arc::new(SequenceGenerator::new(vec![]))).await;
        let (status, body, _) = post_json(&ctx.app, &json!({ "url": input }).to_string()).await;
        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert_eq!(
            body,
            json!({
                "error": "invalid_url",
                "message": "URL must be a valid http or https URL of at most 2048 characters"
            })
        );
    }
}

#[tokio::test]
async fn overlong_url_is_rejected() {
    let ctx = setup(Arc::new(SequenceGenerator::new(vec![]))).await;
    let base = "https://example.com/";
    let input = format!("{}{}", base, "a".repeat(2049 - base.len()));
    assert_eq!(input.len(), 2049);

    let (status, body, _) = post_json(&ctx.app, &json!({ "url": input }).to_string()).await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(body["error"], "invalid_url");
}

#[tokio::test]
async fn malformed_requests_are_rejected() {
    for raw in ["not-json", "{}", "{\"url\": 123}"] {
        let ctx = setup(Arc::new(SequenceGenerator::new(vec![]))).await;
        let (status, body, _) = post_json(&ctx.app, raw).await;
        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert_eq!(
            body,
            json!({
                "error": "invalid_request",
                "message": "Request body must be a JSON object with a url string"
            })
        );
    }
}

#[tokio::test]
async fn unknown_code_returns_not_found() {
    let ctx = setup(Arc::new(SequenceGenerator::new(vec![]))).await;
    let (status, body, _) = request(&ctx.app, "GET", "/doesnotexist", None).await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(
        body,
        json!({ "error": "not_found", "message": "Short code not found" })
    );
}

#[tokio::test]
async fn health_returns_ok() {
    let ctx = setup(Arc::new(SequenceGenerator::new(vec![]))).await;
    let (status, body, _) = request(&ctx.app, "GET", "/health", None).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body, json!({ "status": "ok" }));
}

#[tokio::test]
async fn internal_failure_returns_the_structured_error() {
    let ctx = setup(Arc::new(SequenceGenerator::new(vec![]))).await;
    sqlx::query("DROP TABLE urls")
        .execute(&ctx.pool)
        .await
        .unwrap();

    let (status, body, _) = post_json(
        &ctx.app,
        &json!({ "url": "https://example.com/fail" }).to_string(),
    )
    .await;
    assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
    assert_eq!(
        body,
        json!({ "error": "internal_error", "message": "Internal server error" })
    );
}

#[tokio::test]
async fn trailing_slash_in_public_base_url_is_trimmed() {
    let ctx = setup_with_base(
        Arc::new(SequenceGenerator::new(vec!["aB3xY7z".to_string()])),
        "https://sho.rt/",
    )
    .await;
    let (status, body, _) = post_json(
        &ctx.app,
        &json!({ "url": "https://example.com/config" }).to_string(),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(body["short_url"], "https://sho.rt/aB3xY7z");
}
