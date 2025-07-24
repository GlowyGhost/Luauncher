# Contributing
## Requirements
To work on this project, you need to have installed [Flutter](https://docs.flutter.dev/get-started/install?_gl=1*h6bu5u*_ga*MTg5MDAyODE1OS4xNzUzMTgwMzIy*_ga_04YGWK0175*czE3NTMzNTExMjYkbzIkZzAkdDE3NTMzNTExMjYkajYwJGwwJGgw), [Rust](https://www.rust-lang.org/learn/get-started), and Tauri using `cargo install tauri-cli`. You may need to install tauri dependencies depending on your platform, there's info about this [here](https://v1.tauri.app/v1/guides/getting-started/prerequisites).

## Building
### Flutter
Everything for Flutter side of things is in the flutter sub-directory. All the files are in the lib/ sub-directory. To build, run `flutter build web` while being in the flutter sub-directory. This will build the app in build/web sub-directory. This will create loads of files to work, usually you'd only use `index.html`, `main.dart.js` and `assets/`. But when testing, use all the files before stripping it down. Put these files in src in the root, you make need to make the src folder.

### Tauri
All the files on the Tauri side will be in src-tauri sub-directory. src has the rust files that you should be editing. In the root of the project run `cargo tauri dev`, this will install and build Tauri dependencies in the project before building the actual project. After building it will run the app for you. This will be located in src-tauri/target.
