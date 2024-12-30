use crate::mediainfo;
use maud::{html, Markup, Render};
use mediainfo::MediaInfo as MediaInfoStruct;

pub struct MediaInfo {
    pub mediainfo: Option<MediaInfoStruct>,
}

impl Render for MediaInfo {
    fn render(&self) -> Markup {
        html! {
          @if let Some(mediainfo) = &self.mediainfo {
            @if let Some(title) = &mediainfo.title {
              p.title { (title) }
            }
            @if let Some(artist) = &mediainfo.artist {
              p.artist { (artist) }
            }
          } @else {
            p.title { "No media info" }
          }
        }
    }
}
