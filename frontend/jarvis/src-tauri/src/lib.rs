use tauri::{TitleBarStyle, Manager};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn insult(name: &str) -> String {
    format!("{} is a doofus", name)
}

pub fn run() {
  tauri::Builder::default()
    .setup(|app| {
      // Configure transparent window for macOS
      #[cfg(target_os = "macos")]
      {
        if let Some(window) = app.get_webview_window("main") {
          window.set_title_bar_style(TitleBarStyle::Transparent).ok();
          
          // Set window to fully transparent
          use cocoa::appkit::{NSColor, NSWindow};
          use cocoa::base::id;
          
          unsafe {
            let ns_window = window.ns_window().unwrap() as id;
            
            // Set background to fully transparent
            let transparent_color = NSColor::colorWithRed_green_blue_alpha_(
              cocoa::base::nil,
              0.0,
              0.0,
              0.0,
              0.0,
            );
            ns_window.setBackgroundColor_(transparent_color);
            
            // Make the window non-opaque (key for transparency)
            ns_window.setOpaque_(0);
          }
        }
      }
      Ok(())
    })
    .plugin(tauri_plugin_liquid_glass::init())
    .invoke_handler(tauri::generate_handler![greet, insult])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}