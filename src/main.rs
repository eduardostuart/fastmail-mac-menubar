#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#[cfg(target_os = "macos")]
#[macro_use]
extern crate objc;

use tauri::{CustomMenuItem, SystemTrayMenu, ActivationPolicy};
use tauri::{Manager, SystemTray};


fn main() {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let tray_menu = SystemTrayMenu::new().add_item(quit);

    let tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .setup(|app| {
            // Hide app from dock
            app.set_activation_policy(ActivationPolicy::Accessory);
            // Webview properties
            let main_window = app.get_window("main").unwrap();
            main_window.set_resizable(false)?;
            main_window.with_webview(|webview| unsafe {
                let () = msg_send![webview.inner(), setPageZoom: 1.];
                let () = msg_send![webview.controller(), removeAllUserScripts];
                let bg_color: cocoa::base::id =
                    msg_send![class!(NSColor), colorWithDeviceRed:0.5 green:0.2 blue:0.4 alpha:1.];
                let () = msg_send![webview.ns_window(), setBackgroundColor: bg_color];
            })?;
            Ok(())
        })
        .system_tray(tray)
        .on_system_tray_event(|app, event| match event {
            tauri::SystemTrayEvent::MenuItemClick {  id, .. } => {
                match id.as_str() {
                    "quit" => app.exit(exitcode::OK),
                    _ => {
                        //
                    }
                }
            }
            tauri::SystemTrayEvent::LeftClick {
                position,
                ..
            } => {
                if let Some(window) = app.get_window("main") {
                    window.set_position(position).unwrap();
                    if window.is_visible().unwrap() {
                        window.hide().unwrap();
                    } else {
                        window.show().unwrap();
                        window.set_focus().unwrap();
                    }
                }
            }
            _ => {
                // Do nothing
            }
        })
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|_app_handle, event| match event {
            tauri::RunEvent::ExitRequested { api, .. } => {
                api.prevent_exit();
            }
            _ => {}
        });
}
