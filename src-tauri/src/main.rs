// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

/*
use std::ptr::null_mut;
use std::io::Cursor;

use winapi::um::wingdi::*;
use winapi::um::winuser::*;
use winapi::um::winnt::*;
use winapi::shared::windef::*;
use winapi::shared::minwindef::*;

use image::{png::PngEncoder, ColorType};
use base64::{engine::general_purpose, Engine as _};
 */

use tauri::Manager;


mod utils;
mod lua_utils;
mod files;



#[tauri::command]
fn get_games() -> Vec<String> {
    files::list_scripts()
        .unwrap_or_else(|_| Vec::new())
}

#[tauri::command]
fn run_game(game_name: String) {
    let _ = lua_utils::lua_run_game(&game_name);
}

/*
        So this is a function that was writen to get the games ico file to render in flutter.
        This does work and gets an error so,
        Anyone who knows Rust fix my spagetti code 🙏
        All imports used for this should be in Cargo.toml and commented above.        -- GlowyGhost 25/7/25

#[tauri::command]
fn get_icon(_game_name: String) -> String {
    let exe_path = "C:/Program Files (x86)/Steam/steamapps/common/Balatro/Balatro.exe";
    let wide_path = utils::to_wide(exe_path);

    unsafe {
        let hicon = LoadImageW(
            null_mut(),
            wide_path.as_ptr(),
            IMAGE_ICON,
            256,
            256,
            LR_LOADFROMFILE,
        ) as HICON;

        if hicon.is_null() {
            eprintln!("Failed to load icon from EXE at {}", exe_path);
            return "".into();
        }

        let mut icon_info = std::mem::zeroed();
        if GetIconInfo(hicon, &mut icon_info) == 0 {
            eprintln!("GetIconInfo failed");
            return "".into();
        }

        let hdc = CreateCompatibleDC(null_mut());
        if hdc.is_null() {
            eprintln!("CreateCompatibleDC failed");
            return "".into();
        }

        let mut bi = BITMAPINFO {
            bmiHeader: BITMAPINFOHEADER {
                biSize: std::mem::size_of::<BITMAPINFOHEADER>() as DWORD,
                biWidth: 256,
                biHeight: -256,
                biPlanes: 1,
                biBitCount: 32,
                biCompression: BI_RGB,
                biSizeImage: (256 * 256 * 4) as DWORD,
                biXPelsPerMeter: 0,
                biYPelsPerMeter: 0,
                biClrUsed: 0,
                biClrImportant: 0,
            },
            bmiColors: [RGBQUAD {
                rgbBlue: 0,
                rgbGreen: 0,
                rgbRed: 0,
                rgbReserved: 0,
            }; 1],
        };

        let mut pixels = vec![0u8; 256 * 256 * 4];
        let old = SelectObject(hdc, icon_info.hbmColor as _);

        let res = GetDIBits(
            hdc,
            icon_info.hbmColor as HBITMAP,
            0,
            256,
            pixels.as_mut_ptr() as _,
            &mut bi,
            DIB_RGB_COLORS,
        );

        SelectObject(hdc, old);
        DeleteDC(hdc);
        DeleteObject(icon_info.hbmColor as _);
        DeleteObject(icon_info.hbmMask as _);

        if res == 0 {
            eprintln!("GetDIBits failed");
            return "".into();
        }

        let mut rgba_data = vec![0u8; 256 * 256 * 4];
        for i in 0..(256 * 256) {
            rgba_data[i * 4] = pixels[i * 4 + 2];     // R
            rgba_data[i * 4 + 1] = pixels[i * 4 + 1]; // G
            rgba_data[i * 4 + 2] = pixels[i * 4];     // B
            rgba_data[i * 4 + 3] = pixels[i * 4 + 3]; // A
        }

        let mut png_buf = Cursor::new(Vec::new());
        let encoder = PngEncoder::new(&mut png_buf);
        if let Err(e) = encoder.encode(&rgba_data, 256, 256, ColorType::Rgba8) {
            eprintln!("PNG encode failed: {}", e);
            return "".into();
        }

        let base64 = general_purpose::STANDARD.encode(png_buf.get_ref());
        base64
    }
} */

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            if let Some(window) = app.get_webview_window("launcher") {
                window.hide().unwrap();

                let mut args: Vec<String> = std::env::args().collect();

                if args.len() > 1 && !args[1].is_empty() {
                    args.remove(0);

                    let mut main_arg: String = "".to_owned();

                    for arg in args {
                        main_arg = main_arg + &arg + " ";
                    }

                    main_arg.pop();
                    run_game(main_arg);
                    std::process::exit(0);
                } else {
                    files::make_dirs();
                    window.show().unwrap();
                }
            } else {
                println!("No window labeled 'launcher' found.");
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_games, run_game])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
