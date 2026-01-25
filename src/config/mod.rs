use ratatui::style::Color;
use serde::{Deserialize, Serialize};
use std::fs;
use std::num::ParseIntError;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub theme: Theme,
    pub refresh_rate: u64,
    #[serde(default = "default_module_order")]
    pub modules: Vec<String>,
}

fn default_module_order() -> Vec<String> {
    vec![
        "os".to_string(),
        "host".to_string(),
        "kernel".to_string(),
        "uptime".to_string(),
        "packages".to_string(),
        "shell".to_string(),
        "display".to_string(),
        "de".to_string(),
        "wm".to_string(),
        "wm_theme".to_string(),
        "theme".to_string(),
        "icons".to_string(),
        "font".to_string(),
        "cursor".to_string(),
        "terminal".to_string(),
        "cpu".to_string(),
        "gpu".to_string(),
        "memory".to_string(),
        "disk".to_string(),
        "battery".to_string(),
        "locale".to_string(),
    ]
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Theme {
    pub border_color: String,
    pub title_color: String,
    pub text_color: String,
    pub key_color: String,
    pub value_color: String,
    pub gauge_cpu_low: String,
    pub gauge_cpu_high: String,
    pub gauge_ram: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            refresh_rate: 250,
            theme: Theme {
                border_color: "#00ffff".to_string(),   // Cyan (Hex)
                title_color: "#00ffff".to_string(),    // Cyan (Hex)
                text_color: "#ffffff".to_string(),     // White (Hex)
                key_color: "#ff00ff".to_string(),      // Magenta (Hex)
                value_color: "#00ffff".to_string(),    // Cyan (Hex)
                gauge_cpu_low: "#00ffff".to_string(),  // Cyan (Hex)
                gauge_cpu_high: "#ff0000".to_string(), // Red (Hex)
                gauge_ram: "#ff00ff".to_string(),      // Magenta (Hex)
            },
            modules: default_module_order(),
        }
    }
}

impl Config {
    pub fn load() -> Self {
        let config_path = Self::get_config_path();

        if let Some(path) = config_path {
            if let Ok(contents) = fs::read_to_string(&path) {
                if let Ok(config) = toml::from_str(&contents) {
                    return config;
                }
            } else {
                // Create default config file if it doesn't exist
                let default_config = Self::default();
                let _ = default_config.save(); // Use save method to write default
                return default_config;
            }
        }

        Self::default()
    }

    pub fn save(&self) -> std::io::Result<()> {
        if let Some(path) = Self::get_config_path() {
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)?;
            }
            let toml_str = toml::to_string_pretty(self).map_err(std::io::Error::other)?;
            fs::write(path, toml_str)?;
        }
        Ok(())
    }

    fn get_config_path() -> Option<PathBuf> {
        dirs::config_dir().map(|mut p| {
            p.push("mizu-fetch");
            p.push("config.toml");
            p
        })
    }
}

impl Theme {
    pub fn from_name(name: &str) -> Self {
        match name.to_lowercase().as_str() {
            "dracula" => Self {
                border_color: "#bd93f9".to_string(),   // Purple
                title_color: "#ff79c6".to_string(),    // Pink
                text_color: "#f8f8f2".to_string(),     // Foreground
                key_color: "#8be9fd".to_string(),      // Cyan
                value_color: "#f1fa8c".to_string(),    // Yellow
                gauge_cpu_low: "#50fa7b".to_string(),  // Green
                gauge_cpu_high: "#ff5555".to_string(), // Red
                gauge_ram: "#ffb86c".to_string(),      // Orange
            },
            "github" => Self {
                border_color: "#30363d".to_string(),   // Border Gray
                title_color: "#58a6ff".to_string(),    // Blue
                text_color: "#c9d1d9".to_string(),     // FG
                key_color: "#d2a8ff".to_string(),      // Purple
                value_color: "#79c0ff".to_string(),    // Light Blue
                gauge_cpu_low: "#3fb950".to_string(),  // Green
                gauge_cpu_high: "#ff7b72".to_string(), // Red
                gauge_ram: "#d2a8ff".to_string(),      // Purple
            },
            "material" => Self {
                border_color: "#89ddff".to_string(),   // Cyan
                title_color: "#c792ea".to_string(),    // Purple
                text_color: "#eeffff".to_string(),     // FG
                key_color: "#f07178".to_string(),      // Red
                value_color: "#ffcb6b".to_string(),    // Yellow
                gauge_cpu_low: "#c3e88d".to_string(),  // Green
                gauge_cpu_high: "#ff5370".to_string(), // Red
                gauge_ram: "#f78c6c".to_string(),      // Orange
            },
            "catppuccin" => Self {
                border_color: "#cba6f7".to_string(),   // Mauve
                title_color: "#89b4fa".to_string(),    // Blue
                text_color: "#cdd6f4".to_string(),     // Text
                key_color: "#f9e2af".to_string(),      // Yellow
                value_color: "#a6e3a1".to_string(),    // Green
                gauge_cpu_low: "#94e2d5".to_string(),  // Teal
                gauge_cpu_high: "#f38ba8".to_string(), // Red
                gauge_ram: "#fab387".to_string(),      // Peach
            },
            // Default / Neon
            _ => Self {
                border_color: "#00ffff".to_string(),
                title_color: "#00ffff".to_string(),
                text_color: "#ffffff".to_string(),
                key_color: "#ff00ff".to_string(),
                value_color: "#00ffff".to_string(),
                gauge_cpu_low: "#00ffff".to_string(),
                gauge_cpu_high: "#ff0000".to_string(),
                gauge_ram: "#ff00ff".to_string(),
            },
        }
    }
}

pub fn parse_color(color_str: &str) -> Color {
    if color_str.starts_with('#') {
        if let Ok((r, g, b)) = hex_to_rgb(color_str) {
            return Color::Rgb(r, g, b);
        }
    } else if let Ok(u8_val) = color_str.parse::<u8>() {
        return Color::Indexed(u8_val);
    }

    match color_str.to_lowercase().as_str() {
        "black" => Color::Black,
        "red" => Color::Red,
        "green" => Color::Green,
        "yellow" => Color::Yellow,
        "blue" => Color::Blue,
        "magenta" => Color::Magenta,
        "cyan" => Color::Cyan,
        "gray" => Color::Gray,
        "dark_gray" | "darkgray" => Color::DarkGray,
        "light_red" | "lightred" => Color::LightRed,
        "light_green" | "lightgreen" => Color::LightGreen,
        "light_yellow" | "lightyellow" => Color::LightYellow,
        "light_blue" | "lightblue" => Color::LightBlue,
        "light_magenta" | "lightmagenta" => Color::LightMagenta,
        "light_cyan" | "lightcyan" => Color::LightCyan,
        "white" => Color::White,
        _ => Color::White, // Default fallback
    }
}

fn hex_to_rgb(hex: &str) -> Result<(u8, u8, u8), ParseIntError> {
    let hex = hex.trim_start_matches('#');
    let r = u8::from_str_radix(&hex[0..2], 16)?;
    let g = u8::from_str_radix(&hex[2..4], 16)?;
    let b = u8::from_str_radix(&hex[4..6], 16)?;
    Ok((r, g, b))
}
