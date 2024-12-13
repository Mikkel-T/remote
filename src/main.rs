mod keyboard;
mod mediainfo;
mod volume;

use askama::Template;
use futures_util::{SinkExt, StreamExt, TryFutureExt};
use keyboard::{tap, KeyCode};
use mediainfo::{get_media_info, get_session_manager, listen_media_info, MediaInfo};
use serde::{Deserialize, Serialize};
use serde_json::{from_str, Value};
use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};
use tokio::sync::{mpsc, RwLock};
use tokio_stream::wrappers::UnboundedReceiverStream;
use volume::listen_volume;
use warp::{
    ws::{Message, WebSocket},
    Filter,
};

pub type Users = Arc<RwLock<HashMap<usize, mpsc::UnboundedSender<Message>>>>;
static NEXT_USER_ID: AtomicUsize = AtomicUsize::new(1);

#[derive(Serialize, Deserialize)]
/// The data that a WebSocket message should have to be understood by the `parse_message` function
struct WsMessage {
    action: String,
    data: Option<f32>,
}

#[derive(Template)]
#[template(path = "remote.html", escape = "none")]
/// The data that should be passed when rendering the remote template
struct RemoteTemplate {
    volume: u32,
    music: bool,
    media: bool,
    mediainfo: Option<MediaInfoTemplate>,
}

#[derive(Template)]
#[template(path = "mediainfo.html")]
/// The data that should be passed when rendering the media info template
pub struct MediaInfoTemplate {
    mediainfo: Option<MediaInfo>,
}

#[derive(Clone, Copy)]
enum MessageAction {
    Pause,
    Right,
    Left,
    Mute,
    Unmute,
    Vol(f32),
    PlayPause,
    Next,
    Prev,
    Stop,
}

pub async fn send_message_to_all(users: Users, message: Message) {
    for (_uid, tx) in users.read().await.iter() {
        tx.send(message.clone()).unwrap_or_else(|e| {
            eprintln!("websocket send error: {}", e);
        });
    }
}

#[tokio::main]
async fn main() {
    let users_obj = Users::default();

    // We need to keep the listeners in scope, so we assign the variables
    let _media_session_manager = listen_media_info(users_obj.clone());
    let _vol_endpoint = listen_volume(users_obj.clone());

    let users = warp::any().map(move || users_obj.clone());

    // GET / -> remote.html
    let index = warp::path::end()
        .and(warp::query::<HashMap<String, String>>())
        .map(|p: HashMap<String, String>| RemoteTemplate {
            volume: (volume::get().unwrap() * 100.) as u32,
            music: p.contains_key("music"),
            media: p.contains_key("media"),
            mediainfo: if p.contains_key("media") {
                Some(MediaInfoTemplate {
                    mediainfo: get_media_info(&get_session_manager()),
                })
            } else {
                None
            },
        });

    // GET /ws -> Initiate websocket connection
    let realtime = warp::path("ws")
        .and(warp::path::end())
        .and(warp::ws())
        .and(users)
        .map(|ws: warp::ws::Ws, users| ws.on_upgrade(move |socket| ws_connected(socket, users)));

    let routes = warp::get().and(index.or(realtime));

    warp::serve(routes).run(([0, 0, 0, 0], 8080)).await;
}

/// The WebSocket loop. Runs the `handle_action` function each time something is sent from the client.
async fn ws_connected(ws: WebSocket, users: Users) {
    let my_id = NEXT_USER_ID.fetch_add(1, Ordering::Relaxed);

    let (mut user_ws_tx, mut user_ws_rx) = ws.split();

    let (tx, rx) = mpsc::unbounded_channel();
    let mut rx = UnboundedReceiverStream::new(rx);

    tokio::task::spawn(async move {
        while let Some(message) = rx.next().await {
            user_ws_tx
                .send(message)
                .unwrap_or_else(|e| {
                    eprintln!("websocket send error: {}", e);
                })
                .await;
        }
    });

    users.write().await.insert(my_id, tx);

    while let Some(result) = user_ws_rx.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                eprintln!("websocket error: {e}");
                break;
            }
        };

        let action = parse_message(msg);
        handle_action(action);
    }

    users.write().await.remove(&my_id);
}

/// Function to handle the WebSocket messages and act on the messages.
fn parse_message(msg: Message) -> Option<MessageAction> {
    let msg = if let Ok(s) = msg.to_str() {
        s
    } else {
        return None;
    };

    let p: Value = from_str(msg).unwrap();

    if p["pause"].is_string() {
        Some(MessageAction::Pause)
    } else if p["right"].is_string() {
        Some(MessageAction::Right)
    } else if p["left"].is_string() {
        Some(MessageAction::Left)
    } else if p["mute"].is_string() {
        Some(MessageAction::Mute)
    } else if p["unmute"].is_string() {
        Some(MessageAction::Unmute)
    } else if p["vol"].is_string() {
        Some(MessageAction::Vol(
            p["vol"].as_str().unwrap().parse().unwrap(),
        ))
    } else if p["playpause"].is_string() {
        Some(MessageAction::PlayPause)
    } else if p["next"].is_string() {
        Some(MessageAction::Next)
    } else if p["prev"].is_string() {
        Some(MessageAction::Prev)
    } else if p["stop"].is_string() {
        Some(MessageAction::Stop)
    } else {
        None
    }
}

fn handle_action(action: Option<MessageAction>) {
    if let Some(a) = action {
        match a {
            MessageAction::Pause => tap(KeyCode::Space),
            MessageAction::Right => tap(KeyCode::Right),
            MessageAction::Left => tap(KeyCode::Left),
            MessageAction::Mute => volume::mute().unwrap(),
            MessageAction::Unmute => volume::unmute().unwrap(),
            MessageAction::Vol(vol) => volume::set(vol / 100.).unwrap(),
            MessageAction::PlayPause => tap(KeyCode::PlayPause),
            MessageAction::Next => tap(KeyCode::Next),
            MessageAction::Prev => tap(KeyCode::Prev),
            MessageAction::Stop => tap(KeyCode::Stop),
        }
    }
}
