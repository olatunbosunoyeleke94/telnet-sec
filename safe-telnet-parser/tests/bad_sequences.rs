use safe_telnet_parser::{TelnetEvent, TelnetParser};

#[test]
fn repeated_iac_does_not_crash() {
    let mut parser = TelnetParser::new();

    let bad = [0xFF, 0xFF, 0xFF, 0xFF];

    for b in bad {
        let ev = parser.feed(b);
        assert!(ev.is_some());
    }
}

#[test]
fn oversized_subnegotiation_detected() {
    let mut parser = TelnetParser::new();

    // IAC SB
    parser.feed(0xFF);
    parser.feed(0xFA);

    for _ in 0..2000 {
        if let Some(TelnetEvent::OversizedSubnegotiation) = parser.feed(b'A') {
            return; // success
        }
    }

    panic!("Oversized subnegotiation not detected");
}

#[test]
fn invalid_telnet_command_is_handled() {
    let mut parser = TelnetParser::new();

    let ev = parser.feed(0xFE); // invalid in many parsers
    assert!(ev.is_some());
}
