use crate::prelude::*;
use crate::proto::Message;
use futures::stream::StreamExt;
use tokio::sync::broadcast::Sender;
use tokio_stream::wrappers::BroadcastStream;
use tonic::{Response, Status};

/// 實作 new_msg rpc
pub async fn run(chan: &Sender<Message>) -> Result<Response<super::NewMsgStream>, Status> {
    Ok(Response::new(Box::pin(
        BroadcastStream::new(chan.subscribe()).map(|v| v.to500()),
    )))
}
