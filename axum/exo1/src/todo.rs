use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::{delete, get, post, put},
};
use serde::{Deserialize, Serialize};

use crate::{
    AppError::{self, NotFound},
    AppState,
};

#[derive(Deserialize)]
struct CreateTodo {
    title: String,
}

#[derive(Deserialize)]
struct UpdateTodo {
    title: String,
    completed: bool,
}

#[derive(Serialize, Clone)]
pub struct Todo {
    pub id: u64,
    pub title: String,
    pub completed: bool,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list))
        .route("/{id}", get(read_by_id))
        .route("/", post(create))
        .route("/{id}", put(update))
        .route("/{id}", delete(delete_by_id))
}

async fn list(State(state): State<AppState>) -> Json<Vec<Todo>> {
    let todos = state.todos.lock().unwrap();
    Json(todos.clone())
}

async fn read_by_id(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<Json<Todo>, AppError> {
    let todos = state.todos.lock().unwrap();
    let todo = todos.iter().find(|todo| todo.id == id);
    if let Some(todo) = todo {
        Ok(Json(todo.clone()))
    } else {
        Err(NotFound(format!("Todo {} est introuvable", id)))
    }
}

async fn create(
    State(state): State<AppState>,
    Json(CreateTodo { title }): Json<CreateTodo>,
) -> (StatusCode, Json<Todo>) {
    let mut todos = state.todos.lock().unwrap();
    let id = state.get_id();
    let new_todo = Todo {
        id,
        title,
        completed: false,
    };
    todos.push(new_todo.clone());
    (StatusCode::CREATED, Json(new_todo))
}

async fn update(
    State(state): State<AppState>,
    Path(id): Path<u64>,
    Json(updated_todo): Json<UpdateTodo>,
) -> Result<Json<Todo>, AppError> {
    let mut todos = state.todos.lock().unwrap();
    let todo = todos.iter_mut().find(|todo| todo.id == id);
    if let Some(todo) = todo {
        *todo = Todo {
            id,
            title: updated_todo.title,
            completed: updated_todo.completed,
        };
        Ok(Json(todo.clone()))
    } else {
        Err(NotFound(format!("Todo {} est introuvable", id)))
    }
}

async fn delete_by_id(State(state): State<AppState>, Path(id): Path<u64>) -> StatusCode {
    let mut todos = state.todos.lock().unwrap();
    let todo_index = todos.iter().position(|todo| todo.id == id);
    if let Some(todo_index) = todo_index {
        todos.remove(todo_index);
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}
