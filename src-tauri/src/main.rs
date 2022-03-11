#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::{Arc, Mutex};

use crate::player::Player;
use serde::Serialize;
use tauri::{Manager, State, Window};
use tauri_plugin_shadows::Shadows;

mod player;

struct AppState {
    player: Arc<Mutex<Player>>,
}

#[derive(Serialize, Clone, Copy, Debug, PartialEq)]
struct OnPlayerUpdatePayload {
    pub time: i64,
    duration: i64,
    visible: bool,
    paused: bool,
    volume: i64,
}

#[tauri::command(async)]
fn start(window: Window, state: State<AppState>) {
    let mut wid = 0;
    #[cfg(target_os = "windows")]
    {
        wid = window.hwnd().unwrap() as i64;
    }

    let clone = state.player.clone();
    let mut player = clone.lock().unwrap();
    player.attach(wid);
    player.load_file(r"C:\Users\dylan\Downloads\video.mkv");
    player.play();
    player.visible = true;
    std::mem::drop(player);

    let clone2 = state.player.clone();

    std::thread::spawn(move || {
        let mut last_sent_event: Option<OnPlayerUpdatePayload> = None;
        loop {
            let mut player = clone2.lock().unwrap();
            player.update();

            let to_send = OnPlayerUpdatePayload {
                time: player.time,
                duration: player.duration,
                visible: player.visible,
                paused: player.paused,
                volume: player.volume,
            };

            if last_sent_event.is_none() {
                last_sent_event = Some(to_send);
            }

            if last_sent_event.unwrap() != to_send {
                last_sent_event = Some(to_send);
                window.emit("on-player-update", to_send).unwrap();
            }

            if !player.visible {
                break;
            }
        }
    });
}

#[tauri::command(async)]
fn toggle_playing(state: State<AppState>) {
    let clone = state.player.clone();
    let player = clone.lock().unwrap();
    if player.paused {
        player.play();
    } else {
        player.pause();
    }
}

#[tauri::command(async)]
fn stop(state: State<AppState>) {
    let clone = state.player.clone();
    let player = clone.lock().unwrap();
    player.stop();
}

#[tauri::command(async)]
fn seek(state: State<AppState>, time_pos: i64) {
    let clone = state.player.clone();
    let player = clone.lock().unwrap();

    player.set_time_position(time_pos);
}

#[tauri::command(async)]
fn set_volume(state: State<'_, AppState>, volume: i64) {
    let clone = state.player.clone();
    let player = clone.lock().unwrap();

    player.set_volume(volume);
}

fn main() {
    tauri::Builder::default()
        .manage(AppState {
            player: Arc::new(Mutex::new(Player::new())),
        })
        .setup(|app| {
            let window = &app.get_window("main").unwrap();
            window.set_shadow(true);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            start,
            toggle_playing,
            stop,
            seek,
            set_volume
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
