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

/// RAII Guard to ensure COM is initialized and uninitialized correctly per thread.
#[cfg(target_os = "windows")]
pub struct ComGuard;

#[cfg(target_os = "windows")]
impl ComGuard {
    pub fn new() -> windows::core::Result<Self> {
        unsafe {
            // Tauri initialized threads are usually MTA, but we handle existing modes gracefully.
            match CoInitializeEx(None, COINIT_MULTITHREADED) {
                Ok(_) | Err(_) => Ok(ComGuard),
            }
        }
    }
}

#[cfg(target_os = "windows")]
impl Drop for ComGuard {
    fn drop(&mut self) {
        unsafe {
            CoUninitialize();
        }
    }
}

#[derive(serde::Serialize)]
pub struct AudioDeviceInfo {
    pub name: String,
    pub category: String,
}

#[command]
pub fn get_system_mute_status() -> bool {
    #[cfg(target_os = "windows")]
    unsafe {
        let _com = ComGuard::new();
        if let Ok(enumerator) =
            CoCreateInstance::<_, IMMDeviceEnumerator>(&MMDeviceEnumerator, None, CLSCTX_ALL)
        {
            if let Ok(endpoint) = enumerator.GetDefaultAudioEndpoint(eRender, eConsole) {
                if let Ok(volume) = endpoint.Activate::<IAudioEndpointVolume>(CLSCTX_ALL, None) {
                    return volume.GetMute().map(|b| b.as_bool()).unwrap_or(false);
                }
            }
        }
    }
    false
}

#[command]
pub fn get_current_audio_device() -> AppResult<AudioDeviceInfo> {
    #[cfg(target_os = "windows")]
    unsafe {
        let _com = ComGuard::new();
        use windows::Win32::System::Com::StructuredStorage::{
            PropVariantClear, PropVariantToStringAlloc,
        };
        use windows::Win32::UI::Shell::PropertiesSystem::PROPERTYKEY;

        let enumerator: IMMDeviceEnumerator =
            CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL).map_err(|e| {
                AppError::Internal(format!("Failed to create MMDeviceEnumerator: {}", e))
            })?;

        let endpoint: IMMDevice = enumerator
            .GetDefaultAudioEndpoint(eRender, eConsole)
            .map_err(|e| {
                AppError::Internal(format!("Failed to get default audio endpoint: {}", e))
            })?;

        let property_store = endpoint
            .OpenPropertyStore(STGM_READ)
            .map_err(|e| AppError::Internal(format!("Failed to open property store: {}", e)))?;

        // ── Discovery: Hardware Name ────────────────────────
        let mut name = String::new();
        let name_keys = [
            (
                windows::core::GUID::from_u128(0xa45c_2502_df9d_4c39_918a_983b_0704_f1fb),
                14,
            ), // PKEY_Device_FriendlyName
            (
                windows::core::GUID::from_u128(0xb3f8_fa74_ad30_4e0d_b44c_b7ca_0155_b40d),
                2,
            ), // PKEY_DeviceInterface_FriendlyName
            (
                windows::core::GUID::from_u128(0xa45c_2502_df9d_4c39_918a_983b_0704_f1fb),
                2,
            ), // PKEY_Device_DeviceDesc
            (
                windows::core::GUID::from_u128(0x540b_947e_4f44_4194_b0ba_330c_0d18_3d70),
                4,
            ), // PKEY_Device_BusReportedDeviceDesc
            (
                windows::core::GUID::from_u128(0x78c3_5b0e_151a_43a2_afbb_d050_2dec_34b5),
                256,
            ), // PKEY_Device_InstanceId
        ];

        for (guid, pid) in name_keys {
            let pkey = PROPERTYKEY { fmtid: guid, pid };
            if let Ok(prop) = property_store.GetValue(&pkey) {
                if let Ok(pwsz) = PropVariantToStringAlloc(&prop) {
                    if !pwsz.is_null() {
                        let candidate = pwsz.to_string().unwrap_or_default();
                        if !candidate.trim().is_empty() {
                            name = candidate;
                            windows::Win32::System::Com::CoTaskMemFree(Some(pwsz.0 as *const _));
                        }
                    }
                }

                // Final safety internal decode
                if name.is_empty()
                    && prop.Anonymous.Anonymous.vt == windows::Win32::System::Variant::VT_LPWSTR
                {
                    let pwsz = prop.Anonymous.Anonymous.Anonymous.pwszVal;
                    if !pwsz.0.is_null() {
                        name = windows::core::PWSTR(pwsz.0).to_string().unwrap_or_default();
                    }
                }
                let mut prop_mut = prop;
                let _ = PropVariantClear(&mut prop_mut);
                if !name.trim().is_empty() {
                    break;
                }
            }
        }

        if name.is_empty() {
            if let Ok(id) = endpoint.GetId() {
                name = if id.to_string().unwrap_or_default().len() > 15 {
                    "System Speaker (ID)".into()
                } else {
                    id.to_string().unwrap_or_default()
                };
            }
        }

        // ── Discovery: Hardware Form Factor ─────────────────
        // PKEY_AudioEndpoint_FormFactor: {1da5d803-d492-4edd-8c23-e0c0ffee7f0e}, 0
        let pkey_form_factor = PROPERTYKEY {
            fmtid: windows::core::GUID::from_u128(0x1da5_d803_d492_4edd_8c23_e0c0_ffee_7f0e),
            pid: 0,
        };

        let mut form_factor = 10; // UnknownFormFactor default
        if let Ok(prop) = property_store.GetValue(&pkey_form_factor) {
            form_factor = prop.Anonymous.Anonymous.Anonymous.uiVal;
            let mut prop_mut = prop;
            let _ = PropVariantClear(&mut prop_mut);
        }

        // ── Final Name refinement ───────────────────────────
        // If we still have a generic fallback or empty name, generate a pretty name from the form factor
        if name.is_empty() || name == "System Speaker (ID)" {
            name = pretty_name_from_form_factor(form_factor as u32);
        }

        let category = classify_device(&name, form_factor as u32);
        Ok(AudioDeviceInfo { name, category })
    }
    #[cfg(not(target_os = "windows"))]
    {
        Ok(AudioDeviceInfo {
            name: "Default Device".into(),
            category: "speaker".into(),
        })
    }
}

