use std::time::Duration;

use super::queries::sql_delete_expired_sessions;
use crate::state::AppState;

pub async fn clean_expired_sessions(state: AppState) -> anyhow::Result<()> {
    let mut interval = tokio::time::interval(Duration::from_secs(60 * 60 * 24));
    interval.tick().await;
    loop {
        interval.tick().await;
        match sql_delete_expired_sessions(&state.db).await {
          Ok(count) => println!("{count} sessions were deleted"),
          Err(e) => eprintln!("Session cleanup failed (session table probably empty): {e:#}"),
        };
    }
}
