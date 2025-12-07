//! # GNTP - Growl Notification Transport Protocol Client
//!
//! A complete implementation of the Growl Notification Transport Protocol (GNTP)
//! for sending desktop notifications to Growl-compatible clients.
//!
//! ## Features
//!
//! - Full GNTP 1.0 protocol implementation
//! - Binary icon support
//! - Multiple notification types
//! - Priority and sticky notifications
//! - Error handling
//! - No external dependencies (except uuid)
//!
//! ## Quick Start
//!
//! ```no_run
//! use gntp::{GntpClient, NotificationType};
//!
//! # fn main() -> Result<(), gntp::GntpError> {
//! // Create client
//! let mut client = GntpClient::new("My App");
//!
//! // Define notification type
//! let notification = NotificationType::new("alert")
//!     .with_display_name("Alert Notification");
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
//! ## Protocol Specification
//!
//! GNTP requires two separate steps:
//!
//! 1. **REGISTER** - Register your application and notification types (once)
//! 2. **NOTIFY** - Send notifications (multiple times)
//!
//! ## Examples
//!
//! See the [examples](https://github.com/cumulus13/gntp/tree/master/examples) directory for more usage examples.

use std::io::{Read, Write};
use std::net::TcpStream;
use std::path::Path;
use std::fs;
use std::time::Duration;

const GNTP_VERSION: &str = "1.0";
const DEFAULT_PORT: u16 = 23053;
const CRLF: &str = "\r\n";

/// GNTP errors
#[derive(Debug)]
pub enum GntpError {
    /// Connection error
    ConnectionError(String),
    /// I/O error
    IoError(String),
    /// Protocol error
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

/// Binary resource (icon)
#[derive(Clone)]
pub struct Resource {
    pub identifier: String,
    pub data: Vec<u8>,
}

impl Resource {
    /// Load resource from file
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, GntpError> {
        let data = fs::read(&path).map_err(|e| {
            GntpError::IoError(format!("Failed to read file: {}", e))
        })?;
        
        let identifier = uuid::Uuid::new_v4().to_string();
        
        Ok(Resource {
            identifier,
            data,
        })
    }
    
    /// Create resource from bytes
    pub fn from_bytes(data: Vec<u8>) -> Self {
        Resource {
            identifier: uuid::Uuid::new_v4().to_string(),
            data,
        }
    }
}

/// Notification type definition
#[derive(Clone)]
pub struct NotificationType {
    pub name: String,
    pub display_name: Option<String>,
    pub enabled: bool,
    pub icon: Option<Resource>,
}

impl NotificationType {
    /// Create new notification type
    pub fn new(name: &str) -> Self {
        NotificationType {
            name: name.to_string(),
            display_name: None,
            enabled: true,
            icon: None,
        }
    }
    
    /// Set display name
    pub fn with_display_name(mut self, display_name: &str) -> Self {
        self.display_name = Some(display_name.to_string());
        self
    }
    
    /// Set icon
    pub fn with_icon(mut self, icon: Resource) -> Self {
        self.icon = Some(icon);
        self
    }
    
    /// Set enabled
    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
}

/// GNTP Client
pub struct GntpClient {
    pub host: String,
    pub port: u16,
    pub application_name: String,
    pub application_icon: Option<Resource>,
    pub password: Option<String>,
    registered: bool,
}

impl GntpClient {
    /// Create new GNTP client
    pub fn new(application_name: &str) -> Self {
        GntpClient {
            host: "localhost".to_string(),
            port: DEFAULT_PORT,
            application_name: application_name.to_string(),
            application_icon: None,
            password: None,
            registered: false,
        }
    }
    
    /// Set server host
    pub fn with_host(mut self, host: &str) -> Self {
        self.host = host.to_string();
        self
    }
    
    /// Set server port
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
    pub fn with_password(mut self, password: &str) -> Self {
        self.password = Some(password.to_string());
        self
    }
    
    /// Register application with Growl
    /// This MUST be called before sending notifications
    pub fn register(&mut self, notifications: Vec<NotificationType>) -> Result<String, GntpError> {
        let mut packet = String::new();
        let mut resources = Vec::new();
        
        // Build REGISTER packet
        packet.push_str(&format!("GNTP/{} REGISTER NONE{}", GNTP_VERSION, CRLF));
        packet.push_str(&format!("Application-Name: {}{}", self.application_name, CRLF));
        
        // Application icon
        if let Some(ref icon) = self.application_icon {
            packet.push_str(&format!("Application-Icon: x-growl-resource://{}{}", 
                icon.identifier, CRLF));
            resources.push(icon.clone());
        }
        
        // Notification count
        packet.push_str(&format!("Notifications-Count: {}{}", notifications.len(), CRLF));
        packet.push_str(CRLF);
        
        // Each notification type
        for notif in &notifications {
            packet.push_str(&format!("Notification-Name: {}{}", notif.name, CRLF));
            
            if let Some(ref display) = notif.display_name {
                packet.push_str(&format!("Notification-Display-Name: {}{}", display, CRLF));
            }
            
            packet.push_str(&format!("Notification-Enabled: {}{}", 
                if notif.enabled { "True" } else { "False" }, CRLF));
            
            if let Some(ref icon) = notif.icon {
                packet.push_str(&format!("Notification-Icon: x-growl-resource://{}{}", 
                    icon.identifier, CRLF));
                resources.push(icon.clone());
            }
            
            packet.push_str(CRLF);
        }
        
        // Add binary resources
        for resource in &resources {
            packet.push_str(&format!("Identifier: {}{}", resource.identifier, CRLF));
            packet.push_str(&format!("Length: {}{}", resource.data.len(), CRLF));
            packet.push_str(CRLF);
        }
        
        // Send packet
        let response = self.send_packet_fixed(&packet, &resources)?;
        self.registered = true;
        
        Ok(response)
    }
    
