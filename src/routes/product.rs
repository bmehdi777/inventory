use axum::{response::Json, http::StatusCode, extract::State};
use serde_json::{Value, json};
use mongodb::Database;

pub async fn get_products(State(db_client): State<Database>) -> Result<Json<Value>, StatusCode> {
    let list = db_client.list_collection_names(None).await;
    match list {
        Ok(l) => {
            for collections in l {
                log::info!("Collections available : {}", collections);
            }
            return Ok(Json(json!({"title": "test"})));
        },
        Err(e) => {
            log::error!("{}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }

    }
}
