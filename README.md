# mpv-player running with a webview ui

## How to setup development on Windows

To run the app you need to set the MPV_DIR variable, see `src-tauri/.env.example` on how you can create a suitable `.env` file.

### 1. Download mpv player from SourceForge

https://sourceforge.net/projects/mpv-player-windows/files/libmpv/

### 2. Extract downloaded contents to a folder

### 3. Point `MPV_DIR` to the folder e.g. `C:\\Users\\user\\Downloads\\mpv`

### 4. Run `cargo build` inside `src-tauri`

### 5. Copy `mpv-2.dll` to `src-tauri/target/debug` and rename it to `mpv.dll`

If you skip this step you will get

```
error: process didn't exit successfully: `target\debug\app.exe` (exit code: 0xc0000135, STATUS_DLL_NOT_FOUND)
```

### 6. Install node_modules with `yarn` or `npm`
