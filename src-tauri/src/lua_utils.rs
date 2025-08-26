use mlua::{Lua, Value};
use mlua::prelude::*;
use std::process::Command;
use std::path::PathBuf;
use tokio::time::{sleep, Duration};
use rfd::MessageDialog;

#[cfg(target_os = "windows")]
use winapi::um::winuser::FindWindowA;
#[cfg(target_os = "windows")]
use std::{ffi::CString, ptr::null_mut};

use crate::files::{self, get_scripts_dir};

fn get_custom_lua() -> Lua {
    let lua = Lua::new();
    let globals = lua.globals();

    let _ = globals.set("openApp", lua.create_function(open_app).unwrap());
    let _ = globals.set("closeApp", lua.create_function(close_app).unwrap());
    let _ = globals.set("forceCloseApp", lua.create_function(force_close_app).unwrap());
    let _ = globals.set("isAppOpen", lua.create_function(is_app_open).unwrap());
    let _ = globals.set("isWindowOpen", lua.create_function(is_window_open).unwrap());
    let _ = globals.set("openURL", lua.create_function(open_url).unwrap());
    let _ = globals.set("exit", lua.create_function(exit).unwrap());
    let _ = globals.set("runCommand", lua.create_function(command).unwrap());

    let _ = globals.set("waitUntilWindowClose", lua.create_async_function(|_lua, window_name: String| async move {
        wait_until_window_closed_async(window_name).await
    }).unwrap());

    let _ = globals.set("waitUntilAppClose", lua.create_async_function(|_lua, process_name: String| async move {
        wait_until_app_closed_async(process_name).await
    }).unwrap());

    let _ = globals.set("waitUntilWindowOpen", lua.create_async_function(|_lua, (window_name, timeout): (String, u64)| async move {
        wait_window_opened_async(window_name, timeout).await
    }).unwrap());

    let _ = globals.set("waitUntilAppOpen", lua.create_async_function(|_lua, (process_name, timeout): (String, u64)| async move {
        wait_app_opened_async(process_name, timeout).await
    }).unwrap());
    
    let _ = globals.set("sleep", lua.create_async_function(|_, milliseconds: u64| async move {
        sleep(Duration::from_millis(milliseconds)).await;
        Ok(())
    }).unwrap());

    let _ = globals.set("messageBox", lua.create_function(|_, (title, desc): (String, String)| {
        message_box(title, desc);

        Ok(())
    }).unwrap());

    let _ = globals.set("system", lua.create_string(system()).unwrap_or(
        lua.create_string("Unknown".to_owned()).unwrap()
    ));
    lua
}

