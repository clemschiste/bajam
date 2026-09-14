use crate::db::delete_session::sql_delete_expired_sessions;
use crate::state::AppState;
use std::time::Duration;

pub async fn clean_expired_sessions(state: AppState) -> anyhow::Result<()> {
    let mut interval = tokio::time::interval(Duration::from_secs(60 * 60 * 24));

    loop {
        interval.tick().await;
        sql_delete_expired_sessions(&state).await?;
    }
}
