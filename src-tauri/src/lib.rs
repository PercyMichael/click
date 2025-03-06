use chrono::prelude::*;
use device_query::{DeviceEvents, DeviceEventsHandler, MouseButton};
use std::thread;
use std::time::Duration;

#[tauri::command]
fn greet() -> String {
    format!("Hello, Percy!")
}

#[tauri::command]
fn greet_and_date() -> String {
    // Get the current date and time
    let current_date = Local::now();
    // Format the greeting and current date as a string
    let greeting = "Hello, Percy!";
    let date = current_date.format("%Y-%m-%d %H:%M:%S").to_string();
    // Combine both greeting and date in one message
    format!("{}, Current date and time: {}", greeting, date)
}

#[tauri::command]
fn track_mouse_events() {
     // Initialize the device events handler with a polling interval of 0ms
    let event_handler = DeviceEventsHandler::new(Duration::from_millis(0))
        .expect("Could not initialize event loop");

    // Register a key down event callback
    // The guard is used to keep the callback alive
    let _key_down_guard = device_events.on_key_down(|key: &Keycode| {
        println!("Key down: {:?}", key);
    });

    // Register a mouse button up event callback
    let _mouse_button_guard = event_handler.on_mouse_up(|button: &MouseButton| {
        println!("Mouse button {:?} was released", button);
    });

    // Keep the main thread alive to listen for events
    loop {
        // thread::sleep(Duration::from_secs(5));
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
    .setup(|_app| {
        println!("App started successfully!");

            // Spawn the mouse event tracking in a new thread
            thread::spawn(track_mouse_events);

        Ok(())
    })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, greet_and_date]) // Add both functions here
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
