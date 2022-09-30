mod volume;

use askama::Template;
use autopilot::key::Code;
use autopilot::key::KeyCode::{self, LeftArrow, RightArrow, Space};
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use warp::ws::{Message, WebSocket};
use warp::Filter;

#[derive(Serialize, Deserialize)]
/// The data that a WebSocket message should have to be understood by the `handle_message` function
struct WsMessage {
    action: String,
    data: Option<f32>,
}

#[derive(Template)]
#[template(path = "remote.html")]
/// The data that should be passed when rendering the remote template
struct RemoteTemplate {
    volume: u32,
}

#[tokio::main]
async fn main() {
    // GET / -> remote.html
    let index = warp::path::end().map(|| RemoteTemplate {
        volume: (volume::get().unwrap() * 100.) as u32,
    });

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

/// The WebSocket loop. Runs the `handle_message` function each time something is sent from the client.
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

/// Function to handle the WebSocket messages and act on the messages.
fn handle_message(msg: Message) {
    let msg = if let Ok(s) = msg.to_str() {
        s
    } else {
        return;
    };

    let p: WsMessage = serde_json::from_str(msg).unwrap();

    match p.action.as_str() {
        "pause" => tap(Space),
        "right" => tap(RightArrow),
        "left" => tap(LeftArrow),
        "mute" => {
            volume::mute().unwrap();
            return;
        }
        "unmute" => {
            volume::unmute().unwrap();
            return;
        }
        "vol" => {
            let vol = p.data.unwrap() / 100.;
            volume::set(vol).unwrap();
            return;
        }
        _ => return,
    };
}
