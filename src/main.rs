use grpc_chatroom::{proto::chat_server::ChatServer, Server};
use sqlx::Pool;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let bind: SocketAddr = "0.0.0.0:50051".parse().unwrap();

    let connstr = "postgresql://postgres:qwer1234@localhost:5432/chatroom";
    let db = Pool::connect_lazy(connstr).unwrap();

    let srv = Server::new(db);
    let svc = ChatServer::new(srv);

    let web = tonic_web::config().allow_all_origins().expose_headers(vec![
        "x-forworded-for",
        "x-real-ip",
        "cf-connecting-ip",
        "cf-ipcountry",
    ]);
    tonic::transport::server::Server::builder()
        .accept_http1(true)
        .add_service(web.enable(svc))
        .serve(bind)
        .await
        .unwrap();
}
