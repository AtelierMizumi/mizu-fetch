use std::process::Command;

pub struct DisplayInfo;

impl DisplayInfo {
    pub fn new() -> String {
        // Try generic Wayland/X11 tools or specific DE tools

        // 1. Try wlr-randr (Wayland generic)
        if let Ok(output) = Command::new("wlr-randr").output() {
            if output.status.success() {
                // Parsing logic would go here, but wlr-randr output is complex.
                // Simplified for now.
            }
        }

        // 2. Try xrandr (X11 generic) - often works on Wayland via XWayland but might report virtual screens
        // Let's rely on a simpler approach if possible: parsing /sys/class/drm or using a command like `kscreen-doctor` for KDE since the user uses KDE.

        // KDE Plasma specific
        if let Ok(output) = Command::new("kscreen-doctor").arg("-o").output() {
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                // Output example:
                // Output: 1 eDP-1 enabled connected priority 1 pos 0,0 size 1920x1080@144Hz scale 1.0
                for line in stdout.lines() {
                    if line.contains("enabled") && line.contains("connected") {
                        // Extract resolution and refresh rate
                        // naive parsing
                        let parts: Vec<&str> = line.split_whitespace().collect();
                        for part in parts {
                            if part.contains('x') && part.contains('@') {
                                return part.to_string(); // Found "1920x1080@144Hz"
                            }
                            // Sometimes they are separate "1920x1080 60Hz"
                        }
                    }
                }
            }
        }

        // Fallback: Generic X11/XWayland resolution detection via xrandr
        if let Ok(output) = Command::new("xrandr").arg("--current").output() {
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                // connected primary 1920x1080+0+0
                for line in stdout.lines() {
                    if line.contains(" connected") {
                        // Find the resolution part. usually after "primary" or the device name
                        // eDP-1 connected primary 1920x1080+0+0 ...
                        if let Some(res_start) = line.find(|c: char| c.is_numeric()) {
                            // This is very loose, but catching "1920x1080"
                            let rest = &line[res_start..];
                            if let Some(end) = rest.find('+') {
                                let res = &rest[..end];
                                // Try to find refresh rate on the next line usually
                                // But on one line mode it might be: 1920x1080 144.00*+
                                return res.to_string();
                            }
                        }
                    }
                }
            }
        }

        "Unknown".to_string()
    }
}
