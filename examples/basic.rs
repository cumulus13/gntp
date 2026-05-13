// File: examples\basic.rs
// Author: Hadi Cahyadi <cumulus13@gmail.com>
// Date: 2026-05-13
// Description:
// License: MIT

// examples/basic.rs
// Basic GNTP notification example
//
// Run with: cargo run --example basic

use gntp::{GntpClient, NotificationType};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Basic GNTP Notification Example ===\n");

    // Create GNTP client
    let mut client = GntpClient::new("Basic Example App");

    // Define notification type
    let notification = NotificationType::new("basic")
        .with_display_name("Basic Notification")
        .with_enabled(true);

    // Register with Growl
    println!("Registering with Growl...");
    client.register(vec![notification])?;
    println!("✓ Registered successfully\n");

    // Send notification
    println!("Sending notification...");
    client.notify(
        "basic",
        "Hello from Rust!",
        "This is a basic GNTP notification without any icon.",
    )?;
    println!("✓ Notification sent\n");

    println!("✅ Example completed!");
    println!("\nYou should see a notification on your screen now.");

    Ok(())
}
