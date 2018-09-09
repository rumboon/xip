extern crate pnet;
extern crate xcb;

use pnet::datalink;
use std::{thread, time};

fn main() {
    if !is_connected() {
        println!("Could not connect to X!");
        return;
    }

    let delay = 5000; // 5 seconds

    let wait_time = time::Duration::from_millis(delay);

    loop {
        if !is_connected() {
            break;
        }

        print_ips();
        thread::sleep(wait_time);
    }
}

fn is_connected() -> bool {
    let entries = xcb::Connection::connect(None);
    let mut has_connection = false;
    for _entry in entries {
        has_connection = true;
        break;
    }
    return has_connection
}

fn print_ips() {
    let mut ipv6 = String::new();
    let mut ipv4 = String::new();

    for iface in datalink::interfaces() {
        for ipv in iface.ips {
            if ipv.is_ipv6() {
                ipv6 = String::from(ipv.ip().to_string());
            }
            else if ipv.is_ipv4() {
                ipv4 = String::from(ipv.ip().to_string())
            }
        }
    }

    println!("%IPV6{}", ipv6);
    println!("%IPV4{}", ipv4);
}
