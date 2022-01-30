use crate::proto::{chat_server::Chat, *};
use futures::Stream;
use sqlx::PgPool;
use std::pin::Pin;
use tonic::{Request, Response, Status};
use tracing::{debug, instrument};

mod join;
mod new_msg;
mod say;

/// 取得 remote ip
fn ip<T>(req: &Request<T>) -> String {
    let m = req.metadata();

    if let Some(x) = m.get("cf-connecting-ip") {
        if let Ok(x) = x.to_str() {
            return x.to_owned();
        }
    }
    if let Some(x) = m.get("x-real-ip") {
        if let Ok(x) = x.to_str() {
            return x.to_owned();
        }
    }

    match req.remote_addr() {
        None => "".to_owned(),
        Some(x) => x.to_string(),
    }
}

/// grpc server 的程式
pub struct Server {
    db: PgPool,
    chan: tokio::sync::broadcast::Sender<Message>,
}

impl Server {
    /// 建立 Server instance
    #[instrument(skip(db), fields(db_closed = %db.is_closed()))]
    pub fn new(db: PgPool) -> Self {
        let (chan, _) = tokio::sync::broadcast::channel(10);
        tracing::info!("已建立 server instance");
        Server { db, chan }
    }
}

/// 利用 Drop 的機制，在函式執行完畢的時候寫 log
struct DeferLog(&'static str);

impl Drop for DeferLog {
    fn drop(&mut self) {
        debug!("{}", self.0);
    }
}

pub type NewMsgStream = Pin<Box<dyn Stream<Item = Result<Message, Status>> + Send>>;

#[tonic::async_trait]
impl Chat for Server {
    #[instrument(skip(self), fields(ip = %ip(&_request)))]
    async fn join(&self, _request: Request<JoinReq>) -> Result<Response<JoinRes>, Status> {
        let _l = DeferLog("已完成 join rpc");
        join::run(&self.db).await
    }
    #[instrument(skip(self), fields(ip = %ip(&request)))]
    async fn say(&self, request: Request<SayReq>) -> Result<Response<SayRes>, Status> {
        let _l = DeferLog("已完成 say rpc");
        say::run(&self.db, &self.chan, request).await
    }

    type NewMsgStream = NewMsgStream;
    #[instrument(skip(self), fields(ip = %ip(&_request)))]
    async fn new_msg(
        &self,
        _request: Request<NewMsgReq>,
    ) -> Result<Response<Self::NewMsgStream>, Status> {
        let _l = DeferLog("已完成 new_msg rpc");
        new_msg::run(&self.chan).await
    }
}
