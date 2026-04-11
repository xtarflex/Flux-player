#[cfg(target_os = "windows")]
use crate::utils::error::AppError;
use crate::utils::error::AppResult;
use tauri::command;
#[cfg(target_os = "windows")]
use windows::Win32::Media::Audio::Endpoints::*;
#[cfg(target_os = "windows")]
use windows::Win32::Media::Audio::*;
#[cfg(target_os = "windows")]
use windows::Win32::System::Com::*;

#[command]
pub fn get_system_mute_status() -> AppResult<bool> {
    #[cfg(target_os = "windows")]
    unsafe {
        // Initialize COM (usually already initialized by Tauri but let's be safe)
        let _ = CoInitializeEx(None, COINIT_MULTITHREADED);

        let enumerator: IMMDeviceEnumerator =
            CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL).map_err(|e| {
                AppError::Internal(format!("Failed to create MMDeviceEnumerator: {}", e))
            })?;

        let endpoint: IMMDevice = enumerator
            .GetDefaultAudioEndpoint(eRender, eConsole)
            .map_err(|e| {
                AppError::Internal(format!("Failed to get default audio endpoint: {}", e))
            })?;

        // 0.52.0 Activate signature expects Option<*const PROPVARIANT>
        let audio_endpoint_volume: IAudioEndpointVolume = endpoint
            .Activate::<IAudioEndpointVolume>(CLSCTX_ALL, None)
            .map_err(|e| {
                AppError::Internal(format!("Failed to activate audio endpoint volume: {}", e))
            })?;

        let is_muted = audio_endpoint_volume
            .GetMute()
            .map_err(|e| AppError::Internal(format!("Failed to get mute status: {}", e)))?;

        Ok(is_muted.as_bool())
    }
    #[cfg(not(target_os = "windows"))]
    {
        Ok(false)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_device_classification() {
        // Bluetooth/Wireless
        assert_eq!(classify_device("Sony WH-1000XM4 (Bluetooth)"), "bluetooth");
        assert_eq!(classify_device("Wireless Stereo Headset"), "bluetooth");

        // Headphones
        assert_eq!(
            classify_device("Studio Headphones (High Definition Audio)"),
            "headphone"
        );
        assert_eq!(
            classify_device("Gaming Headset (Logitech G Pro)"),
            "headphone"
        );

        // HDMI
        assert_eq!(
            classify_device("Samsung TV (NVIDIA High Definition Audio/HDMI)"),
            "hdmi"
        );
        assert_eq!(classify_device("HDMI Out (Digital Display)"), "hdmi");

        // USB
        assert_eq!(classify_device("USB Audio Codec"), "usb");
        assert_eq!(classify_device("Blue Yeti USB Mic Speaker Output"), "usb");

        // Default / Speakers
        assert_eq!(classify_device("Realtek Audio Speakers"), "speaker");
        assert_eq!(classify_device("Internal Laptop Sound"), "speaker");
    }
}
