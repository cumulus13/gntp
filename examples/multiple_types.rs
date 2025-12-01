// examples/multiple_types.rs
// GNTP with multiple notification types
// Run with: cargo run --example multiple_types

use gntp::{GntpClient, NotificationType};
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Multiple Notification Types Example ===\n");
    
    // Create GNTP client
    let mut client = GntpClient::new("Multi Type App");
    
    // Define multiple notification types
    println!("Defining 4 notification types:");
    println!("  • info    - Information messages");
    println!("  • warning - Warning messages");
    println!("  • error   - Error messages");
    println!("  • success - Success messages");
    
    let notifications = vec![
        NotificationType::new("info")
            .with_display_name("Information")
            .with_enabled(true),
        
        NotificationType::new("warning")
            .with_display_name("Warning")
            .with_enabled(true),
        
        NotificationType::new("error")
            .with_display_name("Error")
            .with_enabled(true),
        
        NotificationType::new("success")
            .with_display_name("Success")
            .with_enabled(true),
    ];
    
    // Register all types at once
    println!("\nRegistering all notification types...");
    client.register(notifications)?;
    println!("✓ All types registered successfully");
    
    // Send notifications of different types
    println!("\nSending notifications (one per second):");
    
    println!("  [1/4] Sending info notification...");
    client.notify("info", "Information", "This is an informational message")?;
    println!("  ✓ Info sent");
    thread::sleep(Duration::from_millis(1000));
    
    println!("  [2/4] Sending warning notification...");
    client.notify("warning", "Warning", "This is a warning message")?;
    println!("  ✓ Warning sent");
    thread::sleep(Duration::from_millis(1000));
    
    println!("  [3/4] Sending error notification...");
    client.notify("error", "Error", "This is an error message")?;
    println!("  ✓ Error sent");
    thread::sleep(Duration::from_millis(1000));
    
    println!("  [4/4] Sending success notification...");
    client.notify("success", "Success", "Operation completed successfully!")?;
    println!("  ✓ Success sent");
    
    println!("\n✅ Example completed!");
    println!("You should have seen 4 different notifications.");
    
    Ok(())
}