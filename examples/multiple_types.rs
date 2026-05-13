// File: examples\multiple_types.rs
// Author: Hadi Cahyadi <cumulus13@gmail.com>
// Date: 2026-05-13
// Description:
// License: MIT

// examples/multiple_types.rs
// Example: Multiple notification types (info, warning, error)
//
// Run with: cargo run --example multiple_types

use gntp::{GntpClient, IconMode, NotificationType};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Multiple Notification Types Example ===\n");

    // Create GNTP client
    let mut client = GntpClient::new("Multi-Type App").with_icon_mode(IconMode::DataUrl);

    // Define multiple notification types
    let info = NotificationType::new("info")
        .with_display_name("Information")
        .with_enabled(true);

    let warning = NotificationType::new("warning")
        .with_display_name("Warning")
        .with_enabled(true);

    let error = NotificationType::new("error")
        .with_display_name("Error")
        .with_enabled(true);

    // Register all types at once
    println!("Registering notification types...");
    client.register(vec![info, warning, error])?;
    println!("✓ Registered 3 notification types\n");

    // Send different types of notifications
    println!("Sending info notification...");
    client.notify(
        "info",
        "System Information",
        "Application started successfully",
    )?;
    println!("✓ Info sent\n");

    std::thread::sleep(std::time::Duration::from_secs(2));

    println!("Sending warning notification...");
    client.notify("warning", "Low Disk Space", "Only 10% disk space remaining")?;
    println!("✓ Warning sent\n");

    std::thread::sleep(std::time::Duration::from_secs(2));

    println!("Sending error notification...");
    client.notify("error", "Critical Error", "Database connection failed")?;
    println!("✓ Error sent\n");

    println!("✅ Example completed!");
    println!("\nYou should see 3 different notifications on your screen.");

    Ok(())
}
