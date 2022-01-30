use crate::prelude::*;
use crate::proto::*;
use sqlx::PgPool;
use tonic::{Request, Response, Status};

/// 實作 say rpc
pub async fn run(db: &PgPool, req: Request<SayReq>) -> Result<Response<SayRes>, Status> {
    let args = req.get_ref();
    crate::save_msg(db, &args.name, &args.msg).await.to500()?;

    Ok(Response::new(SayRes {}))
}
