use anyhow::Result;
use sqlx::{postgres::PgPoolOptions, PgPool};

pub async fn connect_db() -> Result<PgPool> {
    let url = if std::path::Path::exists(std::path::Path::new(".env")) {
        let content = std::fs::read(std::path::Path::new(".env"))
            .unwrap_or_else(|_| panic!("could not read file"));
        let url: String = String::from_utf8(content)
            .unwrap_or_else(|_| panic!("could not read file"))
            .split("=")
            .map_windows(|t: &[&str; 2]| (t[0], t[1]))
            .step_by(2)
            .filter(|(key, _)| key == &"DB_ENGINE")
            .map(|(_, val)| val)
            .take(1)
            .collect();
        url.trim_end().into()
    } else {
        std::env::var("DB_URI").unwrap_or_else(|_| panic!("Incorrect DB URI"))
    };
    tracing::debug!(url, "conneting to db!");

    let db = PgPoolOptions::new().connect(&url).await?;
    tracing::debug!("connected!");
    Ok(db)
}
