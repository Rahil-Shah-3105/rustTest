use axum::extract::State;
use axum::Json;
use axum::response::IntoResponse;
use serde_json::json;
use crate::AppState;
use crate::db::ping_db;

pub async fn health_check(State(state): State<AppState>) -> impl IntoResponse {
	eprintln!("[ERROR] >>> Entering health_check()");
    //
	eprintln!("[WARN] <<< Exiting health_check()");
    match ping_db(&state.dbcp).await {
        true => Json(json!({ "database": "ok" })),
        false => Json(json!({ "database": "err" })),
    }
}
