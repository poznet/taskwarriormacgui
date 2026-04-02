mod commands;
mod taskwarrior;

use tauri::{
    menu::{MenuBuilder, MenuItemBuilder},
    tray::TrayIconBuilder,
    Manager,
};

#[cfg(target_os = "macos")]
use cocoa::appkit::{NSColor, NSWindow};
#[cfg(target_os = "macos")]
use cocoa::base::{id, nil};
use tauri_plugin_autostart::ManagerExt;
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutState};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            None,
        ))
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_positioner::init())
        .plugin(tauri_plugin_notification::init())
        .invoke_handler(tauri::generate_handler![
            commands::get_tasks,
            commands::add_task,
            commands::complete_task,
            commands::uncomplete_task,
            commands::modify_task,
            commands::annotate_task,
            commands::delete_task,
            commands::get_projects,
            commands::get_tags,
            commands::check_taskwarrior,
        ])
        .setup(|app| {
            let window = app
                .get_webview_window("main")
                .expect("main window not found");

            // Make window background fully transparent (fixes black corners)
            #[cfg(target_os = "macos")]
            {
                let ns_window = window.ns_window().unwrap() as id;
                unsafe {
                    let clear = NSColor::clearColor(nil);
                    ns_window.setBackgroundColor_(clear);
                }
            }

            // --- Autostart: enable on first run ---
            let autostart = app.autolaunch();
            if !autostart.is_enabled().unwrap_or(false) {
                let _ = autostart.enable();
            }

            // --- Global shortcut: ⌥⌘T to toggle window ---
            let shortcut: Shortcut = "Alt+CommandOrControl+T".parse().unwrap();
            let win_for_shortcut = window.clone();
            app.global_shortcut().on_shortcut(shortcut, move |_app, _shortcut, event| {
                if event.state == ShortcutState::Pressed {
                    if win_for_shortcut.is_visible().unwrap_or(false) {
                        let _ = win_for_shortcut.hide();
                    } else {
                        let _ = win_for_shortcut.show();
                        let _ = win_for_shortcut.set_focus();
                    }
                }
            })?;

            // --- Tray icon with menu ---
            let show_hide = MenuItemBuilder::with_id("show_hide", "Pokaż/Ukryj")
                .build(app)?;
            let quit = MenuItemBuilder::with_id("quit", "Zamknij TaskFloat")
                .build(app)?;
            let menu = MenuBuilder::new(app)
                .item(&show_hide)
                .separator()
                .item(&quit)
                .build()?;

            let win_for_tray = window.clone();
            TrayIconBuilder::new()
                .icon(app.default_window_icon().cloned().unwrap())
                .tooltip("TaskFloat")
                .menu(&menu)
                .on_menu_event(move |_app, event| {
                    match event.id().as_ref() {
                        "show_hide" => {
                            if win_for_tray.is_visible().unwrap_or(false) {
                                let _ = win_for_tray.hide();
                            } else {
                                let _ = win_for_tray.show();
                                let _ = win_for_tray.set_focus();
                            }
                        }
                        "quit" => {
                            std::process::exit(0);
                        }
                        _ => {}
                    }
                })
                .on_tray_icon_event(|tray, event| {
                    if let tauri::tray::TrayIconEvent::Click { button: tauri::tray::MouseButton::Left, .. } = event {
                        if let Some(app) = tray.app_handle().get_webview_window("main") {
                            if app.is_visible().unwrap_or(false) {
                                let _ = app.hide();
                            } else {
                                let _ = app.show();
                                let _ = app.set_focus();
                            }
                        }
                    }
                })
                .build(app)?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running TaskFloat");
}
