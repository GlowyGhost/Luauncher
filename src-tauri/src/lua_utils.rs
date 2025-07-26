use mlua::{Lua, Value};
use std::process::Command;

use crate::files;

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

    let mut path: String = "______/".to_owned();

    if script_name.contains(".") {
        path = path + script_name;
    } else {
        path = path + script_name + ".lua";
    }

    let chunk = lua.load(
        files::get_file_content(path)
    );

    let res = chunk.eval::<Value>()?;

    println!("[{script_name}] {res:?}");

    Ok(())
}
