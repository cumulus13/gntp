// examples/remote.rs
// Example: Send notifications to remote Growl server
//
// Run with: cargo run --example remote

use gntp::{GntpClient, IconMode, NotificationType};
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Remote Growl Server Example ===\n");

    // Get remote host from environment or use default
    let remote_host = env::var("GROWL_HOST").unwrap_or_else(|_| "192.168.1.100".to_string());

    let remote_port = env::var("GROWL_PORT")
        .unwrap_or_else(|_| "23053".to_string())
        .parse::<u16>()
        .unwrap_or(23053);

    println!("Target: {}:{}", remote_host, remote_port);
    println!("(Set GROWL_HOST and GROWL_PORT environment variables to change)\n");

    // Create client for remote server
    let mut client = GntpClient::new("Remote Example")
        .with_host(&remote_host)
        .with_port(remote_port)
        .with_icon_mode(IconMode::DataUrl); // Best for remote/Android

    // Define notification type
    let notification = NotificationType::new("remote").with_display_name("Remote Notification");

    // Register
    println!("Registering with remote Growl...");
    match client.register(vec![notification]) {
        Ok(_) => println!("✓ Registered successfully\n"),
        Err(e) => {
            eprintln!("❌ Registration failed: {}", e);
            eprintln!("\nTroubleshooting:");
            eprintln!("  1. Is Growl running on {}?", remote_host);
            eprintln!("  2. Is port {} open in firewall?", remote_port);
            eprintln!("  3. Is remote host reachable? (ping {})", remote_host);
            return Err(e.into());
        }
    }

    // Send notification
    println!("Sending notification...");
    client.notify(
        "remote",
        "Hello from Remote!",
        "This notification was sent over the network",
    )?;
    println!("✓ Notification sent\n");

    println!("✅ Example completed!");

    Ok(())
}
