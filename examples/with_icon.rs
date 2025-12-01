// examples/with_icon.rs
// GNTP notification with icon example
// Run with: cargo run --example with_icon

use gntp::{GntpClient, NotificationType, NotifyOptions, Resource};
use std::env;
use std::path::PathBuf;
use std::io;

fn get_icon_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    if cfg!(debug_assertions) {
        // Development mode
        Ok(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("mkfile.jpg"))
    } else {
        // Production mode
        let exe_path = env::current_exe()?;
        let exe_dir = exe_path.parent()
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Cannot get executable directory"))?;
        
        let possible_paths = vec![
            exe_dir.join("mkfile.jpg"),
            exe_dir.join("icons").join("mkfile.jpg"),
            exe_dir.join("resources").join("mkfile.jpg"),
            env::current_dir()?.join("mkfile.jpg"),
            PathBuf::from("mkfile.jpg"),
        ];
        
        // Find first existing file
        for path in possible_paths {
            if path.exists() {
                return Ok(path);
            }
        }
        
        // Return default even if not exists
        Ok(PathBuf::from("mkfile.jpg"))
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== GNTP With Icon Example ===\n");
    
    // Create GNTP client
    let mut client = GntpClient::new("Icon Example App");
    
    // Try to load application icon (optional)
    println!("Loading application icon...");
    let icon_path = get_icon_path()?; // <- Gunakan ? untuk unwrap atau return error
    println!("Looking for icon at: {:?}", icon_path);
    
    // Coba load icon dari file
    match Resource::from_file(&icon_path) {
        Ok(app_icon) => {
            client = client.with_icon(app_icon);
            println!("✓ Application icon loaded from {:?}", icon_path);
        }
        Err(e) => {
            println!("✗ Failed to load icon: {}", e);
            println!("ℹ No application icon found (optional - will work without it)");
        }
    }
    
    // Try to load notification icon (optional)
    println!("\nLoading notification icon...");
    let notif_icon = match Resource::from_file("notification.png") {
        Ok(icon) => {
            println!("✓ Notification icon loaded from notification.png");
            Some(icon)
        }
        Err(e) => {
            println!("ℹ No notification icon found: {} (optional)", e);
            None
        }
    };
    
    // Define notification type
    let mut notification = NotificationType::new("alert")
        .with_display_name("Alert Notification")
        .with_enabled(true);
    
    // Add icon to notification type if available
    if let Some(ref icon) = notif_icon {
        notification = notification.with_icon(icon.clone());
        println!("✓ Icon attached to notification type");
    }
    
    // Register
    println!("\nRegistering with Growl...");
    client.register(vec![notification])?;
    println!("✓ Registered successfully");
    
    // Send notification with icon in options
    println!("\nSending notification with icon...");
    let mut options = NotifyOptions::new();
    if let Some(ref icon) = notif_icon {
        options = options.with_icon(icon.clone());
    }
    
    client.notify_with_options(
        "alert",
        "Alert with Icon!",
        "This notification includes an icon",
        options
    )?;
    println!("✓ Notification sent");
    
    println!("\n✅ Example completed!");
    println!("\nNote: To test with icons:");
    println!("  1. Put mkfile.jpg in the same directory as this example");
    println!("  2. Put notification.png in the same directory");
    println!("  3. Run: cargo run --example with_icon");
    
    Ok(())
}