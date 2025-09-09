// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{collections::HashMap, net::TcpStream, time::Duration};
use serde::Deserialize;
use tauri::Manager;
use rfd::{FileDialog, MessageDialog, MessageDialogResult};
use base64::{engine::general_purpose, Engine as _};
use image::{DynamicImage, ImageOutputFormat};
use std::io::Cursor;

#[cfg(target_os = "macos")]
use icns::{IconFamily, IconType, PixelFormat};

#[cfg(target_os = "macos")]
use image::{ImageBuffer, Rgba};

#[cfg(target_os = "macos")]
use std::{fs::File, io::BufReader, path::Path};

mod lua_utils;
mod files;
mod output;

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
fn save_settings(dark: bool, dev: bool, close: bool, games: HashMap<String, String>, iconupdates: bool) -> String {
    let _ = files::save_settings(&files::Settings{
        dark,
        dev,
        close,
        games,
        iconupdates
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

    match found {
        Some(path) => Ok(path),
        None => {
            output::add_log(format!("[Getting Game Path] Game '{}' not found in settings", gameName), output::LogLevel::Error, false);
            Err(format!("Game '{}' not found in settings", gameName))
        },
    }
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
    games.insert(name.clone(), path.clone());

    settings.games = games;

    let oldnlua = format!("{}.lua", oldn);

    let scripts_dir = files::get_scripts_dir().unwrap();
    let content = files::get_file_content(format!("{}\\{oldnlua}", scripts_dir.to_string_lossy().to_string()));

    let _ = files::delete_file(scripts_dir.join(&oldnlua));
    let _ = files::save_script(&name, &content);

    let _ = files::save_settings(&settings);

    #[cfg(target_os = "windows")]
    {
        let oldicon = format!("{}.ico", oldn);

        let icons_dir = files::get_icon_dir().unwrap();

        let _ = files::delete_file(icons_dir.join(&oldicon));
        let _ = files::create_icon(&path, &name);
    }

    #[cfg(target_os = "macos")]
    {
        let oldicon = format!("{}.icns", oldn);

        let icons_dir = files::get_icon_dir().unwrap();

        let _ = files::delete_file(icons_dir.join(&oldicon));
        let _ = files::create_icon(&path, &name);
    }

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

    #[cfg(target_os = "windows")]
    let _ = files::delete_file(files::get_icon_dir().unwrap().join(&format!("{}.ico", name)));

    #[cfg(target_os = "macos")]
    let _ = files::delete_file(files::get_icon_dir().unwrap().join(&format!("{}.icns", name)));

    Ok("Deleted Game".to_string())
}

#[tauri::command]
fn hide_app(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("Luauncher") {
        window.hide().map_err(|e| e.to_string())?;
        Ok(())
    } else {
        output::add_log("[Hiding App] Failed to hide window.".to_owned(), output::LogLevel::Warning, false);
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
                output::add_log(format!("[Saving Log] Failed to save log file: {e}"), output::LogLevel::Warning, false);
                return Err(e.to_string())
            }
        }
    } else {
        return Ok("Cancelled".to_string())
    }
}

