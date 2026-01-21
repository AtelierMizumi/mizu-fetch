use sysinfo::Networks;

pub struct NetworkInfo {
    pub name: String,
    pub rx: u64,
    pub tx: u64,
    pub total_rx: u64,
    pub total_tx: u64,
}

pub struct NetworkProvider;

impl NetworkProvider {
    pub fn get_networks(net_handle: &Networks) -> Vec<NetworkInfo> {
        net_handle
            .iter()
            .map(|(name, data)| NetworkInfo {
                name: name.to_string(),
                rx: data.received(),
                tx: data.transmitted(),
                total_rx: data.total_received(),
                total_tx: data.total_transmitted(),
            })
            .collect()
    }

    pub fn get_local_ip(net_handle: &Networks) -> String {
        for (name, network) in net_handle {
            if name != "lo" {
                for ip in network.ip_networks() {
                    if let std::net::IpAddr::V4(ipv4) = ip.addr {
                        return ipv4.to_string();
                    }
                }
            }
        }
        "127.0.0.1".to_string()
    }
}
