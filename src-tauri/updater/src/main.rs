use crossterm::{
    event::{self, Event},
    terminal
};
use std::{
    io::{stdout, Write},
    thread,
    time::Duration,
    fs,
    path::Path,
    process::Command
};

fn presskeytoquit() -> Result<(), Box<dyn std::error::Error>> {
    println!("Press any key to exit...");
    stdout().flush()?;

    while event::poll(Duration::from_millis(0))? {
        let _ = event::read()?;
    }
    loop {
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(_) = event::read()? {
                break;
            }
        }
    }

    println!();
    Ok(())
}

fn delete_folder(dir: String) -> Result<(), Box<dyn std::error::Error>> {
    if !Path::new(&dir).exists() {
        return Err("Directory does not exist.".into());
    }

    return Ok(fs::remove_dir_all(&dir)?);
}
fn delete_file(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let file_path = Path::new(&path);
    if !file_path.exists() {
        return Err("File does not exist.".into());
    }
    if !is_executable(file_path) {
        return Err("File is not an executable.".into());
    }
    return Ok(fs::remove_file(file_path)?);
}
fn is_executable(path: &Path) -> bool {
    #[cfg(target_os = "windows")]
    {
        path.extension()
            .map_or(false, |ext| ext.eq_ignore_ascii_case("exe"))
    }

    #[cfg(not(target_os = "windows"))]
    {
        use std::os::unix::fs::PermissionsExt;
        match fs::metadata(path) {
            Ok(metadata) => {
                let permissions = metadata.permissions();
                permissions.mode() & 0o111 != 0
            }
            Err(_) => false,
        }
    }
}
fn close_luauncher(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut res;

    #[cfg(target_os = "windows")]
    {
        // Extract just the file name (e.g., "launcher.exe")
        let process_name = Path::new(&path)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or(&path);

        let output = Command::new("tasklist")
            .arg("/FI")
            .arg(format!("IMAGENAME eq {}", process_name))
            .output();

        let stdout = String::from_utf8_lossy(&output.unwrap().stdout).to_string();
        res = stdout.contains(process_name);
    }

    #[cfg(any(target_os = "linux", target_os = "macos"))]
    {
        let status = Command::new("pgrep")
            .arg("-f")
            .arg(&path)
            .status();

        res = status.success();
    }

    if res {
        #[cfg(target_os = "windows")]
        {
            let process_name = Path::new(&path)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or(&path);

            let _ = Command::new("taskkill")
                .args(["/IM", process_name])
                .output();
        }

        #[cfg(any(target_os = "linux", target_os = "macos"))]
        {
            let _ = Command::new("pkill")
                .arg(&path)
                .output();
        }
    }

    Ok(())
}
fn is_folder_unlocked(path: String) -> bool {
    let orig = Path::new(&path);
    let tmp = orig.with_extension("tmp_check_lock");

    // Try to rename the folder
    match fs::rename(orig, &tmp) {
        Ok(_) => {
            // Rename back to original
            let _ = fs::rename(&tmp, orig);
            true
        }
        Err(_) => false,
    }
}

fn update(old_path: String) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", old_path);

    presskeytoquit()
}

