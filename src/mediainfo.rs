use windows::Media::Control::GlobalSystemMediaTransportControlsSessionManager;

#[derive(Clone)]

pub struct MediaInfo {
    pub title: Option<String>,
    pub artist: Option<String>,
}

pub fn get_media_info() -> Option<MediaInfo> {
    let session_manager = GlobalSystemMediaTransportControlsSessionManager::RequestAsync()
        .unwrap()
        .get()
        .unwrap();

    let session = session_manager.GetCurrentSession();

    if let Ok(session) = session {
        let properties = session.TryGetMediaPropertiesAsync().ok().unwrap().get();
        if let Ok(properties) = properties {
            let title = properties.Title().ok().unwrap().to_string();
            let artist = properties.Artist().ok().unwrap().to_string();

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

    None
}
