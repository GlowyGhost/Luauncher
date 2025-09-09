use directories::BaseDirs;
use std::collections::HashMap;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use std::{env, fs, io::Write, fs::File, process::Command, path::Path};
use uuid::Uuid;

#[cfg(target_os = "windows")]
use ico::{IconDir, IconImage, ResourceType};
#[cfg(target_os = "windows")]
use windows::{
    core::{PCWSTR, Interface},
    Win32::{
        UI::Shell::{IShellLinkW, ShellLink},
        System::Com::{IPersistFile, CoInitializeEx, CoCreateInstance, CoUninitialize, CLSCTX_INPROC_SERVER, COINIT_APARTMENTTHREADED},
    },
};
#[cfg(target_os = "macos")]
use std::os::unix::fs::PermissionsExt;

use crate::output;

#[cfg(target_os = "windows")]
const EMBEDDED_BIN: &[u8] = include_bytes!("../updater/target/release/updater.exe");

#[cfg(target_os = "linux")]
const EMBEDDED_BIN: &[u8] = include_bytes!("../updater/target/release/updater");

#[cfg(target_os = "macos")]
const EMBEDDED_BIN: &[u8] = include_bytes!("../updater/target/release/updater");

pub(crate) fn get_app_base() -> Option<PathBuf> {
    BaseDirs::new().map(|dirs| dirs.config_dir().join("Luauncher"))
}

pub(crate) fn get_settings_path() -> Option<PathBuf> {
    get_app_base().map(|base| base.join("settings.json"))
}

pub(crate) fn get_scripts_dir() -> Option<PathBuf> {
    get_app_base().map(|base| base.join("scripts"))
}

pub(crate) fn get_icon_dir() -> Option<PathBuf> {
    get_app_base().map(|base| base.join("icons"))
}

pub(crate) fn get_file_content(path: String) -> String {
    fs::read_to_string(path).unwrap_or_else(|_| String::new())
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Settings {
    pub(crate) dark: bool,
    pub(crate) dev: bool,
    pub(crate) close: bool,
    #[serde(default)]
    pub(crate) games: HashMap<String, String>,
    pub(crate) iconupdates: bool,
}

pub(crate) fn save_settings(settings: &Settings) -> std::io::Result<()> {
    if let Some(path) = get_settings_path() {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let json = serde_json::to_string_pretty(settings)?;
        fs::write(path, json)?;
    }
    Ok(())
}

pub(crate) fn load_settings() -> std::io::Result<Option<Settings>> {
    if let Some(path) = get_settings_path() {
        if path.exists() {
            let data = fs::read_to_string(path)?;
            let settings: Settings = serde_json::from_str(&data)?;
            return Ok(Some(settings));
        }
    }
    Ok(None)
}

pub(crate) fn save_script(name: &str, content: &str) -> std::io::Result<()> {
    if let Some(scripts_dir) = get_scripts_dir() {
        fs::create_dir_all(&scripts_dir)?;
        let plugin_path = scripts_dir.join(format!("{}.lua", name));
        fs::write(plugin_path, content)?;
    }
    Ok(())
}

pub(crate) fn list_scripts() -> std::io::Result<Vec<String>> {
    if let Some(dir) = get_scripts_dir() {
        let mut names = Vec::new();

        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                    names.push(stem.to_string());
                }
            }
        }

        return Ok(names);
    }

    Ok(vec![])
}

pub(crate) fn delete_file(path: PathBuf) -> std::io::Result<()> {
    fs::remove_file(path)
}

