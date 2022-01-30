use crate::proto::{chat_server::Chat, *};
use futures::Stream;
use sqlx::PgPool;
use std::pin::Pin;
use tonic::{Request, Response, Status};

mod join;
mod new_msg;
mod say;

/// grpc server 的程式
pub struct Server {
    db: PgPool,
    chan: tokio::sync::broadcast::Sender<Message>,
}

impl Server {
    /// 建立 Server instance
    pub fn new(db: PgPool) -> Self {
        let (chan, _) = tokio::sync::broadcast::channel(10);
        Server { db, chan }
    }
}

pub type NewMsgStream = Pin<Box<dyn Stream<Item = Result<Message, Status>> + Send>>;

#[tonic::async_trait]
impl Chat for Server {
    async fn join(&self, _request: Request<JoinReq>) -> Result<Response<JoinRes>, Status> {
        join::run(&self.db).await
    }
    async fn say(&self, request: Request<SayReq>) -> Result<Response<SayRes>, Status> {
        say::run(&self.db, &self.chan, request).await
    }

    type NewMsgStream = NewMsgStream;
    async fn new_msg(
        &self,
        _request: Request<NewMsgReq>,
    ) -> Result<Response<Self::NewMsgStream>, Status> {
        new_msg::run(&self.chan).await
    }
}
