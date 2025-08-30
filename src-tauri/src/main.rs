// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::HashMap;
use std::net::TcpStream;
use std::time::Duration;
use serde::Deserialize;
use tauri::Manager;

use rfd::{FileDialog, MessageDialog, MessageDialogResult};
#[cfg(target_os = "windows")]
use windows_icons::get_icon_base64_by_path;
#[cfg(target_os = "macos")]
use std::path::Path;
#[cfg(target_os = "macos")]
use icns::IconFamily;
#[cfg(target_os = "macos")]
use base64::Engine;

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
    let settings = files::load_settings().unwrap().unwrap();

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
fn make_plugin(name: String, path: String, code: String) -> Result<String, String> {
    let mut settings = files::load_settings().unwrap().unwrap();
    let mut games = settings.games;
    games.insert(name.clone(), path);

    settings.games = games;

    let _ = files::save_settings(&settings);

    let _ = files::save_script(&name, &files::get_file_content(code));
    
    Ok("Made Plugin".to_string())
}

#[tauri::command]
fn save_game(mut path: String, name: String, oldn: String) -> Result<String, String> {
    let mut settings = files::load_settings().unwrap().unwrap();
    let mut games = settings.games;

    if path == "" {
        path = get_game_path(&oldn).unwrap_or("".to_string());

        if path == "" {
            return Ok("Err".to_owned())
        }
    }

    games.remove(&oldn);
    games.insert(name.clone(), path);

    settings.games = games;

    let oldnlua = format!("{}.lua", oldn);

    let scripts_dir = files::get_scripts_dir().unwrap();
    let content = files::get_file_content(format!("{}\\{oldnlua}", scripts_dir.to_string_lossy().to_string()));

    let _ = files::delete_file(scripts_dir.join(&oldnlua));
    let _ = files::save_script(&name, &content);

    let _ = files::save_settings(&settings);
    
    Ok("Saved Game".to_string())
}

#[tauri::command]
fn delete_game(name: String) -> Result<String, String> {
    let path = files::get_scripts_dir().unwrap();
    
    let _ = files::delete_file(path.join(&format!("{}.lua", &name)));

    let mut settings = files::load_settings().unwrap().unwrap();
    let mut games = settings.games;
    games.remove(&name);

    settings.games = games;

    let _ = files::save_settings(&settings);

    Ok("Deleted Game".to_string())
}

#[tauri::command]
fn hide_app(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("Luauncher") {
        window.hide().map_err(|e| e.to_string())?;
        Ok(())
    } else {
        Err("Window 'Luauncher' not found".to_owned())
    }
}

#[tauri::command]
fn get_settings() -> Result<Option<files::Settings>, String> {
    files::load_settings().map_err(|e| e.to_string())
}

#[tauri::command]
fn save_log(log: String) -> Result<String, String> {
    if let Some(path) = FileDialog::new()
        .set_file_name("log.txt")
        .save_file()
    {
        let res = files::write_file(path, &log);

        match res {
            Ok(_file) => {
                return Ok("Success".to_string())
            }

            Err(e) => {
                return Err(e.to_string())
            }
        }
    } else {
        return Ok("Cancelled".to_string())
    }
}

#[tauri::command]
fn uninstall() -> Result<String, String> {
    let res = MessageDialog::new()
        .set_title("Uninstall")
        .set_description("Are you sure you want to uninstall Luauncher?")
        .set_buttons(rfd::MessageButtons::YesNo)
        .show();

    if res == MessageDialogResult::Yes {
        return files::extract_updater("uninstall", std::env::current_exe().unwrap());
    }
    Ok("Undid".to_string())
}

#[derive(Deserialize)]
struct Release {
    name: String
}

#[tauri::command]
async fn update() -> Result<String, String> {
    let connected = TcpStream::connect_timeout(
        &("1.1.1.1:80".parse().unwrap()),
        Duration::from_secs(2)
    ).is_ok();

    if connected == false {
        return Ok("No Internet".to_string());
    }

    let url = "https://api.github.com/repos/GlowyGhost/Luauncher/releases/latest";

    let client = reqwest::Client::new();
    let mut res = client
        .get(url)
        .header("User-Agent", "Luauncher-app")
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json::<Release>()
        .await
        .map_err(|e| e.to_string())?;

    res.name.remove(0);

    if res.name == env!("CARGO_PKG_VERSION") {
        return Ok("No Update".to_string());
    }

    let res_msg = MessageDialog::new()
        .set_title("Update")
        .set_description(format!("Are you sure you want to update Luauncher from {} to {}?", env!("CARGO_PKG_VERSION"), res.name))
        .set_buttons(rfd::MessageButtons::YesNo)
        .show();

    if res_msg == MessageDialogResult::Yes {
        return files::extract_updater("update", std::env::current_exe().unwrap());
    }
    Ok("Undid".to_string())
}

#[tauri::command]
fn get_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[tauri::command]
fn open_link(url: String) -> Result<(), String> {
    open::that(url).map_err(|e| e.to_string())
}

#[cfg(target_os = "windows")]
#[tauri::command]
fn get_icon(exePath: String) -> Result<Option<String>, String> {
    //      ^^^^^^^ Still camelCase...
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
    //      ^^^^^^^ Still camelCase... I hope you get the point now...
    let icon_path = Path::new(&exe_path)
        .join("Contents")
        .join("Resources")
        .join("AppIcon.icns");

    if !icon_path.exists() {
        return Ok(None);
    }

    let icns_data = std::fs::read(&icon_path).map_err(|e| e.to_string())?;
    let reader = IconFamily::read(Cursor::new(icns_data)).map_err(|e| e.to_string())?;

    let preferred_icons = [
        IconType::ARGB32_512x512,
        IconType::ARGB32_256x256,
        IconType::ARGB32_128x128,
        IconType::RGB24_128x128,
    ];

    let image = preferred_icons
        .iter()
        .find_map(|icon_type| reader.get_icon_with_type(*icon_type).ok())
        .ok_or("No suitable icon found in ICNS")?;

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
            if let Some(window) = app.get_webview_window("Luauncher") {
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
                println!("No window labeled 'Luauncher' found.");
            }

            Ok(())})
        .invoke_handler(tauri::generate_handler![get_games, run_game, save_settings, get_settings, restart_app, hide_app,
            get_icon, get_game_path, make_plugin, save_game, delete_game, save_log, uninstall, update, get_version, open_link])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
