pub const TOKEN: &str = "fs_rs_rwa_token";

#[derive(Debug)]
pub struct AppState {
    pub token: Option<String>,
}

impl AppState {
    pub fn new() -> Self {
		eprintln!("[ERROR] >>> Entering new()");
		eprintln!("[WARN] <<< Exiting new()");
        Self { token: None }
    }
}
