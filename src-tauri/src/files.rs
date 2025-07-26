use directories::BaseDirs;
use std::fs;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};

pub(crate) fn get_app_base() -> Option<PathBuf> {
    BaseDirs::new().map(|dirs| dirs.config_dir().join("launcher"))
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
    volume: f32,
    theme: String,
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
        let plugin_path = scripts_dir.join(format!("{}.txt", name));
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

pub(crate) fn make_dirs() {
    let app_dir = get_app_base();

    let settings_path = app_dir
        .as_ref()
        .unwrap()
        .join("settings.json");

    let scripts_dir = app_dir
        .as_ref()
        .unwrap()
        .join("scripts");

    fs::create_dir_all(&scripts_dir).expect("Failed to create scripts dir");

    fs::write(&settings_path, r#"{"volume": 0.5, "theme": "dark"}"#)
        .expect("Failed to write settings");

    println!("Wrote settings to: {:?}", settings_path);
}
