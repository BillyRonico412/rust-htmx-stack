mod todo;

use std::{
    sync::{Arc, Mutex},
    vec,
};

use axum::{Json, Router, http::StatusCode, response::IntoResponse, routing::get};
use serde_json::json;

use crate::todo::Todo;

const URL: &str = "127.0.0.1:8080";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app_state = AppState {
        current_id: Arc::new(Mutex::new(2)),
        todos: Arc::new(Mutex::new(vec![
            Todo {
                id: 1,
                completed: true,
                title: String::from("Apprendre typescript"),
            },
            Todo {
                id: 2,
                completed: false,
                title: String::from("Apprendre Axum"),
            },
        ])),
    };
    let app = Router::new()
        .route("/", get(|| async { "Bienvenue sur l'API !" }))
        .route("/health", get(|| async { (StatusCode::OK, "OK") }))
        .route("/version", get(|| async { "v1.0.0" }))
        .nest("/todos", todo::router())
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind(URL).await?;
    println!("Serveur démarré sur le serveur {}", URL);

    axum::serve(listener, app).await?;

    Ok(())
}

#[derive(Default, Clone)]
pub struct AppState {
    current_id: Arc<Mutex<u64>>,
    todos: Arc<Mutex<Vec<Todo>>>,
}

impl AppState {
    pub fn get_id(&self) -> u64 {
        let mut current_id = self.current_id.lock().unwrap();
        *current_id += 1;
        *current_id
    }
}

pub enum AppError {
    NotFound(String),
    InternalError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status_code, msg) = match self {
            AppError::InternalError(s) => (StatusCode::INTERNAL_SERVER_ERROR, s),
            Self::NotFound(s) => (StatusCode::NOT_FOUND, s),
        };
        let body = Json(json!({
            "error": msg
        }));
        (status_code, body).into_response()
    }
}