fn uninstall(old_path: String) -> Result<(), Box<dyn std::error::Error>> {
    println!("Final chance to exit.");
    stdout().flush()?;
    
    let mut seconds = 3;
    let mut elapsed_ms = 0;
    let tick_ms = 100;

    while event::poll(Duration::from_millis(0))? {
        let _ = event::read()?;
    }
    loop {
        if event::poll(Duration::from_millis(0))? {
            if let Event::Key(_) = event::read()? {
                return Ok(());
            }
        }

        if elapsed_ms % 1000 == 0 {
            print!("\rYou have {} seconds to press any key to exit.", seconds);
            stdout().flush()?;
            seconds -= 1;

            if seconds < 0 {
                println!();
                println!();
                break;
            }
        }

        thread::sleep(Duration::from_millis(tick_ms));
        elapsed_ms += tick_ms;
    }

    //Close Luauncher if running
    stdout().flush()?;
    print!("Closing Luauncher...");

    let _ = close_luauncher(old_path.clone());

    //Wait for Unlock
    print!("\rWaiting for Unlock...");
    stdout().flush()?;

    #[cfg(target_os = "windows")]
    let path_com = env!("LOCALAPPDATA").to_string() + "\\com.glowyghost.luauncher";

    #[cfg(target_os = "linux")]
    let path_com = std::env::var("HOME").unwrap_or_default() + "/.local/share/com.glowyghost.luauncher";

    #[cfg(target_os = "macos")]
    let path_com = std::env::var("HOME").unwrap_or_default() + "/Library/Application Support/com.glowyghost.luauncher";

    loop {
        if is_folder_unlocked(path_com.clone()) {
            break;
        }
        if !Path::new(&path_com).exists() {
            break;
        }

        thread::sleep(Duration::from_millis(1000));
    }

    print!("\rClosed Luauncher...         ");
    stdout().flush()?;
    println!();

    let mut errored = false;

    //com.glowyghost.luauncher deletion
    stdout().flush()?;
    println!();

    print!("Deleting WebView data...");

    let res_com = delete_folder(path_com.clone());

    match res_com {
        Ok(_) => {
            print!("\rDeleted WebView data...");
        }
        Err(e) => {
            if e.to_string().contains("Directory does not exist") {
                print!("\rDirectory \"{}\" does not exist. It's been deleted already.", path_com);
            } else {
                print!("\rFailed to delete folder \"{}\": {}", path_com, e);
                errored = true;
            }
        }
    }
    println!();

    //save data deletion
    stdout().flush()?;
    println!();

    print!("Deleting Save Data...");

    #[cfg(target_os = "windows")]
    let path_save = std::env::var("APPDATA").unwrap_or_default() + "\\Luauncher";

    #[cfg(target_os = "linux")]
    let path_save = std::env::var("HOME").unwrap_or_default() + "/.local/share/Luauncher";

    #[cfg(target_os = "macos")]
    let path_save = std::env::var("HOME").unwrap_or_default() + "/Library/Application Support/Luauncher";

    let res_save = delete_folder(path_save.clone());

    match res_save {
        Ok(_) => {
            print!("\rDeleted Save Data...");
        }
        Err(e) => {
            if e.to_string().contains("Directory does not exist") {
                print!("\rDirectory \"{}\" does not exist. It's been deleted already.", path_save);
            } else {
                print!("\rFailed to delete folder \"{}\": {}", path_save, e);
                errored = true;
            }
        }
    }
    println!();

    //save data deletion
    stdout().flush()?;
    println!();

    println!("{}", old_path);

    print!("Deleting Application...");

    let res_app = delete_file(old_path.clone());

    match res_app {
        Ok(_) => {
            print!("\rDeleted Application...");
        }
        Err(e) => {
            if e.to_string().contains("Directory does not exist") {
                print!("\rApplication doesn't exist. This should not happen. If you can reliably reproduce this issue, please report it at https://github.com/GlowyGhost/Luauncher/issues");
                errored = true;
            } else {
                print!("\rFailed to delete app: {}", e);
                errored = true;
            }
        }
    }

    println!();
    println!();
    stdout().flush()?;

    if errored {
        println!("Deletion Failed Somewhere.");
        println!("You may need to delete some files your self.");
    } else {
        println!("Deletion Successful.");
        println!("Thank you for using Luauncher!");
    }

    println!();

    presskeytoquit()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    terminal::enable_raw_mode()?;

    let args: Vec<String> = std::env::args().collect();

    if args.len() > 2 {
        let command = args[1].clone();
        let path = args[2].clone();

        match command.as_str() {
            "update" => update(path)?,
            "uninstall" => uninstall(path)?,
            _ => println!("Unknown command {}", command),
        }
    } else {
        println!("Meant to be ran like this:");
        println!("uninstall|update <path to Luauncher>");
    }

    terminal::disable_raw_mode()?;
    Ok(())
}
