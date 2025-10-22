use serde::de::DeserializeOwned;
use tauri::{
    plugin::{PluginApi, PluginHandle},
    AppHandle, Runtime,
};

use crate::models::*;

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_bluetooth);

// initializes the Kotlin or Swift plugin classes
pub fn init<R: Runtime, C: DeserializeOwned>(
    _app: &AppHandle<R>,
    api: PluginApi<R, C>,
) -> crate::Result<PluginBase<R>> {
    #[cfg(target_os = "android")]
    let handle =
        api.register_android_plugin("org.studio26f.tauri.plugin.bluetooth", "ExamplePlugin")?;
    #[cfg(target_os = "ios")]
    let handle = api.register_ios_plugin(init_plugin_bluetooth)?;
    Ok(PluginBase(handle))
}

/// Access to the bluetooth APIs.
pub struct PluginBase<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> PluginBase<R> {
    pub fn ping(&self, payload: PingRequest) -> crate::Result<PingResponse> {
        self.0
            .run_mobile_plugin("ping", payload)
            .map_err(Into::into)
    }

    pub fn scan_devices(&self, options: serde_json::Value) -> crate::Result<Vec<crate::DeviceInfo>> {
        #[derive(serde::Deserialize)]
        struct ScanResponse {
            devices: Vec<crate::DeviceInfo>,
        }

        let response: ScanResponse = self.0
            .run_mobile_plugin("scanDevices", options)
            .map_err(|e| crate::Error::PluginInvoke(e))?;

        Ok(response.devices)
    }

    pub fn start_continuous_scan(&self) -> crate::Result<()> {
        self.0
            .run_mobile_plugin("startContinuousScan", ())
            .map_err(|e| crate::Error::PluginInvoke(e))
    }

    pub fn stop_continuous_scan(&self) -> crate::Result<()> {
        self.0
            .run_mobile_plugin("stopContinuousScan", ())
            .map_err(|e| crate::Error::PluginInvoke(e))
    }

    pub fn get_continuous_scan_results(&self) -> crate::Result<Vec<crate::DeviceInfo>> {
        #[derive(serde::Deserialize)]
        struct ScanResponse {
            devices: Vec<crate::DeviceInfo>,
        }

        let response: ScanResponse = self.0
            .run_mobile_plugin("getContinuousScanResults", ())
            .map_err(|e| crate::Error::PluginInvoke(e))?;

        Ok(response.devices)
    }
}
