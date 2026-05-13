# GNTP - Growl Notification Transport Protocol Client

[![Crates.io](https://img.shields.io/crates/v/gntp.svg)](https://crates.io/crates/gntp)
[![Documentation](https://docs.rs/gntp/badge.svg)](https://docs.rs/config-get/latest/gntp)
[![CI](https://github.com/cumulus13/gntp/actions/workflows/ci.yml/badge.svg?branch=master)](https://github.com/cumulus13/gntp/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://www.apache.org/licenses/LICENSE-2.0)

A robust, production-ready Rust implementation of the **Growl Notification Transport Protocol (GNTP)** for sending desktop notifications to Growl-compatible clients across multiple platforms.

## ✨ Features

- ✅ **Full GNTP 1.0 protocol implementation**
- ✅ **Multiple icon delivery modes** (Binary, File URL, Data URL/Base64)
- ✅ **Windows Growl compatibility** with automatic workarounds
- ✅ **Cross-platform support** (Windows, macOS, Linux, Android)
- ✅ **Binary resource deduplication** to prevent protocol errors
- ✅ **Comprehensive error handling** with detailed error types
- ✅ **Production-ready** with extensive documentation
- ✅ **Zero external dependencies** (except `uuid` for unique identifiers)
- ✅ Multiple notification types per application
- ✅ Priority levels (-2 to 2)
- ✅ Sticky notifications

## 📦 Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
gntp = "0.1.15"
```

## Requirements

You need a GNTP-compatible notification client:

- **Windows**: [Growl for Windows](https://github.com/briandunnington/growl-for-windows/releases)
- **macOS**: Growl for Mac (legacy) or compatible client
- **Linux**: Snarl or compatible GNTP daemon
- **Android**: Googling :)

Default server: `localhost:23053`

## 🚀 Quick Start

```bash

# Build library only (no CLI) `sendgrowl`
cargo build --release

# Build dengan CLI tool (sendgrowl)
cargo build --release --features cli --bin sendgrowl

# Test sendgrowl
cargo run --features cli --bin sendgrowl -- TEST APP "Title" "Message" -i growl.png -v

# more options
sendgrowl --help
```

```rust
use gntp::{GntpClient, NotificationType, Resource, IconMode};

fn main() -> Result<(), gntp::GntpError> {
    // Create client with DataUrl mode (safest, most compatible)
    let mut client = GntpClient::new("My App")
        .with_icon_mode(IconMode::DataUrl);

    // Load icon from file
    let icon = Resource::from_file("icon.png")?;

    // Define notification type with icon
    let notification = NotificationType::new("alert")
        .with_display_name("Alert Notification")
        .with_icon(icon);

    // Register (must be called first!)
    client.register(vec![notification])?;

    // Send notification
    client.notify("alert", "Hello", "This is a test notification")?;

    Ok(())
}
```

## 🎯 Icon Delivery Modes

### DataUrl Mode (Recommended - Default)

Embeds icons as base64-encoded data URLs. **Most compatible** across all platforms, especially Windows.

```rust
let client = GntpClient::new("App")
    .with_icon_mode(IconMode::DataUrl);
```

**Pros:**
- ✅ Works on all platforms
- ✅ No external files required
- ✅ Bypasses Growl for Windows binary resource bug

**Cons:**
- ⚠️ Larger packet size (~33% increase due to base64)

### Binary Mode (GNTP Spec Compliant)

Sends icons as binary resources according to GNTP specification.

```rust
let client = GntpClient::new("App")
    .with_icon_mode(IconMode::Binary);
```

**Pros:**
- ✅ Smallest packet size
- ✅ Fastest transmission
- ✅ GNTP spec compliant

**Cons:**
- ❌ Broken on Growl for Windows (causes timeout)

### FileUrl Mode

References icons via `file://` URLs. Requires icon files to exist on disk.

```rust
let client = GntpClient::new("App")
    .with_icon_mode(IconMode::FileUrl);
```

**Pros:**
- ✅ No data in packet
- ✅ Good for shared icons

**Cons:**
- ⚠️ Requires files on disk
- ⚠️ Path must be accessible to Growl server

## 🖼️ Working with Icons

### From File

```rust
let icon = Resource::from_file("icon.png")?;
```

### From Memory

```rust
let image_data: Vec<u8> = load_icon_from_memory();
let icon = Resource::from_bytes(image_data, "image/png");
```

### Supported Formats

- PNG (`.png`) - Recommended
- JPEG (`.jpg`, `.jpeg`)
- GIF (`.gif`)
- BMP (`.bmp`)
- ICO (`.ico`)
- SVG (`.svg`)
- WebP (`.webp`)

## 📋 Advanced Usage

### Multiple Notification Types

```rust
let info = NotificationType::new("info")
    .with_display_name("Information");
let warning = NotificationType::new("warning")
    .with_display_name("Warning");
let error = NotificationType::new("error")
    .with_display_name("Error");

client.register(vec![info, warning, error])?;

client.notify("info", "Info", "Something happened")?;
client.notify("warning", "Warning", "Be careful!")?;
client.notify("error", "Error", "Something went wrong!")?;
```

### Notification Options

```rust
use gntp::NotifyOptions;

let options = NotifyOptions::new()
    .with_sticky(true)        // Stays on screen until dismissed
    .with_priority(2);        // Emergency priority

client.notify_with_options(
    "alert",
    "Important",
    "This stays on screen",
    options
)?;
```

### Remote Notifications

```rust
let client = GntpClient::new("Remote App")
    .with_host("192.168.1.100")
    .with_port(23053);
```

### Android Notifications with Retry

Android devices may have network delays. Use retry mechanism:

```bash
# sendgrowl with retry
sendgrowl.exe MyApp Event "Title" "Message" \
  -H 192.168.1.50 \
  -r 3 \              # Retry 3 times on failure
  --retry-delay 2000  # Wait 2 seconds between retries
```

Or in Rust code with manual retry:

```rust
let mut client = GntpClient::new("Android App")
    .with_host("192.168.1.50")
    .with_icon_mode(IconMode::DataUrl);

for attempt in 1..=3 {
    match client.register(vec![notification.clone()]) {
        Ok(_) => break,
        Err(e) if attempt < 3 => {
            eprintln!("Retry {}/3...", attempt);
            std::thread::sleep(Duration::from_secs(2));
        }
        Err(e) => return Err(e),
    }
}
```

### Debug Mode

```rust
let client = GntpClient::new("Debug App")
    .with_debug(true); // Prints detailed packet information
```

## 🐛 Windows Compatibility Notes

Growl for Windows has a known bug where it doesn't properly handle binary resources according to the GNTP specification. When the server receives binary data, it may not respond, causing timeout errors (error code 10060).

**Solution:** Use `IconMode::DataUrl` (default) which embeds icons as base64 strings. This bypasses the binary resource issue entirely.

## 📊 Platform Compatibility

| Platform | Binary Mode | File URL | Data URL | Recommended |
|----------|-------------|----------|----------|-------------|
| Windows (Growl for Windows) | ⚠️ Buggy | ✅ Works | ✅ **Best** | `DataUrl` |
| macOS (Growl) | ✅ Works | ✅ Works | ✅ Works | `Binary` |
| Linux (Growl-compatible) | ✅ Works | ✅ Works | ✅ Works | `Binary` |

`Buggy`: most tests pass

## 🔧 Error Handling

```rust
match client.register(vec![notification]) {
    Ok(_) => println!("Registered successfully"),
    Err(gntp::GntpError::ConnectionError(msg)) => {
        eprintln!("Connection failed: {}", msg);
    }
    Err(gntp::GntpError::IoError(msg)) => {
        eprintln!("I/O error: {}", msg);
    }
    Err(gntp::GntpError::ProtocolError(msg)) => {
        eprintln!("Protocol error: {}", msg);
    }
}
```

## 📚 Examples

Run examples with:

```bash
# Basic notification
cargo run --example basic

# Notification with icon
cargo run --example with_icon

# Notification with full path icon
cargo run --example with_icon_binary

# Multiple notification types
cargo run --example multiple_types

# Remote notifications
GROWL_HOST=192.168.1.100 cargo run --example remote

# Android notifications with retry
ANDROID_HOST=192.168.1.50 cargo run --example android

# Error handling patterns
cargo run --example error_handling
```

### Basic Notification

```rust
use gntp::{GntpClient, NotificationType};

fn main() {
    let mut client = GntpClient::new("Example App");
    let notification = NotificationType::new("message");
    client.register(vec![notification]).unwrap();
    client.notify("message", "Hello", "Basic notification").unwrap();
}

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

## 📄 License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## Resources

- [GNTP Protocol Specification](http://www.growlforwindows.com/gfw/help/gntp.aspx)
- [Growl for Windows](https://github.com/briandunnington/growl-for-windows)
- [Documentation](https://docs.rs/gntp)

## 👤 Author

[Hadi Cahyadi](mailto:cumulus13@gmail.com)
    

[![Buy Me a Coffee](https://www.buymeacoffee.com/assets/img/custom_images/orange_img.png)](https://www.buymeacoffee.com/cumulus13)

[![Donate via Ko-fi](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/cumulus13)
 
[Support me on Patreon](https://www.patreon.com/cumulus13)

## 🙏 Acknowledgments

- Based on the GNTP specification by The Growl Project
- Inspired by various GNTP client implementations

## 📖 Resources

- [GNTP Protocol Specification](http://www.growlforwindows.com/gfw/help/gntp.aspx)
- [Growl Project](http://growl.info/)
- [Growl for Windows](http://www.growlforwindows.com/)

---

**Note:** This is a production-ready library with comprehensive Windows compatibility. If you encounter any issues, please open an issue on GitHub.
