mod api;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![api::check_breach])
        .run(tauri::generate_context!())
        .expect("error while running Tauri application");
}
