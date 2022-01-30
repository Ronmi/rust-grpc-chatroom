use crate::proto::{chat_server::Chat, *};
use futures::Stream;
use sqlx::PgPool;
use std::pin::Pin;
use tonic::{Request, Response, Status};

mod join;

/// grpc server 的程式
pub struct Server {
    db: PgPool,
}

impl Server {
    /// 建立 Server instance
    pub fn new(db: PgPool) -> Self {
        Server { db }
    }
}

type NewMsgStream = Pin<Box<dyn Stream<Item = Result<Message, Status>> + Send>>;

#[tonic::async_trait]
impl Chat for Server {
    async fn join(&self, _request: Request<JoinReq>) -> Result<Response<JoinRes>, Status> {
        join::run(&self.db).await
    }
    async fn say(&self, request: Request<SayReq>) -> Result<Response<SayRes>, Status> {
        Err(Status::unimplemented(""))
    }

    type NewMsgStream = NewMsgStream;
    async fn new_msg(
        &self,
        request: Request<NewMsgReq>,
    ) -> Result<Response<Self::NewMsgStream>, Status> {
        Err(Status::unimplemented(""))
    }
}
