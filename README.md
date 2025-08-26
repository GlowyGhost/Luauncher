<div align="center">

# Luauncher

Another Game Launcher I guess...

</div>

## Overview

A cross-platform game launcher using:
 * Flutter for frontend.
 * Rust for backend.
 * Lua for user written scripts.

**All contributions are welcome!** More info about contrinution [here](./docs/CONTRIBUTING.md).

Please note that this code is licensed under the [MIT License](./LICENSE).

## User Scripts

Luauncher has lua built in via [mlua](https://github.com/mlua-rs/mlua). There are even custom functions too! There is [examples](./examples) yet here is every custom function introduced by Luauncher.

```lua
sleep() --Sleeps for the duration in milliseconds.
openApp() --Opens the app name sent in by the function.
closeApp() --Closes the app name sent in by the function.
forceCloseApp() --Closes the app name sent in by the function.
isAppOpen() --Returns a bool if the app name sent in is open.
isWindowOpen() --Returns a bool if the window name is open.
openURL() --Opens the URL.
waitUntilWindowClose() --Waits until the window name is closed.
waitUntilAppClose() --Waits until the app name is closed.
waitUntilWindowOpen() --Waits for the amount of time and when the window opens, it continues. Returns true if it succeeded, false if it didn't.
waitUntilAppOpen() --Waits for the amount of time and when the app is opens, it continues. Returns true if it succeeded, false if it didn't.
exit() --Exits the lua script. You can exit with an error code.
runCommand() --Runs a command on the users terminal. The user must have dev mode on.
messageBox() --Makes a message box pop up with title and desc.
```

There's also some variables.

```lua
system --String thats says the users system (Windows, Linux, MacOS)
```
