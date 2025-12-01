// examples/with_icon.rs
// GNTP notification with icon example
// Run with: cargo run --example with_icon

use gntp::{GntpClient, NotificationType, NotifyOptions, Resource};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== GNTP With Icon Example ===\n");
    
    // Create GNTP client
    let mut client = GntpClient::new("Icon Example App");
    
    // Try to load application icon (optional)
    println!("Loading application icon...");
    if let Ok(app_icon) = Resource::from_file("mkfile.jpg") {
        client = client.with_icon(app_icon);
        println!("✓ Application icon loaded from mkfile.jpg");
    } else {
        println!("ℹ No application icon found (optional - will work without it)");
    }
    
    // Try to load notification icon (optional)
    println!("Loading notification icon...");
    let notif_icon = Resource::from_file("notification.png").ok();
    if notif_icon.is_some() {
        println!("✓ Notification icon loaded from notification.png");
    } else {
        println!("ℹ No notification icon found (optional)");
    }
    
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
    if let Some(icon) = notif_icon {
        options = options.with_icon(icon);
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
    println!("  1. Put mkfile.jpg in the same directory");
    println!("  2. Put notification.png in the same directory");
    println!("  3. Run: cargo run --example with_icon");
    
    Ok(())
}