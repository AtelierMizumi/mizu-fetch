use ratatui::style::Color;
use serde::{Deserialize, Serialize};
use std::fs;
use std::num::ParseIntError;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub theme: Theme,
    pub refresh_rate: u64,
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
        }
    }
}

impl Config {
    pub fn load() -> Self {
        let config_path = Self::get_config_path();

        if let Some(path) = config_path {
            if path.exists() {
                if let Ok(contents) = fs::read_to_string(&path) {
                    if let Ok(config) = toml::from_str(&contents) {
                        return config;
                    }
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
            let toml_str = toml::to_string_pretty(self)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
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
