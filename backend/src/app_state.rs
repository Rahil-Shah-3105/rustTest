use crate::{
    db::DbConnPool,
    domain::logic::{ArticlesMgr, AuthMgr},
    repos::{ArticlesRepo, UsersRepo},
};
use axum::extract::FromRef;
use std::sync::Arc;

/// The (global) state of the app.
#[derive(Clone, FromRef)]
pub struct AppState {
    pub dbcp: Arc<DbConnPool>,
    pub user_repo: Arc<UsersRepo>,
    pub auth_mgr: AuthMgr,
    pub articles_mgr: ArticlesMgr,
}

impl AppState {
    //
    pub fn new(dbcp: DbConnPool) -> Self {
		log::debug!(">>> Entering new(dbcp=(unprintable))");
        let dbcp = Arc::new(dbcp);
        let user_repo = Arc::new(UsersRepo::new(dbcp.clone()));
		log::debug!("new(dbcp=(unprintable)): user_repo → (unprintable)");
		log::debug!("new(dbcp=(unprintable)): auth_mgr → (unprintable)");
		log::debug!("new(dbcp=(unprintable)): articles_repo → (unprintable)");
        let auth_mgr = AuthMgr::new(user_repo.clone());
        let articles_repo = ArticlesRepo::new(dbcp.clone());
        let articles_mgr = ArticlesMgr::new(articles_repo, user_repo.clone());
		log::debug!("new(dbcp=(unprintable)): articles_mgr → (unprintable)");
        log::debug!("new(dbcp=(unprintable)): dbcp → (unprintable)");
        log::debug!("<<< Exiting new(dbcp=(unprintable))");
        Self {
            dbcp,
            user_repo,
            auth_mgr,
            articles_mgr,
        }
    }
}
