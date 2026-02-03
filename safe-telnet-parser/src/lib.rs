//! Hardened Telnet protocol parser.
//!
//! This crate parses Telnet byte streams into structured events while
//! defending against malformed control sequences, oversized subnegotiation,
//! and state desynchronization attacks.


mod parser;

pub use parser::{TelnetEvent, TelnetParser};
