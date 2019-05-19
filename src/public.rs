use std::process::Command;
use std::net::IpAddr;
use std::error::Error;

pub fn get_public_ip() -> Result<IpAddr, Box<Error>> {
    let result = Command::new("curl")
        .arg("https://api.ipify.org")
        .output();

    match String::from_utf8(result?.stdout)?.parse::<IpAddr>() {
       Err(err) => Err(Box::new(err)),
       Ok(ip_addr) => Ok(ip_addr)
    }
}

#[test]
fn test_public_ip() {
    let public_ip = get_public_ip();

    // Write your known public ip here to test it.
    assert_eq!("176.42.134.158".parse::<IpAddr>().unwrap(), public_ip.unwrap());
}

