use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr};

fn main() {
    println!("---");
    ip_to_int("127.10.0.1:8080".parse().unwrap());
    ip_to_int("[2001:db8::1]:8080".parse().unwrap());
}

fn ip_to_int(ip: SocketAddr) -> usize {
    match ip {
        SocketAddr::V4(ip_v4) => {
            let ip_num: u32 = (*ip_v4.ip()).into();
            println!("ip_v4 {}", ip_num);
            let ip_addr: Ipv4Addr = Ipv4Addr::from(ip_num);
            println!("ip_addr {}", ip_addr);
        }
        SocketAddr::V6(ip_v6) => {
            let ip_num: u128 = (*ip_v6.ip()).into();
            println!("ip_v6 {}", ip_num);
            let ip_addr: Ipv6Addr = Ipv6Addr::from(ip_num);
            println!("ip_addr {}", ip_addr);
        }
    }
    // let num = (o1 << 24) + (o2 << 16) + (o3 << 8) + o4;
    return 1;
}
