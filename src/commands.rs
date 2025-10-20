use tauri::{command, AppHandle, Runtime};

use crate::bluetooth::models::RequestDeviceOptions;
use crate::models::*;
use crate::Result;
use crate::{Error, PluginExt};

#[command]
pub(crate) async fn ping<R: Runtime>(
    app: AppHandle<R>,
    payload: PingRequest,
) -> Result<PingResponse> {
    app.plugin_base().ping(payload)
}

#[command]
pub(crate) async fn gatt_connect<R: Runtime>(app: AppHandle<R>, device_id: String) -> Result<()> {
    app.bluetooth_manager().gatt_connect(device_id).await
}

#[command]
pub(crate) async fn gatt_connected<R: Runtime>(app: AppHandle<R>, device_id: String) -> Result<bool> {
    app.bluetooth_manager().gatt_connected(device_id).await
}

#[command]
pub(crate) async fn get_availability<R: Runtime>(_app: AppHandle<R>) -> Result<bool> {
    #[cfg(any(target_os = "android", target_os = "ios"))]
    {
        // Mobile: Always return true if Bluetooth hardware exists
        Ok(true)
    }

    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    {
        _app.bluetooth_manager().get_availability().await
    }
}

#[command]
pub(crate) async fn request_device<R: Runtime>(
    app: AppHandle<R>,
    options: RequestDeviceOptions,
) -> Result<DeviceInfo> {
    if !options.accept_all_devices.unwrap_or(false) && options.filters.is_none() {
        return Err(Error::InvalidRequestDeviceOptions);
    }
    app.bluetooth_manager().request_device(options).await
}

#[command]
pub(crate) async fn scan_devices<R: Runtime>(
    app: AppHandle<R>,
    options: RequestDeviceOptions,
) -> Result<Vec<DeviceInfo>> {
    log::info!("scan_devices command called");

    if !options.accept_all_devices.unwrap_or(false) && options.filters.is_none() {
        return Err(Error::InvalidRequestDeviceOptions);
    }

    #[cfg(any(target_os = "android", target_os = "ios"))]
    {
        log::info!("Using mobile (Kotlin/Swift) implementation for Android/iOS");
        // Mobile: Use Kotlin/Swift implementation
        let json_options = serde_json::to_value(&options).map_err(|e| Error::Unknown(e.to_string()))?;
        return app.plugin_base().scan_devices(json_options);
    }

    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    {
        log::info!("Using desktop (btleplug) implementation");
        // Desktop: Use btleplug
        app.bluetooth_manager().scan_devices(options).await
    }
}

pub fn collect_handlers<R: Runtime>() -> impl Fn(tauri::ipc::Invoke<R>) -> bool {
    tauri::generate_handler![ping, gatt_connect, gatt_connected, get_availability, request_device, scan_devices]
}
