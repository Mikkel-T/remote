use winapi::um::winuser;
use winapi::um::winuser::{keybd_event, KEYEVENTF_KEYUP};

type Key = i32;

#[derive(Clone, Copy)]
pub enum KeyCode {
    Space,
    Right,
    Left,
    PlayPause,
    Next,
    Prev,
    Stop,
}

pub fn tap<T: Into<Key> + Copy>(key: T) {
    unsafe {
        keybd_event(key.into() as u8, 0, 0, 0);
        keybd_event(key.into() as u8, 0, KEYEVENTF_KEYUP, 0);
    }
}

impl From<KeyCode> for Key {
    fn from(key: KeyCode) -> Key {
        match key {
            KeyCode::Space => winuser::VK_SPACE,
            KeyCode::Right => winuser::VK_RIGHT,
            KeyCode::Left => winuser::VK_LEFT,
            KeyCode::PlayPause => winuser::VK_MEDIA_PLAY_PAUSE,
            KeyCode::Next => winuser::VK_MEDIA_NEXT_TRACK,
            KeyCode::Prev => winuser::VK_MEDIA_PREV_TRACK,
            KeyCode::Stop => winuser::VK_MEDIA_STOP,
        }
    }
}
