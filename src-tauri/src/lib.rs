mod commandes;
use commandes::get_interfaces;
mod menus;
use log::info;
use menus::{tray_menu::set_tray_menu, window_menu::set_window_menu};
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        //
        // liste des plugins
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        //
        // Actions au lancement
        .setup(|app| {
            get_os();
            set_tray_menu(app.app_handle());
            Ok(())
        })
        //
        // Menu
        .menu(|handle| 
            set_window_menu(handle)
        )
        //
        // Commandes
        .invoke_handler(tauri::generate_handler![get_interfaces])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn get_os() {
    let platform = tauri_plugin_os::platform();
    info!("Platform: {}", platform);
}
