use sysinfo::System;

pub struct OsInfo {
    pub name: String,
    pub kernel: String,
    pub hostname: String,
    pub shell: String,
    pub de_wm: String,
    pub wm: String,
    pub terminal: String,
    pub locale: String,
}

impl OsInfo {
    pub fn new(sys: &mut System) -> Self {
        let name = System::name().unwrap_or_else(|| "Unknown".to_string());
        let kernel = System::kernel_version().unwrap_or_else(|| "Unknown".to_string());
        let hostname = System::host_name().unwrap_or_else(|| "localhost".to_string());

        // Shell
        let shell_path = std::env::var("SHELL").unwrap_or_else(|_| "Unknown".to_string());
        let shell = shell_path
            .split('/')
            .next_back()
            .unwrap_or("Unknown")
            .to_string();

        // DE/WM
        let de_wm = std::env::var("XDG_CURRENT_DESKTOP")
            .or_else(|_| std::env::var("DESKTOP_SESSION"))
            .unwrap_or_else(|_| "Unknown".to_string());

        let wm = Self::detect_wm(sys);
        let terminal = Self::detect_terminal(sys);
        let locale = std::env::var("LANG").unwrap_or_else(|_| "Unknown".to_string());

        Self {
            name,
            kernel,
            hostname,
            shell,
            de_wm,
            wm,
            terminal,
            locale,
        }
    }

    fn detect_wm(sys: &System) -> String {
        let wms = [
            "kwin_wayland",
            "kwin_x11",
            "gnome-shell",
            "mutter",
            "sway",
            "hyprland",
            "openbox",
            "i3",
            "bspwm",
            "xfwm4",
            "metacity",
            "weston",
            "labwc",
            "wayfire",
        ];

        for (_pid, process) in sys.processes() {
            let name = process.name().to_string_lossy();
            for wm in wms {
                if name.contains(wm) {
                    if wm.starts_with("kwin") {
                        return "KWin".to_string();
                    }
                    if wm == "gnome-shell" {
                        return "Mutter (GNOME)".to_string();
                    }
                    return wm.to_string();
                }
            }
        }
        "Unknown".to_string()
    }

    fn detect_terminal(sys: &mut System) -> String {
        // refresh_processes is heavy, so it should be called carefully.
        // In the original code it was called inside detect_terminal.
        // We will assume the caller might have refreshed it or we do it here strictly.
        // For safety, let's refresh here as it relies on PIDs which might be stale if not refreshed.
        sys.refresh_processes(sysinfo::ProcessesToUpdate::All, true);
        if let Ok(pid) = sysinfo::get_current_pid()
            && let Some(process) = sys.process(pid)
            && let Some(parent_pid) = process.parent()
            && let Some(parent) = sys.process(parent_pid)
            && let Some(grandparent_pid) = parent.parent()
            && let Some(grandparent) = sys.process(grandparent_pid)
        {
            return grandparent.name().to_string_lossy().to_string();
        }
        "Unknown".to_string()
    }
}