fn open_app(_lua: &Lua, arg: String) -> mlua::Result<()> {
    #[cfg(target_os = "windows")]
    {
        let escaped = arg.replace("&", "^&");

        Command::new("cmd")
            .args(["/C", "start", "", &escaped])
            .spawn()
            .map_err(mlua::Error::external)?;
    }

    #[cfg(target_os = "macos")]
    {
        Command::new("open")
            .arg(&arg)
            .spawn()
            .map_err(mlua::Error::external)?;
    }

    #[cfg(target_os = "linux")]
    {
        Command::new(&arg)
            .spawn()
            .map_err(mlua::Error::external)?;
    }

    Ok(())
}
fn close_app(_lua: &Lua, process_name: String) -> mlua::Result<()> {
    #[cfg(target_os = "windows")]
    {
        let escaped = process_name.replace("&", "^&");

        Command::new("taskkill")
            .args(["/IM", &escaped])
            .output()
            .map_err(mlua::Error::external)?;
    }

    #[cfg(any(target_os = "linux", target_os = "macos"))]
    {
        Command::new("pkill")
            .arg(&process_name)
            .output()
            .map_err(mlua::Error::external)?;
    }

    Ok(())
}
fn force_close_app(_lua: &Lua, process_name: String) -> mlua::Result<()> {
    #[cfg(target_os = "windows")]
    {
        let escaped = process_name.replace("&", "^&");

        Command::new("taskkill")
            .args(["/IM", &escaped, "/F"])
            .output()
            .map_err(mlua::Error::external)?;
    }

    #[cfg(any(target_os = "linux", target_os = "macos"))]
    {
        Command::new("pkill")
            .arg("-9", &process_name)
            .output()
            .map_err(mlua::Error::external)?;
    }

    Ok(())
}
fn is_app_open(_lua: &Lua, process_name: String) -> mlua::Result<bool> {
    #[cfg(target_os = "windows")]
    {
        let escaped = process_name.replace("&", "^&");

        let output = Command::new("tasklist")
            .arg("/FI")
            .arg(format!("IMAGENAME eq {}", escaped))
            .output()
            .map_err(mlua::Error::external)?;

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        Ok(stdout.contains(&process_name))
    }

    #[cfg(any(target_os = "linux", target_os = "macos"))]
    {
        let status = Command::new("pgrep")
            .arg("-f")
            .arg(&process_name)
            .status()
            .map_err(mlua::Error::external)?;

        Ok(status.success())
    }
}
fn is_window_open(_lua: &Lua, window_title: String) -> mlua::Result<bool> {
    #[cfg(target_os = "windows")]
    {
        use std::ffi::CString;
        use std::ptr::null_mut;

        let c_title = CString::new(window_title.clone()).map_err(mlua::Error::external)?;
        let hwnd = unsafe { FindWindowA(null_mut(), c_title.as_ptr()) };

        Ok(!hwnd.is_null())
    }

    #[cfg(target_os = "macos")]
    {
        let output = Command::new("osascript")
            .arg("-e")
            .arg(format!(
                r#"tell application "System Events" to get the name of every window of every process"#,
            ))
            .output()
            .map_err(mlua::Error::external)?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        Ok(stdout.contains(&window_title))
    }

    #[cfg(target_os = "linux")]
    {
        let output = Command::new("xdotool")
            .args(["search", "--name", &window_title])
            .output()
            .map_err(mlua::Error::external)?;

        Ok(!String::from_utf8_lossy(&output.stdout).trim().is_empty())
    }
}
fn open_url(_lua: &Lua, url: String) -> mlua::Result<bool> {
    #[cfg(target_os = "windows")]
    {
        let escaped = url.replace("&", "^&");

        Command::new("cmd")
            .args(["/C", "start", "", &escaped])
            .spawn()
            .map_err(mlua::Error::external)?;
    }

    #[cfg(target_os = "macos")]
    {
        Command::new("open")
            .arg(&url)
            .spawn()
            .map_err(mlua::Error::external)?;
    }

    #[cfg(target_os = "linux")]
    {
        Command::new("xdg-open")
            .arg(&url)
            .spawn()
            .map_err(mlua::Error::external)?;
    }

    Ok(true)
}
fn check_window_open(window_title: &str) -> std::io::Result<bool> {
    #[cfg(target_os = "windows")]
    {
        let c_title = CString::new(window_title).unwrap();
        let hwnd = unsafe { FindWindowA(null_mut(), c_title.as_ptr()) };
        Ok(!hwnd.is_null())
    }

    #[cfg(target_os = "macos")]
    {
        let output = Command::new("osascript")
            .arg("-e")
            .arg(r#"tell application "System Events" to get the name of every window of every process"#)
            .output()?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        Ok(stdout.contains(window_title))
    }

    #[cfg(target_os = "linux")]
    {
        let output = Command::new("xdotool")
            .args(["search", "--name", window_title])
            .output()?;

        Ok(!String::from_utf8_lossy(&output.stdout).trim().is_empty())
    }
}
async fn wait_until_window_closed_async(window_title: String) -> LuaResult<()> {
    loop {
        let open = check_window_open(&window_title)
            .map_err(mlua::Error::external)?;

        if !open {
            break;
        }
        sleep(Duration::from_secs(1)).await;
    }
    Ok(())
}
async fn wait_until_app_closed_async(process_name: String) -> LuaResult<()> {
    loop {
        let running = {
            #[cfg(target_os = "windows")]
            {
                let escaped = process_name.replace("&", "^&");

                let output = Command::new("tasklist")
                    .arg("/FI")
                    .arg(format!("IMAGENAME eq {}", &escaped))
                    .output()
                    .map_err(|e| LuaError::external(e))?;

                let stdout = String::from_utf8_lossy(&output.stdout);
                stdout.contains(&process_name)
            }
            #[cfg(any(target_os = "linux", target_os = "macos"))]
            {
                let status = Command::new("pgrep")
                    .arg("-f")
                    .arg(&process_name)
                    .status()
                    .map_err(|e| LuaError::external(e))?;

                status.success()
            }
        };

        if !running {
            break;
        }

        sleep(Duration::from_secs(1)).await;
    }
    Ok(())
}
fn is_window_open_util(title: &str) -> std::io::Result<bool> {
    #[cfg(target_os = "windows")] {
        let c_title = CString::new(title).unwrap();
        let hwnd = unsafe { FindWindowA(null_mut(), c_title.as_ptr()) };
        Ok(!hwnd.is_null())
    }
    #[cfg(target_os = "macos")] {
        let out = Command::new("osascript")
            .args([
              "-e",
              r#"tell application "System Events" to get the name of every window of every process"#,
            ])
            .output()?;
        Ok(String::from_utf8_lossy(&out.stdout).contains(title))
    }
    #[cfg(target_os = "linux")] {
        let out = Command::new("xdotool")
            .args(["search", "--name", title])
            .output()?;
        Ok(!String::from_utf8_lossy(&out.stdout).trim().is_empty())
    }
}
fn is_process_running(name: &str) -> std::io::Result<bool> {
    #[cfg(target_os = "windows")] {
        let escaped = name.replace("&", "^&");

        let output = Command::new("tasklist")
            .args(["/FI", &format!("IMAGENAME eq {}", escaped)])
            .output()?;
        Ok(String::from_utf8_lossy(&output.stdout).contains(&escaped))
    }
    #[cfg(any(target_os = "linux", target_os = "macos"))] {
        let status = Command::new("pgrep")
            .args(["-f", name])
            .status()?;
        Ok(status.success())
    }
}
async fn wait_app_opened_async(name: String, timeout_ms: u64) -> LuaResult<bool> {
    let mut waited = 0;
    let interval = 100;
    while waited < timeout_ms {
        if let Ok(true) = is_process_running(&name) {
            return Ok(true);
        }
        sleep(Duration::from_millis(interval)).await;
        waited += interval;
    }
    Ok(false)
}
async fn wait_window_opened_async(title: String, timeout_ms: u64) -> LuaResult<bool> {
    let mut waited = 0;
    let interval = 100;
    while waited < timeout_ms {
        if let Ok(true) = is_window_open_util(&title) {
            return Ok(true);
        }
        sleep(Duration::from_millis(interval)).await;
        waited += interval;
    }
    Ok(false)
}
fn exit(_lua: &Lua, arg: Option<u64>) -> mlua::Result<()> {
    let err_code = arg.unwrap_or(0)
                            .try_into().unwrap_or(1);

    match files::load_settings() {
        Ok(Some(settings)) => {
            if settings.close {
                std::process::exit(err_code);
            }
        }
        Ok(None) => {
            println!("Settings not found.");
        }
        Err(e) => {
            return Err(LuaError::external(format!("Failed to load settings: {}", e)));
        }
    }

    Ok(())
}
fn system() -> String {
    #[cfg(target_os = "windows")]
    {
        return "Windows".to_owned()
    }

    #[cfg(target_os = "macos")]
    {
        return "MacOS".to_owned()
    }

    #[cfg(target_os = "linux")]
    {
        return "Linux".to_owned()
    }
}
fn run_command(cmd: String) {
    let _ = Command::new("cmd")
        .args(&["/C", &cmd])
        .status()
        .expect("Failed to execute command");
}
fn command(lua: &Lua, cmd: String) -> mlua::Result<()> {
    match files::load_settings() {
        Ok(Some(settings)) => {
            if settings.dev {
                run_command(cmd);
            } else {
                message_box("Developer mod off".to_owned(), "Script attempted to run \"runCommand\" on your computer without Developer mod on. For your safety, the script wasn't allowed to run their command.".to_owned());
                return exit(lua, Some(99));
            }
        }
        Ok(None) => {
            println!("Settings not found.");
        }
        Err(e) => {
            return Err(LuaError::external(format!("Failed to load settings: {}", e)));
        }
    }

    Ok(())
}
fn message_box(title: String, desc: String) {
    MessageDialog::new()
        .set_title(title)
        .set_description(desc)
        .set_buttons(rfd::MessageButtons::Ok)
        .show();
}

pub(crate) async fn lua_run_game(script_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let lua = get_custom_lua();

    let mut path: PathBuf = get_scripts_dir().expect("Scripts dir not found");

    if script_name.ends_with(".lua") {
        path.push(script_name);
    } else {
        path.push(format!("{}.lua", script_name));
    }

    let chunk = lua.load(
        files::get_file_content(path.to_string_lossy().to_string())
    );

    let res = chunk.eval_async::<Value>().await?;

    println!("[{script_name}] {res:?}");

    let _ = exit(&lua, Some(0));

    Ok(())
}
