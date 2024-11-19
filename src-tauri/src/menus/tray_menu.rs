use tauri::menu::{MenuBuilder, MenuItemBuilder};
use tauri::tray::TrayIconBuilder;
use tauri::Manager;

pub fn set_tray_menu(app: &tauri::AppHandle) {
    let quit = MenuItemBuilder::new("Quit").id("quit").build(app).unwrap();
    let hide = MenuItemBuilder::new("Hide").id("hide").build(app).unwrap();
    let show = MenuItemBuilder::new("Show").id("show").build(app).unwrap();

    let tray_menu = MenuBuilder::new(app)
        .items(&[&quit, &hide, &show])
        .build()
        .unwrap();

    let _tray_menue_api = TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&tray_menu)
        .on_menu_event(|app, event| match event.id().as_ref() {
            "quit" => app.exit(0),
            "hide" => {
                dbg!("menu item hide clicked");
                let window = app.get_webview_window("main").unwrap();
                window.hide().unwrap();
            }
            "show" => {
                dbg!("menu item show clicked");
                let window = app.get_webview_window("main").unwrap();
                window.show().unwrap();
            }
            _ => {}
        })
        .build(app);
}
