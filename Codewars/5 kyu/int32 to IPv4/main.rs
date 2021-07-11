fn int32_to_ip(int: u32) -> String {
    std::net::Ipv4Addr::from(int).to_string()
}

fn main() {
    assert_eq!(int32_to_ip(2154959208), "128.114.17.104");
    assert_eq!(int32_to_ip(2149583361), "128.32.10.1");
    assert_eq!(int32_to_ip(0), "0.0.0.0");
}
