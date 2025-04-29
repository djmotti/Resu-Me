// קובץ src/lib.rs עבור טאורי גרסה 2.x
#![allow(unused_imports)]

use tauri::{App, AppHandle, Manager};
use serde::Serialize;

#[cfg(feature = "desktop")]
mod desktop;

// עדכון רישום הפלאגין בטאורי 2.x
#[cfg_attr(mobile, tauri::android_app_lib)]
pub fn init_app() -> tauri::App {
    tauri::Builder::default()
        // עדכון רישום הפלאגין
        .plugin(tauri_plugin_log::Builder::default().build())
        .setup(|app| {
            // לוגיקת אתחול כאן
            Ok(())
        })
        // עדכון שיטת הבנייה בטאורי 2
        .build(tauri::generate_context!())
        .expect("Failed to build Tauri application")
}

// נדרש אם משתמשים בתכונת מצב שולחני
#[cfg(feature = "desktop")]
pub fn run() {
    init_app().run(|_, _| {});
}
