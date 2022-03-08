use autopilot::key::Code;
use autopilot::key::KeyCode::{self, DownArrow, LeftArrow, RightArrow, Space, UpArrow};

use warp::ws::{Message, WebSocket};
use warp::Filter;

use futures_util::StreamExt;

#[tokio::main]
async fn main() {
    // GET / -> index.html
    let index = warp::path::end().and(warp::fs::file("./public/index.html"));

    // GET /ws -> Initiate websocket connection
    let realtime = warp::path("ws")
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| ws.on_upgrade(move |socket| ws_connected(socket)));

    let routes = index.or(realtime);

    warp::serve(routes).run(([0, 0, 0, 0], 8080)).await;
}

fn tap(key: KeyCode) -> String {
    autopilot::key::tap(&Code(key), &[], 0, 0);
    String::from(format!("Tapped {:?}", key))
}

async fn ws_connected(ws: WebSocket) {
    let (_, mut rx) = ws.split();
    while let Some(result) = rx.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                eprintln!("websocket error: {}", e);
                break;
            }
        };

        handle_message(msg);
    }
}

fn handle_message(msg: Message) {
    let msg = if let Ok(s) = msg.to_str() {
        s
    } else {
        return;
    };

    match msg {
        "pause" => tap(Space),
        "right" => tap(RightArrow),
        "left" => tap(LeftArrow),
        "up" => tap(UpArrow),
        "down" => tap(DownArrow),
        _ => return,
    };
}
