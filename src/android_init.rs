// Android-specific initialization for btleplug
#[cfg(target_os = "android")]
use jni::JNIEnv;

#[cfg(target_os = "android")]
#[no_mangle]
pub extern "system" fn Java_org_studio26f_tauri_plugin_bluetooth_ExamplePlugin_initBtleplug(
    env: JNIEnv,
    _class: jni::objects::JClass,
) {
    if let Err(e) = btleplug::platform::init(&env) {
        log::error!("Failed to initialize btleplug: {:?}", e);
    } else {
        log::info!("btleplug initialized successfully");
    }
}
