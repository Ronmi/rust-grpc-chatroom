use sqlx::{
    types::chrono::{DateTime, Utc},
    PgPool,
};

/// DB 裡的訊息的格式
pub struct Message {
    name: String,
    message: String,
    create_at: DateTime<Utc>,
}

/// 載入最近 50 筆訊息
pub async fn load_recent_msg(conn: &PgPool) -> Result<Vec<Message>, sqlx::Error> {
    let res = sqlx::query_as!(
        Message,
        r#"
SELECT
  name, message, create_at
FROM messages
ORDER BY create_at DESC
LIMIT 50
"#,
    )
    .fetch_all(conn)
    .await?;

    Ok(res)
}

/// 寫入一筆訊息
pub async fn save_msg(conn: &PgPool, user: &str, msg: &str) -> sqlx::Result<()> {
    sqlx::query!(
        r#"
INSERT INTO messages (name, message, create_at) VALUES ($1,$2,CURRENT_TIMESTAMP)
"#,
        user,
        msg,
    )
    .execute(conn)
    .await?;

    Ok(())
}
