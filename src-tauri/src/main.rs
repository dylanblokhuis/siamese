#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::{sync::{Mutex, Arc}};

use serde::Serialize;
use tauri::{Manager, State, Window};
use tauri_plugin_shadows::Shadows;
use crate::player::Player;

mod player;

struct AppState {
  player: Arc<Mutex<Player>>
}

#[derive(Serialize, Clone, Copy)]
struct OnPlayerUpdatePayload {
  pub time: i64,
  duration: i64,
  visible: bool
}

#[tauri::command]
fn start(window: Window, state: State<AppState>) {
  let mut wid = 0;
  #[cfg(target_os = "windows")] {
    wid = window.hwnd().unwrap() as i64;
  }

  let clone = state.player.clone();
  let mut player = clone.lock().unwrap();
  player.attach(wid);
  player.load_file(r"https://ia600200.us.archive.org/7/items/Sintel/sintel-2048-surround_512kb.mp4");
  // player.play();
  player.pause();
  player.visible = true;

  let clone2 = state.player.clone();

  std::thread::spawn(move || {
    let mut last_sent_event: Option<OnPlayerUpdatePayload> = None; 
    loop {
      let mut player = clone2.lock().unwrap();
      player.update();

      let to_send = OnPlayerUpdatePayload { 
        time: player.time,
        duration: player.duration,
        visible: player.visible
      };

      if let None = last_sent_event {
        last_sent_event = Some(to_send.clone());
      }

      if last_sent_event.unwrap().time != to_send.time {
        last_sent_event = Some(to_send.clone());
        window.emit("on-player-update", to_send).unwrap();
      }

      if player.visible == false {
        break;
      }
    }
  });
}

#[tauri::command]
fn play(state: State<AppState>) {
  let clone = state.player.clone();
  let player = clone.lock().unwrap();
  player.play();
}

#[tauri::command]
fn pause(state: State<AppState>) {
  let clone = state.player.clone();
  let player = clone.lock().unwrap();
  player.pause();
}

#[tauri::command]
fn stop(state: State<AppState>) {
  let clone = state.player.clone();
  let player = clone.lock().unwrap();
  player.stop();
}

fn main() {  
  tauri::Builder::default()
    .manage(AppState {
      player: Arc::new(Mutex::new(Player::new()))
    })
    .setup(|app| {
      let window = &app.get_window("main").unwrap();
      window.set_shadow(true);

      Ok(())
    })
    .invoke_handler(tauri::generate_handler![start, pause, play, stop])    
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
