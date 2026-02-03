use telnet_sanitizer::TelnetSanitizer;

fn main() {
    // Simulated malicious Telnet payload
    // "-f root" + IAC SB (classic injection pattern)
    let malicious = b"-f root\xFF\xFA";

    let mut sanitizer = TelnetSanitizer::new();
    let clean = sanitizer.sanitize(malicious);

    println!("Raw input bytes:");
    for b in malicious {
        print!("{:02x} ", b);
    }
    println!();

    println!("Sanitized output bytes:");
    for b in &clean {
        print!("{:02x} ", b);
    }
    println!();

    println!("Sanitized output as text:");
    println!("{}", String::from_utf8_lossy(&clean));
}
