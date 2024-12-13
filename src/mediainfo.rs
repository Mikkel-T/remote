use crate::{send_message_to_all, MediaInfoTemplate, Users};
use warp::filters::ws::Message;
use windows::{
    core::Result,
    Foundation::TypedEventHandler,
    Media::Control::{
        CurrentSessionChangedEventArgs, GlobalSystemMediaTransportControlsSessionManager,
    },
};

#[derive(Clone)]
pub struct MediaInfo {
    pub title: Option<String>,
    pub artist: Option<String>,
}

pub fn listen_media_info(users: Users) -> Result<GlobalSystemMediaTransportControlsSessionManager> {
    let session_manager = get_session_manager();

    session_manager.CurrentSessionChanged(&TypedEventHandler::<
        GlobalSystemMediaTransportControlsSessionManager,
        CurrentSessionChangedEventArgs,
    >::new(move |event, _| {
        if let Some(session_manager) = event {
            futures::executor::block_on(send_message_to_all(
                users.clone(),
                Message::text(format!(
                    "<div id=\"mediainfo\" hx-swap-oob=\"innerHTML\">{}</div>",
                    MediaInfoTemplate {
                        mediainfo: get_media_info(session_manager),
                    }
                )),
            ));
        }

        Ok(())
    }))?;

    Ok(session_manager)
}

pub fn get_session_manager() -> GlobalSystemMediaTransportControlsSessionManager {
    GlobalSystemMediaTransportControlsSessionManager::RequestAsync()
        .unwrap()
        .get()
        .unwrap()
}

pub fn get_media_info(
    session_manager: &GlobalSystemMediaTransportControlsSessionManager,
) -> Option<MediaInfo> {
    let session = session_manager.GetCurrentSession();

    if let Ok(session) = session {
        let properties = session.TryGetMediaPropertiesAsync().ok().unwrap().get();
        if let Ok(properties) = properties {
            let title = properties.Title().ok().unwrap().to_string();
            let artist = properties.Artist().ok().unwrap().to_string();

            if title.is_empty() && artist.is_empty() {
                return None;
            } else {
                return Some(MediaInfo {
                    title: if title.is_empty() { None } else { Some(title) },
                    artist: if artist.is_empty() {
                        None
                    } else {
                        Some(artist)
                    },
                });
            }
        }
    }

    None
}
