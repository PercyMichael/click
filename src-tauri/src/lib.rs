use chrono::prelude::*;
use device_query::{DeviceEvents, DeviceEventsHandler, MouseButton};
use std::thread;
use std::time::Duration;
use tauri::Emitter;


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
fn track_mouse_events(app: tauri::AppHandle) {
    // Initialize the device events handler with a polling interval of 0ms
    let event_handler = DeviceEventsHandler::new(Duration::from_millis(0))
        .expect("Could not initialize event loop");  

    // Register a mouse button up event callback
    let _mouse_button_guard = event_handler.on_mouse_up(move |button: &MouseButton| {
        println!("Mouse button {:?} was released", button);

        // Get the current time
        let current_time = Local::now();
        let time_string = current_time.format("%Y-%m-%d %H:%M:%S").to_string();

        // Emit a greeting after mouse click, including the button information and time
        app.emit("greet-on-mouse-click", format!("Mouse button {:?} was clicked at {}", button, time_string)).unwrap();
    });

    // Keep the main thread alive to listen for events
    loop {
        // thread::sleep(Duration::from_secs(5));
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
    .setup(|app| {
        println!("App started successfully!");

        // Get a handle to the app and clone it
        let app_handle = app.handle(); // Get the app handle
        let app_handle_clone = app_handle.clone(); // Clone the app handle

        // Spawn the mouse event tracking in a new thread, moving app into the closure
        thread::spawn(move || track_mouse_events(app_handle_clone)); // Pass the cloned app handle

        Ok(())
    })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, greet_and_date]) // Add both functions here
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
