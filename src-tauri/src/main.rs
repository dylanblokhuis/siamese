#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::{path::{PathBuf}, str::FromStr};

use tauri::{Manager, WindowBuilder,WindowEvent, Window};
use vlc::{Media, MediaPlayer, Instance, EventType, State};
use windows::Win32::Foundation::HWND;
use tauri_plugin_shadows::Shadows;

fn setup_player(window: Window) -> (Media, MediaPlayer) {
  let hwnd = window.hwnd().expect("failed to get window `hwnd`");
  let instance = Instance::new().unwrap();
  let path = PathBuf::from_str(r"C:\Users\dylan\dev\something\src-tauri\video.mp4").unwrap();

  let md = Media::new_path(&instance, path).unwrap();
  let mdp = MediaPlayer::new(&instance).unwrap();
  mdp.set_media(&md);
  mdp.set_hwnd(hwnd);

  return (md, mdp);
}

fn main() {
  tauri::Builder::default()
    .create_window("player", tauri::WindowUrl::App("player.html".into()), |win, webview| {
      let win = win
        .title("Player")
        .resizable(false)
        .transparent(false)
        .decorations(false)
        // 16-9 since height doesnt work properly due to removed titlebar height not being calculated in frameless mode
        .inner_size(1333.0 , 720.0);

      return (win, webview);
    })
    .setup(|app| {
      let player_window = app.get_window("player").unwrap();
      player_window.set_shadow(true);

      let ui_window = app
        .create_window("ui".to_string(), tauri::WindowUrl::App("index.html".into()), |window_builder, webview_attributes| {
          (
            window_builder
              .title("ui")
              .owner_window(HWND(player_window.hwnd().unwrap() as _))
              .decorations(false)
              .transparent(true)
              .resizable(true)
              // https://github.com/tauri-apps/tao/issues/194
              .inner_size((player_window.inner_size().unwrap().width - 16).into(), (player_window.inner_size().unwrap().height - 39).into())
              .position(player_window.outer_position().unwrap().x.into() , player_window.outer_position().unwrap().y.into()),
            webview_attributes
          )
        })
        .unwrap();
      ui_window.set_shadow(false);
      
      let ui_window2 = ui_window.clone();
      let player_window2 = player_window.clone();
      ui_window.on_window_event(move |event| {
        match event {
          WindowEvent::Resized(size) => {
            let phys = tauri::Size::Physical(tauri::PhysicalSize {
              width: size.width - 16,
              height: size.height - 39
            });

            println!("width: {:?}, height: {:?}", size.width - 16, size.height - 39);

            player_window2.set_size(phys).unwrap();

            if ui_window2.is_maximized().unwrap() {
              player_window2.set_position(
                tauri::Position::Physical(tauri::PhysicalPosition {
                  x: 0,
                  y: 0
                })
              ).unwrap()
            }
          }
          WindowEvent::Moved(position) => {
            let pos = tauri::Position::Physical(tauri::PhysicalPosition {
              x: position.x,
              y: position.y
            });

            println!("x: {:?}, y: {:?}", position.x, position.y);

            player_window2.set_position(pos).unwrap();
          },
          _ => {
            println!("unknown event fired")
          }
        }
      });

      let (window_tx, window_rx) = std::sync::mpsc::channel::<String>();
      let (md, mdp) = setup_player(player_window);

      let tx = window_tx.clone();
      tauri::async_runtime::spawn(async move {
        let em = md.event_manager();
        em.attach(EventType::MediaStateChanged, move |e, _| {
            match e {
                vlc::Event::MediaStateChanged(s) => {
                    println!("State : {:?}", s);
                    if s == State::Ended || s == State::Error {
                      tx.send("Ended".to_string()).unwrap();
                    }
                },
                _ => (),
            }
        }).unwrap();

        loop {
          let message = window_rx.recv().unwrap();

          if message == "Play" {
            println!("Playing!!");
            mdp.play().unwrap();
          }

          if message == "Pause" {
            println!("Pausing!!");
            mdp.pause();
          }

          if message == "Ended" {
            println!("Video ended");
            break;
          }
        }
      });
     
      let tx = window_tx.clone();
      ui_window.listen("start", move |_| {
        tx.send("Play".into()).unwrap();
      });
      
      let tx = window_tx.clone();
      ui_window.listen("pause", move |_| {
        tx.send("Pause".into()).unwrap();
      });

      Ok(())
    })
    // .invoke_handler(tauri::generate_handler![my_custom_command])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
