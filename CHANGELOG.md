# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.16] - 2026-05-13

### 🎉 Production Release

This is a complete rewrite with breaking changes for production readiness.

### Added

- **Multiple icon delivery modes**: Binary, FileUrl, DataUrl, HttpUrl, Auto
- **Android Growl support**: Fully tested with Growl for Android
- **Windows Growl workarounds**: Automatic handling of Windows-specific bugs
- **Resource deduplication**: Prevents duplicate binary data transmission
- **RFC 4648 compliant base64**: Proper encoding with line breaks for strict parsers
- **IconMode enum**: Choose best delivery method per platform
- **Debug mode**: Verbose packet logging with `.with_debug(true)`
- **Comprehensive documentation**: Examples, platform notes, error handling
- **NotifyOptions**: Sticky notifications, priority levels, per-notification icons
- **Better error types**: ConnectionError, IoError, ProtocolError with context

### Changed

- **BREAKING**: `GntpClient::new()` now requires explicit icon mode selection
- **BREAKING**: `Resource` struct changed to support multiple delivery modes
- **BREAKING**: Default icon mode is now `DataUrl` (was `Binary`)
- **BREAKING**: Removed automatic localhost conversion (now explicit per-host)
- Improved base64 encoding for better compatibility
- Better timeout handling (10s default, graceful connection close)
- Cleaner error messages with context

### Fixed

- **Windows timeout issue**: Binary resources causing 10060 errors
- **Android 500 errors**: Strict GNTP parsing issues
- **Duplicate resources**: Same icon sent multiple times
- **Base64 format**: Some servers require MIME-style line breaks
- **Connection handling**: Proper cleanup on errors
- **Icon size warnings**: Alert on icons >500KB

### Removed

- **BREAKING**: Removed password authentication (was unimplemented)
- **BREAKING**: Removed `notify()` without options (use `notify_with_options()`)

## [0.1.4] - 2024-XX-XX (Previous version)

### Issues in 0.1.x

- Binary resources didn't work on Windows Growl
- Timeout errors with large icons
- No Android support
- Limited error context
- No icon delivery mode options

---

## Migration Guide: 0.1.x → 1.0.0

### Basic Usage

**Before (0.1.x):**
```rust
use gntp::GntpClient;

let mut client = GntpClient::new("App");
client.register(vec![])?;
client.notify("alert", "Title", "Text")?;
```

**After (1.0.0):**
```rust
use gntp::{GntpClient, IconMode};

let mut client = GntpClient::new("App")
    .with_icon_mode(IconMode::DataUrl); // Required!
client.register(vec![])?;
client.notify("alert", "Title", "Text")?;
```

### With Icons

**Before (0.1.x):**
```rust
use gntp::Resource;

let icon = Resource::from_file("icon.png")?;
// May timeout on Windows!
```

**After (1.0.0):**
```rust
use gntp::{Resource, IconMode};

let icon = Resource::from_file("icon.png")?;
let client = GntpClient::new("App")
    .with_icon_mode(IconMode::DataUrl); // Works everywhere!
```

### Error Handling

**Before (0.1.x):**
```rust
match client.notify(...) {
    Err(e) => println!("Error: {}", e), // Generic
}
```

**After (1.0.0):**
```rust
use gntp::GntpError;

match client.notify(...) {
    Err(GntpError::ConnectionError(msg)) => {
        eprintln!("Can't connect: {}", msg);
    }
    Err(GntpError::IoError(msg)) => {
        eprintln!("I/O failed: {}", msg);
    }
    Err(GntpError::ProtocolError(msg)) => {
        eprintln!("Protocol error: {}", msg);
    }
    Ok(_) => println!("Success!"),
}
```

### Platform-Specific Recommendations

```rust
use gntp::IconMode;

// Windows Growl
let client = GntpClient::new("App")
    .with_icon_mode(IconMode::DataUrl); // Best for Windows

// Android Growl
let client = GntpClient::new("App")
    .with_icon_mode(IconMode::DataUrl) // Or HttpUrl
    .with_host("192.168.1.100");

// macOS/Linux Growl
let client = GntpClient::new("App")
    .with_icon_mode(IconMode::Binary); // Fastest
```

---

## Support

- **Issues**: https://github.com/cumulus13/gntp/issues
- **Docs**: https://docs.rs/gntp
- **Email**: cumulus13@gmail.com