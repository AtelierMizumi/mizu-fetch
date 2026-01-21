use sysinfo::Disks;

pub struct DiskProvider;

impl DiskProvider {
    pub fn get_disk_usage(disk_handle: &Disks) -> String {
        let mut usage_info = Vec::new();
        let target_mounts = ["/", "/home"];

        for disk in disk_handle {
            let mount_point = disk.mount_point();
            if target_mounts
                .iter()
                .any(|&m| mount_point == std::path::Path::new(m))
            {
                let total_gb = disk.total_space() as f64 / 1024.0 / 1024.0 / 1024.0;
                let available_gb = disk.available_space() as f64 / 1024.0 / 1024.0 / 1024.0;
                let used_gb = total_gb - available_gb;
                let percent = (used_gb / total_gb) * 100.0;
                let fs = disk.file_system().to_string_lossy();

                let mount_name = if mount_point == std::path::Path::new("/") {
                    "Root"
                } else {
                    "Home"
                };

                usage_info.push(format!(
                    "{}: {:.2} GiB / {:.2} GiB ({:.0}%) - {}",
                    mount_name, used_gb, total_gb, percent, fs
                ));
            }
        }

        if usage_info.is_empty() {
            "Unknown".to_string()
        } else {
            usage_info.join(" | ")
        }
    }
}
