// קובץ src/lib.rs עבור טאורי גרסה 1.x
#![allow(unused_imports)]

use tauri::{App, AppHandle, Manager};
use serde::Serialize;

// פונקציה לאתחול האפליקציה
pub fn init_app() -> tauri::App {
    tauri::Builder::default()
        .setup(|app| {
            // לוגיקת אתחול כאן
            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("Failed to build Tauri application")
}

// פונקציית הפעלה
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            // לוגיקת אתחול כאן
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("Error while running tauri application");
}
