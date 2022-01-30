use clap::Parser;
use grpc_chatroom::{proto::chat_server::ChatServer, Server};
use sqlx::Pool;
use std::net::SocketAddr;

#[derive(Parser, Debug)]
#[clap()]
struct Args {
    #[clap(short, long)]
    bind: String,
    #[clap(short, long)]
    database: String,
}

#[tokio::main]
async fn main() {
    let app = Args::parse();

    let bind: SocketAddr = app.bind.as_str().parse().unwrap();

    let db = Pool::connect_lazy(&app.database).unwrap();

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
