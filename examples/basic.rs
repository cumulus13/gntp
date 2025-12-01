// examples/basic.rs
// Basic GNTP notification example
// Run with: cargo run --example basic

use gntp::{GntpClient, NotificationType};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Basic GNTP Example ===\n");
    
    // Create GNTP client
    println!("Creating GNTP client...");
    let mut client = GntpClient::new("Example App");
    
    // Define notification type
    println!("Defining notification type...");
    let notification = NotificationType::new("message")
        .with_display_name("Message Notification")
        .with_enabled(true);
    
    // STEP 1: Register with Growl (must be done first!)
    println!("\nStep 1: Registering with Growl...");
    match client.register(vec![notification]) {
        Ok(_) => println!("✓ Registration successful"),
        Err(e) => {
            eprintln!("✗ Registration failed: {}", e);
            eprintln!("\nMake sure Growl is running on localhost:23053");
            return Err(e.into());
        }
    }
    
    // STEP 2: Send notification
    println!("\nStep 2: Sending notification...");
    client.notify("message", "Hello from GNTP!", "This is a basic notification")?;
    println!("✓ Notification sent successfully");
    
    println!("\n✅ Example completed!");
    println!("You should see a notification from Growl.");
    
    Ok(())
}