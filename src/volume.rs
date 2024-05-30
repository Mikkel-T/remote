use windows::{
    core::Result,
    Win32::{
        Media::Audio::{
            eMultimedia, eRender, Endpoints::IAudioEndpointVolume, IMMDeviceEnumerator,
            MMDeviceEnumerator,
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
