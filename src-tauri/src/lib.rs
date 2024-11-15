use std::sync::atomic::AtomicBool;

use tauri_plugin_notification;

#[tauri::command]
fn start_fibonacci(app_handle: tauri::AppHandle) {
    static START: AtomicBool = AtomicBool::new(true);

    if START.load(std::sync::atomic::Ordering::Relaxed) {
        START.swap(false, std::sync::atomic::Ordering::Relaxed);
        std::thread::spawn(move || {
            let mut a = 0;
            let mut b = 1;

            loop {
                use tauri_plugin_notification::NotificationExt;

                app_handle
                    .notification()
                    .builder()
                    .title("Fibonacci sequence")
                    .body(format!("Number: {}", a))
                    .ongoing()
                    .show()
                    .expect("Failed to show notification");

                let next = a + b;
                a = b;
                b = next;

                // Delay to avoid spamming
                std::thread::sleep(std::time::Duration::from_secs(5));
            }
        });
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![start_fibonacci])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
