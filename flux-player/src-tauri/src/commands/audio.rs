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

        // Activate the volume control interface
        let audio_endpoint_volume: IAudioEndpointVolume =
            endpoint.Activate(CLSCTX_ALL, None).map_err(|e| {
                AppError::Internal(format!("Failed to activate audio endpoint volume: {}", e))
            })?;

        let is_muted = audio_endpoint_volume
            .GetMute()
            .map_err(|e| AppError::Internal(format!("Failed to get mute status: {}", e)))?;

        Ok(is_muted.as_bool())
    }
}

#[derive(serde::Serialize)]
pub struct AudioDeviceInfo {
    pub name: String,
    pub category: String,
}

#[command]
pub fn get_current_audio_device() -> AppResult<AudioDeviceInfo> {
    #[cfg(target_os = "windows")]
    unsafe {
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

        // Open the property store
        let property_store = endpoint.OpenPropertyStore(STGM_READ).map_err(|e| {
            AppError::Internal(format!("Failed to open property store: {}", e))
        })?;

        // Get the friendly name
        // DEVPKEY_Device_FriendlyName is {A45C2502-DF9D-4C39-918A-983B0704F1FB}, 14
        // The property key for friendly name in the MMDevice property store is actually PKEY_Device_FriendlyName
        // which has the same GUID and ID but is sometimes named differently in windows-rs.
        // If DEVPKEY_Device_FriendlyName doesn't work, we'll use the GUID directly.
        
        let property_key = windows::Win32::UI::Shell::PropertiesSystem::PROPERTYKEY {
            fmtid: windows::core::GUID::from_u128(0xa45c2502_df9d_4c39_918a_983b0704f1fb),
            pid: 14,
        };

        let value = property_store.GetValue(&property_key).map_err(|e| {
            AppError::Internal(format!("Failed to get friendly name property: {}", e))
        })?;

        // Extract string from PROPVARIANT
        use windows::Win32::System::Variant::VT_LPWSTR;
        let name = if value.Anonymous.Anonymous.vt == VT_LPWSTR {
            let pwsz = value.Anonymous.Anonymous.Anonymous.pwszVal;
            if !pwsz.0.is_null() {
                let mut len = 0;
                while *pwsz.0.offset(len) != 0 {
                    len += 1;
                }
                let slice = std::slice::from_raw_parts(pwsz.0, len as usize);
                String::from_utf16_lossy(slice)
            } else {
                "System Speaker".to_string()
            }
        } else {
            "System Speaker".to_string()
        };

        let category = classify_device(&name);

        Ok(AudioDeviceInfo { name, category })
    }
    #[cfg(not(target_os = "windows"))]
    {
        Ok(AudioDeviceInfo {
            name: "Default Device".to_string(),
            category: "speaker".to_string(),
        })
    }
}

pub fn classify_device(name: &str) -> String {
    let name_lower = name.to_lowercase();

    if name_lower.contains("bluetooth")
        || name_lower.contains("wireless")
        || name_lower.contains("airpods")
        || name_lower.contains("buds")
        || name_lower.contains("wh-")
        || name_lower.contains("wf-")
    {
        "bluetooth".to_string()
    } else if name_lower.contains("headphone") || name_lower.contains("headset") || name_lower.contains("earphone") {
        "headphone".to_string()
    } else if name_lower.contains("hdmi") || name_lower.contains("tv") {
        "hdmi".to_string()
    } else if name_lower.contains("usb") || name_lower.contains("scarlett") {
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
        assert_eq!(classify_device("AirPods Pro - Find My"), "bluetooth");
        assert_eq!(classify_device("Galaxy Buds2 Pro (B8E1)"), "bluetooth");

        // Headphones
        assert_eq!(
            classify_device("Studio Headphones (High Definition Audio)"),
            "headphone"
        );
        assert_eq!(
            classify_device("Gaming Headset (Logitech G Pro)"),
            "headphone"
        );
        assert_eq!(classify_device("Earphones (Realtek)"), "headphone");

        // HDMI
        assert_eq!(
            classify_device("Samsung TV (NVIDIA High Definition Audio/HDMI)"),
            "hdmi"
        );
        assert_eq!(classify_device("HDMI Out (Digital Display)"), "hdmi");
        assert_eq!(classify_device("LG TV (Intel(R) Display Audio)"), "hdmi");

        // USB
        assert_eq!(classify_device("USB Audio Codec"), "usb");
        assert_eq!(classify_device("Blue Yeti USB Mic Speaker Output"), "usb");
        assert_eq!(classify_device("Focusrite Scarlett 2i2 USB"), "usb");

        // Default / Speakers
        assert_eq!(classify_device("Realtek Audio Speakers"), "speaker");
        assert_eq!(classify_device("Internal Laptop Sound"), "speaker");
        assert_eq!(classify_device("High Definition Audio Device"), "speaker");
        assert_eq!(classify_device("Unknown Device"), "speaker");
    }
}
