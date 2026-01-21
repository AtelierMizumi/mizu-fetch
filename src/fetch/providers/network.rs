use sysinfo::Networks;

pub struct NetworkInfo {
    pub name: String,
    pub rx: u64,
    pub tx: u64,
    pub total_rx: u64,
    pub total_tx: u64,
    pub ip_v4: String,
}

pub struct NetworkProvider;

impl NetworkProvider {
    pub fn get_networks(net_handle: &Networks) -> Vec<NetworkInfo> {
        net_handle
            .iter()
            .map(|(name, data)| {
                let mut ip_v4 = "N/A".to_string();
                for ip in data.ip_networks() {
                    if let std::net::IpAddr::V4(ipv4) = ip.addr {
                        ip_v4 = ipv4.to_string();
                        break;
                    }
                }

                NetworkInfo {
                    name: name.to_string(),
                    rx: data.received(),
                    tx: data.transmitted(),
                    total_rx: data.total_received(),
                    total_tx: data.total_transmitted(),
                    ip_v4,
                }
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
