use sysinfo::System;

pub struct CpuInfo {
    pub models: Vec<String>,
    pub cores: usize,
}

impl CpuInfo {
    pub fn new(sys: &System) -> Self {
        let mut cpu_models: Vec<String> =
            sys.cpus().iter().map(|c| c.brand().to_string()).collect();
        cpu_models.sort();
        cpu_models.dedup();

        if cpu_models.is_empty() {
            cpu_models.push("Unknown CPU".to_string());
        }

        Self {
            models: cpu_models,
            cores: sys.cpus().len(),
        }
    }
}
