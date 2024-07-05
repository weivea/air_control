
pub mod mouse;
use salvo::serve_static::StaticDir;
use salvo::websocket::WebSocketUpgrade;
use salvo::{prelude::*, websocket::Message};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
struct User {
    id: usize,
    name: String,
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

                match msg.to_str() {
                    Ok(msg) => {
                        if msg == "left" {
                            // move 10px to the left

                            crate::mouse::move_mouse(-10, 0);
                        } else if msg == "right" {
                            // move 10px to the right
                            crate::mouse::move_mouse(10, 0);
                        } else if msg == "up" {
                            // move 10px to the up
                            crate::mouse::move_mouse(0, -10);
                        } else if msg == "down" {
                            // move 10px to the down
                            crate::mouse::move_mouse(0, 10);
                        } else if msg == "click" {
                            continue;
                        } else if msg == "right_click" {
                            continue;
                        }
                    }
                    Err(_) => {}
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
