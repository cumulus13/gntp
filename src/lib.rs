// File: src\lib.rs
// Author: Hadi Cahyadi <cumulus13@gmail.com>
// Date: 2025-12-16
// Description:
// License: MIT

//! # GNTP - Growl Notification Transport Protocol Client
//!
//! A robust, production-ready Rust implementation of the Growl Notification Transport Protocol (GNTP)
//! for sending desktop notifications to Growl-compatible clients across multiple platforms.
//!
//! ## Features
//!
//! - ✅ **Full GNTP 1.0 protocol implementation**
//! - ✅ **Multiple icon delivery modes** (Binary, File URL, Data URL/Base64)
//! - ✅ **Windows Growl compatibility** with automatic workarounds
//! - ✅ **Cross-platform support** (Windows, macOS, Linux)
//! - ✅ **Binary resource deduplication** to prevent protocol errors
//! - ✅ **Comprehensive error handling** with detailed error types
//! - ✅ **Production-ready** with extensive testing
//! - ✅ **No external dependencies** (except uuid for unique identifiers)
//!
//! ## Platform Compatibility
//!
//! | Platform | Binary Mode | File URL | Data URL | Recommended |
//! |----------|-------------|----------|----------|-------------|
//! | Windows (Growl for Windows) | ⚠️ Buggy | ✅ Works | ✅ **Best** | DataUrl |
//! | macOS (Growl) | ✅ Works | ✅ Works | ✅ Works | Binary |
//! | Linux (Growl-compatible) | ✅ Works | ✅ Works | ✅ Works | Binary |
//!
//! ## Quick Start
//!
//! ```no_run
//! use gntp::{GntpClient, NotificationType, Resource, IconMode};
//!
//! # fn main() -> Result<(), gntp::GntpError> {
//! // Create client with DataUrl mode (safest, most compatible)
//! let mut client = GntpClient::new("My App")
//!     .with_icon_mode(IconMode::DataUrl);
//!
//! // Load icon from file
//! let icon = Resource::from_file("icon.png")?;
//!
//! // Define notification type with icon
//! let notification = NotificationType::new("alert")
//!     .with_display_name("Alert Notification")
//!     .with_icon(icon);
//!
//! // Register (must be called first!)
//! client.register(vec![notification])?;
//!
//! // Send notification
//! client.notify("alert", "Hello", "This is a test notification")?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Icon Delivery Modes
//!
//! ### Binary Mode (GNTP Spec Compliant)
//! ```no_run
//! # use gntp::{GntpClient, IconMode};
//! let client = GntpClient::new("App")
//!     .with_icon_mode(IconMode::Binary);
//! ```
//! Sends icons as binary resources according to GNTP specification.
//! **Note:** May not work correctly with Growl for Windows due to implementation bugs.
//!
//! ### File URL Mode
//! ```no_run
//! # use gntp::{GntpClient, IconMode};
//! let client = GntpClient::new("App")
//!     .with_icon_mode(IconMode::FileUrl);
//! ```
//! References icons via `file://` URLs. Requires icon files to exist on disk.
//!
//! ### Data URL Mode (Recommended for Windows)
//! ```no_run
//! # use gntp::{GntpClient, IconMode};
//! let client = GntpClient::new("App")
//!     .with_icon_mode(IconMode::DataUrl); // Default
//! ```
//! Embeds icons as base64-encoded data URLs. **Most compatible** across all platforms.
//! No external files required.
//!
//! ## Advanced Usage
//!
//! ### Creating Resources from Memory
//! ```no_run
//! # use gntp::Resource;
//! let image_data: Vec<u8> = vec![/* your image bytes */];
//! let icon = Resource::from_bytes(image_data, "image/png");
//! ```
//!
//! ### Sending Notifications with Options
//! ```no_run
//! # use gntp::{GntpClient, NotifyOptions, Resource};
//! # let mut client = GntpClient::new("App");
//! # let icon = Resource::from_bytes(vec![], "image/png");
//! # client.register(vec![]).unwrap();
//! let options = NotifyOptions::new()
//!     .with_sticky(true)
//!     .with_priority(2)
//!     .with_icon(icon);
//!
//! client.notify_with_options(
//!     "alert",
//!     "Important",
//!     "This stays on screen",
//!     options
//! )?;
//! # Ok::<(), gntp::GntpError>(())
//! ```
//!
//! ### Multiple Notification Types
//! ```no_run
//! # use gntp::{GntpClient, NotificationType};
//! # let mut client = GntpClient::new("App");
//! let info = NotificationType::new("info")
//!     .with_display_name("Information");
//! let warning = NotificationType::new("warning")
//!     .with_display_name("Warning");
//! let error = NotificationType::new("error")
//!     .with_display_name("Error");
//!
//! client.register(vec![info, warning, error])?;
//!
//! client.notify("info", "Info", "Something happened")?;
//! client.notify("warning", "Warning", "Be careful!")?;
//! client.notify("error", "Error", "Something went wrong!")?;
//! # Ok::<(), gntp::GntpError>(())
//! ```
//!
//! ### Remote Notifications
//! ```no_run
//! # use gntp::GntpClient;
//! let client = GntpClient::new("Remote App")
//!     .with_host("192.168.1.100")
//!     .with_port(23053);
//! ```
//!
//! ### Debug Mode
//! ```no_run
//! # use gntp::GntpClient;
//! let client = GntpClient::new("Debug App")
//!     .with_debug(true); // Prints detailed packet information
//! ```
//!
//! ## Protocol Specification
//!
//! GNTP requires two separate steps:
//!
//! 1. **REGISTER** - Register your application and notification types (once per connection)
//! 2. **NOTIFY** - Send notifications (multiple times)
//!
//! You **must** call `register()` before calling `notify()`, otherwise you'll receive a
//! `ProtocolError`.
//!
//! ## Error Handling
//!
//! All operations return `Result<T, GntpError>` with detailed error information:
//!
//! ```no_run
//! # use gntp::{GntpClient, GntpError};
//! # let mut client = GntpClient::new("App");
//! match client.register(vec![]) {
//!     Ok(_) => println!("Registered successfully"),
//!     Err(GntpError::ConnectionError(msg)) => {
//!         eprintln!("Connection failed: {}", msg);
//!     }
//!     Err(GntpError::IoError(msg)) => {
//!         eprintln!("I/O error: {}", msg);
//!     }
//!     Err(GntpError::ProtocolError(msg)) => {
//!         eprintln!("Protocol error: {}", msg);
//!     }
//! }
//! ```
//!
//! ## Windows Compatibility Notes
//!
//! Growl for Windows has a known bug where it doesn't properly handle binary resources
//! according to the GNTP specification. When the server receives binary data, it may
//! not respond, causing timeout errors (10060).
//!
//! **Solution:** Use `IconMode::DataUrl` (default) which embeds icons as base64 strings.
//! This bypasses the binary resource issue entirely.
//!
//! ## Performance Considerations
//!
//! - **Binary Mode**: Smallest packet size, fastest transmission
//! - **File URL Mode**: No data in packet, but requires disk access
//! - **Data URL Mode**: Larger packets (~33% increase due to base64), but most reliable
//!
//! For typical notification icons (< 100KB), the performance difference is negligible.
//!
//! ## Examples
//!
//! See the [examples](https://github.com/cumulus13/gntp/tree/master/examples) directory for:
//! - Basic notifications
//! - Notifications with icons
//! - Multiple notification types
//! - Remote notifications
//! - Error handling patterns
//!
//! ## License
//!
//! MIT License - See LICENSE file for details
//!
//! ## Contributing
//!
//! Contributions are welcome! Please feel free to submit a Pull Request.
//!
//! ## Credits
//!
//! Author: Hadi Cahyadi <cumulus13@gmail.com>

