use std::fs;

pub struct BatteryInfo {
    // Fields preserved for potential future structured use, though only string output is used now
    #[allow(dead_code)]
    pub percentage: u8,
    #[allow(dead_code)]
    pub status: String,
}

impl BatteryInfo {
    pub fn new() -> String {
        let mut battery_output = Vec::new();

        // Check for power_supply class
        if let Ok(entries) = fs::read_dir("/sys/class/power_supply") {
            for entry in entries.flatten() {
                let path = entry.path();
                let name = path.file_name().unwrap_or_default().to_string_lossy();

                // Usually BAT0, BAT1, etc.
                if name.starts_with("BAT") {
                    let cap_path = path.join("capacity");
                    let status_path = path.join("status");

                    if let Ok(cap_str) = fs::read_to_string(cap_path) {
                        if let Ok(status_str) = fs::read_to_string(status_path) {
                            let cap = cap_str.trim().parse::<u8>().unwrap_or(0);
                            let status = status_str.trim().to_string(); // Charging, Discharging, Full

                            // Format: 100% [AC Connected] or similar
                            // Map status to user friendly string if needed, but "Charging"/"Discharging" is fine.
                            // The user example had: "100% [AC Connected]"
                            // Let's try to mimic that slightly or just use the status.

                            let pretty_status = match status.as_str() {
                                "Charging" => "Charging",
                                "Discharging" => "Discharging",
                                "Full" => "Full",
                                "Not charging" => "AC Connected", // Sometimes happens at 100%
                                _ => &status,
                            };

                            battery_output.push(format!("{}% [{}]", cap, pretty_status));
                        }
                    }
                }
            }
        }

        if battery_output.is_empty() {
            return "N/A".to_string();
        }

        battery_output.join(", ")
    }
}
