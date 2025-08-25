use std::sync::OnceLock;
use serde_json::json;
use tauri::{AppHandle, Emitter, Manager};

static APP_HANDLE: OnceLock<AppHandle> = OnceLock::new();

pub(crate) fn set_handle(handle: &AppHandle) {
    APP_HANDLE.get_or_init(|| handle.clone());
}

pub(crate) fn log_to_output(name: &str, log: String) {
    let handle = APP_HANDLE.get().expect("Global not initialized");

    if let Some(window) = handle.get_webview_window("Luauncher") {
        let bracket = "[".to_owned()+name+"] ";

        println!("Sending: {bracket}{log}");

        window.emit("log", json!({"info": bracket+&log})).unwrap();
    }
}
