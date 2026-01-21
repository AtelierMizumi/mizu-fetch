use std::fs;

pub struct GpuInfo {
    pub names: Vec<String>,
}

impl GpuInfo {
    pub fn new() -> Self {
        let mut names = Vec::new();

        // Try native Linux detection first
        if let Ok(entries) = fs::read_dir("/sys/class/drm/") {
            for entry in entries.flatten() {
                let path = entry.path();
                // We are looking for directories like "card0", "card1"
                // but NOT "card0-HDMI-A-1" (connectors)
                // usually checking if it has a "device" symlink is a good indicator
                let filename = path.file_name().unwrap_or_default().to_string_lossy();

                if filename.starts_with("card") && !filename.contains('-') {
                    // Try to read vendor and device
                    let vendor_path = path.join("device/vendor");
                    let device_path = path.join("device/device");

                    if let (Ok(vendor), Ok(device)) = (
                        fs::read_to_string(&vendor_path),
                        fs::read_to_string(&device_path),
                    ) {
                        let vendor_id = vendor.trim().trim_start_matches("0x");
                        let device_id = device.trim().trim_start_matches("0x");

                        // Simple mapping (In a real app, use pci.ids database or a crate)
                        let vendor_name = match vendor_id {
                            "8086" => "Intel",
                            "10de" => "NVIDIA",
                            "1002" => "AMD",
                            "1022" => "AMD",
                            _ => "Unknown",
                        };

                        names.push(format!("{} Device {}", vendor_name, device_id));
                    }
                }
            }
        }

        // Fallback to lspci if native failed or returned nothing
        if names.is_empty()
            && let Ok(output) = std::process::Command::new("sh")
                .arg("-c")
                .arg("lspci -mm | grep -E -i \"VGA|3D\"")
                .output()
        {
            let raw = String::from_utf8(output.stdout).unwrap_or_default();
            for line in raw.lines() {
                let parts: Vec<&str> = line.split('"').collect();
                if parts.len() >= 6 {
                    names.push(format!("{} {}", parts[3], parts[5]).trim().to_string());
                }
            }
        }

        if names.is_empty() {
            names.push("Unknown GPU".to_string());
        }

        Self { names }
    }
}
