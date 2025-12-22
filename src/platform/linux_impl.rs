use super::PlatformHandler;
use std::collections::HashSet;
use std::fs;
use std::path::Path;
use std::process::Command;

pub struct LinuxHandler;

impl PlatformHandler for LinuxHandler {
    fn find_browsers(&self) -> Vec<String> {
        let mut browsers = HashSet::new();

        // Common browsers to check
        let browser_commands = [
            "google-chrome",
            "google-chrome-stable",
            "chromium",
            "chromium-browser",
            "firefox",
            "firefox-esr",
            "librewolf",
            "brave-browser",
            "opera",
            "vivaldi",
            "thorium-browser",
            "floorp",
        ];

        for cmd in &browser_commands {
            if let Ok(output) = Command::new("which").arg(cmd).output()
                && output.status.success()
                && let Ok(path_str) = String::from_utf8(output.stdout)
            {
                let path = path_str.trim();
                if !path.is_empty() && Path::new(path).exists() {
                    browsers.insert(crate::browser_discovery::normalize_path(path));
                }
            }
        }

        // Also check common installation paths
        let common_paths = [
            "/usr/bin/google-chrome",
            "/usr/bin/google-chrome-stable",
            "/usr/bin/chromium",
            "/usr/bin/chromium-browser",
            "/usr/bin/firefox",
            "/usr/bin/firefox-esr",
            "/usr/bin/librewolf",
            "/usr/bin/brave-browser",
            "/usr/bin/opera",
            "/usr/bin/vivaldi",
            "/usr/bin/thorium-browser",
            "/usr/bin/floorp",
            "/opt/google/chrome/chrome",
            "/opt/brave.com/brave/brave",
            "/opt/opera/opera",
            "/opt/vivaldi/vivaldi",
        ];

        for path in &common_paths {
            if Path::new(path).exists() {
                browsers.insert(crate::browser_discovery::normalize_path(path));
            }
        }

        browsers.into_iter().collect()
    }

    fn set_as_default_handler(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Get the executable path
        let exe_path = std::env::current_exe()?
            .to_str()
            .ok_or("Failed to get executable path")?
            .to_string();

        // Create .desktop file content
        let desktop_content = format!(
            r#"[Desktop Entry]
Version=1.0
Type=Application
Name=Rust Browser Handler
Exec="{}" %u
Terminal=false
Categories=Network;WebBrowser;
MimeType=x-scheme-handler/http;x-scheme-handler/https;
StartupNotify=false
"#,
            exe_path
        );

        // Create applications directory if it doesn't exist
        let desktop_dir = dirs::data_dir()
            .ok_or("Could not find data directory")?
            .join("applications");
        fs::create_dir_all(&desktop_dir)?;

        // Write the .desktop file
        let desktop_file = desktop_dir.join("rust-browser-handler.desktop");
        fs::write(&desktop_file, desktop_content)?;

        // Set MIME type associations using xdg-mime
        let http_status = Command::new("xdg-mime")
            .args(&[
                "default",
                "rust-browser-handler.desktop",
                "x-scheme-handler/http",
            ])
            .status();

        let https_status = Command::new("xdg-mime")
            .args(&[
                "default",
                "rust-browser-handler.desktop",
                "x-scheme-handler/https",
            ])
            .status();

        match (http_status, https_status) {
            (Ok(http), Ok(https)) if http.success() && https.success() => {
                println!("Successfully registered as default browser using XDG standards.");
                println!(
                    "The application is now set as the default handler for HTTP and HTTPS URLs."
                );
                Ok(())
            }
            _ => {
                // Clean up the desktop file if MIME association failed
                let _ = fs::remove_file(&desktop_file);
                Err("Failed to set MIME type associations. Ensure xdg-mime is available.".into())
            }
        }
    }

    fn unregister_handler(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Remove the .desktop file
        let desktop_file = dirs::data_dir()
            .ok_or("Could not find data directory")?
            .join("applications/rust-browser-handler.desktop");

        if desktop_file.exists() {
            fs::remove_file(desktop_file)?;
            println!("Removed desktop file and unregistered as default browser.");
        } else {
            println!("Desktop file not found - application may not be registered.");
        }

        // Note: We can't easily reset to the previous default browser
        // The system will fall back to whatever was previously configured
        // or the distribution's default browser
        println!("Browser associations have been reset. The system will use its default browser.");

        Ok(())
    }

    fn is_default_handler(&self) -> bool {
        // Check if our desktop file is associated with HTTP scheme
        if let Ok(output) = Command::new("xdg-mime")
            .args(&["query", "default", "x-scheme-handler/http"])
            .output()
        {
            if let Ok(mime_default) = String::from_utf8(output.stdout) {
                return mime_default.trim() == "rust-browser-handler.desktop";
            }
        }

        // Fallback: check if desktop file exists
        if let Some(data_dir) = dirs::data_dir() {
            let desktop_file = data_dir.join("applications/rust-browser-handler.desktop");
            return desktop_file.exists();
        }

        false
    }
}
