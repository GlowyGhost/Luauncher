use directories::BaseDirs;
use std::collections::HashMap;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use std::{env, fs, io::Write, process::Command};
use uuid::Uuid;

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
    if let Some(plugins_dir) = get_scripts_dir() {
        let mut names = Vec::new();

        for entry in fs::read_dir(plugins_dir)? {
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
        eprintln!("Failed to get app base directory.");
        return;
    }

    let app_dir = app_dir.unwrap();

    let settings_path = app_dir.join("settings.json");
    let scripts_dir = app_dir.join("scripts");

    if let Err(e) = fs::create_dir_all(&scripts_dir) {
        eprintln!("Failed to create scripts directory: {e}");
    }

    if !settings_path.exists() {
        let default_settings = r#"{
            "dark": true,
            "dev": false,
            "close": true,
            "games": {}
        }"#;

        if let Err(e) = fs::write(&settings_path, default_settings) {
            eprintln!("Failed to write default settings: {e}");
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
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&temp_path)
        .map_err(|e| e.to_string())?
        .permissions();

        fs::set_permissions(&temp_path, perms).map_err(|e| e.to_string())?;
    }

    let _ = Command::new("cmd")
        .args(["/C", "start", "", &temp_path.to_string_lossy().to_string(), arg, &path.to_string_lossy().to_string()])
        .spawn();

    Ok(filename)
}
