// examples/android.rs
// Example: Send notifications to Growl for Android with retry
//
// Run with: cargo run --example android

use gntp::{GntpClient, IconMode, NotificationType, NotifyOptions, Resource};
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Growl for Android Example ===\n");

    // Get Android device IP from environment
    let android_host = env::var("ANDROID_HOST").unwrap_or_else(|_| {
        println!("⚠ ANDROID_HOST not set, using default");
        println!("  Set with: set ANDROID_HOST=192.168.1.100\n");
        "192.168.1.100".to_string()
    });

    println!("Target Android device: {}", android_host);

    // Create client optimized for Android
    let mut client = GntpClient::new("Android Example")
        .with_host(&android_host)
        .with_port(23053)
        .with_icon_mode(IconMode::DataUrl) // Best for Android
        .with_debug(false);

    // Try to load icon (optional)
    let icon = match Resource::from_file("icon.png") {
        Ok(icon) => {
            println!("✓ Icon loaded: icon.png");
            Some(icon)
        }
        Err(_) => {
            println!("ℹ No icon found (optional)");
            None
        }
    };

    // Define notification type with icon
    let mut notification =
        NotificationType::new("android").with_display_name("Android Notification");

    if let Some(icon) = icon {
        notification = notification.with_icon(icon);
        println!("✓ Icon attached to notification");
    }

    println!();

    // Register with retry (Android may need retry due to network)
    println!("Registering with Growl for Android...");
    let mut register_ok = false;

    for attempt in 1..=3 {
        match client.register(vec![notification.clone()]) {
            Ok(_) => {
                println!(
                    "✓ Registered successfully{}\n",
                    if attempt > 1 {
                        format!(" (attempt {})", attempt)
                    } else {
                        String::new()
                    }
                );
                register_ok = true;
                break;
            }
            Err(e) => {
                if attempt < 3 {
                    println!("⚠ Attempt {} failed, retrying... ({})", attempt, e);
                    std::thread::sleep(std::time::Duration::from_secs(2));
                } else {
                    eprintln!("❌ Registration failed after 3 attempts: {}", e);
                    eprintln!("\nTroubleshooting:");
                    eprintln!("  1. Is Growl for Android running?");
                    eprintln!("  2. Is {} the correct IP address?", android_host);
                    eprintln!("  3. Are both devices on the same network?");
                    eprintln!("  4. Check Android firewall settings");
                    return Err(e.into());
                }
            }
        }
    }

    if !register_ok {
        return Err("Registration failed".into());
    }

    // Send notification with options
    println!("Sending notification...");
    let options = NotifyOptions::new()
        .with_sticky(false) // Don't make it sticky on mobile
        .with_priority(1); // High priority

    match client.notify_with_options(
        "android",
        "Hello Android!",
        "This notification was sent from Rust",
        options,
    ) {
        Ok(_) => {
            println!("✓ Notification sent\n");
            println!("✅ Check your Android device for the notification!");
        }
        Err(e) => {
            eprintln!("❌ Failed to send: {}", e);
            return Err(e.into());
        }
    }

    Ok(())
}
