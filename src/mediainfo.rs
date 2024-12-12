use windows::{
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

pub fn get_session_manager() -> GlobalSystemMediaTransportControlsSessionManager {
    GlobalSystemMediaTransportControlsSessionManager::RequestAsync()
        .unwrap()
        .get()
        .unwrap()
}

pub fn get_event_handler<F>(
    handler: F,
) -> TypedEventHandler<
    GlobalSystemMediaTransportControlsSessionManager,
    CurrentSessionChangedEventArgs,
>
where
    F: Fn(MediaInfo) + Send + 'static,
{
    TypedEventHandler::<
        GlobalSystemMediaTransportControlsSessionManager,
        CurrentSessionChangedEventArgs,
    >::new(move |event, _| {
        if let Some(session_manager) = event {
            if let Some(mediainfo) = get_media_info(session_manager) {
                handler(mediainfo);
            }
        }

        Ok(())
    })
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
