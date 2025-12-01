// examples/error_handling.rs
// Proper GNTP error handling example
// Run with: cargo run --example error_handling

use gntp::{GntpClient, NotificationType};

fn main() {
    println!("=== Error Handling Example ===\n");
    
    // Try to connect to default server (localhost:23053)
    example_with_error_handling();
    
    println!("\n---\n");
    
    // Try to connect to wrong server
    example_wrong_server();
    
    println!("\n---\n");
    
    // Try to notify without registering
    example_notify_without_register();
}

fn example_with_error_handling() {
    println!("Example 1: Proper error handling\n");
    
    let mut client = GntpClient::new("Error Example")
        .with_host("localhost")
        .with_port(23053);
    
    let notification = NotificationType::new("test")
        .with_display_name("Test Notification");
    
    // Try to register
    println!("Attempting to register...");
    match client.register(vec![notification]) {
        Ok(response) => {
            println!("✓ Registration successful!");
            println!("  Server response: {}", response.lines().next().unwrap_or(""));
            
            // Now try to notify
            println!("\nAttempting to send notification...");
            match client.notify("test", "Success!", "Connected to Growl successfully") {
                Ok(_) => println!("✓ Notification sent"),
                Err(e) => println!("✗ Notification error: {}", e),
            }
        }
        Err(e) => {
            println!("✗ Registration failed: {}", e);
            println!("\n📋 Troubleshooting:");
            println!("  1. Is Growl running?");
            println!("     • Windows: Check system tray for Growl icon");
            println!("     • macOS: Check if Growl is installed and running");
            println!("\n  2. Check Growl settings:");
            println!("     • Allow network notifications");
            println!("     • Port should be 23053 (default)");
            println!("\n  3. Firewall:");
            println!("     • Make sure port 23053 is not blocked");
            println!("\n  4. Install Growl:");
            println!("     • Windows: https://github.com/briandunnington/growl-for-windows/releases");
            println!("     • macOS: Growl for Mac or compatible client");
        }
    }
}

fn example_wrong_server() {
    println!("Example 2: Wrong server address\n");
    
    let mut client = GntpClient::new("Wrong Server")
        .with_host("192.168.999.999")  // Invalid IP
        .with_port(23053);
    
    let notification = NotificationType::new("test");
    
    println!("Attempting to connect to invalid server...");
    match client.register(vec![notification]) {
        Ok(_) => println!("✓ Connected (unexpected)"),
        Err(e) => {
            println!("✗ Connection failed (expected): {}", e);
            println!("  This is normal for an invalid server address");
        }
    }
}

fn example_notify_without_register() {
    println!("Example 3: Notify without registering first\n");
    
    let client = GntpClient::new("Unregistered");
    
    println!("Attempting to notify WITHOUT registering first...");
    match client.notify("test", "Test", "This should fail") {
        Ok(_) => println!("✓ Sent (unexpected)"),
        Err(e) => {
            println!("✗ Failed (expected): {}", e);
            println!("  GNTP requires register() before notify()");
        }
    }
}

// Helper function showing best practices
#[allow(dead_code)]
fn best_practice_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("Best Practice: Graceful error handling\n");
    
    // 1. Create client
    let mut client = GntpClient::new("My App");
    
    // 2. Define notifications
    let notification = NotificationType::new("alert");
    
    // 3. Try to register with graceful fallback
    match client.register(vec![notification]) {
        Ok(_) => {
            println!("✓ Growl notifications enabled");
            
            // 4. Use notifications throughout your app
            if let Err(e) = client.notify("alert", "Event", "Something happened") {
                eprintln!("Warning: Failed to send notification: {}", e);
                // Continue with app logic - don't crash
            }
        }
        Err(e) => {
            // Don't crash - just log and continue without notifications
            eprintln!("Warning: Growl notifications disabled: {}", e);
            eprintln!("Continuing without notifications...");
            
            // Your app continues to work, just without notifications
        }
    }
    
    Ok(())
}