pub(crate) fn make_dirs() {
    let app_dir = get_app_base();

    if app_dir.is_none() {
        output::add_log("[Getting App Settings Directory] Failed to get app base directory.".to_owned(), output::LogLevel::Error, false);
        return;
    }

    let app_dir = app_dir.unwrap();

    let settings_path = app_dir.join("settings.json");
    let scripts_dir = app_dir.join("scripts");
    let icon_dir = app_dir.join("icons");

    if let Err(e) = fs::create_dir_all(&scripts_dir) {
        output::add_log(format!("[Creating Scripts Directory] Failed to create scripts directory: {e}"), output::LogLevel::Error, false);
    }

    if let Err(e) = fs::create_dir_all(&icon_dir) {
        output::add_log(format!("[Creating Icon Directory] Failed to create icon directory: {e}"), output::LogLevel::Error, false);
    }

    if !settings_path.exists() {
        let default_settings = r#"{
            "dark": true,
            "dev": false,
            "close": true,
            "games": {},
            "iconupdates": false
        }"#;

        if let Err(e) = fs::write(&settings_path, default_settings) {
            output::add_log(format!("[Writing Default Settings] Failed to write default settings: {e}"), output::LogLevel::Error, false);
        }
    }
}

pub(crate) fn write_file(path: PathBuf, contents: &str) -> Result<(), std::io::Error> {
    fs::write(path, contents)
}

pub(crate) fn extract_updater(arg: &str, path: PathBuf) -> Result<String, String> {
    let mut temp_path = env::temp_dir();

    #[cfg(target_os = "windows")]
    let filename = "Luauncher-Uninstaller-".to_string()+&Uuid::new_v4().to_string()+".exe";
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    let filename = "Luauncher-Uninstaller-".to_string()+&Uuid::new_v4().to_string();

    temp_path.push(&filename);

    let mut file = fs::File::create(&temp_path).unwrap();
    let _ = file.write_all(EMBEDDED_BIN);

    #[cfg(unix)]
    {
        let perms = fs::metadata(&temp_path)
            .map_err(|e| e.to_string())?
            .permissions();

        fs::set_permissions(&temp_path, perms).map_err(|e| e.to_string())?;
    }

    let _ = Command::new("cmd")
        .args(["/C", "start", "", &temp_path.to_string_lossy().to_string(), arg, &path.to_string_lossy().to_string()])
        .spawn();

    Ok(filename)
}

pub(crate) fn create_icon(path: &str, name: &str) -> Result<String, String> {
    #[cfg(target_os = "windows")]
    {
        let new_path = Path::new(&get_icon_dir().unwrap().to_string_lossy().to_string()).join(format!("{}.ico", name));
        if new_path.exists() {
            match load_settings() {
                Ok(Some(settings)) => {
                    if !settings.iconupdates {
                        return Ok(name.to_owned() + ".ico");
                    }
                }
                Ok(None) => {
                    println!("Settings not found.");
                }
                Err(e) => {
                    output::add_log(format!("[Loading Settings] Failed to load settings: {e}"), output::LogLevel::Error, false);
                }
            }
        }

        let icon_base64 = windows_icons::get_icon_base64_by_path(path)
            .map_err(|e| format!("Failed to extract icon: {e}"))?;

        let icon_bytes = base64::decode(&icon_base64)
            .map_err(|e| format!("Failed to decode base64: {e}"))?;

        let image = image::load_from_memory(&icon_bytes).map_err(|e| format!("Failed to load PNG: {e}"))?;
        let rgba = image.to_rgba8();
        let (width, height) = rgba.dimensions();
        let icon_image = IconImage::from_rgba_data(width, height, rgba.into_raw());

        let mut icon_dir = IconDir::new(ResourceType::Icon);
        icon_dir.add_entry(ico::IconDirEntry::encode(&icon_image).map_err(|e| format!("Failed to encode icon: {e}"))?);

        let mut file = File::create(new_path).map_err(|e| format!("Failed to create ico file: {e}"))?;
        icon_dir.write(&mut file).map_err(|e| format!("Failed to write ico: {e}"))?;

        Ok(name.to_owned() + ".ico")
    }
    
    #[cfg(target_os = "macos")]
    {
        let new_path = Path::new(&get_icon_dir().unwrap().to_string_lossy().to_string()).join(format!("{:?}.icns", name));
        if new_path.exists() {
            match load_settings() {
                Ok(Some(settings)) => {
                    if !settings.iconupdates {
                        return Ok(name.to_owned() + ".icns");
                    }
                }
                Ok(None) => {
                    println!("Settings not found.");
                }
                Err(e) => {
                    output::add_log(format!("[Loading Settings] Failed to load settings: {e}"), output::LogLevel::Error, false);
                }
            }
        }

        let icon_path = Path::new(&path).join("Contents/Resources/AppIcon.icns");
        if !icon_path.exists() {
            return Ok("".to_string());
        }

        fs::copy(&icon_path, &new_path)
            .map_err(|e| format!("Failed to copy icns: {}", e))?;

        Ok(name.to_owned() + ".icns")
    }

    #[cfg(target_os = "linux")]
    {
        Err("Linux icon creation not implemented".to_string())
    }
}

