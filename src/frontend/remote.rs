use crate::{
    frontend::{Button, Head},
    mediainfo::MediaInfo,
};
use maud::{html, Markup, Render, DOCTYPE};

pub struct Remote {
    pub volume: u32,
    pub music: bool,
    pub media: bool,
    pub mediainfo: Option<MediaInfo>,
}

impl Render for Remote {
    fn render(&self) -> Markup {
        html! {
          (DOCTYPE)
          html lang="en" {
            (Head);
            body {
              div.wrapper hx-ext="ws" ws-connect="/ws" {
                @if self.media {
                  div #mediainfo .double.info {
                    @if let Some(mediainfo) = &self.mediainfo {
                      (mediainfo)
                    }
                  }
                }

                @if self.music {
                  (Button::new("playpause", "play").icon2("pause"))
                  (Button::new("stop", "stop"))
                  (Button::new("prev", "backward"))
                  (Button::new("next", "forward"))
                } @else {
                  (Button::new("pause", "minus").double())
                  (Button::new("left", "arrow-left"))
                  (Button::new("right", "arrow-right"))
                }

                div #vol-field {
                  input #vol type="range" min="0" max="100" value=(self.volume) name="vol" hx-trigger="input" ws-send;
                  span #volume { (self.volume) }
                }

                (Button::new("mute", "volume-mute"))
                (Button::new("unmute", "volume-high"))
              }
            }
          }
        }
    }
}
