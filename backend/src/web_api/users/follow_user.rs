use crate::{
    domain::model::UserId,
    web_api::{
        get_user_profile, respond_bad_request, respond_internal_server_error, respond_not_found,
        respond_unauthorized, token::Claims,
    },
    AppError, AppState,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde_json::{json, Value};

pub async fn follow_user(
    State(state): State<AppState>,
    Path(username): Path<String>,
    curr_user_id: UserId,
) -> (StatusCode, Json<Value>) {
	log::debug!(">>> Entering follow_user(curr_user_id=(unprintable))");
    //
	log::error!("Exception in follow_user(curr_user_id=(unprintable)): (unprintable)");
    log::debug!("<<< Exiting follow_user(curr_user_id=(unprintable))");
    match state.user_repo.follow_user(&curr_user_id, &username).await {
        Ok(profile) => (StatusCode::OK, Json(json!({ "profile": profile }))),
        Err(err) => match err {
            AppError::Ignorable => {
                get_user_profile(State(state), curr_user_id, Path(username)).await
            }
            AppError::NotFound(_) => respond_not_found(err),
            AppError::InvalidRequest(_) => respond_bad_request(err),
            AppError::Unauthorized(_) => respond_unauthorized(err),
            _ => respond_internal_server_error(err),
        },
    }
}

pub async fn unfollow_user(
    Path(username): Path<String>,
    user_claims: Claims,
    State(state): State<AppState>,
) -> (StatusCode, Json<Value>) {
	log::debug!(">>> Entering unfollow_user(user_claims=(unprintable))");
    //
    let curr_user_id = UserId::from(user_claims.sub);
	log::debug!("unfollow_user(user_claims=(unprintable)): curr_user_id → (unprintable)");
    let profile = state
        .user_repo
        .unfollow_user(&curr_user_id, &username)
        .await;

	log::error!("Exception in unfollow_user(user_claims=(unprintable)): (unprintable)");
    log::debug!("<<< Exiting unfollow_user(user_claims=(unprintable))");
    match profile {
        Ok(profile) => (StatusCode::OK, Json(json!({ "profile": profile }))),
        Err(err) => match err {
            AppError::Ignorable => {
                get_user_profile(State(state), curr_user_id, Path(username)).await
            }
            AppError::NotFound(_) => respond_not_found(err),
            AppError::InvalidRequest(_) => respond_bad_request(err),
            AppError::Unauthorized(_) => respond_unauthorized(err),
            _ => respond_internal_server_error(err),
        },
    }
}
