use anyhow::{anyhow, Result};
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use windows::core::PCWSTR;
use windows::Win32::Graphics::Gdi::{
    ChangeDisplaySettingsW, EnumDisplayDevicesW, EnumDisplaySettingsW, CDS_UPDATEREGISTRY,
    DEVMODEW, DISPLAY_DEVICEW, DISP_CHANGE_SUCCESSFUL, DM_DISPLAYFREQUENCY, DM_PELSHEIGHT,
    DM_PELSWIDTH, ENUM_CURRENT_SETTINGS, ENUM_DISPLAY_SETTINGS_MODE,
};

#[derive(Debug, Clone)]
pub struct Monitor {
    pub device_name: String,
    pub description: String,
    pub current_width: u32,
    pub current_height: u32,
    pub current_refresh_rate: u32,
    pub available_refresh_rates: Vec<u32>,
}

impl Monitor {
    pub fn enumerate_monitors() -> Result<Vec<Monitor>> {
        let mut monitors = Vec::new();
        let mut device_index = 0;

        loop {
            let mut display_device = DISPLAY_DEVICEW {
                cb: std::mem::size_of::<DISPLAY_DEVICEW>() as u32,
                ..Default::default()
            };

            let success = unsafe {
                EnumDisplayDevicesW(PCWSTR::null(), device_index, &mut display_device, 0)
            };

            if !success.as_bool() {
                break;
            }

            // Skip if not attached to desktop
            if display_device.StateFlags & 0x00000001 == 0 {
                device_index += 1;
                continue;
            }

            let device_name = wide_string_to_string(&display_device.DeviceName);
            let description = wide_string_to_string(&display_device.DeviceString);

            if let Ok(monitor) = Self::get_monitor_info(&device_name, &description) {
                monitors.push(monitor);
            }

            device_index += 1;
        }

        Ok(monitors)
    }

    fn get_monitor_info(device_name: &str, description: &str) -> Result<Monitor> {
        let device_name_wide = string_to_wide(device_name);

        // Get current display settings
        let mut current_mode = DEVMODEW {
            dmSize: std::mem::size_of::<DEVMODEW>() as u16,
            ..Default::default()
        };

        let success = unsafe {
            EnumDisplaySettingsW(
                PCWSTR(device_name_wide.as_ptr()),
                ENUM_CURRENT_SETTINGS,
                &mut current_mode,
            )
        };

        if !success.as_bool() {
            return Err(anyhow!(
                "Failed to get current display settings for {}",
                device_name
            ));
        }

        let current_width = current_mode.dmPelsWidth;
        let current_height = current_mode.dmPelsHeight;
        let current_refresh_rate = current_mode.dmDisplayFrequency;

        // Enumerate all available refresh rates for current resolution
        let mut available_refresh_rates = Vec::new();
        let mut mode_index = 0;

        loop {
            let mut mode = DEVMODEW {
                dmSize: std::mem::size_of::<DEVMODEW>() as u16,
                ..Default::default()
            };

            let success = unsafe {
                EnumDisplaySettingsW(
                    PCWSTR(device_name_wide.as_ptr()),
                    ENUM_DISPLAY_SETTINGS_MODE(mode_index),
                    &mut mode,
                )
            };

            if !success.as_bool() {
                break;
            }

            // Only include modes with the same resolution as current
            if mode.dmPelsWidth == current_width && mode.dmPelsHeight == current_height {
                if !available_refresh_rates.contains(&mode.dmDisplayFrequency) {
                    available_refresh_rates.push(mode.dmDisplayFrequency);
                }
            }

            mode_index += 1;
        }

        available_refresh_rates.sort();

        Ok(Monitor {
            device_name: device_name.to_string(),
            description: description.to_string(),
            current_width,
            current_height,
            current_refresh_rate,
            available_refresh_rates,
        })
    }

    pub fn set_refresh_rate(&self, refresh_rate: u32) -> Result<()> {
        if !self.available_refresh_rates.contains(&refresh_rate) {
            return Err(anyhow!(
                "Refresh rate {}Hz is not available for monitor {}. Available rates: {:?}",
                refresh_rate,
                self.description,
                self.available_refresh_rates
            ));
        }

        let new_mode = DEVMODEW {
            dmSize: std::mem::size_of::<DEVMODEW>() as u16,
            dmFields: DM_PELSWIDTH | DM_PELSHEIGHT | DM_DISPLAYFREQUENCY,
            dmPelsWidth: self.current_width,
            dmPelsHeight: self.current_height,
            dmDisplayFrequency: refresh_rate,
            ..Default::default()
        };

        let result = unsafe { ChangeDisplaySettingsW(Some(&new_mode), CDS_UPDATEREGISTRY) };

        match result {
            DISP_CHANGE_SUCCESSFUL => Ok(()),
            _ => Err(anyhow!(
                "Failed to change refresh rate to {}Hz for monitor {}. Error code: {}",
                refresh_rate,
                self.description,
                result.0
            )),
        }
    }
}

fn wide_string_to_string(wide_str: &[u16]) -> String {
    let end = wide_str
        .iter()
        .position(|&c| c == 0)
        .unwrap_or(wide_str.len());
    OsString::from_wide(&wide_str[..end])
        .to_string_lossy()
        .to_string()
}

fn string_to_wide(s: &str) -> Vec<u16> {
    let mut wide: Vec<u16> = s.encode_utf16().collect();
    wide.push(0); // null terminator
    wide
}
