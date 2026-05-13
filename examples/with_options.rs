// File: examples\with_options.rs
// Author: Hadi Cahyadi <cumulus13@gmail.com>
// Date: 2026-05-13
// Description:
// License: MIT

// examples/with_options.rs
// GNTP with notification options (priority, sticky)
// Run with: cargo run --example with_options

use gntp::{GntpClient, NotificationType, NotifyOptions};
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Notification Options Example ===\n");

    // Create GNTP client
    let mut client = GntpClient::new("Options Example");

    // Define notification type
    let notification = NotificationType::new("message").with_display_name("Message");

    // Register
    println!("Registering...");
    client.register(vec![notification])?;
    println!("✓ Registered\n");

    // Example 1: Normal priority notification
    println!("Example 1: Normal notification (default priority)");
    client.notify("message", "Normal", "This is a normal notification")?;
    println!("✓ Sent (priority: 0)\n");
    thread::sleep(Duration::from_secs(2));

    // Example 2: High priority notification
    println!("Example 2: High priority notification");
    let high_priority = NotifyOptions::new().with_priority(2); // Highest priority: 2

    client.notify_with_options(
        "message",
        "High Priority",
        "This is a high priority notification!",
        high_priority,
    )?;
    println!("✓ Sent (priority: 2)\n");
    thread::sleep(Duration::from_secs(2));

    // Example 3: Low priority notification
    println!("Example 3: Low priority notification");
    let low_priority = NotifyOptions::new().with_priority(-2); // Lowest priority: -2

    client.notify_with_options(
        "message",
        "Low Priority",
        "This is a low priority notification",
        low_priority,
    )?;
    println!("✓ Sent (priority: -2)\n");
    thread::sleep(Duration::from_secs(2));

    // Example 4: Sticky notification (stays on screen)
    println!("Example 4: Sticky notification (stays on screen)");
    let sticky = NotifyOptions::new().with_sticky(true);

    client.notify_with_options(
        "message",
        "Sticky Notification",
        "This notification will stay on screen until dismissed",
        sticky,
    )?;
    println!("✓ Sent (sticky: true)\n");
    thread::sleep(Duration::from_secs(2));

    // Example 5: High priority + sticky
    println!("Example 5: High priority AND sticky");
    let critical = NotifyOptions::new().with_priority(2).with_sticky(true);

    client.notify_with_options(
        "message",
        "Critical Alert!",
        "High priority sticky notification - requires manual dismissal",
        critical,
    )?;
    println!("✓ Sent (priority: 2, sticky: true)\n");

    println!("✅ Example completed!");
    println!("\nNote:");
    println!("  • Priority range: -2 (lowest) to 2 (highest)");
    println!("  • Sticky notifications stay on screen until dismissed");
    println!("  • You can combine priority + sticky");

    Ok(())
}
