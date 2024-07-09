pub mod mouse;
use std::env::current_exe;
use salvo::serve_static::StaticDir;
use salvo::websocket::WebSocketUpgrade;
use salvo::{prelude::*, websocket::Message};
use serde::{Deserialize, Serialize};
use serde_json;
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone)]
#[repr(i32)]
enum PosType {
    None = 0,
    Move = 1,
    Scroll = 2,
    LeftDown = 3,
    LeftUp = 4,
    RightDown = 5,
    RightUp = 6,
    Click = 7,
    RightClick = 8,
    Drag = 9,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct User {
    id: usize,
    name: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct Pos {
    pos_type: PosType,
    s: i32,
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
                // print!("msg string: {:#?} ", msg.to_str().unwrap());
                let pos: serde_json::Result<Pos> = serde_json::from_str(msg.to_str().unwrap());
                if pos.is_ok() {
                    let pos = pos.unwrap();
                    // println!("{:#?} ", pos);
                    match pos.pos_type {
                        PosType::Move => {
                            mouse::move_mouse(pos.x, pos.y);
                        }
                        PosType::Click => {
                            mouse::click_mouse(mouse::MouseButton::Left);
                        }
                        PosType::RightClick => {
                            mouse::click_mouse(mouse::MouseButton::Right);
                        }
                        PosType::Scroll => {
                            mouse::scroll(pos.s);
                        }
                        PosType::LeftDown => {
                            mouse::mouse_down(mouse::MouseButton::Left);
                        }
                        PosType::LeftUp => {
                            mouse::mouse_up(mouse::MouseButton::Left);
                        }
                        PosType::RightDown => {
                            mouse::mouse_down(mouse::MouseButton::Right);
                        }
                        PosType::RightUp => {
                            mouse::mouse_up(mouse::MouseButton::Right);
                        }
                        PosType::Drag => {
                            mouse::mouse_drag(pos.x , pos.y)
                        }
                        _ => {}
                    }
                } else {
                    println!("{:#?} ", pos);
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
    let static_path = current_exe().unwrap().parent().unwrap().join("static");

    println!("static_path  {:#?} ", static_path.to_str().unwrap());
    let static_router = Router::with_path("<**path>").get(
        StaticDir::new([static_path.to_str().unwrap()])
            .defaults("index.html")
            .auto_list(true),
    );
    let ws_router = Router::with_path("ws").goal(connect);
    let router = Router::new()
        .push(api_router)
        .push(ws_router)
        .push(static_router);

    let acceptor = TcpListener::new("0.0.0.0:5800").bind().await;
    Server::new(acceptor).serve(router).await;
}