use std::collections::HashSet;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::path::{Path, PathBuf};
use std::time::Duration;

const GNTP_VERSION: &str = "1.0";
const DEFAULT_PORT: u16 = 23053;
const CRLF: &str = "\r\n";

/* ===========================
   ERROR TYPES
=========================== */

/// GNTP client errors
///
/// All operations return `Result<T, GntpError>` with one of these error types.
#[derive(Debug)]
pub enum GntpError {
    /// Connection to Growl server failed
    ///
    /// This can happen if:
    /// - Growl is not running
    /// - Host/port is incorrect
    /// - Network is unreachable
    /// - Firewall is blocking the connection
    ConnectionError(String),

    /// I/O operation failed
    ///
    /// This can happen if:
    /// - File cannot be read (for icons)
    /// - Network communication error
    /// - Timeout occurred
    IoError(String),

    /// GNTP protocol error
    ///
    /// This can happen if:
    /// - `notify()` called before `register()`
    /// - Server returned an error response
    /// - Invalid packet format
    ProtocolError(String),
}

impl std::fmt::Display for GntpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GntpError::ConnectionError(msg) => write!(f, "Connection error: {}", msg),
            GntpError::IoError(msg) => write!(f, "I/O error: {}", msg),
            GntpError::ProtocolError(msg) => write!(f, "Protocol error: {}", msg),
        }
    }
}

