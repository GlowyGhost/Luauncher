fn main() {
    cc::Build::new()
        .file("csrc/lua_calls.c")
        .compile("lua");

    tauri_build::build()
}
