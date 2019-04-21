use std::process::Command;
use std::net::{IpAddr, Ipv4Addr};

use regex::Regex;

// LINUX EXAMPLE IFCONFIG
//
// eth0: flags=4163<UP,BROADCAST,RUNNING,MULTICAST>  mtu 1500
//         inet 172.17.0.2  netmask 255.255.0.0  broadcast 0.0.0.0gf
//         inet6 fe80::42:acff:fe11:2  prefixlen 64  scopeid 0x20<link>
//         ether 02:42:ac:11:00:02  txqueuelen 0  (Ethernet)
//         RX packets 20775  bytes 151512533 (144.4 MiB)
//         RX errors 0  dropped 0  overruns 0  frame 0
//         TX packets 21587  bytes 2799736 (2.6 MiB)
//         TX errors 0  dropped 0 overruns 0  carrier 0  collisions 0
//
// lo: flags=73<UP,LOOPBACK,RUNNING>  mtu 65536
//         inet 127.0.0.1  netmask 255.0.0.0
//         inet6 ::1  prefixlen 128  scopeid 0x10<host>
//         loop  txqueuelen 1000  (Local Loopback)
//         RX packets 11001  bytes 570940 (557.5 KiB)
//         RX errors 0  dropped 0  overruns 0  frame 0
//         TX packets 11001  bytes 570940 (557.5 KiB)
//         TX errors 0  dropped 0 overruns 0  carrier 0  collisions 0


#[cfg(target_os = "linux")]
pub fn get_local_ip() -> Option<IpAddr> {
    let output = Command::new("ifconfig")
        .output()
        .expect("failed to execute `ifconfig`");

    let stdout = String::from_utf8(output.stdout).unwrap();

    let regex = Regex::new(r#"(?m)^.*inet (addr:)?(([0-9]*\.){3}[0-9]*).*$"#).unwrap();

    for cap in regex.captures_iter(&stdout) {
        if let Some(host) = cap.at(2) {
            if host != "127.0.0.1" {
                if let Ok(addr) = host.parse::<Ipv4Addr>() {
                    return Some(IpAddr::V4(addr))
                }
            }
        }
    }

    None
}

#[cfg(target_os = "macos")]
pub fn get_local_ip() -> Option<IpAddr> {
    let output = Command::new("ifconfig")
        .output()
        .expect("failed to execute `ifconfig`");

    let stdout = String::from_utf8(output.stdout).unwrap();

    let regex = Regex::new(r#"(?m)^.*inet (addr:)?(([0-9]*\.){3}[0-9]*).*$"#).unwrap();

    for cap in regex.captures_iter(&stdout) {
        if let Some(host) = cap.at(2) {
            if host != "127.0.0.1" {
                if let Ok(addr) = host.parse::<Ipv4Addr>() {
                    return Some(IpAddr::V4(addr))
                }
            }
        }
    }

    None
}


// WINDOWS EXAMPLE IPCONFIG
//
// Windows IP Configuration
// Ethernet adapter Ethernet:
//    Media State . . . . . . . . . . . : Media disconnected
//    Connection-specific DNS Suffix  . : home
//
// Wireless LAN adapter Local Area Connection* 2:
//    Media State . . . . . . . . . . . : Media disconnected
//    Connection-specific DNS Suffix  . :
//
// Wireless LAN adapter Local Area Connection* 3:
//    Media State . . . . . . . . . . . : Media disconnected
//    Connection-specific DNS Suffix  . :
//
// Ethernet adapter Ethernet 2:
//    Media State . . . . . . . . . . . : Media disconnected
//    Connection-specific DNS Suffix  . :
//
// Wireless LAN adapter Wi-Fi:
//    Connection-specific DNS Suffix  . : home
//    Link-local IPv6 Address . . . . . : fe80::9091:50fe:cf97:9af6%11
//    IPv4 Address. . . . . . . . . . . : 192.168.1.47
//    Subnet Mask . . . . . . . . . . . : 255.255.255.0
//    Default Gateway . . . . . . . . . : 192.168.1.1

#[cfg(target_os = "windows")]
pub fn get_local_ip() -> Option<IpAddr> {
    let output = Command::new("ipconfig")
        .output()
        .expect("failed to execute `ipconfig`");

    let stdout = String::from_utf8(output.stdout).unwrap();
    
    let regex = Regex::new(r#"(?m)^.*IPv4 Address. . . . . . . . . . . : (Addr:)?(([0-9]*\.){3}[0-9]*).*$"#).unwrap();

    for cap in regex.captures_iter(&stdout) {
        if let Some(host) = cap.at(2) {
            if let Ok(addr) = host.parse::<Ipv4Addr>() {
                return Some(IpAddr::V4(addr))
            }
        }
    }

    None
}
