use windows::core::*;
use windows::Win32::Media::Audio::Endpoints::*;
use windows::Win32::Media::Audio::*;
use windows::Win32::System::Com::*;

fn get_endpoint() -> Result<IAudioEndpointVolume> {
    unsafe {
        CoInitializeEx(std::ptr::null_mut(), COINIT_MULTITHREADED)?;
        let immde: IMMDeviceEnumerator = CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL)?;
        let imm = immde.GetDefaultAudioEndpoint(eRender, eMultimedia)?;
        let mut iae: Option<IAudioEndpointVolume> = None;
        imm.Activate(
            &IAudioEndpointVolume::IID,
            CLSCTX_INPROC_SERVER,
            core::ptr::null(),
            &mut iae as *mut _ as *mut _,
        )?;

        Ok(iae.unwrap())
    }
}

pub fn get() -> Result<f32> {
    unsafe { get_endpoint().unwrap().GetMasterVolumeLevelScalar() }
}
pub fn set(vol: f32) -> Result<()> {
    unsafe {
        get_endpoint()
            .unwrap()
            .SetMasterVolumeLevelScalar(vol, core::ptr::null())
    }
}

pub fn mute() -> Result<()> {
    unsafe { get_endpoint().unwrap().SetMute(true, core::ptr::null()) }
}

pub fn unmute() -> Result<()> {
    unsafe { get_endpoint().unwrap().SetMute(false, core::ptr::null()) }
}
