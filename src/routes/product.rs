use axum::{response::Json};
use serde_json::{Value, json};

pub async fn get_products() -> Json<Value> {
    Json(json!({"title": "test"}))
}
