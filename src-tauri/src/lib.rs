use chrono::prelude::*;

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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, greet_and_date]) // Add both functions here
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
