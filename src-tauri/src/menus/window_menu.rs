use tauri::menu::{Menu, MenuItem, Submenu};
use tauri::{AppHandle, Error, Wry};

pub fn set_window_menu(handle: &AppHandle) -> Result<Menu<Wry>, Error> {
    // Créer le menu en gérant les erreurs potentiellement générées
    let menu = Menu::with_items(
        handle,
        &[
            &Submenu::with_items(
                handle,
                "File",
                true,
                &[
                    &MenuItem::new(handle, "Open", true, None::<&str>)?,
                    &MenuItem::new(handle, "Open recent", true, None::<&str>)?,
                    &MenuItem::new(handle, "Import", true, None::<&str>)?,
                    &MenuItem::new(handle, "Export", true, None::<&str>)?,
                    &MenuItem::new(handle, "Quit", true, None::<&str>)?,
                ],
            )?,
            &Submenu::with_items(
                handle,
                "Edit",
                true,
                &[&MenuItem::new(handle, "Parametres", true, None::<&str>)?],
            )?,
            &Submenu::with_items(
                handle,
                "Options",
                true,
                &[&MenuItem::new(handle, "A Propos", true, None::<&str>)?],
            )?,
            &Submenu::with_items(
                handle,
                "Help",
                true,
                &[
                    &MenuItem::new(handle, "Pause", true, None::<&str>)?,
                    &MenuItem::new(handle, "Filtres", true, None::<&str>)?,
                ],
            )?,
        ],
    )?;


    Ok(menu)
}
