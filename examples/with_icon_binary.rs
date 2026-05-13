// File: examples\with_icon_binary.rs
// Author: Hadi Cahyadi <cumulus13@gmail.com>
// Date: 2026-05-13
// Description:
// License: MIT

// examples/with_icon_binary.rs
// Test with Binary mode (GNTP spec compliant)

use gntp::{GntpClient, IconMode, NotificationType, Resource};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Testing Binary Mode ===\n");

    let icon = Resource::from_file("growl.png")?;
    println!("✓ Icon loaded: {} bytes\n", icon.data.len());

    let mut client = GntpClient::new("Binary Test")
        .with_icon_mode(IconMode::Binary) // Try Binary mode
        .with_debug(true);

    let notification = NotificationType::new("alert")
        .with_display_name("Alert")
        .with_icon(icon);

    println!("Registering...");
    client.register(vec![notification])?;

    println!("\nSending...");
    client.notify("alert", "Binary Mode Test", "Does icon show?")?;

    println!("\n✅ Done! Check if icon appears.");

    Ok(())
}
