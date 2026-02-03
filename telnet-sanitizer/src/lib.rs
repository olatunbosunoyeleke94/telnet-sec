//! Opinionated Telnet input sanitizer.
//!
//! Drops all Telnet control sequences and returns only safe application data.
//! Intended for legacy Telnet services, proxies, and sandboxes.

use safe_telnet_parser::{TelnetEvent, TelnetParser};

pub struct TelnetSanitizer {
    parser: TelnetParser,
}

impl Default for TelnetSanitizer {
    fn default() -> Self {
        Self::new()
    }
}

impl TelnetSanitizer {
    pub fn new() -> Self {
        Self {
            parser: TelnetParser::new(),
        }
    }

    pub fn sanitize(&mut self, input: &[u8]) -> Vec<u8> {
        self.parser
            .parse(input)
            .into_iter()
            .filter_map(|e| {
                if let TelnetEvent::Data(b) = e {
                    Some(b)
                } else {
                    None
                }
            })
            .collect()
    }
}
