// קובץ src/main.rs עבור טאורי גרסה 1.x
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// פונקציית main פשוטה לגרסה 1.x
fn main() {
    app_lib::run();
}
