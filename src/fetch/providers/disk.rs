use sysinfo::Disks;

pub struct DiskProvider;

impl DiskProvider {
    pub fn get_disk_usage(disk_handle: &Disks) -> String {
        for disk in disk_handle {
            if disk.mount_point() == std::path::Path::new("/") {
                let total_gb = disk.total_space() as f64 / 1024.0 / 1024.0 / 1024.0;
                let available_gb = disk.available_space() as f64 / 1024.0 / 1024.0 / 1024.0;
                let used_gb = total_gb - available_gb;
                let percent = (used_gb / total_gb) * 100.0;
                let fs = disk.file_system().to_string_lossy();
                return format!(
                    "{:.2} GiB / {:.2} GiB ({:.0}%) - {}",
                    used_gb, total_gb, percent, fs
                );
            }
        }
        "Unknown".to_string()
    }
}
