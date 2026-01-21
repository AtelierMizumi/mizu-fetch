use sysinfo::{Disks, Networks, System};
use crate::app::ProcessSortMode;

pub mod providers;

use providers::battery::BatteryInfo;
use providers::cpu::CpuInfo;
use providers::display::DisplayInfo;
use providers::gpu::GpuInfo;
use providers::memory::MemoryInfo;
use providers::style::StyleInfo;

// New imports
use providers::disk::DiskProvider;
use providers::network::{NetworkInfo, NetworkProvider};
use providers::os::OsInfo;
// PackageProvider is used in App, not here directly for async reasons,
// but SystemInfo holds the display string.

pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub cpu: f32,
    pub mem: u64,
}

pub struct SystemInfo {
    // Modular Components
    pub os: OsInfo,
    pub cpu_info: CpuInfo,
    
    // Static / Lazy fields
    pub gpus: Vec<String>,
    pub wm_theme: String,
    pub theme: String,
    pub icons: String,
    pub font: String,
    pub cursor: String,
    pub battery: String,
    pub display: String,

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
    pub local_ip: String,
    
    // Async Fields (Updated by App)
    pub packages: String,

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
        let mut sys = System::new();
        
        // Initialize handles
        let net_handle = Networks::new_with_refreshed_list();
        let disk_handle = Disks::new_with_refreshed_list();

        // Refresh specific components for static info
        sys.refresh_cpu_usage(); 
        
        // OS Info (Static)
        let os = OsInfo::new(&mut sys);

        // Lazy load CPU info
        sys.refresh_cpu_all();
        let cpu_info = CpuInfo::new(&sys);

        // GPU Detection
        let gpu_info = GpuInfo::new();

        // Style Info
        let style = StyleInfo::new(&sys);

        // Display Info
        let display = DisplayInfo::new();

        // Battery Info
        let battery = BatteryInfo::new();

        // Initial Memory Fetch
        let mem_info = MemoryInfo::new(&mut sys);
        
        // Network & Disk initial fetch
        let networks = NetworkProvider::get_networks(&net_handle);
        let local_ip = NetworkProvider::get_local_ip(&net_handle);
        let disk_usage = DiskProvider::get_disk_usage(&disk_handle);

        Self {
            os,
            cpu_info,
            gpus: gpu_info.names,
            wm_theme: style.wm_theme,
            theme: style.theme,
            icons: style.icons,
            font: style.font,
            cursor: style.cursor,
            battery,
            display,
            uptime: System::uptime(),
            cpu_usage: 0.0,
            memory_used: mem_info.used,
            memory_total: mem_info.total,
            swap_used: mem_info.swap_used,
            swap_total: mem_info.swap_total,
            disk_usage,
            processes: Vec::new(),
            networks,
            local_ip,
            packages: "Calculating...".to_string(), // Async placeholder
            sys,
            net_handle,
            disk_handle,
        }
    }

    pub fn refresh(&mut self, update_processes: bool, sort_mode: ProcessSortMode) {
        // Targeted refreshes only
        self.sys.refresh_cpu_usage();
        self.sys.refresh_memory();

        if update_processes {
            self.sys.refresh_processes(sysinfo::ProcessesToUpdate::All, true);
        }

        self.net_handle.refresh(true);
        self.disk_handle.refresh(true);

        // Update Dynamic Fields
        self.uptime = System::uptime();
        self.cpu_usage = self.sys.global_cpu_usage();
        
        // MemoryInfo::new refreshes nothing inside (it assumes sys is fresh)
        self.memory_used = self.sys.used_memory();
        self.memory_total = self.sys.total_memory();
        self.swap_used = self.sys.used_swap();
        self.swap_total = self.sys.total_swap();

        if update_processes {
            self.update_processes(sort_mode);
        }

        self.networks = NetworkProvider::get_networks(&self.net_handle);
        self.local_ip = NetworkProvider::get_local_ip(&self.net_handle);
        self.disk_usage = DiskProvider::get_disk_usage(&self.disk_handle);
    }

    fn update_processes(&mut self, sort_mode: ProcessSortMode) {
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

        match sort_mode {
            ProcessSortMode::Cpu => {
                processes.sort_by(|a, b| b.cpu.partial_cmp(&a.cpu).unwrap_or(std::cmp::Ordering::Equal));
            }
            ProcessSortMode::Memory => {
                processes.sort_by(|a, b| b.mem.cmp(&a.mem));
            }
            ProcessSortMode::Pid => {
                processes.sort_by(|a, b| a.pid.cmp(&b.pid));
            }
        }
        
        processes.truncate(50);
        self.processes = processes;
    }

    pub fn get_formatted_uptime(&self) -> String {
        let seconds = self.uptime;
        let days = seconds / 86400;
        let hours = (seconds % 86400) / 3600;
        let minutes = (seconds % 3600) / 60;
        let secs = seconds % 60;

        if days > 0 {
            format!("{} days, {} hours, {} mins", days, hours, minutes)
        } else if hours > 0 {
            format!("{} hours, {} mins", hours, minutes)
        } else {
            format!("{} mins, {} secs", minutes, secs)
        }
    }
}