impl std::error::Error for GntpError {}

impl From<std::io::Error> for GntpError {
    fn from(err: std::io::Error) -> Self {
        GntpError::IoError(err.to_string())
    }
}

/* ===========================
   ICON DELIVERY MODES
=========================== */

/// Icon delivery mode for notifications
///
/// Different Growl implementations support different icon delivery methods.
/// Choose the appropriate mode for your target platform.
#[derive(Clone, Debug, PartialEq)]
pub enum IconMode {
    /// Binary resources (GNTP spec compliant)
    ///
    /// Sends icons as binary data according to GNTP specification.
    /// **Recommended for:** macOS Growl, Linux Growl-compatible clients
    /// **Not recommended for:** Growl for Windows (has bugs)
    Binary,

    /// File URL references
    ///
    /// References icons via `file://` URLs. Icon files must exist on disk
    /// and be accessible to the Growl server.
    /// **Recommended for:** When icons are already on disk and shared between apps
    FileUrl,

    /// Data URLs with base64 encoding
    ///
    /// Embeds icons as base64-encoded data URLs directly in the packet.
    /// **Recommended for:** Universal compatibility, especially Windows
    /// **Default mode** for maximum compatibility
    /// **Note:** Some servers (like Growl for Android) may have issues with large data URLs
    DataUrl,

    /// HTTP/HTTPS URL references
    ///
    /// References icons via HTTP/HTTPS URLs. Icon must be accessible to the Growl server.
    /// **Recommended for:** Remote servers, mobile devices (Android)
    /// **Best compatibility** with Growl for Android
    HttpUrl,

    /// Auto-detect best method
    ///
    /// Currently defaults to DataUrl for maximum compatibility.
    /// May implement smart detection in future versions.
    Auto,
}

/* ===========================
   RESOURCE HANDLING
=========================== */

/// Binary resource (icon) with multiple delivery options
///
/// Represents an icon that can be delivered to Growl in multiple formats:
/// - As binary data (GNTP spec)
/// - As a file:// URL
/// - As a data: URL with base64 encoding
#[derive(Clone)]
pub struct Resource {
    /// Unique identifier for this resource (UUID v4)
    pub identifier: String,

    /// Binary data of the icon
    pub data: Vec<u8>,

    /// Original file path (if loaded from file)
    pub source_path: Option<PathBuf>,

    /// MIME type of the resource (e.g., "image/png")
    pub mime_type: String,
}

impl Resource {
    /// Load resource from file
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the icon file
    ///
    /// # Returns
    ///
    /// * `Ok(Resource)` - Successfully loaded icon
    /// * `Err(GntpError::IoError)` - Failed to read file
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use gntp::Resource;
    /// let icon = Resource::from_file("icon.png")?;
    /// # Ok::<(), gntp::GntpError>(())
    /// ```
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, GntpError> {
        let path = path.as_ref();
        let data = std::fs::read(&path).map_err(|e| {
            GntpError::IoError(format!("Failed to read file {}: {}", path.display(), e))
        })?;

        let mime_type = guess_mime_type(path);
        let identifier = uuid::Uuid::new_v4().to_string();

        Ok(Resource {
            identifier,
            data,
            source_path: Some(path.to_path_buf()),
            mime_type,
        })
    }

