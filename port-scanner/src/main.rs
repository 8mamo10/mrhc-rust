use pnet::packet::tcp;
use std::net;

struct PacketInfo {
    my_ipaddr: net::Ipv4Addr,
    target_ipaddr: net::Ipv4Addr,
    my_port: u16,
    maxinum_port: u16,
    scan_type: ScanType,
}

#[derive(Copy, Clone)]
enum ScanType {
    SynScan = tcp::TcpFlags::SYN as isize,
    FinScan = tcp::TcpFlags::FIN as isize,
    XmasScan = (tcp::TcpFlags::FIN | tcp::TcpFlags::URG | tcp::TcpFlags::PSH) as isize,
    NullScan,
}

fn main() {
    println!("Hello, world!");
}
