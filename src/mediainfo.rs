use crate::{send_message_to_all, Users};
use maud::{html, Markup, Render};
use warp::filters::ws::Message;
use windows::{
    core::Result,
    Foundation::TypedEventHandler,
    Media::Control::{
        CurrentSessionChangedEventArgs, GlobalSystemMediaTransportControlsSessionManager,
    },
};

#[derive(Clone, Default)]
pub struct MediaInfo {
    title: Option<String>,
    artist: Option<String>,
}

impl MediaInfo {
    pub fn new(session_manager: Option<&GlobalSystemMediaTransportControlsSessionManager>) -> Self {
        let session = session_manager.map_or_else(
            || get_session_manager().GetCurrentSession(),
            GlobalSystemMediaTransportControlsSessionManager::GetCurrentSession,
        );

        if let Ok(session) = session {
            let properties = session.TryGetMediaPropertiesAsync().ok().unwrap().get();
            if let Ok(properties) = properties {
                let title = properties.Title().ok().unwrap().to_string();
                let artist = properties.Artist().ok().unwrap().to_string();

                if title.is_empty() && artist.is_empty() {
                    return Self::default();
                }

                return Self {
                    title: if title.is_empty() { None } else { Some(title) },
                    artist: if artist.is_empty() {
                        None
                    } else {
                        Some(artist)
                    },
                };
            }
        }

        Self::default()
    }

    fn is_empty(&self) -> bool {
        self.title.is_none() && self.artist.is_none()
    }
}

impl Render for MediaInfo {
    fn render(&self) -> Markup {
        html! {
            @if !self.is_empty() {
                @if let Some(title) = &self.title {
                    p.title { (title) }
                }
                @if let Some(artist) = &self.artist {
                    p.artist { (artist) }
                }
            } @else {
                p.title { "No media info" }
            }
        }
    }
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
                Message::text(html! {
                    dix #mediainfo hx-swap-oob="innerHTML" {
                        (MediaInfo::new(Some(session_manager)))
                    }
                }),
            ));
        }

        Ok(())
    }))?;

    Ok(session_manager)
}

fn get_session_manager() -> GlobalSystemMediaTransportControlsSessionManager {
    GlobalSystemMediaTransportControlsSessionManager::RequestAsync()
        .unwrap()
        .get()
        .unwrap()
}