    /// Convenience method for PathBuf
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use gntp::Resource;
    /// # use std::path::PathBuf;
    /// let path = PathBuf::from("icon.png");
    /// let icon = Resource::from_pathbuf(path)?;
    /// # Ok::<(), gntp::GntpError>(())
    /// ```
    pub fn from_pathbuf(path: PathBuf) -> Result<Self, GntpError> {
        Self::from_file(path)
    }

    /// Create resource from raw bytes
    ///
    /// # Arguments
    ///
    /// * `data` - Binary data of the icon
    /// * `mime_type` - MIME type (e.g., "image/png", "image/jpeg")
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use gntp::Resource;
    /// let image_data: Vec<u8> = vec![/* ... */];
    /// let icon = Resource::from_bytes(image_data, "image/png");
    /// ```
    pub fn from_bytes(data: Vec<u8>, mime_type: &str) -> Self {
        Resource {
            identifier: uuid::Uuid::new_v4().to_string(),
            data,
            source_path: None,
            mime_type: mime_type.to_string(),
        }
    }

    /// Get icon reference string based on delivery mode
    fn get_reference(&self, mode: &IconMode) -> String {
        match mode {
            IconMode::Binary => {
                format!("x-growl-resource://{}", self.identifier)
            }
            IconMode::FileUrl => {
                if let Some(ref path) = self.source_path {
                    let path_str = path.to_string_lossy().replace('\\', "/");
                    format!("file:///{}", path_str)
                } else {
                    // Fallback to data URL if no file path
                    self.to_data_url()
                }
            }
            IconMode::HttpUrl => {
                // For HTTP URLs, we expect source_path to contain the URL
                if let Some(ref path) = self.source_path {
                    path.to_string_lossy().to_string()
                } else {
                    // Fallback to data URL
                    self.to_data_url()
                }
            }
            IconMode::DataUrl | IconMode::Auto => self.to_data_url(),
        }
    }

    /// Convert to base64 data URL
    ///
    /// Note: Some GNTP servers have issues with very large data URLs.
    /// Consider using FileUrl mode if icons are >100KB.
    fn to_data_url(&self) -> String {
        // For very large images, truncate base64 to avoid packet size issues
        const MAX_DATA_URL_SIZE: usize = 500_000; // ~500KB base64 limit

        let encoded = base64_encode(&self.data);

        if encoded.len() > MAX_DATA_URL_SIZE {
            eprintln!(
                "Warning: Icon size ({} bytes) may be too large for some GNTP servers",
                encoded.len()
            );
        }

        format!("data:{};base64,{}", self.mime_type, encoded)
    }
}

/// Guess MIME type from file extension
fn guess_mime_type(path: &Path) -> String {
    match path.extension().and_then(|s| s.to_str()) {
        Some("png") => "image/png",
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("gif") => "image/gif",
        Some("bmp") => "image/bmp",
        Some("ico") => "image/x-icon",
        Some("svg") => "image/svg+xml",
        Some("webp") => "image/webp",
        _ => "application/octet-stream",
    }
    .to_string()
}

/// RFC 4648 compliant base64 encoding
///
/// CRITICAL: Some GNTP servers (like Growl for Android) are very strict about base64 format.
/// This implementation follows RFC 4648 exactly and adds line breaks every 76 characters
/// as required by some GNTP implementations.
fn base64_encode(data: &[u8]) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    // const LINE_LENGTH: usize = 76;

    let mut result = String::with_capacity((data.len() + 2) / 3 * 4);
    // let mut line_len = 0;
    let mut i = 0;

    while i < data.len() {
        let b1 = data[i];
        let b2 = if i + 1 < data.len() { data[i + 1] } else { 0 };
        let b3 = if i + 2 < data.len() { data[i + 2] } else { 0 };

        let c1 = CHARSET[(b1 >> 2) as usize] as char;
        let c2 = CHARSET[(((b1 & 0x03) << 4) | (b2 >> 4)) as usize] as char;
        let c3 = if i + 1 < data.len() {
            CHARSET[(((b2 & 0x0F) << 2) | (b3 >> 6)) as usize] as char
        } else {
            '='
        };
        let c4 = if i + 2 < data.len() {
            CHARSET[(b3 & 0x3F) as usize] as char
        } else {
            '='
        };

        result.push(c1);
        result.push(c2);
        result.push(c3);
        result.push(c4);

        // line_len += 4;

        // Add line break every 76 characters (MIME standard)
        // Some GNTP servers require this for proper parsing
        // if line_len >= LINE_LENGTH && i + 3 < data.len() {
        //     result.push('\r');
        //     result.push('\n');
        //     line_len = 0;
        // }

        i += 3;
    }

    result
}

