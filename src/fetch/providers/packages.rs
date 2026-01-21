use std::process::Command;

pub struct PackageProvider;

impl PackageProvider {
    // This function is potentially slow and fits well for async execution
    pub fn count_packages() -> String {
        let mut pkgs = Vec::new();
        // Pacman
        if let Ok(output) = Command::new("sh")
            .arg("-c")
            .arg("pacman -Qq | wc -l")
            .output()
        {
            let count = String::from_utf8(output.stdout)
                .unwrap_or_default()
                .trim()
                .to_string();
            if !count.is_empty() && count != "0" {
                pkgs.push(format!("{} (pacman)", count));
            }
        }
        // Dpkg (Debian/Ubuntu)
        if let Ok(output) = Command::new("sh")
            .arg("-c")
            .arg("dpkg-query -f '${binary:Package}\n' -W | wc -l")
            .output()
        {
            let count = String::from_utf8(output.stdout)
                .unwrap_or_default()
                .trim()
                .to_string();
            if !count.is_empty() && count != "0" {
                pkgs.push(format!("{} (dpkg)", count));
            }
        }
        // RPM (Fedora/RHEL)
        if let Ok(output) = Command::new("sh").arg("-c").arg("rpm -qa | wc -l").output() {
            let count = String::from_utf8(output.stdout)
                .unwrap_or_default()
                .trim()
                .to_string();
            if !count.is_empty() && count != "0" {
                pkgs.push(format!("{} (rpm)", count));
            }
        }
        // Flatpak
        if let Ok(output) = Command::new("sh")
            .arg("-c")
            .arg("flatpak list --app | wc -l")
            .output()
        {
            let count = String::from_utf8(output.stdout)
                .unwrap_or_default()
                .trim()
                .to_string();
            if !count.is_empty() && count != "0" {
                pkgs.push(format!("{} (flatpak-user)", count));
            }
        }

        if pkgs.is_empty() {
            "Unknown".to_string()
        } else {
            pkgs.join(", ")
        }
    }
}
