use mlua::{Lua, Value};
use std::process::Command;
use std::path::PathBuf;

use crate::files::{self, get_scripts_dir};

fn get_custom_lua() -> Lua {
    let lua = Lua::new();

    let globals = lua.globals();
    globals.set("openApp", lua.create_function(open_app).unwrap()).unwrap();

    lua
}

fn open_app(_lua: &Lua, arg: String) -> mlua::Result<()> {
    #[cfg(target_os = "windows")]
    {
        Command::new("cmd")
            .args(["/C", "start", "", &arg])
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

pub(crate) fn lua_run_game(script_name: &str) -> Result<(), Box<dyn std::error::Error>> {
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

    let res = chunk.eval::<Value>()?;

    println!("[{script_name}] {res:?}");

    Ok(())
}