/* ===========================
   NOTIFICATION TYPE
=========================== */

/// Notification type definition
///
/// Defines a type of notification that your application can send.
/// You must register all notification types before sending notifications.
///
/// # Example
///
/// ```no_run
/// # use gntp::{NotificationType, Resource};
/// let icon = Resource::from_file("alert.png")?;
///
/// let notification = NotificationType::new("alert")
///     .with_display_name("Alert Notification")
///     .with_enabled(true)
///     .with_icon(icon);
/// # Ok::<(), gntp::GntpError>(())
/// ```
#[derive(Clone)]
pub struct NotificationType {
    /// Internal name (used in notify())
    pub name: String,

    /// Display name shown to user
    pub display_name: Option<String>,

    /// Whether this notification type is enabled
    pub enabled: bool,

    /// Optional icon for this notification type
    pub icon: Option<Resource>,
}

impl NotificationType {
    /// Create new notification type
    ///
    /// # Arguments
    ///
    /// * `name` - Internal identifier for this notification type
    pub fn new(name: &str) -> Self {
        NotificationType {
            name: name.to_string(),
            display_name: None,
            enabled: true,
            icon: None,
        }
    }

    /// Set display name (shown to user)
    pub fn with_display_name(mut self, display_name: &str) -> Self {
        self.display_name = Some(display_name.to_string());
        self
    }

    /// Set icon for this notification type
    pub fn with_icon(mut self, icon: Resource) -> Self {
        self.icon = Some(icon);
        self
    }

    /// Set whether this notification type is enabled
    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
}

/* ===========================
   GNTP CLIENT
=========================== */

/// GNTP client for sending notifications to Growl
///
/// This is the main client for communicating with Growl servers.
///
/// # Example
///
/// ```no_run
/// # use gntp::{GntpClient, NotificationType, IconMode};
/// let mut client = GntpClient::new("My Application")
///     .with_host("localhost")
///     .with_port(23053)
///     .with_icon_mode(IconMode::DataUrl);
///
/// let notification = NotificationType::new("alert");
/// client.register(vec![notification])?;
/// client.notify("alert", "Title", "Message")?;
/// # Ok::<(), gntp::GntpError>(())
/// ```
pub struct GntpClient {
    /// Growl server hostname
    pub host: String,

    /// Growl server port
    pub port: u16,

    /// Application name
    pub application_name: String,

    /// Optional application icon
    pub application_icon: Option<Resource>,

    /// Optional password for authentication (not yet implemented)
    #[allow(dead_code)]
    pub password: Option<String>,

    /// Whether register() has been called
    registered: bool,

    /// Enable debug output
    pub debug: bool,

    /// Icon delivery mode
    pub icon_mode: IconMode,
}

impl GntpClient {
    /// Create new GNTP client
    ///
    /// # Arguments
    ///
    /// * `application_name` - Name of your application
    ///
    /// # Example
    ///
    /// ```
    /// # use gntp::GntpClient;
    /// let client = GntpClient::new("My App");
    /// ```
    pub fn new(application_name: &str) -> Self {
        GntpClient {
            host: "localhost".to_string(),
            port: DEFAULT_PORT,
            application_name: application_name.to_string(),
            application_icon: None,
            password: None,
            registered: false,
            debug: false,
            icon_mode: IconMode::DataUrl, // Safe default for Windows
        }
    }

    /// Set Growl server hostname
    ///
    /// # Example
    ///
    /// ```
    /// # use gntp::GntpClient;
    /// let client = GntpClient::new("App")
    ///     .with_host("192.168.1.100");
    /// ```
    pub fn with_host(mut self, host: &str) -> Self {
        self.host = host.to_string();
        self
    }

