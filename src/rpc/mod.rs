use crate::proto::{chat_server::Chat, *};
use futures::Stream;
use std::pin::Pin;
use tonic::{Request, Response, Status};

/// grpc server 的程式
pub struct Server {}

type NewMsgStream = Pin<Box<dyn Stream<Item = Result<Message, Status>> + Send>>;

#[tonic::async_trait]
impl Chat for Server {
    async fn join(&self, request: Request<JoinReq>) -> Result<Response<JoinRes>, Status> {
        Err(Status::unimplemented(""))
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
