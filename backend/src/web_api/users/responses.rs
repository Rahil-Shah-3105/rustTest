use axum::{http::StatusCode, Json};
use common_model::UserDTO;
use serde_json::{json, Value};

/// Utility function for responding with `User` payload in multiple use cases.
pub fn respond_with_user_dto(
    email: String,
    token: Option<String>,
    username: String,
    bio: String,
    image: Option<String>,
) -> (StatusCode, Json<Value>) {
	log::debug!(">>> Entering respond_with_user_dto()");
    //
    let dto = UserDTO {
        email,
        token,
        username,
        bio,
        image,
    };
	log::debug!("<<< Exiting respond_with_user_dto()");
    (StatusCode::OK, Json(json!({ "user": dto })))
}
