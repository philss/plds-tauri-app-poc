#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::api::process::{Command, CommandEvent};

fn main() {
    let (mut rx, _child) = Command::new_sidecar("plds")
        .expect("should run the sidecar")
        .args(["server", "-p", "8099"])
        .spawn()
        .expect("Failed to spawn plds");

    tauri::async_runtime::spawn(async move {

        // let mut i = 0;
        while let Some(event) = rx.recv().await {
            if let CommandEvent::Stdout(line) = event {
                println!("got: {}", line);
                // i += 1;
                // if i == 4 {
                //   child.write("message from Rust\n".as_bytes()).unwrap();
                //   i = 0;
                // }
            }
        }
    });

    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
