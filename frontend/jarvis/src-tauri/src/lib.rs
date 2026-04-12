use tauri::{TitleBarStyle, Manager, Size, Position};


#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn start(app: tauri::AppHandle) {
    // Get the main window
    if let Some(window) = app.get_webview_window("main") {
        // Hide decorations (title bar) before animation starts
        let _ = window.set_decorations(false);
        
        // Animate window collapse over 1.2 seconds with 30 steps
        let steps = 30;
        let duration_ms = 1200; // 1.2 seconds
        let step_duration = duration_ms / steps;
        
        let original_height = 600.0;
        let target_height = 50.0;
        let original_y = 200.0; // Approximate starting position
        let target_y = 0.0;
        
        for i in 0..=steps {
            let progress = i as f64 / steps as f64;
            
            // Smooth easing function (ease-out)
            let eased = 1.0 - (1.0 - progress).powi(3);
            
            // Calculate current height and position
            let current_height = original_height - ((original_height - target_height) * eased);
            let current_y = original_y - ((original_y - target_y) * eased);
            
            // Resize window
            let _ = window.set_size(Size::Logical(tauri::LogicalSize {
                width: 850.0,
                height: current_height,
            }));
            
            // Move window upward
            let _ = window.set_position(Position::Logical(tauri::LogicalPosition {
                x: 325.0,
                y: current_y,
            }));
            
            // Wait before next step (except on last step)
            if i < steps {
                std::thread::sleep(std::time::Duration::from_millis(step_duration as u64));
            }
        }
        
        // Ensure window is at y = 0 when fully collapsed
        let _ = window.set_position(Position::Logical(tauri::LogicalPosition {
            x: 325.0,
            y: 0.0,
        }));
    }
}

#[tauri::command]
async fn expand(app: tauri::AppHandle) {
    // Get the main window
    if let Some(window) = app.get_webview_window("main") {
        // Restore decorations (title bar) before animation starts
        let _ = window.set_decorations(true);
        
        // Animate window expansion over 1.2 seconds with 30 steps
        let steps = 30;
        let duration_ms = 1200; // 1.2 seconds
        let step_duration = duration_ms / steps;
        
        let original_height = 50.0;
        let target_height = 600.0;
        let original_y = 0.0; // Current collapsed position
        let target_y = 200.0; // Return to original position
        
        for i in 0..=steps {
            let progress = i as f64 / steps as f64;
            
            // Smooth easing function (ease-out)
            let eased = 1.0 - (1.0 - progress).powi(3);
            
            // Calculate current height and position
            let current_height = original_height + ((target_height - original_height) * eased);
            let current_y = original_y + (target_y - original_y) * eased;
            
            // Resize window
            let _ = window.set_size(Size::Logical(tauri::LogicalSize {
                width: 800.0,
                height: current_height,
            }));
            
            // Move window back down
            let _ = window.set_position(Position::Logical(tauri::LogicalPosition {
                x: 325.0,
                y: current_y,
            }));
            
            // Wait before next step (except on last step)
            if i < steps {
                std::thread::sleep(std::time::Duration::from_millis(step_duration as u64));
            }
        }
    }
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
            ns_window.setHasShadow_(0);
          }
        }
      }
      Ok(())
    })
    .plugin(tauri_plugin_liquid_glass::init())
    .invoke_handler(tauri::generate_handler![greet, start, expand])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}