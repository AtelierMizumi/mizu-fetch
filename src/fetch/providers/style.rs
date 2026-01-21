use std::env;
use std::fs;
use std::path::Path;

pub struct StyleInfo {
    pub theme: String,
    pub icons: String,
    pub font: String,
    pub cursor: String,
    pub wm_theme: String,
}

impl StyleInfo {
    pub fn new(_sys: &sysinfo::System) -> Self {
        // Detect DE first
        let de = env::var("XDG_CURRENT_DESKTOP")
            .or_else(|_| env::var("DESKTOP_SESSION"))
            .unwrap_or_default()
            .to_lowercase();

        if de.contains("kde") || de.contains("plasma") {
            return Self::get_kde_style();
        } else if de.contains("gnome") || de.contains("gtk") {
            return Self::get_gtk_style();
        }

        // Fallback or generic
        Self {
            theme: "Unknown".to_string(),
            icons: "Unknown".to_string(),
            font: "Unknown".to_string(),
            cursor: "Unknown".to_string(),
            wm_theme: "Unknown".to_string(),
        }
    }

    fn get_kde_style() -> Self {
        let home = env::var("HOME").unwrap_or_default();
        let kdeglobals = Path::new(&home).join(".config/kdeglobals");
        let kcminputrc = Path::new(&home).join(".config/kcminputrc");
        let kwinrc = Path::new(&home).join(".config/kwinrc");

        let theme = Self::parse_ini(&kdeglobals, "General", "ColorScheme")
            .or_else(|| Self::parse_ini(&kdeglobals, "KDE", "widgetStyle"))
            .unwrap_or_else(|| "Breeze".to_string());

        let icons =
            Self::parse_ini(&kdeglobals, "Icons", "Theme").unwrap_or_else(|| "breeze".to_string());

        let font = Self::parse_ini(&kdeglobals, "General", "font")
            .unwrap_or_else(|| "Noto Sans,10,-1,5,50,0,0,0,0,0".to_string()); // Raw KDE font string

        // Clean up KDE font string: "Noto Sans,10,-1,..." -> "Noto Sans (10pt)"
        let clean_font = if let Some(comma_idx) = font.find(',') {
            let name = &font[..comma_idx];
            let rest = &font[comma_idx + 1..];
            let size = rest.split(',').next().unwrap_or("10");
            format!("{} ({}pt)", name, size)
        } else {
            font
        };

        let cursor = Self::parse_ini(&kcminputrc, "Mouse", "cursorTheme")
            .unwrap_or_else(|| "breeze_cursors".to_string());

        let wm_theme = Self::parse_ini(&kwinrc, "org.kde.kdecoration2", "theme")
            .unwrap_or_else(|| "Breeze".to_string());

        Self {
            theme: format!("{} [Qt], Breeze [GTK2/3]", theme), // Simplify GTK part for now
            icons,
            font: clean_font,
            cursor,
            wm_theme,
        }
    }

    fn get_gtk_style() -> Self {
        let home = env::var("HOME").unwrap_or_default();
        let gtk3_config = Path::new(&home).join(".config/gtk-3.0/settings.ini");

        let theme = Self::parse_ini(&gtk3_config, "Settings", "gtk-theme-name")
            .unwrap_or_else(|| "Adwaita".to_string());

        let icons = Self::parse_ini(&gtk3_config, "Settings", "gtk-icon-theme-name")
            .unwrap_or_else(|| "Adwaita".to_string());

        let font = Self::parse_ini(&gtk3_config, "Settings", "gtk-font-name")
            .unwrap_or_else(|| "Cantarell 11".to_string());

        let cursor = Self::parse_ini(&gtk3_config, "Settings", "gtk-cursor-theme-name")
            .unwrap_or_else(|| "Adwaita".to_string());

        Self {
            theme,
            icons,
            font,
            cursor,
            wm_theme: "Unknown".to_string(), // GTK usually implies the WM theme matches or is hidden
        }
    }

    fn parse_ini(path: &Path, section: &str, key: &str) -> Option<String> {
        if let Ok(content) = fs::read_to_string(path) {
            let mut current_section = "";
            for line in content.lines() {
                let line = line.trim();
                if line.starts_with('[') && line.ends_with(']') {
                    current_section = &line[1..line.len() - 1];
                } else if current_section == section {
                    if let Some((k, v)) = line.split_once('=') {
                        if k.trim() == key {
                            return Some(v.trim().to_string());
                        }
                    }
                }
            }
        }
        None
    }
}
