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
	eprintln!("[ERROR] >>> Entering respond_with_user_dto(email=<redacted>,token=<redacted>,username={},bio={},image={:?})", username, bio, image);
    //
    let dto = UserDTO {
        email,
        token,
        username,
        bio,
        image,
    };
	eprintln!("[WARN] <<< Exiting respond_with_user_dto(email=<redacted>,token=<redacted>,username={},bio={},image={:?})", username, bio, image);
    (StatusCode::OK, Json(json!({ "user": dto })))
}
