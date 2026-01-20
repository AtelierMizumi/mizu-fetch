# Kaizen Master Plan for Mizu-Fetch

This document tracks the comprehensive improvement plan for `mizu-fetch` to reach parity with `fastfetch` and beyond.

## 1. ⚡ CORE & PERFORMANCE
- [ ] **P1.1: Lazy Loading Architecture**
    - [ ] Refactor `SystemInfo` to remove monolithic `new()`.
    - [ ] Split into `CpuInfo`, `GpuInfo`, `MemInfo`, etc.
    - [ ] Remove `sys.refresh_all()` in favor of targeted refreshes.
- [ ] **P1.2: Native GPU Detection**
    - [ ] Linux: Read from `/sys/class/drm/` instead of `lspci`.
    - [ ] Implement vendor ID mapping (basic).
- [ ] **P1.3: Caching Mechanism**
    - [ ] Implement `~/.cache/mizu/` storage for static info (CPU name, OS version).
- [ ] **P1.4: Asynchronous Fetching**
    - [ ] Use threading for slow I/O (Public IP, etc.).

## 2. 📦 MODULES EXPANSION
- [ ] **P2.1: Hardware Deep-dive**
    - [ ] **Disk:** Root `/` and Home `/home` usage.
    - [ ] **Resolution:** Screen resolution and refresh rate.
    - [ ] **Battery:** Battery percentage and status.
- [ ] **P2.2: Software & Environment**
    - [ ] **Packages:** Count packages (pacman, dpkg, flatpak, snap, cargo, npm).
    - [ ] **Terminal:** Detect actual terminal emulator (via PPID or env).
- [ ] **P2.3: Network**
    - [ ] **Local IP:** Get LAN IP.
- [ ] **P2.4: Utilities**
    - [ ] **Uptime:** Format uptime prettily (days, hours, mins).

## 3. 🎨 VISUAL & CUSTOMIZATION
- [ ] **P3.1: Config System**
    - [ ] Implement TOML/JSON config support.
    - [ ] Allow customizing module order and colors.
- [ ] **P3.2: Logo Gallery**
    - [ ] Add ASCII art for top 20 distros.
    - [ ] Auto-detect distro for logo selection.

## 4. 🛠 CLI & UX
- [ ] **P4.1: CLI Args**
    - [ ] `--no-logo` flag.
    - [ ] `--config` flag.

## 5. 🛡️ CODE QUALITY
- [ ] **P5.1: Error Handling**
    - [ ] Use `Result` and `Option` instead of `unwrap_or_else("Unknown")`.
