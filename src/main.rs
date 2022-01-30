use chrono::offset::Local;
use clap::Parser;
use grpc_chatroom::{proto::chat_server::ChatServer, Server};
use sqlx::Pool;
use std::net::SocketAddr;
use time::macros::format_description;
use time::UtcOffset;
use tracing::Level;
use tracing_subscriber::fmt::time::OffsetTime;

#[derive(Parser, Debug)]
#[clap()]
struct Args {
    #[clap(short, long)]
    bind: String,
    #[clap(short, long)]
    database: String,
    /// log level
    #[clap(short, long, default_value = "info")]
    level: Level,
}

#[tokio::main]
async fn main() {
    let app = Args::parse();

    // init log
    let offset = Local::now().offset().local_minus_utc();
    let offset = UtcOffset::from_whole_seconds(offset).unwrap();
    tracing_subscriber::fmt()
        .with_ansi(true)
        .with_level(true)
        .with_line_number(true)
        .with_file(true)
        .with_max_level(app.level)
        .with_timer(OffsetTime::new(
            offset,
            format_description!(
                "[year]-[month]-[day] [hour]:[minute]:[second] [offset_hour sign:mandatory]:00"
            ),
        ))
        .pretty()
        .init();

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

    let signal = async_ctrlc::CtrlC::new().unwrap();

    tracing::info!(?bind, %offset, "啟動 rpc server");
    tonic::transport::server::Server::builder()
        .accept_http1(true)
        .add_service(web.enable(svc))
        .serve_with_shutdown(bind, signal)
        .await
        .unwrap();
}
