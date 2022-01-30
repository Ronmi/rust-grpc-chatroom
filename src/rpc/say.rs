use crate::prelude::*;
use crate::proto::*;
use sqlx::types::chrono::Utc;
use sqlx::PgPool;
use tokio::sync::broadcast::Sender;
use tonic::{Request, Response, Status};

/// 實作 say rpc
pub async fn run(
    db: &PgPool,
    chan: &Sender<Message>,
    req: Request<SayReq>,
) -> Result<Response<SayRes>, Status> {
    let args = req.into_inner();
    crate::save_msg(db, &args.name, &args.msg).await.to500()?;

    // 只有聊天室無人的時候才會噴錯，但那也沒差
    let _ = chan.send(Message {
        name: args.name,
        msg: args.msg,
        create_at: Utc::now().timestamp(),
    });

    Ok(Response::new(SayRes {}))
}
