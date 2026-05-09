use std::collections::HashMap;
use std::path::Path;

/// Mapping from executable file names to official browser names
pub static BROWSER_NAME_MAP: std::sync::OnceLock<HashMap<&'static str, &'static str>> =
    std::sync::OnceLock::new();

pub fn get_browser_map() -> &'static HashMap<&'static str, &'static str> {
    BROWSER_NAME_MAP.get_or_init(|| {
        let mut m = HashMap::new();
        m.insert("chrome", "Google Chrome");
        m.insert("chromium", "Chromium");
        m.insert("chromium-browser", "Chromium");
        m.insert("firefox", "Mozilla Firefox");
        m.insert("firefox-esr", "Mozilla Firefox ESR");
        m.insert("librewolf", "LibreWolf");
        m.insert("brave-browser", "Brave");
        m.insert("opera", "Opera");
        m.insert("vivaldi", "Vivaldi");
        m.insert("thorium-browser", "Thorium");
        m.insert("floorp", "Floorp");
        m
    })
}

/// Gets a displayable browser name from its path using the official name if possible
pub fn get_browser_name_from_path(path: &str) -> String {
    Path::new(path)
        .file_name()
        .and_then(|name| name.to_str())
        .and_then(|name| {
            let lower = name.to_ascii_lowercase();
            get_browser_map()
                .get(lower.as_str())
                .map(|s| s.to_string())
                .or(Some(name.to_string()))
        })
        .unwrap_or_else(|| path.to_string())
}

/**
 * Normalizes a path for cross-platform deduplication.
 *
 * Strategy:
 * - Prefer filesystem-based canonicalization when available.
 * - On error, fall back to a deterministic, purely syntactic normalization:
 *   - Convert '\' to '/' so separators are consistent across platforms.
 *   - Remove redundant trailing separators (except for root "/").
 */
pub fn normalize_path(path: &str) -> String {
    match Path::new(path).canonicalize() {
        Ok(canonical) => canonical.to_string_lossy().replace('\\', "/"),
        Err(_) => normalize_fallback(path),
    }
}

/// Syntactic, filesystem-independent normalization used when `canonicalize` fails.
fn normalize_fallback(path: &str) -> String {
    // Normalize separators to forward slashes
    let mut normalized = path.replace('\\', "/");

    // Collapse trailing separators, but keep "/" as-is
    while normalized.len() > 1 && normalized.ends_with('/') {
        normalized.pop();
    }

    normalized
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_browser_name_from_path() {
        assert_eq!(
            get_browser_name_from_path("C:/Program Files/Google/Chrome/Application/chrome"),
            "Google Chrome".to_string()
        );
        assert_eq!(
            get_browser_name_from_path("C:/Program Files/Mozilla Firefox/firefox"),
            "Mozilla Firefox".to_string()
        );
        assert_eq!(get_browser_name_from_path(""), "".to_string());
        assert_eq!(
            get_browser_name_from_path("C:/not_a_browser.txt"),
            "not_a_browser.txt".to_string()
        );
    }

    #[test]
    fn test_normalize_path() {
        assert_eq!(
            normalize_path("C:\\Program Files\\Google\\Chrome\\Application\\chrome"),
            "C:/Program Files/Google/Chrome/Application/chrome".to_string()
        );
        assert_eq!(
            normalize_path("/usr/bin/firefox"),
            "/usr/bin/firefox".to_string()
        );
        assert_eq!(
            normalize_path("C:/not_a_browser.txt/"),
            "C:/not_a_browser.txt".to_string()
        );
    }

    #[test]
    fn test_fallback_normalize_path() {
        // Simulate a path that cannot be canonicalized (e.g., non-existent path)
        let non_existent_path = "C:/this/path/does/not/exist";
        let normalized = normalize_path(non_existent_path);
        assert_eq!(normalized, "C:/this/path/does/not/exist".to_string());
    }
}
