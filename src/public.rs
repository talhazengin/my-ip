use std::process::Command;
use std::net::IpAddr;

pub fn get_public_ip() -> Option<IpAddr> {
    let output = Command::new("curl https://api.ipify.org")
        .output()
        .expect("failed to execute `curl`");

    let stdout = String::from_utf8(output.stdout).unwrap();
    
    stdout.parse::<IpAddr>().ok()
}

#[test]
fn test_public_ip() {
    let public_ip = get_public_ip();

    // Write your known public ip here to test it.
    assert_eq!("176.42.134.158".parse::<IpAddr>().unwrap(), public_ip.unwrap());
}

