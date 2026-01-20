# Mizu Fetch

A stylish, responsive system fetch tool written in Rust, featuring a Nintendo DS-inspired dual-screen layout and live hardware monitoring.

![Mizu Fetch Demo](https://via.placeholder.com/800x400.png?text=Mizu+Fetch+Preview)

## ✨ Features

*   **Dual-Screen Layout**: Unique design inspired by the Nintendo DS interface.
*   **Live Monitoring**: Real-time CPU, RAM, Swap usage visualization.
*   **Responsive UI**: Automatically adapts layout for small terminals (Compact Mode).
*   **Customizable Theme**: Full RGB support via `config.toml`.
*   **Fast**: Built with Rust and Ratatui for high performance.

## 🚀 Installation

### From Source

```bash
git clone https://github.com/thuanc177/mizu-fetch.git
cd mizu-fetch
cargo install --path .
```

## 🎮 Usage

Simply run `mizu` in your terminal:

```bash
mizu
```

Or enable live mode directly (default behavior now runs interactively):

```bash
mizu --live
```

**Controls:**
*   `q`: Quit the application.

## 🎨 Configuration

Mizu Fetch automatically creates a configuration file at `~/.config/mizu-fetch/config.toml` on first run.

You can customize colors using standard names, HEX codes, or ANSI index numbers.

**Example Config (Catppuccin Mocha):**

```toml
[theme]
border_color = "#89b4fa"
title_color = "#cba6f7"
text_color = "#cdd6f4"
key_color = "#f38ba8"
value_color = "#fab387"
gauge_cpu_low = "#a6e3a1"
gauge_cpu_high = "#f38ba8"
gauge_ram = "#f9e2af"
```

## 🛠️ Architecture

*   **Language**: Rust
*   **TUI Framework**: [Ratatui](https://github.com/ratatui-org/ratatui)
*   **System Info**: `sysinfo` crate
*   **Architecture**: TEA (The Elm Architecture) / Model-View-Update pattern.

## 📝 License

MIT License.