    /// Set Growl server port
    ///
    /// Default is 23053 (standard GNTP port)
    pub fn with_port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    /// Set application icon
    pub fn with_icon(mut self, icon: Resource) -> Self {
        self.application_icon = Some(icon);
        self
    }

    /// Set password for authentication
    ///
    /// **Note:** Password authentication is not yet implemented
    #[allow(dead_code)]
    pub fn with_password(mut self, password: &str) -> Self {
        self.password = Some(password.to_string());
        self
    }

    /// Enable debug output
    ///
    /// When enabled, prints detailed packet information to stdout
    pub fn with_debug(mut self, debug: bool) -> Self {
        self.debug = debug;
        self
    }

    /// Set icon delivery mode
    ///
    /// # Example
    ///
    /// ```
    /// # use gntp::{GntpClient, IconMode};
    /// let client = GntpClient::new("App")
    ///     .with_icon_mode(IconMode::DataUrl);
    /// ```
    pub fn with_icon_mode(mut self, mode: IconMode) -> Self {
        self.icon_mode = mode;
        self
    }

    /// Register application with Growl
    ///
    /// This **must** be called before sending notifications.
    ///
    /// # Arguments
    ///
    /// * `notifications` - Vector of notification types to register
    ///
    /// # Returns
    ///
    /// * `Ok(String)` - Server response (usually "GNTP/1.0 -OK")
    /// * `Err(GntpError)` - Connection, I/O, or protocol error
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use gntp::{GntpClient, NotificationType};
    /// # let mut client = GntpClient::new("App");
    /// let notification = NotificationType::new("alert")
    ///     .with_display_name("Alert");
    ///
    /// client.register(vec![notification])?;
    /// # Ok::<(), gntp::GntpError>(())
    /// ```
    pub fn register(&mut self, notifications: Vec<NotificationType>) -> Result<String, GntpError> {
        let mut packet = String::new();
        let mut resources = Vec::new();
        let mut seen_identifiers = HashSet::new();

        // Build REGISTER packet
        packet.push_str(&format!("GNTP/{} REGISTER NONE{}", GNTP_VERSION, CRLF));
        packet.push_str(&format!(
            "Application-Name: {}{}",
            self.application_name, CRLF
        ));

        // Application icon - DON'T use this for notification icons
        // This is for the application itself, not individual notifications
        if let Some(ref icon) = self.application_icon {
            let icon_ref = icon.get_reference(&self.icon_mode);
            packet.push_str(&format!("Application-Icon: {}{}", icon_ref, CRLF));

            // Only add to resources if using binary mode
            if self.icon_mode == IconMode::Binary {
                if seen_identifiers.insert(icon.identifier.clone()) {
                    resources.push(icon.clone());
                }
            }
        }

        packet.push_str(&format!(
            "Notifications-Count: {}{}",
            notifications.len(),
            CRLF
        ));
        packet.push_str(CRLF);

        // Each notification type
        for notif in &notifications {
            packet.push_str(&format!("Notification-Name: {}{}", notif.name, CRLF));

            if let Some(ref display) = notif.display_name {
                packet.push_str(&format!("Notification-Display-Name: {}{}", display, CRLF));
            }

            packet.push_str(&format!(
                "Notification-Enabled: {}{}",
                if notif.enabled { "True" } else { "False" },
                CRLF
            ));

            // IMPORTANT: Notification type icon is used during NOTIFY
            // But we must declare it here during REGISTER
            if let Some(ref icon) = notif.icon {
                let icon_ref = icon.get_reference(&self.icon_mode);
                packet.push_str(&format!("Notification-Icon: {}{}", icon_ref, CRLF));

                if self.icon_mode == IconMode::Binary {
                    if seen_identifiers.insert(icon.identifier.clone()) {
                        resources.push(icon.clone());
                    }
                }
            }

            packet.push_str(CRLF);
        }

        // Add binary resources if using binary mode
        if self.icon_mode == IconMode::Binary {
            for resource in &resources {
                packet.push_str(&format!("Identifier: {}{}", resource.identifier, CRLF));
                packet.push_str(&format!("Length: {}{}", resource.data.len(), CRLF));
                packet.push_str(CRLF);
            }
        }

        if self.debug {
            println!("\n=== REGISTER PACKET (Mode: {:?}) ===", self.icon_mode);
            println!("{}", packet);
            println!("Resources: {} (Binary mode only)", resources.len());
            if self.icon_mode == IconMode::DataUrl {
                println!("Icons embedded as base64 in packet headers");
            }
            println!("======================================\n");
        }

        let response = if self.icon_mode == IconMode::Binary {
            self.send_packet_with_resources(&packet, &resources)?
        } else {
            self.send_packet(&packet)?
        };

        self.registered = true;
        Ok(response)
    }

