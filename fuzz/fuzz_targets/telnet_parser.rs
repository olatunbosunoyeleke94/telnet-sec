#![no_main]

use libfuzzer_sys::fuzz_target;
use safe_telnet_parser::TelnetParser;

fuzz_target!(|data: &[u8]| {
    let mut parser = TelnetParser::new();

    for &b in data {
        let _ = parser.push(b);
    }
});

