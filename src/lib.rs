// קובץ src/lib.rs מפושט עבור טאורי גרסה 2.x
#![allow(unused_imports)]

use tauri::{App, AppHandle, Manager};
use serde::Serialize;

// יצירת האפליקציה בצורה פשוטה יותר
pub fn init_app() -> tauri::App {
    tauri::Builder::default()
        .setup(|app| {
            // לוגיקת אתחול כאן
            Ok(())
        })
        .build()
        .expect("Failed to build Tauri application")
}

// פונקציית הפעלה בסיסית
pub fn run() {
    init_app().run(|_, _| {});
}