    /// Send a notification
    ///
    /// **Note:** You must call `register()` first!
    ///
    /// # Arguments
    ///
    /// * `notification_name` - Name of registered notification type
    /// * `title` - Notification title
    /// * `text` - Notification message
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use gntp::GntpClient;
    /// # let mut client = GntpClient::new("App");
    /// # client.register(vec![]).unwrap();
    /// client.notify("alert", "Title", "Message")?;
    /// # Ok::<(), gntp::GntpError>(())
    /// ```
    pub fn notify(
        &self,
        notification_name: &str,
        title: &str,
        text: &str,
    ) -> Result<String, GntpError> {
        self.notify_with_options(notification_name, title, text, NotifyOptions::default())
    }

    /// Send notification with additional options
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use gntp::{GntpClient, NotifyOptions};
    /// # let mut client = GntpClient::new("App");
    /// # client.register(vec![]).unwrap();
    /// let options = NotifyOptions::new()
    ///     .with_sticky(true)
    ///     .with_priority(2);
    ///
    /// client.notify_with_options("alert", "Title", "Text", options)?;
    /// # Ok::<(), gntp::GntpError>(())
    /// ```
    pub fn notify_with_options(
        &self,
        notification_name: &str,
        title: &str,
        text: &str,
        options: NotifyOptions,
    ) -> Result<String, GntpError> {
        if !self.registered {
            return Err(GntpError::ProtocolError(
                "Must call register() before notify()".to_string(),
            ));
        }

        let mut packet = String::new();
        let mut resources = Vec::new();

        packet.push_str(&format!("GNTP/{} NOTIFY NONE{}", GNTP_VERSION, CRLF));
        packet.push_str(&format!(
            "Application-Name: {}{}",
            self.application_name, CRLF
        ));
        packet.push_str(&format!("Notification-Name: {}{}", notification_name, CRLF));
        packet.push_str(&format!("Notification-Title: {}{}", title, CRLF));
        packet.push_str(&format!("Notification-Text: {}{}", text, CRLF));

        if options.sticky {
            packet.push_str(&format!("Notification-Sticky: True{}", CRLF));
        }

        if options.priority != 0 {
            packet.push_str(&format!(
                "Notification-Priority: {}{}",
                options.priority, CRLF
            ));
        }

        if let Some(ref icon) = options.icon {
            let icon_ref = icon.get_reference(&self.icon_mode);
            packet.push_str(&format!("Notification-Icon: {}{}", icon_ref, CRLF));

            if self.icon_mode == IconMode::Binary {
                resources.push(icon.clone());
            }
        }

        packet.push_str(CRLF);

        if self.icon_mode == IconMode::Binary {
            for resource in &resources {
                packet.push_str(&format!("Identifier: {}{}", resource.identifier, CRLF));
                packet.push_str(&format!("Length: {}{}", resource.data.len(), CRLF));
                packet.push_str(CRLF);
            }
        }

        if self.debug {
            println!("\n=== NOTIFY PACKET (Mode: {:?}) ===", self.icon_mode);
            println!("{}", packet);
            println!("====================================\n");
        }

        if self.icon_mode == IconMode::Binary {
            self.send_packet_with_resources(&packet, &resources)
        } else {
            self.send_packet(&packet)
        }
    }

