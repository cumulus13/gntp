// File: examples\error_handling.rs
// Author: Hadi Cahyadi <cumulus13@gmail.com>
// Date: 2026-05-13
// Description: 
// License: MIT

// examples/error_handling.rs
// Example: Proper error handling patterns
//
// Run with: cargo run --example error_handling

use gntp::{GntpClient, GntpError, IconMode, NotificationType, Resource};

fn main() {
    println!("=== Error Handling Example ===\n");

    // Example 1: Handle connection errors
    println!("1. Testing connection to non-existent server...");
    match test_connection() {
        Ok(_) => println!("   ✓ Connected\n"),
        Err(e) => println!("   ✗ Failed (expected): {}\n", e),
    }

    // Example 2: Handle file not found
    println!("2. Testing with non-existent icon...");
    match test_missing_icon() {
        Ok(_) => println!("   ✓ Icon loaded\n"),
        Err(e) => println!("   ✗ Failed (expected): {}\n", e),
    }

    // Example 3: Handle protocol errors
    println!("3. Testing notify before register...");
    match test_protocol_error() {
        Ok(_) => println!("   ✓ Notification sent\n"),
        Err(e) => println!("   ✗ Failed (expected): {}\n", e),
    }

    // Example 4: Proper error handling with match
    println!("4. Demonstrating detailed error handling...");
    demonstrate_error_types();

    println!("\n✅ Example completed!");
}

fn test_connection() -> Result<(), GntpError> {
    let mut client = GntpClient::new("Test App")
        .with_host("255.255.255.255") // Invalid host
        .with_port(65535)
        .with_icon_mode(IconMode::DataUrl);

    let notification = NotificationType::new("test");
    client.register(vec![notification])?;

    Ok(())
}

fn test_missing_icon() -> Result<(), GntpError> {
    let _icon = Resource::from_file("this_file_does_not_exist.png")?;
    Ok(())
}

fn test_protocol_error() -> Result<(), GntpError> {
    let client = GntpClient::new("Test App").with_icon_mode(IconMode::DataUrl);

    // Try to notify without registering first
    client.notify("test", "Title", "Text")?;

    Ok(())
}

fn demonstrate_error_types() {
    let result = test_connection();

    match result {
        Ok(_) => {
            println!("   Success!");
        }
        Err(GntpError::ConnectionError(msg)) => {
            println!("   ✓ Caught ConnectionError:");
            println!("     - Message: {}", msg);
            println!("     - Action: Check if Growl is running");
            println!("     - Action: Verify host/port");
        }
        Err(GntpError::IoError(msg)) => {
            println!("   ✓ Caught IoError:");
            println!("     - Message: {}", msg);
            println!("     - Action: Check file permissions");
            println!("     - Action: Verify file exists");
        }
        Err(GntpError::ProtocolError(msg)) => {
            println!("   ✓ Caught ProtocolError:");
            println!("     - Message: {}", msg);
            println!("     - Action: Call register() before notify()");
            println!("     - Action: Check packet format");
        }
    }
}
