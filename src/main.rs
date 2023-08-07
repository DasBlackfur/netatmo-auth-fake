mod config;
use config::{get_config, AuthConfig};
use serde::Deserialize;
use tokio::sync::RwLock;

use std::{collections::HashMap, net::SocketAddr, sync::Arc};

use axum::{extract::{State, Path, Query}, response::Html, routing::get, Router};

struct AppState {
    pub auth_scopes: Vec<AuthConfig>,
    pub auth_codes: HashMap<String, String>
}

#[derive(Debug, Deserialize)]
struct AuthParameters {
    state: String,
    code: String
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let app_state = Arc::new(RwLock::new(AppState {
        auth_scopes: get_config().await,
        auth_codes: HashMap::new()
    }));


    tracing::info!(
        "Found ({}) auth scopes: {:?}",
        app_state.read().await.auth_scopes.len(),
        app_state.read().await
            .auth_scopes
            .iter()
            .map(|value| &value.name)
            .collect::<Vec<_>>()
    );

    let app = Router::new()
        .route("/", get(root))
        .route("/auth", get(auth))
        .route("/get/:scope", get(get_code))
        .with_state(app_state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8081));
    tracing::info!("Starting server on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root(State(app_state): State<Arc<RwLock<AppState>>>) -> Html<String> {
    Html(
        app_state.read().await
            .auth_scopes
            .iter()
            .map(|value| {
                format!(
                    "<a href=\"https://api.netatmo.com/oauth2/authorize?client_id={}&redirect_uri={}&scope=read_station&state={}\">Generate code for {}</a></br>",
                    value.client_id, value.refer, value.name, value.name
                )
            })
            .collect::<String>(),
    )
}

async fn auth(params: Query<AuthParameters>, State(app_state): State<Arc<RwLock<AppState>>>) -> Html<&'static str> {
    app_state.write().await.auth_codes.insert(params.state.clone(), params.code.clone());
    Html("OK!")
}

async fn get_code(Path(scope): Path<String>, State(app_state): State<Arc<RwLock<AppState>>>) -> String {
    app_state.read().await.auth_codes.get(&scope).unwrap_or(&"NONE".to_string()).to_string()
}

