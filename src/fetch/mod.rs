use std::process::Command;
use sysinfo::{Disks, Networks, System};

mod providers;

use providers::cpu::CpuInfo;
use providers::gpu::GpuInfo;
use providers::memory::MemoryInfo;

pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub cpu: f32,
    pub mem: u64,
}

pub struct NetworkInfo {
    pub name: String,
    pub rx: u64,
    pub tx: u64,
    pub total_rx: u64,
    pub total_tx: u64,
}

pub struct SystemInfo {
    // Static / Lazy Loaded fields
    pub os_name: String,
    pub kernel_version: String,
    pub hostname: String,
    pub shell: String,
    pub terminal: String,
    pub de_wm: String,
    pub packages: String,
    pub local_ip: String,

    // Hardware Info (Cached/Lazy)
    pub cpu_info: CpuInfo,
    pub gpus: Vec<String>,

    // Dynamic Fields (Refreshed on tick)
    pub uptime: u64,
    pub cpu_usage: f32,
    pub memory_used: u64,
    pub memory_total: u64,
    pub swap_used: u64,
    pub swap_total: u64,
    pub disk_usage: String,
    pub processes: Vec<ProcessInfo>,
    pub networks: Vec<NetworkInfo>,

    // Private Handles
    sys: System,
    net_handle: Networks,
    disk_handle: Disks,
}

impl Default for SystemInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl SystemInfo {
    pub fn new() -> Self {
        // Initialize handles - BUT DO NOT REFRESH ALL
        // System::new() creates an empty system object
        let mut sys = System::new();

        // Only refresh specific components needed for static info
        sys.refresh_cpu_usage(); // Minimal cpu refresh for static info if needed, but `cpus()` needs `refresh_cpu()`

        // For static info like OS name, Hostname, we don't need a full refresh
        let os_name = System::name().unwrap_or_else(|| "Unknown".to_string());
        let kernel_version = System::kernel_version().unwrap_or_else(|| "Unknown".to_string());
        let hostname = System::host_name().unwrap_or_else(|| "localhost".to_string());

        // Lazy load CPU info
        // We need to refresh cpu list once to get brands
        sys.refresh_cpu_all();
        let cpu_info = CpuInfo::new(&sys);

        // GPU Detection (Native + Fallback)
        let gpu_info = GpuInfo::new();

        // Shell & DE
        let shell_path = std::env::var("SHELL").unwrap_or_else(|_| "Unknown".to_string());
        let shell = shell_path
            .split('/')
            .next_back()
            .unwrap_or("Unknown")
            .to_string();

        let de_wm = std::env::var("XDG_CURRENT_DESKTOP")
            .or_else(|_| std::env::var("DESKTOP_SESSION"))
            .unwrap_or_else(|_| "Unknown".to_string());

        // Packages (Expensive, could be lazy loaded or async)
        // For now keep it here but we should move it out of the main thread in future
        let packages = Self::count_packages();

        // Terminal
        let terminal = Self::detect_terminal(&mut sys);

        let net_handle = Networks::new_with_refreshed_list();
        let disk_handle = Disks::new_with_refreshed_list();

        // Initial Memory Fetch
        let mem_info = MemoryInfo::new(&mut sys);

        let mut info = Self {
            os_name,
            kernel_version,
            hostname,
            uptime: System::uptime(),
            shell,
            terminal,
            de_wm,
            packages,
            local_ip: "127.0.0.1".to_string(),
            cpu_info,
            cpu_usage: 0.0,
            gpus: gpu_info.names,
            memory_used: mem_info.used,
            memory_total: mem_info.total,
            swap_used: mem_info.swap_used,
            swap_total: mem_info.swap_total,
            disk_usage: "Unknown".to_string(),
            processes: Vec::new(),
            networks: Vec::new(),
            sys,
            net_handle,
            disk_handle,
        };

        // Initial partial refresh for dynamic data
        info.refresh();
        info
    }

    fn count_packages() -> String {
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
                pkgs.push(format!("{} (flatpak)", count));
            }
        }

        if pkgs.is_empty() {
            "Unknown".to_string()
        } else {
            pkgs.join(", ")
        }
    }

    fn detect_terminal(sys: &mut System) -> String {
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

    pub fn refresh(&mut self) {
        // Targeted refreshes only
        self.sys.refresh_cpu_usage(); // Just usage, not full list
        self.sys.refresh_memory();
        // Processes need list refresh but try to minimize impact if possible
        // refresh_processes is heavy.
        self.sys
            .refresh_processes(sysinfo::ProcessesToUpdate::All, true);

        self.net_handle.refresh(true);
        self.disk_handle.refresh(true);

        // Update Dynamic Fields
        self.uptime = System::uptime();
        self.cpu_usage = self.sys.global_cpu_usage();
        self.memory_used = self.sys.used_memory();
        self.memory_total = self.sys.total_memory();
        self.swap_used = self.sys.used_swap();
        self.swap_total = self.sys.total_swap();

        // Processes (Top 50)
        self.update_processes();

        // Network Stats
        self.update_networks();

        // Disk Usage
        self.update_disks();

        // Local IP
        self.update_local_ip();
    }

    fn update_processes(&mut self) {
        let mut processes: Vec<ProcessInfo> = self
            .sys
            .processes()
            .iter()
            .map(|(pid, process)| ProcessInfo {
                pid: pid.as_u32(),
                name: process.name().to_string_lossy().to_string(),
                cpu: process.cpu_usage(),
                mem: process.memory(),
            })
            .collect();

        processes.sort_by(|a, b| {
            b.cpu
                .partial_cmp(&a.cpu)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        processes.truncate(50);
        self.processes = processes;
    }

    fn update_networks(&mut self) {
        self.networks = self
            .net_handle
            .iter()
            .map(|(name, data)| NetworkInfo {
                name: name.to_string(),
                rx: data.received(),
                tx: data.transmitted(),
                total_rx: data.total_received(),
                total_tx: data.total_transmitted(),
            })
            .collect();
    }

    fn update_disks(&mut self) {
        for disk in &self.disk_handle {
            if disk.mount_point() == std::path::Path::new("/") {
                let total_gb = disk.total_space() as f64 / 1024.0 / 1024.0 / 1024.0;
                let available_gb = disk.available_space() as f64 / 1024.0 / 1024.0 / 1024.0;
                let used_gb = total_gb - available_gb;
                let percent = (used_gb / total_gb) * 100.0;
                let fs = disk.file_system().to_string_lossy();
                self.disk_usage = format!(
                    "{:.2} GiB / {:.2} GiB ({:.0}%) - {}",
                    used_gb, total_gb, percent, fs
                );
                return;
            }
        }
        self.disk_usage = "Unknown".to_string();
    }

    fn update_local_ip(&mut self) {
        for (name, network) in &self.net_handle {
            if name != "lo" {
                for ip in network.ip_networks() {
                    if let std::net::IpAddr::V4(ipv4) = ip.addr {
                        self.local_ip = ipv4.to_string();
                        return;
                    }
                }
            }
        }
        self.local_ip = "127.0.0.1".to_string();
    }
}
