use sysinfo::System;

pub struct MemoryInfo {
    pub total: u64,
    pub used: u64,
    pub swap_total: u64,
    pub swap_used: u64,
}

impl MemoryInfo {
    pub fn new(sys: &mut System) -> Self {
        sys.refresh_memory();
        Self {
            total: sys.total_memory(),
            used: sys.used_memory(),
            swap_total: sys.total_swap(),
            swap_used: sys.used_swap(),
        }
    }
}
