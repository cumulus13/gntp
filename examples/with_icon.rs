// examples/with_icon.rs
// GNTP notification with icon example
// Run with: cargo run --example with_icon

use gntp::{GntpClient, NotificationType, Resource, IconMode};
use std::env;
use std::path::PathBuf;

fn get_icon_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    if cfg!(debug_assertions) {
        // Development mode - look in project root
        Ok(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("growl.png"))
    } else {
        // Production mode - look near executable
        let exe_path = env::current_exe()?;
        let exe_dir = exe_path.parent()
            .ok_or_else(|| std::io::Error::new(
                std::io::ErrorKind::NotFound, 
                "Cannot get executable directory"
            ))?;
        
        let possible_paths = vec![
            exe_dir.join("growl.png"),
            exe_dir.join("icons").join("growl.png"),
            exe_dir.join("resources").join("growl.png"),
            env::current_dir()?.join("growl.png"),
            PathBuf::from("growl.png"),
        ];
        
        // Find first existing file
        for path in &possible_paths {
            if path.exists() {
                return Ok(path.clone());
            }
        }
        
        // Return error if not found
        Err(format!("Icon not found in any of: {:?}", possible_paths).into())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== GNTP With Icon Example ===\n");
    
    // Create GNTP client with DataUrl mode (best for icon display)
    let mut client = GntpClient::new("Icon Example App")
        .with_icon_mode(IconMode::DataUrl)
        .with_debug(true);  // Enable debug to see packet
    
    // Try to load icon
    println!("Loading icon...");
    let icon_path = get_icon_path()?;
    println!("Found icon at: {:?}", icon_path);
    
    let icon = match Resource::from_file(&icon_path) {
        Ok(icon) => {
            println!("✓ Icon loaded successfully ({} bytes)\n", icon.data.len());
            Some(icon)
        }
        Err(e) => {
            println!("✗ Failed to load icon: {}\n", e);
            println!("ℹ Continuing without icon...\n");
            None
        }
    };
    
    // Define notification type
    // IMPORTANT: Only attach icon HERE (not to client or options)
    let mut notification = NotificationType::new("alert")
        .with_display_name("Alert Notification")
        .with_enabled(true);
    
    if let Some(icon) = icon {
        notification = notification.with_icon(icon);
        println!("✓ Icon attached to notification type");
    }
    
    // Register
    println!("Registering with Growl...");
    client.register(vec![notification])?;
    println!("✓ Registered successfully\n");
    
    // Send notification WITHOUT icon in options
    // (icon already in notification type)
    println!("Sending notification...");
    client.notify(
        "alert",
        "Alert with Icon!",
        "This notification includes an icon from the notification type"
    )?;
    println!("✓ Notification sent\n");
    
    println!("✅ Example completed!");
    println!("\nNote: To test with a custom icon:");
    println!("  1. Place 'growl.png' in the project root directory");
    println!("  2. Run: cargo run --example with_icon");
    println!("\nIcon delivery mode: DataUrl (base64 embedded)");
    println!("This works best across all platforms including Android.");
    
    Ok(())
}