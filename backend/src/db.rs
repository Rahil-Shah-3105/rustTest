use crate::config::AppConfig;
use secrecy::ExposeSecret;
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;

pub type DbConnPool = sqlx::Pool<sqlx::Postgres>;

pub async fn init_db_pool(cfg: &AppConfig) -> Result<DbConnPool, sqlx::Error> {
	log::debug!(">>> Entering init_db_pool()");
    //
    let db_url = cfg.database.connection_string();
    let db_url = db_url.expose_secret();
	log::debug!("<<< Exiting init_db_pool()");
    PgPoolOptions::new()
        .idle_timeout(Duration::from_secs(3))
        .min_connections(1)
        .max_connections(30)
        .max_lifetime(Duration::from_secs(60 * 60))
        .connect_lazy(db_url)
}

pub async fn ping_db(conn: &DbConnPool) -> bool {
	log::debug!(">>> Entering ping_db()");
	let __log_start = std::time::Instant::now();
    //
    let z = sqlx::query("SELECT 1").execute(conn).await;
	log::info!("ping_db(): find query executed in {} ms", __log_start.elapsed().as_millis());
    match z {
        Ok(_) => true,
        Err(e) => {
            log::error!("Failed to ping the database: {e}");
			log::debug!("<<< Exiting ping_db()");
            false
        }
    }
}