#[tauri::command]
fn get_logs() -> Result<Vec<output::LogEntry>, String> {
    Ok(output::get_logs())
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

#[tauri::command]
fn create_shortcut(name: String) -> Result<String, String> {
    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    {
        return Err("Linux".to_string());
    }

    #[cfg(any(target_os = "windows", target_os = "macos"))]
    {
        #[cfg(target_os = "windows")]
        let filename = format!("{}.lnk", name);
        #[cfg(target_os = "macos")]
        let filename = format!("{}.command", name);

        if let Some(path) = FileDialog::new()
            .set_file_name(&filename)
            .save_file()
        {
            return files::create_shortcut(&path.to_string_lossy(), &name);
        } else {
            return Ok("Cancelled".to_string())
        }
    }
}

#[tauri::command]
fn get_icon(exePath: String, name: String) -> Result<Option<String>, String> {
    #[cfg(target_os = "windows")]
    {
        let icon_name = files::create_icon(&exePath, &name).map_err(|e| format!("Icon creation failed: {e}"))?;
        let icon_path = files::get_icon(&icon_name)
            .ok_or_else(|| "Icon path not found".to_string())?;

        let icon_bytes = std::fs::read(&icon_path).map_err(|e| format!("Failed to read icon file: {e}"))?;
        let icon_dir = ico::IconDir::read(std::io::Cursor::new(&icon_bytes))
            .map_err(|e| format!("Failed to parse ico: {e}"))?;
        let entry = icon_dir.entries().iter().max_by_key(|e| e.width()).ok_or("No icon entries found")?;
        let icon_image = (*entry).decode().map_err(|e| format!("Failed to decode icon: {e}"))?;
        let width = icon_image.width();
        let height = icon_image.height();
        let rgba = icon_image.rgba_data().to_vec();
        let img_buf = image::ImageBuffer::<image::Rgba<u8>, _>::from_vec(width, height, rgba)
            .ok_or("Failed to create ImageBuffer from icon data")?;
        let dyn_img = DynamicImage::ImageRgba8(img_buf);

        let mut png_data = Vec::new();
        dyn_img.write_to(&mut Cursor::new(&mut png_data), ImageOutputFormat::Png)
            .map_err(|e| format!("Failed to encode PNG: {e}"))?;

        Ok(Some(general_purpose::STANDARD.encode(&png_data)))
    }

    #[cfg(target_os = "macos")]
    {
        let icon_name = files::create_icon(&exePath, &name).map_err(|e| format!("Icon creation failed: {e}"))?;
        let icon_path = files::get_icon(&icon_name)
            .ok_or_else(|| "Icon path not found".to_string())?;

        let file = BufReader::new(File::open(&icon_path).map_err(|e| e.to_string())?);
        let family = IconFamily::read(file).map_err(|e| e.to_string())?;

        let &best_type = family
            .available_icons()
            .iter()
            .max_by_key(|t| t.pixel_width() * t.pixel_height())
            .ok_or("ICNS file contains no icons")?;

        let image = family
            .get_icon_with_type(best_type)
            .map_err(|e| e.to_string())?;

        let width = image.width() as u32;
        let height = image.height() as u32;
        let data = image.data();

        let rgba_bytes: Vec<u8> = match image.pixel_format() {
            PixelFormat::RGBA => data.to_vec(),
            PixelFormat::RGB => {
                let mut out = Vec::with_capacity((width * height * 4) as usize);
                for chunk in data.chunks_exact(3) {
                    out.push(chunk[0]);
                    out.push(chunk[1]);
                    out.push(chunk[2]);
                    out.push(255);
                }
                out
            }
            PixelFormat::GrayAlpha => {
                let mut out = Vec::with_capacity((width * height * 4) as usize);
                for chunk in data.chunks_exact(2) {
                    let gray = chunk[0];
                    let alpha = chunk[1];
                    out.push(gray);
                    out.push(gray);
                    out.push(gray);
                    out.push(alpha);
                }
                out
            }
            PixelFormat::Gray => {
                let mut out = Vec::with_capacity((width * height * 4) as usize);
                for &g in data.iter() {
                    out.push(g);
                    out.push(g);
                    out.push(g);
                    out.push(255);
                }
                out
            }
            PixelFormat::Alpha => {
                return Err("ICNS image contains alpha mask only; unsupported".into());
            }
        };

        let img_buf: ImageBuffer<Rgba<u8>, _> =
            ImageBuffer::from_vec(width, height, rgba_bytes).ok_or("image buffer size mismatch")?;
        let dyn_img = DynamicImage::ImageRgba8(img_buf);

        let mut png_data = Vec::new();
        dyn_img
            .write_to(&mut Cursor::new(&mut png_data), ImageOutputFormat::Png)
            .map_err(|e| e.to_string())?;

        Ok(Some(general_purpose::STANDARD.encode(&png_data)))
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
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
                output::add_log("[App Initialization] No window labeled 'Luauncher' found.".to_owned(), output::LogLevel::Warning, false);
                println!("No window labeled 'Luauncher' found.");
            }

            Ok(())})
        .invoke_handler(tauri::generate_handler![get_games, run_game, save_settings, get_settings, restart_app, hide_app,
            get_icon, get_game_path, make_plugin, save_game, delete_game, save_log, uninstall, update, get_version, open_link,
            get_logs, create_shortcut])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
