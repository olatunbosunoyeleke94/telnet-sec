#![no_main]
use libfuzzer_sys::fuzz_target;
use safe_telnet_parser::TelnetParser;
use telnet_sanitizer::TelnetSanitizer;

fuzz_target!(|data: &[u8]| {
    let mut parser = TelnetParser::new();
    let _ = parser.parse(data); // parse everything
    let mut sanitizer = TelnetSanitizer::new();
    let _ = sanitizer.sanitize(data); // sanitize malicious bytes
});
