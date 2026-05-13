// File: src/sendgrowl.rs
// Author: Hadi Cahyadi <cumulus13@gmail.com>
// Date: 2025-12-16
// Description: Windows-safe Growl/GNTP notifications
// License: MIT

use clap::{ArgAction, Parser};
use clap_version_flag::colorful_version_full;

// IMPORTANT: Use "gntp" not "sendgrowl" because library name is "gntp"
use gntp::{GntpClient, NotificationType, NotifyOptions, Resource};

use std::{env, path::PathBuf};

/* ===========================
   CLI
=========================== */
#[derive(Parser, Debug)]
#[command(
    name = "sendgrowl",
    version = env!("CARGO_PKG_VERSION"),
    disable_version_flag = true,
    about = "Send Growl notifications to multiple hosts safely on Windows"
)]
struct Args {
    #[arg(short='V', long, action=ArgAction::SetTrue)]
    version: bool,

    app_name: String,
    event_name: String,
    title: String,
    text: String,

    #[arg(short='H', long="host", value_name="HOST[:PORT]", num_args=1..)]
    host: Vec<String>,

    #[arg(short = 'P', long, default_value = "23053")]
    port: u16,

    #[arg(short = 'i', long)]
    icon: Option<PathBuf>,

    #[arg(short = 's', long)]
    sticky: bool,

    #[arg(short = 'p', long, default_value = "0")]
    priority: i8,

    #[arg(short='v', long, action=ArgAction::Count)]
    verbose: u8,

    #[arg(short = 'r', long, default_value = "0")]
    retry: u8,

    #[arg(long, default_value = "2000")]
    retry_delay: u64,

    #[arg(long, default_value="dataurl", value_parser=parse_icon_mode)]
    icon_mode: gntp::IconMode,
}

fn parse_icon_mode(s: &str) -> Result<gntp::IconMode, String> {
    match s.to_lowercase().as_str() {
        "binary" => Ok(gntp::IconMode::Binary),
        "fileurl" => Ok(gntp::IconMode::FileUrl),
        "dataurl" => Ok(gntp::IconMode::DataUrl),
        "httpurl" => Ok(gntp::IconMode::HttpUrl),
        "auto" => Ok(gntp::IconMode::Auto),
        _ => Err(format!(
            "Invalid icon mode: {}. Use: binary, fileurl, dataurl, httpurl, auto",
            s
        )),
    }
}

/* ===========================
   ICON HELPER
=========================== */
fn load_icon(path: &Option<PathBuf>, verbose: u8) -> Option<Resource> {
    let pathbuf = if let Some(p) = path {
        p.clone()
    } else {
        let exe_dir = env::current_exe().ok()?.parent()?.to_path_buf();
        exe_dir.join("growl.png")
    };

    match Resource::from_file(&pathbuf) {
        Ok(icon) => {
            if verbose > 0 {
                println!("✓ Icon loaded: {}", pathbuf.display());
            }
            Some(icon)
        }
        Err(e) => {
            eprintln!("⚠ Icon ignored: {}", e);
            None
        }
    }
}