    /// Send a notification
    /// Must call register() first!
    pub fn notify(&self, 
                  notification_name: &str, 
                  title: &str, 
                  text: &str) -> Result<String, GntpError> {
        self.notify_with_options(notification_name, title, text, 
                                 NotifyOptions::default())
    }
    
    /// Send a notification with options
    pub fn notify_with_options(&self,
                               notification_name: &str,
                               title: &str,
                               text: &str,
                               options: NotifyOptions) -> Result<String, GntpError> {
        if !self.registered {
            return Err(GntpError::ProtocolError(
                "Must call register() before notify()".to_string()
            ));
        }
        
        let mut packet = String::new();
        let mut resources = Vec::new();
        
        // Build NOTIFY packet
        packet.push_str(&format!("GNTP/{} NOTIFY NONE{}", GNTP_VERSION, CRLF));
        packet.push_str(&format!("Application-Name: {}{}", self.application_name, CRLF));
        packet.push_str(&format!("Notification-Name: {}{}", notification_name, CRLF));
        packet.push_str(&format!("Notification-Title: {}{}", title, CRLF));
        packet.push_str(&format!("Notification-Text: {}{}", text, CRLF));
        
        // Optional fields
        if options.sticky {
            packet.push_str(&format!("Notification-Sticky: True{}", CRLF));
        }
        
        if options.priority != 0 {
            packet.push_str(&format!("Notification-Priority: {}{}", options.priority, CRLF));
        }
        
        if let Some(ref icon) = options.icon {
            packet.push_str(&format!("Notification-Icon: x-growl-resource://{}{}", 
                icon.identifier, CRLF));
            resources.push(icon.clone());
        }
        
        packet.push_str(CRLF);
        
        // Binary resources section
        for resource in &resources {
            packet.push_str(&format!("Identifier: {}{}", resource.identifier, CRLF));
            packet.push_str(&format!("Length: {}{}", resource.data.len(), CRLF));
            packet.push_str(CRLF);
        }
        
        // Send packet
        self.send_packet_fixed(&packet, &resources)
    }
    
    /// Send packet with correct binary resource handling
    fn send_packet_fixed(&self, packet: &str, resources: &[Resource]) -> Result<String, GntpError> {
        let address = format!("{}:{}", self.host, self.port);
        
        let mut stream = TcpStream::connect(&address)
            .map_err(|e| GntpError::ConnectionError(format!("Failed to connect to {}: {}", 
                address, e)))?;
        
        stream.set_read_timeout(Some(Duration::from_secs(5)))?;
        stream.set_write_timeout(Some(Duration::from_secs(5)))?;
        
        // Send text packet
        stream.write_all(packet.as_bytes())?;
        
        // Send binary resources
        for resource in resources {
            stream.write_all(&resource.data)?;
            stream.write_all(CRLF.as_bytes())?;
        }
        
        // 3. Message termination - double CRLF
        stream.write_all(CRLF.as_bytes())?;
        
        stream.flush()?;
        
        // Read response
        let mut response = String::new();
        stream.read_to_string(&mut response)?;
        
        // Check for errors
        if response.contains("-ERROR") {
            return Err(GntpError::ProtocolError(format!("Server error: {}", response)));
        }
        
        Ok(response)
    }
}

/// Options for notify
#[derive(Default)]
pub struct NotifyOptions {
    pub sticky: bool,
    pub priority: i8, // -2 to 2
    pub icon: Option<Resource>,
}

impl NotifyOptions {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn with_sticky(mut self, sticky: bool) -> Self {
        self.sticky = sticky;
        self
    }
    
    pub fn with_priority(mut self, priority: i8) -> Self {
        self.priority = priority.max(-2).min(2);
        self
    }
    
    pub fn with_icon(mut self, icon: Resource) -> Self {
        self.icon = Some(icon);
        self
    }
}
// ============================================================================
// HELPER: Debug function to see the packets sent
// ============================================================================

#[allow(dead_code)]
fn debug_packet(packet: &str, resources: &[Resource]) {
    println!("=== GNTP PACKET DEBUG ===");
    println!("{}", packet);
    
    for (i, resource) in resources.iter().enumerate() {
        println!("Binary Resource #{}: {} bytes (ID: {})", 
            i + 1, resource.data.len(), resource.identifier);
    }
    
    println!("=== END PACKET ===\n");
}