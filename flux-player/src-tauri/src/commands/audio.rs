use tauri::command;
use windows::Win32::Media::Audio::*;
use windows::Win32::Media::Audio::Endpoints::*;
use windows::Win32::System::Com::*;

#[command]
pub fn get_system_mute_status() -> Result<bool, String> {
    unsafe {
        // Initialize COM (usually already initialized by Tauri but let's be safe)
        let _ = CoInitializeEx(None, COINIT_MULTITHREADED);

        let enumerator: IMMDeviceEnumerator = CoCreateInstance(
            &MMDeviceEnumerator,
            None,
            CLSCTX_ALL,
        ).map_err(|e| format!("Failed to create MMDeviceEnumerator: {}", e))?;

        let endpoint: IMMDevice = enumerator
            .GetDefaultAudioEndpoint(eRender, eConsole)
            .map_err(|e| format!("Failed to get default audio endpoint: {}", e))?;

        // 0.52.0 Activate signature expects Option<*const PROPVARIANT>
        let audio_endpoint_volume: IAudioEndpointVolume = endpoint
            .Activate::<IAudioEndpointVolume>(CLSCTX_ALL, None)
            .map_err(|e| format!("Failed to activate audio endpoint volume: {}", e))?;

        let is_muted = audio_endpoint_volume.GetMute()
            .map_err(|e| format!("Failed to get mute status: {}", e))?;

        Ok(is_muted.as_bool())
    }
}

pub fn classify_device(name: &str) -> String {
    let name_lower = name.to_lowercase();
    
    if name_lower.contains("bluetooth") || name_lower.contains("wireless") {
        "bluetooth".to_string()
    } else if name_lower.contains("headphone") || name_lower.contains("headset") {
        "headphone".to_string()
    } else if name_lower.contains("hdmi") {
        "hdmi".to_string()
    } else if name_lower.contains("usb") {
        "usb".to_string()
    } else {
        "speaker".to_string()
    }
}
