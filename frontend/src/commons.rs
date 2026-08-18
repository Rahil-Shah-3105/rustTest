pub const TOKEN: &str = "fs_rs_rwa_token";

#[derive(Debug)]
pub struct AppState {
    pub token: Option<String>,
}

impl AppState {
    pub fn new() -> Self {
		log::debug!(">>> Entering new()");
        log::debug!("<<< Exiting new()");
        Self { token: None }
    }
}
