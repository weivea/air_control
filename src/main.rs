pub mod mouse;
use salvo::serve_static::StaticDir;
use salvo::websocket::WebSocketUpgrade;
use salvo::{prelude::*, websocket::Message};
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Clone, Debug, Deserialize, Serialize)]
struct User {
    id: usize,
    name: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct Pos {
    x: i32,
    y: i32,
}

#[handler]
async fn hello() -> &'static str {
    "Hello World"
}

#[handler]
async fn connect(req: &mut Request, res: &mut Response) -> Result<(), StatusError> {
    let user = req.parse_queries::<User>();
    WebSocketUpgrade::new()
        .upgrade(req, res, |mut ws| async move {
            println!("{:#?} ", user);
            while let Some(msg) = ws.recv().await {
                let msg = if let Ok(msg) = msg {
                    msg
                } else {
                    // client disconnected
                    return;
                };

                let pos: serde_json::Result<Pos> = serde_json::from_str(msg.to_str().unwrap());
                if pos.is_ok() {
                    let pos = pos.unwrap();
                    println!("{:#?} ", pos);
                    mouse::move_mouse(pos.x, pos.y);
                }

                // send "Hello, {msg}!" back
                // let msg_re: String = format!("Hello, {}!", msg.to_str().unwrap().to_string());

                // ws.send(Message::text(msg_re)).await.unwrap();

                // if ws.send(msg_re).await.is_err() {
                //     // client disconnected
                //     return;
                // }
            }
        })
        .await
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();
    let api_router = Router::with_path("api").get(hello);

    let static_router = Router::with_path("<**path>").get(
        StaticDir::new(["static"])
            .defaults("index.html")
            .auto_list(true),
    );
    let ws_router = Router::with_path("ws").goal(connect);
    let router = Router::new()
        .push(api_router)
        .push(ws_router)
        .push(static_router);

    let acceptor = TcpListener::new("127.0.0.1:5800").bind().await;
    Server::new(acceptor).serve(router).await;
}
