# Kaizen Master Plan for Mizu-Fetch

This document tracks the comprehensive improvement plan for `mizu-fetch`.

## 1. ⚡ CORE ARCHITECTURE (Refactor & Performance)
- [ ] **Refactor: Modularize SystemInfo**
    - [ ] Move `Disk`, `Network`, `Packages` logic from `mod.rs` to dedicated providers.
    - [ ] Create `OsInfo` struct for static data.
- [ ] **Performance: Async Fetching**
    - [ ] Implement `tokio` or std threading for Package counting (currently blocks startup).
    - [ ] Implement async Public IP fetching (optional/new).

## 2. 🛡️ CODE QUALITY & CLEANUP
- [ ] **Error Handling**: Replace `unwrap_or_else("Unknown")` with proper Result types where meaningful.
- [ ] **Remove Monolith**: Ensure `main.rs` and `app.rs` interact with cleaner interfaces.

## 3. 🎨 VISUAL & CONFIGURATION
- [ ] **Config System**: Support `config.toml` for:
    - [ ] Customizing module order.
    - [ ] Customizing colors.
- [ ] **Logo Gallery**:
    - [ ] Add ASCII art for other distros (Debian, Ubuntu, Fedora, etc.).
    - [ ] Auto-detect distro for logo selection.

## 4. 📦 MODULE ENHANCEMENTS (Existing but improve)
- [x] **GPU Detection**: Implemented (Native & lspci fallback). *Potential: Improve vendor ID DB.*
- [x] **Battery**: Implemented.
- [x] **Display**: Implemented (Basic).
- [ ] **Disk Usage**: Expand to show Home `/home` or other mounts, not just Root `/`.
- [x] **Local IP**: Implemented.
- [x] **Uptime**: Implemented.
