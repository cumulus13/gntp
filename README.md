# GNTP - Growl Notification Transport Protocol Client

[![Crates.io](https://img.shields.io/crates/v/gntp.svg)](https://crates.io/crates/gntp)
[![Documentation](https://docs.rs/gntp/badge.svg)](https://docs.rs/gntp)
[![License](https://img.shields.io/crates/l/gntp.svg)](LICENSE-MIT)

A complete Rust implementation of the Growl Notification Transport Protocol (GNTP) for sending desktop notifications.

## Features

- ✅ Full GNTP 1.0 protocol implementation
- ✅ Binary icon support (PNG, JPG, etc.)
- ✅ Multiple notification types per application
- ✅ Priority levels (-2 to 2)
- ✅ Sticky notifications
- ✅ Comprehensive error handling
- ✅ No external dependencies (except uuid)
- ✅ Cross-platform (Windows, macOS, Linux)

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
gntp = "0.1"
```

## Requirements

You need a GNTP-compatible notification client:

- **Windows**: [Growl for Windows](https://github.com/briandunnington/growl-for-windows/releases)
- **macOS**: Growl for Mac (legacy) or compatible client
- **Linux**: Snarl or compatible GNTP daemon

Default server: `localhost:23053`

## Quick Start

```rust
use gntp::{GntpClient, NotificationType};

fn main() -> Result<(), gntp::GntpError> {
    // Create client
    let mut client = GntpClient::new("My App");

    // Define notification type
    let notification = NotificationType::new("alert")
        .with_display_name("Alert Notification");

    // Step 1: Register (MUST be called first!)
    client.register(vec![notification])?;

    // Step 2: Send notification
    client.notify("alert", "Hello", "This is a test notification")?;

    Ok(())
}
```

## Examples

### Basic Notification

```rust
use gntp::{GntpClient, NotificationType};

let mut client = GntpClient::new("Example App");
let notification = NotificationType::new("message");
client.register(vec![notification])?;
client.notify("message", "Hello", "Basic notification")?;
```

### With Icon

```rust
use gntp::{GntpClient, NotificationType, Resource};

let mut client = GntpClient::new("Icon Example");

// Load application icon
if let Ok(icon) = Resource::from_file("app_icon.png") {
    client = client.with_icon(icon);
}

let notification = NotificationType::new("alert");
client.register(vec![notification])?;
client.notify("alert", "Alert", "With icon")?;
```

### With Options (Priority & Sticky)

```rust
use gntp::{GntpClient, NotificationType, NotifyOptions};

let mut client = GntpClient::new("Options Example");
let notification = NotificationType::new("important");
client.register(vec![notification])?;

let options = NotifyOptions::new()
    .with_sticky(true)
    .with_priority(2);

client.notify_with_options(
    "important",
    "Critical",
    "High priority sticky notification",
    options
)?;
```

### Multiple Notification Types

```rust
let mut client = GntpClient::new("Multi App");

let notifications = vec![
    NotificationType::new("info"),
    NotificationType::new("warning"),
    NotificationType::new("error"),
];

client.register(notifications)?;

client.notify("info", "Info", "Information")?;
client.notify("warning", "Warning", "Warning message")?;
client.notify("error", "Error", "Error occurred")?;
```

## Protocol Details

GNTP requires two separate steps:

1. **REGISTER** - Register your application and notification types (once at startup)
2. **NOTIFY** - Send notifications (multiple times)

Icons are sent as binary resources with unique identifiers, not as file paths.

## Error Handling

```rust
match client.register(vec![notification]) {
    Ok(_) => println!("Registered successfully"),
    Err(e) => {
        eprintln!("Registration failed: {}", e);
        // Handle error (Growl not running, network issue, etc.)
    }
}
```

## Running Examples

```bash
# Basic example
cargo run --example basic

# With icon
cargo run --example with_icon

# Multiple notification types
cargo run --example multiple_types

# With options (priority, sticky)
cargo run --example with_options

# Error handling
cargo run --example error_handling
```

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Resources

- [GNTP Protocol Specification](http://www.growlforwindows.com/gfw/help/gntp.aspx)
- [Growl for Windows](https://github.com/briandunnington/growl-for-windows)
- [Documentation](https://docs.rs/gntp)

## Author
[Hadi Cahyadi](mailto:cumulus13@gmail.com)
    

[![Buy Me a Coffee](https://www.buymeacoffee.com/assets/img/custom_images/orange_img.png)](https://www.buymeacoffee.com/cumulus13)

[![Donate via Ko-fi](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/cumulus13)
 
[Support me on Patreon](https://www.patreon.com/cumulus13)

