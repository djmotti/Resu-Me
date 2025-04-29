// קובץ src/main.rs עבור טאורי גרסה 2.x
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    #[cfg(feature = "desktop")]
    app_lib::run();
}
