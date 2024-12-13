use crate::{send_message_to_all, Users};
use serde_json::json;
use warp::filters::ws::Message;
use windows::{
    core::{implement, Result},
    Win32::{
        Media::Audio::{
            eMultimedia, eRender,
            Endpoints::{
                IAudioEndpointVolume, IAudioEndpointVolumeCallback,
                IAudioEndpointVolumeCallback_Impl,
            },
            IMMDeviceEnumerator, MMDeviceEnumerator, AUDIO_VOLUME_NOTIFICATION_DATA,
        },
        System::Com::{CoCreateInstance, CoInitializeEx, CLSCTX_ALL, COINIT_MULTITHREADED},
    },
};

/// Gets the Windows `IAudioEndpointVolume` that controls the main audio.
fn get_endpoint() -> Result<IAudioEndpointVolume> {
    unsafe {
        CoInitializeEx(None, COINIT_MULTITHREADED).unwrap();
        let immde: IMMDeviceEnumerator = CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL)?;
        let imm = immde.GetDefaultAudioEndpoint(eRender, eMultimedia)?;
        imm.Activate::<IAudioEndpointVolume>(CLSCTX_ALL, None)
    }
}

#[implement(IAudioEndpointVolumeCallback)]
struct VolumeChangeCallback {
    users: Users,
}

impl IAudioEndpointVolumeCallback_Impl for VolumeChangeCallback_Impl {
    fn OnNotify(&self, p_notify: *mut AUDIO_VOLUME_NOTIFICATION_DATA) -> Result<()> {
        if let Some(data) = unsafe { p_notify.as_ref() } {
            let info_obj = json!({
                "type": "volume",
                "data": (data.fMasterVolume * 100.).round()
            });
            let info_str = serde_json::to_string(&info_obj).unwrap();

            futures::executor::block_on(send_message_to_all(
                self.users.clone(),
                Message::text(info_str),
            ));
        }
        Ok(())
    }
}

pub fn listen_volume(users: Users) -> Result<IAudioEndpointVolume> {
    let endpoint = get_endpoint().unwrap();
    let callback: IAudioEndpointVolumeCallback = VolumeChangeCallback { users }.into();
    unsafe { endpoint.RegisterControlChangeNotify(&callback) }?;

    Ok(endpoint)
}

/// Get the current volume
pub fn get() -> Result<f32> {
    unsafe { get_endpoint()?.GetMasterVolumeLevelScalar() }
}

/// Set the volume
pub fn set(vol: f32) -> Result<()> {
    unsafe { get_endpoint()?.SetMasterVolumeLevelScalar(vol, core::ptr::null()) }
}

/// Mute the speaker
pub fn mute() -> Result<()> {
    unsafe { get_endpoint()?.SetMute(true, core::ptr::null()) }
}

/// Unmute the speaker
pub fn unmute() -> Result<()> {
    unsafe { get_endpoint()?.SetMute(false, core::ptr::null()) }
}
