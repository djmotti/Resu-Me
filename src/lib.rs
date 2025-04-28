use tauri::{Builder, Manager};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  let context = tauri::generate_context!();
  
  Builder::default()
    .plugin(tauri_plugin_log::Builder::default().build())
    .setup(|app| {
      // קבלת הפניות לחלונות המוגדרים בקובץ התצורה
      let splash = app.get_webview_window("splashscreen").unwrap();
      let main = app.get_webview_window("main").unwrap();
      
      // לוגיקת הצגת החלון הראשי לאחר ההמתנה
      std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_secs(3));
        splash.close().unwrap();
        main.show().unwrap();
      });
      
      Ok(())
    })
    .build(context)
    .expect("error while building tauri application")
    .run(|_, _| {})
}