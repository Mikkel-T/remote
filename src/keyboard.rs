use windows::Win32::UI::Input::KeyboardAndMouse::{
    SendInput, INPUT, INPUT_0, INPUT_KEYBOARD, KEYBDINPUT, KEYBD_EVENT_FLAGS, KEYEVENTF_KEYUP,
    VIRTUAL_KEY, VK_LEFT, VK_MEDIA_NEXT_TRACK, VK_MEDIA_PLAY_PAUSE, VK_MEDIA_PREV_TRACK,
    VK_MEDIA_STOP, VK_RIGHT, VK_SPACE,
};

type Key = VIRTUAL_KEY;

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
        let pinputs = vec![
            INPUT {
                r#type: INPUT_KEYBOARD,
                Anonymous: INPUT_0 {
                    ki: KEYBDINPUT {
                        wVk: key.into(),
                        wScan: 0,
                        dwFlags: KEYBD_EVENT_FLAGS(0),
                        time: 0,
                        dwExtraInfo: 0,
                    },
                },
            },
            INPUT {
                r#type: INPUT_KEYBOARD,
                Anonymous: INPUT_0 {
                    ki: KEYBDINPUT {
                        wVk: key.into(),
                        wScan: 0,
                        dwFlags: KEYEVENTF_KEYUP,
                        time: 0,
                        dwExtraInfo: 0,
                    },
                },
            },
        ];

        SendInput(&pinputs, std::mem::size_of::<INPUT>() as i32);
    }
}

impl From<KeyCode> for Key {
    fn from(key: KeyCode) -> Key {
        match key {
            KeyCode::Space => VK_SPACE,
            KeyCode::Right => VK_RIGHT,
            KeyCode::Left => VK_LEFT,
            KeyCode::PlayPause => VK_MEDIA_PLAY_PAUSE,
            KeyCode::Next => VK_MEDIA_NEXT_TRACK,
            KeyCode::Prev => VK_MEDIA_PREV_TRACK,
            KeyCode::Stop => VK_MEDIA_STOP,
        }
    }
}