pub(crate) fn get_icon(name: &str) -> Option<PathBuf> {
    if let Some(icon_dir) = get_icon_dir() {
        let icon_path = icon_dir.join(name);
        if icon_path.exists() {
            return Some(icon_path);
        } else {
            return create_icon(&icon_path.to_string_lossy().to_string(), name)
                .ok()
                .and_then(|name| get_icon(&name));
        }
    }
    None
}

pub(crate) fn create_shortcut(path: &str, name: &str) -> Result<String, String> {
    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    {
        return Err("Linux".to_string());
    }

    let app = std::env::current_exe().unwrap().to_string_lossy().to_string();

    #[cfg(target_os = "windows")]
    {
        let icon_path = get_icon_dir()
            .ok_or("Failed to get icon directory".to_string())?
            .join(format!("{}.ico", name));

        unsafe {
            CoInitializeEx(Some(std::ptr::null_mut()), COINIT_APARTMENTTHREADED)
                .ok()
                .map_err(|e| format!("CoInitializeEx failed: {e}"))?;

            let shell_link: IShellLinkW = CoCreateInstance(&ShellLink, None, CLSCTX_INPROC_SERVER)
                .map_err(|e| format!("CoCreateInstance failed: {e}"))?;

            shell_link.SetPath(PCWSTR::from_raw(app.encode_utf16().chain(Some(0)).collect::<Vec<_>>().as_ptr()))
                .map_err(|e| format!("SetPath failed: {e}"))?;

            shell_link.SetArguments(PCWSTR::from_raw(format!("\"{}\"", name).encode_utf16().chain(Some(0)).collect::<Vec<_>>().as_ptr()))
                .map_err(|e| format!("SetArguments failed: {e}"))?;

            shell_link.SetIconLocation(PCWSTR::from_raw(icon_path.to_string_lossy().encode_utf16().chain(Some(0)).collect::<Vec<_>>().as_ptr()), 0)
                .map_err(|e| format!("SetIconLocation failed: {e}"))?;

            let persist_file: IPersistFile = shell_link.cast().map_err(|e| format!("cast to IPersistFile failed: {e}"))?;
            persist_file.Save(PCWSTR::from_raw(path.encode_utf16().chain(Some(0)).collect::<Vec<_>>().as_ptr()), true)
                .map_err(|e| format!("Save failed: {e}"))?;

            CoUninitialize();
        }

        return Ok(path.to_string());
    }

    #[cfg(target_os = "macos")]
    {
        let icon_path = get_icon_dir()
            .ok_or("Failed to get icon directory".to_string())?
            .join(format!("{}.icns", name));

        let script = format!("#!/bin/bash\nopen \"{}\" --args \"{}\"\n", app, name);

        std::fs::write(&path, script).map_err(|e| format!("Failed to write shortcut: {e}"))?;
        let mut perms = std::fs::metadata(&path).map_err(|e| format!("Failed to get metadata: {e}"))?.permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&path, perms).map_err(|e| format!("Failed to set permissions: {e}"))?;

        return Ok(path.to_string());
    }
}
