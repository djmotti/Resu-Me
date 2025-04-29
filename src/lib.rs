// קובץ src/lib.rs עבור טאורי גרסה 2.x
#![allow(unused_imports)]

use tauri::{App, AppHandle, Manager};
use serde::Serialize;

#[cfg(desktop)]
mod desktop;

// יצירת האפליקציה עם טאורי 2.x
#[tauri::module]
fn init_app() -> App {
    tauri::Builder::default()
        .plugin(tauri_plugin_log::init())
        .setup(|app| {
            // לוגיקת אתחול כאן
            Ok(())
        })
        .build()
        .expect("Failed to build Tauri application")
}

// ניתן להגדיר עוד פונקציות ותכונות לפי הצורך

// נדרש אם משתמשים בתכונת מצב מקומי של טאורי
#[cfg(desktop)]
pub fn run() {
    init_app().run(|_, _| {});
}
