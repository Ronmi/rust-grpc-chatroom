use crate::load_recent_msg;
use crate::prelude::*;
use crate::proto::*;
use sqlx::PgPool;
use tonic::{Response, Status};

/// 實作 join rpc
pub async fn run(db: &PgPool) -> Result<Response<JoinRes>, Status> {
    let list = load_recent_msg(db).await.skip_no_data().to500()?;

    Ok(Response::new(JoinRes {
        msgs: list.into_iter().map(Message::from).collect(),
    }))
}
