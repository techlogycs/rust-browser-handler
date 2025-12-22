#[cfg(windows)]
use log::{info, warn};
#[cfg(windows)]
use std::io;
#[cfg(windows)]
use winreg::{RegKey, enums::*};

#[cfg(windows)]
pub fn set_as_default_handler() -> io::Result<()> {
    info!("Attempting to register as default browser handler...");

    let exe_path = std::env::current_exe()?
        .to_str()
        .ok_or_else(|| io::Error::other("Failed to get executable path"))?
        .to_string();

    // Register custom URL scheme (rust-browser-handler)
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let classes_key = hkcu.open_subkey_with_flags("Software\\Classes", KEY_ALL_ACCESS)?;

    let scheme_key = classes_key.create_subkey("rust-browser-handler")?;
    scheme_key
        .0
        .set_value("", &"URL:Rust Browser Handler Protocol".to_string())?;
    scheme_key.0.set_value("URL Protocol", &"")?;

    let default_icon_key = scheme_key.0.create_subkey("DefaultIcon")?;
    default_icon_key
        .0
        .set_value("", &format!("{},0", exe_path))?;

    let command_key = scheme_key.0.create_subkey("shell\\open\\command")?;
    command_key
        .0
        .set_value("", &format!("\"{}\" \"%1\"", exe_path))?;

    // Register as a capable application
    let registered_apps_key =
        hkcu.open_subkey_with_flags("Software\\RegisteredApplications", KEY_ALL_ACCESS)?;
    registered_apps_key.set_value(
        "rust_browser_handler.exe",
        &"Software\\Clients\\StartMenuInternet\\rust_browser_handler.exe\\Capabilities",
    )?;

    let capabilities_key = hkcu.create_subkey(
        "Software\\Clients\\StartMenuInternet\\rust_browser_handler.exe\\Capabilities",
    )?;
    capabilities_key.0.set_value(
        "ApplicationDescription",
        &"Handles URLs based on defined rules.",
    )?;
    capabilities_key
        .0
        .set_value("ApplicationIcon", &format!("{},0", exe_path))?;
    capabilities_key
        .0
        .set_value("ApplicationName", &"Rust Browser Handler")?;

    let url_associations_key = capabilities_key.0.create_subkey("URLAssociations")?;
    url_associations_key
        .0
        .set_value("http", &"rust-browser-handler")?;
    url_associations_key
        .0
        .set_value("https", &"rust-browser-handler")?;

    // Set as default for Start Menu Internet
    let start_menu_internet_key = hkcu.create_subkey("Software\\Clients\\StartMenuInternet")?;
    start_menu_internet_key
        .0
        .set_value("", &"rust_browser_handler.exe")?;

    // Set ProgIDs for http and https to make it the default handler
    let http_key = hkcu.create_subkey("Software\\Classes\\http")?;
    http_key.0.set_value("", &"rust-browser-handler")?;
    let https_key = hkcu.create_subkey("Software\\Classes\\https")?;
    https_key.0.set_value("", &"rust-browser-handler")?;

    info!("Registry entries created/updated.");
    println!("Registry entries created/updated.");
    println!("The application is now registered as the default browser handler.");

    Ok(())
}

#[cfg(windows)]
pub fn unregister_handler() -> io::Result<()> {
    info!("Attempting to unregister as default browser handler...");

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);

    // Remove URLAssociations
    if let Ok(capabilities_key) = hkcu.open_subkey_with_flags(
        "Software\\Clients\\StartMenuInternet\\rust_browser_handler.exe\\Capabilities",
        KEY_ALL_ACCESS,
    ) {
        let _ = capabilities_key.delete_subkey_all("URLAssociations");
        info!("URLAssociations key removed or did not exist.");
    } else {
        warn!("Could not open Capabilities key to remove URLAssociations.");
    }

    // Remove Capabilities key itself (which is rust_browser_handler.exe under StartMenuInternet)
    if let Ok(start_menu_internet_key) =
        hkcu.open_subkey_with_flags("Software\\Clients\\StartMenuInternet", KEY_ALL_ACCESS)
    {
        let _ = start_menu_internet_key.delete_subkey_all("rust_browser_handler.exe");
        let _ = start_menu_internet_key.set_value("", &""); // Clear the default
        info!(
            "Software\\Clients\\StartMenuInternet\\rust_browser_handler.exe key removed or did not exist."
        );
    } else {
        warn!("Could not open Software\\Clients\\StartMenuInternet key.");
    }

    // Remove from RegisteredApplications
    if let Ok(registered_apps_key) =
        hkcu.open_subkey_with_flags("Software\\RegisteredApplications", KEY_ALL_ACCESS)
    {
        let _ = registered_apps_key.delete_value("rust_browser_handler.exe");
        info!(
            "rust_browser_handler.exe value removed from RegisteredApplications or did not exist."
        );
    } else {
        warn!("Could not open RegisteredApplications key.");
    }

    // Clear ProgID settings for http and https
    if let Ok(http_key) = hkcu.open_subkey_with_flags("Software\\Classes\\http", KEY_ALL_ACCESS) {
        let _ = http_key.set_value("", &"");
        info!("Cleared default ProgID for http.");
    } else {
        warn!("Could not open Software\\Classes\\http key.");
    }
    if let Ok(https_key) = hkcu.open_subkey_with_flags("Software\\Classes\\https", KEY_ALL_ACCESS) {
        let _ = https_key.set_value("", &"");
        info!("Cleared default ProgID for https.");
    } else {
        warn!("Could not open Software\\Classes\\https key.");
    }

    // Remove custom URL scheme (rust-browser-handler)
    if let Ok(classes_key) = hkcu.open_subkey_with_flags("Software\\Classes", KEY_ALL_ACCESS) {
        let _ = classes_key.delete_subkey_all("rust-browser-handler");
        info!("Software\\Classes\\rust-browser-handler key removed or did not exist.");
    } else {
        warn!("Could not open Software\\Classes key.");
    }

    info!("Unregistration process completed. Some keys/values might have already been absent.");
    println!("Unregistration process completed.");
    println!(
        "You may need to manually select a new default browser in Windows Settings if Rust Browser Handler was previously set."
    );

    Ok(())
}

#[cfg(not(windows))]
use std::io;

#[cfg(not(windows))]
pub fn set_as_default_handler() -> io::Result<()> {
    Err(io::Error::new(
        io::ErrorKind::Unsupported,
        "This application only supports Windows for default browser registration",
    ))
}

#[cfg(not(windows))]
pub fn unregister_handler() -> io::Result<()> {
    Err(io::Error::new(
        io::ErrorKind::Unsupported,
        "This application only supports Windows for default browser unregistration",
    ))
}
