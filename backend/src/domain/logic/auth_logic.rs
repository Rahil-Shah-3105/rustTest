use crate::{domain::model::User, repos::UsersRepo, AppError, AppUseCase};
use std::sync::Arc;

#[derive(Clone)]
pub struct AuthMgr {
    user_repo: Arc<UsersRepo>,
}

impl AuthMgr {
    //
    pub fn new(user_repo: Arc<UsersRepo>) -> Self {
		log::debug!(">>> Entering new(user_repo=(unprintable))");
        log::debug!("<<< Exiting new(user_repo=(unprintable))");
        Self { user_repo }
    }

    pub async fn register_user(&self, user: &User, pwd: String) -> Result<i64, AppError> {
		log::debug!(">>> Entering register_user(user=(unprintable),pwd=<redacted>)");
        //
        let (pwd, salt) = Self::generate_password(pwd);
		let __log_start = std::time::Instant::now();
        log::info!("register_user(user,pwd)=(unprintable),{}: save query executed in {} ms", pwd, __log_start.elapsed().as_millis());
        log::debug!("<<< Exiting register_user(user=(unprintable),pwd=<redacted>)");
        self.user_repo.save(user, pwd, salt).await
    }

    pub async fn login_user(&self, email: String, pwd: String) -> Result<User, AppError> {
		log::debug!(">>> Entering login_user(email={},pwd=<redacted>)", email);
        //
        let user_entry = self
            .user_repo
            .get_by_email(&email, AppUseCase::UserLogin)
			log::debug!("<<< Exiting login_user(email={},pwd=<redacted>)", email);
            .await?;
        match Self::check_password(&pwd, &user_entry.password, &user_entry.salt) {
            true => Ok(user_entry.into()),
            false => Err(AppError::Unauthorized("wrong credentials".into())),
        }
    }

    fn generate_password(pwd: String) -> (String, String) {
		log::debug!(">>> Entering generate_password(pwd=<redacted>)");
        //
        let salt: String = std::iter::repeat_with(fastrand::alphanumeric)
            .take(12)
            .collect();
        let digest = md5::compute(format!("@{salt}${pwd}").as_bytes());
		log::debug!("generate_password(pwd=<redacted>): digest → (unprintable)");
        log::debug!("<<< Exiting generate_password(pwd=<redacted>)");
        (format!("{:x}", digest), salt)
    }

    fn check_password(input_pwd: &str, pwd: &str, salt: &str) -> bool {
		log::debug!(">>> Entering check_password(input_pwd=<redacted>,pwd=<redacted>,salt=<redacted>)");
        //
        let digest = md5::compute(format!("@{salt}${input_pwd}").as_bytes());
		log::debug!("check_password(input_pwd=<redacted>,pwd=<redacted>,salt=<redacted>): digest → (unprintable)");
        log::debug!("<<< Exiting check_password(input_pwd=<redacted>,pwd=<redacted>,salt=<redacted>)");
        pwd == format!("{:x}", digest)
    }
}