pub fn pretty_name_from_form_factor(form_factor: u32) -> String {
    match form_factor {
        1 => "Speakers".to_string(),
        2 => "Line Out".to_string(),
        3 => "Headphones".to_string(),
        5 => "Headset".to_string(),
        6 => "Handset".to_string(),
        8 => "HDMI / Digital Audio".to_string(),
        9 => "Digital Audio Tape".to_string(),
        _ => "System Audio Device".to_string(),
    }
}

pub fn classify_device(name: &str, form_factor: u32) -> String {
    let name_lower = name.to_lowercase();

    // Exact Form Factor Mapping (First priority)
    // 3 = Headphones, 5 = Headset
    if form_factor == 3 || form_factor == 5 {
        return "headphone".to_string();
    }
    // 8 = SPDIF/Digital, 1 = Speakers
    if form_factor == 8 || form_factor == 1 {
        return "speaker".to_string();
    }

    // String Matching (Secondary priority / specific detection)
    if name_lower.contains("bluetooth")
        || name_lower.contains("wireless")
        || name_lower.contains("buds")
        || name_lower.contains("airpods")
    {
        return "bluetooth".to_string();
    }

    if name_lower.contains("hdmi") || name_lower.contains("tv") {
        return "hdmi".to_string();
    }

    if name_lower.contains("usb") || name_lower.contains("scarlett") {
        return "usb".to_string();
    }

    "speaker".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_device_classification() {
        // Form Factor Priority: Headphones (3) or Headset (5)
        assert_eq!(classify_device("Generic Device", 3), "headphone");
        assert_eq!(classify_device("Generic Device", 5), "headphone");

        // Form Factor Priority: Speakers (1) or Digital (8)
        assert_eq!(classify_device("Generic Device", 1), "speaker");
        assert_eq!(classify_device("Generic Device", 8), "speaker");

        // String Matching Fallback (When FormFactor is unknown/10)
        assert_eq!(
            classify_device("Sony WH-1000XM4 (Bluetooth)", 10),
            "bluetooth"
        );
        assert_eq!(classify_device("Realtek HDMI Output", 10), "hdmi");
        assert_eq!(classify_device("Focusrite Scarlett 2i2 USB", 10), "usb");

        // Specific Bluetooth cases
        assert_eq!(classify_device("AirPods Pro", 10), "bluetooth");
        assert_eq!(classify_device("Galaxy Buds2", 10), "bluetooth");

        // Default
        assert_eq!(classify_device("Unknown Device", 10), "speaker");
    }
}
