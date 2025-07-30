// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::HashMap;

#[cfg(target_os = "windows")]
use windows_icons::get_icon_base64_by_path;
#[cfg(target_os = "macos")]
use std::path::Path;

use tauri::Manager;

mod lua_utils;
mod files;

#[tauri::command]
fn get_games() -> Vec<String> {
    files::list_scripts()
        .unwrap_or_else(|_| Vec::new())
}

#[tauri::command]
async fn run_game(gameName: String) -> Result<String, String> {
    //            ^^^^^^^^   MUST stay as camelCase. tauri invoke goes wrong if its snake_case.
    let _ = lua_utils::lua_run_game(&gameName)
        .await
        .map_err(|e| format!("Lua run error: {}", e));

    Ok("Game executed Succsessfully.".to_string())
}

#[tauri::command]
fn save_settings(dark: bool, dev: bool, close: bool, games: HashMap<String, String>) -> String {
    let _ = files::save_settings(&files::Settings{
        dark,
        dev,
        close,
        games
    });

    "Saved Settings".to_string()
}

#[tauri::command]
fn get_game_path(gameName: &str) -> Result<String, String> {
    //           ^^^^^^^^ Must stay as camelCase
    let path = files::get_settings_path()
        .ok_or("Settings path not found")?;

    let data = std::fs::read_to_string(path)
        .map_err(|e| format!("Failed to read settings file: {}", e))?;

    let settings: files::Settings = serde_json::from_str(&data)
        .map_err(|e| format!("Failed to parse settings file: {}", e))?;

    let trimmed_name = gameName.trim();

    let found = settings.games.iter()
        .find(|(k, _)| k.trim().eq_ignore_ascii_case(trimmed_name))
        .map(|(_, v)| v.clone());

    found.ok_or_else(|| format!("Game '{}' not found in settings", gameName))
}

#[tauri::command]
fn restart_app() -> Result<(), String> {
    use std::process::Command;

    let current_exe = std::env::current_exe().map_err(|e| e.to_string())?;

    Command::new(current_exe)
        .spawn()
        .map_err(|e| e.to_string())?;

    std::process::exit(0);
}

#[tauri::command]
fn hide_app(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("launcher") {
        window.hide().map_err(|e| e.to_string())?;
        Ok(())
    } else {
        Err("Window 'launcher' not found".to_owned())
    }
}

#[tauri::command]
fn get_settings() -> Result<Option<files::Settings>, String> {
    files::load_settings().map_err(|e| e.to_string())
}

#[cfg(target_os = "windows")]
#[tauri::command]
fn get_icon(exePath: String) -> Result<Option<String>, String> {
    match get_icon_base64_by_path(&exePath) {
        Ok(base64str) => Ok(Some(base64str)),
        Err(e) => {
            eprintln!("Icon extraction failed: {}", e);
            Ok(None)
        }
    }
}

#[cfg(target_os = "macos")]
#[tauri::command]
fn get_icon(exePath: String) -> Result<Option<String>, String> {
    let icon_path = Path::new(&exePath)
        .join("Contents")
        .join("Resources")
        .join("AppIcon.icns");

    if !icon_path.exists() {
        return Ok(None);
    }

    let icns_data = std::fs::read(icon_path).map_err(|e| e.to_string())?;
    let reader = icns::IconFamily::read(std::io::Cursor::new(icns_data)).map_err(|e| e.to_string())?;
    let image = reader.get_best_icon().ok_or("No image found in icns")?;
    let png_data = image.encode_png().map_err(|e| e.to_string())?;

    Ok(Some(base64::engine::general_purpose::STANDARD.encode(png_data)))
}

#[cfg(not(any(target_os = "windows", target_os = "macos")))]
#[tauri::command]
fn get_icon(_exePath: String) -> Result<Option<String>, String> {
    Ok(None)
}

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .setup(|app| {
            if let Some(window) = app.get_webview_window("launcher") {
                window.hide().unwrap();

                let mut args: Vec<String> = std::env::args().collect();

                if args.len() > 1 && !args[1].is_empty() {
                    args.remove(0);

                    let main_arg = args.join(" ");

                    tokio::task::block_in_place(move || {
                        let rt = tokio::runtime::Handle::current();
                        rt.block_on(async move {
                            let _ = run_game(main_arg).await;
                            std::process::exit(0);
                        });
                    });
                } else {
                    files::make_dirs();
                    window.show().unwrap();
                }
            } else {
                println!("No window labeled 'launcher' found.");
            }

            Ok(())})
        .invoke_handler(tauri::generate_handler![get_games, run_game, save_settings, get_settings, restart_app, hide_app, get_icon, get_game_path])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