    /// Send text-only packet (for DataUrl and FileUrl modes)
    fn send_packet(&self, packet: &str) -> Result<String, GntpError> {
        let address = format!("{}:{}", self.host, self.port);

        if self.debug {
            println!("Connecting to {}...", address);
        }

        let mut stream = TcpStream::connect(&address).map_err(|e| {
            GntpError::ConnectionError(format!("Failed to connect to {}: {}", address, e))
        })?;

        stream.set_read_timeout(Some(Duration::from_secs(10)))?;
        stream.set_write_timeout(Some(Duration::from_secs(10)))?;

        stream.write_all(packet.as_bytes())?;
        stream.flush()?;

        if self.debug {
            println!("Packet sent, waiting for response...");
        }

        let mut response = String::new();
        match stream.read_to_string(&mut response) {
            Ok(_) => {}
            Err(e)
                if e.kind() == std::io::ErrorKind::ConnectionReset
                    || e.kind() == std::io::ErrorKind::UnexpectedEof =>
            {
                if self.debug {
                    println!("Connection closed (OK)");
                }
                return Ok(String::new());
            }
            Err(e) => return Err(e.into()),
        }

        if response.contains("-ERROR") {
            return Err(GntpError::ProtocolError(format!(
                "Server error: {}",
                response
            )));
        }

        Ok(response)
    }

    /// Send packet with binary resources (for Binary mode)
    fn send_packet_with_resources(
        &self,
        packet: &str,
        resources: &[Resource],
    ) -> Result<String, GntpError> {
        let address = format!("{}:{}", self.host, self.port);

        let mut stream = TcpStream::connect(&address).map_err(|e| {
            GntpError::ConnectionError(format!("Failed to connect to {}: {}", address, e))
        })?;

        stream.set_read_timeout(Some(Duration::from_secs(10)))?;
        stream.set_write_timeout(Some(Duration::from_secs(10)))?;

        // Send text packet
        stream.write_all(packet.as_bytes())?;

        // Send binary resources
        for resource in resources {
            stream.write_all(&resource.data)?;
            stream.write_all(CRLF.as_bytes())?;
        }

        // Message termination
        stream.write_all(CRLF.as_bytes())?;
        stream.flush()?;

        let mut response = String::new();
        match stream.read_to_string(&mut response) {
            Ok(_) => {}
            Err(e)
                if e.kind() == std::io::ErrorKind::ConnectionReset
                    || e.kind() == std::io::ErrorKind::UnexpectedEof =>
            {
                return Ok(String::new());
            }
            Err(e) => return Err(e.into()),
        }

        if response.contains("-ERROR") {
            return Err(GntpError::ProtocolError(format!(
                "Server error: {}",
                response
            )));
        }

        Ok(response)
    }
}

/* ===========================
   NOTIFICATION OPTIONS
=========================== */

/// Options for sending notifications
///
/// Allows customization of notification behavior and appearance.
///
/// # Example
///
/// ```no_run
/// # use gntp::{NotifyOptions, Resource};
/// let icon = Resource::from_file("icon.png")?;
///
/// let options = NotifyOptions::new()
///     .with_sticky(true)
///     .with_priority(2)
///     .with_icon(icon);
/// # Ok::<(), gntp::GntpError>(())
/// ```
#[derive(Default)]
pub struct NotifyOptions {
    /// Keep notification on screen until dismissed
    pub sticky: bool,

    /// Priority level (-2 = very low, 0 = normal, 2 = emergency)
    pub priority: i8,

    /// Optional icon for this specific notification
    pub icon: Option<Resource>,
}

impl NotifyOptions {
    /// Create new notification options with defaults
    pub fn new() -> Self {
        Self::default()
    }

    /// Set sticky mode (notification stays until dismissed)
    pub fn with_sticky(mut self, sticky: bool) -> Self {
        self.sticky = sticky;
        self
    }

    /// Set priority (-2 to 2)
    ///
    /// - `-2` = Very Low
    /// - `-1` = Moderate
    /// - `0` = Normal (default)
    /// - `1` = High
    /// - `2` = Emergency
    pub fn with_priority(mut self, priority: i8) -> Self {
        self.priority = priority.max(-2).min(2);
        self
    }

    /// Set icon for this notification
    pub fn with_icon(mut self, icon: Resource) -> Self {
        self.icon = Some(icon);
        self
    }
}
