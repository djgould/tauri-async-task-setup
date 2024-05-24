use std::sync::atomic::{AtomicBool, Ordering};
use tauri::{Manager, State};

// Tauri command to greet with a name.
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// Define the app state struct
struct AppState {
    value: AtomicBool,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            // Initialize app state
            let app_state = AppState {
                value: AtomicBool::new(false),
            };

            // Manage state in Tauri
            app.manage(app_state);

            // Capture app handle for async task
            let app_handle = app.handle().clone();

            // Spawn the async task
            tauri::async_runtime::spawn(async move {
                // Simulate an async task delay
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

                // Access the app state safely in async context
                let app_state: State<'_, AppState> = app_handle.state();
                println!("State value: {:?}", app_state.value.load(Ordering::Relaxed));

                // Example of state mutation
                app_state.value.store(true, Ordering::Relaxed);
                println!(
                    "State value after mutation: {:?}",
                    app_state.value.load(Ordering::Relaxed)
                );
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