/* ===========================
   SEND NOTIFICATION WITH RETRY

   IMPORTANT: Some GNTP servers (like Growl for Android) don't like
   the same icon referenced multiple times. We only attach icon to
   notification type, not to application or notify options.

   Android and mobile devices may have network issues, so retry is useful.
=========================== */
fn send_growl(
    host: &str,
    port: u16,
    app_name: &str,
    event_name: &str,
    title: &str,
    text: &str,
    icon: Option<Resource>,
    sticky: bool,
    priority: i8,
    verbose: u8,
    icon_mode: gntp::IconMode,
    retry_count: u8,
    retry_delay_ms: u64,
) -> Result<(), Box<dyn std::error::Error>> {
    let host = if cfg!(windows) && (host == "127.0.0.1" || host == "::1") {
        if verbose > 0 {
            println!("⚠ '127.0.0.1' converted to 'localhost' for Windows Growl");
        }
        "localhost".to_string()
    } else {
        host.to_string()
    };

    let mut last_error: Option<Box<dyn std::error::Error>> = None;
    let max_attempts = retry_count + 1;

    for attempt in 1..=max_attempts {
        if attempt > 1 {
            if verbose > 0 {
                println!(
                    "⚠ Retry attempt {}/{} after {}ms delay...",
                    attempt - 1,
                    retry_count,
                    retry_delay_ms
                );
            }
            std::thread::sleep(std::time::Duration::from_millis(retry_delay_ms));
        }

        // FIX: Don't attach icon to client (causes issues with some servers)
        let mut client = GntpClient::new(app_name)
            .with_host(&host)
            .with_port(port)
            .with_icon_mode(icon_mode.clone())
            .with_debug(verbose > 1);

        // FIX: Only attach icon to notification type (safest approach)
        let mut notification = NotificationType::new(event_name)
            .with_display_name(event_name)
            .with_enabled(true);

        if let Some(ref icon) = icon {
            notification = notification.with_icon(icon.clone());
            if verbose > 0 && attempt == 1 {
                println!("✓ Icon attached to notification type");
            }
        }

        if verbose > 0 && attempt == 1 {
            println!("Registering with Growl on {}:{}...", host, port);
        }

        match client.register(vec![notification]) {
            Ok(_) => {
                // Registration successful, now try to send notification
                let options = NotifyOptions::new()
                    .with_sticky(sticky)
                    .with_priority(priority);

                match client.notify_with_options(event_name, title, text, options) {
                    Ok(_) => {
                        if verbose > 0 {
                            if attempt > 1 {
                                println!(
                                    "✓ Notification sent to {}:{} (succeeded on retry {})",
                                    host,
                                    port,
                                    attempt - 1
                                );
                            } else {
                                println!("✓ Notification sent to {}:{}", host, port);
                            }
                        }
                        return Ok(());
                    }
                    Err(e) => {
                        last_error = Some(Box::new(e));
                        if verbose > 0 && attempt < max_attempts {
                            println!("⚠ Notify failed on {}:{}, will retry...", host, port);
                        }
                    }
                }
            }
            Err(e) => {
                last_error = Some(Box::new(e));
                if verbose > 0 && attempt < max_attempts {
                    println!("⚠ Registration failed on {}:{}, will retry...", host, port);
                }
            }
        }
    }

    // All attempts failed
    Err(last_error.unwrap_or_else(|| "Unknown error".into()))
}

/* ===========================
   MAIN
=========================== */
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let os_args: Vec<String> = std::env::args().collect();
    if os_args.len() == 2 && (os_args[1] == "-V" || os_args[1] == "--version") {
        let version = colorful_version_full!(
            "sendgrowl",
            env!("CARGO_PKG_VERSION"),
            env!("CARGO_PKG_AUTHORS")
        );
        version.print_and_exit();
    }

    let args = Args::parse();

    if args.version {
        colorful_version_full!(
            "sendgrowl",
            env!("CARGO_PKG_VERSION"),
            env!("CARGO_PKG_AUTHORS")
        )
        .print_and_exit();
    }

    let icon = load_icon(&args.icon, args.verbose);

    // Prepare hosts list
    let hosts = if args.host.is_empty() {
        vec![format!("localhost:{}", args.port)]
    } else {
        args.host
            .iter()
            .map(|h| {
                if h.contains(':') {
                    h.clone()
                } else {
                    format!("{}:{}", h, args.port)
                }
            })
            .collect()
    };

    // Send notification to each host directly
    for h in hosts {
        let mut split = h.split(':');
        let host = split.next().unwrap();
        let port = split
            .next()
            .and_then(|p| p.parse::<u16>().ok())
            .unwrap_or(args.port);

        if let Err(e) = send_growl(
            host,
            port,
            &args.app_name,
            &args.event_name,
            &args.title,
            &args.text,
            icon.clone(),
            args.sticky,
            args.priority,
            args.verbose,
            args.icon_mode.clone(),
            args.retry,
            args.retry_delay,
        ) {
            eprintln!("❌ Failed to send to {}:{} - {}", host, port, e);
        }
    }

    Ok(())
}
