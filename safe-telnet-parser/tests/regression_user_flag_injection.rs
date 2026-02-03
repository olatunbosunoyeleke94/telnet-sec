use safe_telnet_parser::{TelnetEvent, TelnetParser};

#[test]
fn regression_user_flag_injection() {
    let payload = b"-f root\xFF\xFA";
    let mut parser = TelnetParser::new(); // ← MUST be mutable
    let events = parser.parse(payload);

    for event in &events {
        if let TelnetEvent::Data(byte) = event {
            assert_ne!(*byte, b'\xFF'); // CVE mitigation check
        }
    }
}
