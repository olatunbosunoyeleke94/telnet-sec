use telnet_sanitizer::TelnetSanitizer;

fn main() {
    let mut sanitizer = TelnetSanitizer::new();

    let input = b"\xFF\xFA-f root\xF0";
    let clean = sanitizer.sanitize(input);

    println!("{:?}", clean);
}
