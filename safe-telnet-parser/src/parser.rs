use std::vec::Vec;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TelnetEvent {
    Data(u8),
    Iac,
    Will(u8),
    Wont(u8),
    Do(u8),
    Dont(u8),
    SubnegotiationStart,
    SubnegotiationEnd,
    OversizedSubnegotiation,
}

#[derive(Debug, Clone, Copy)]
enum State {
    Data,
    Iac,
    Subnegotiation,
}

pub struct TelnetParser {
    state: State,
    subnegotiation_len: usize,
}

impl Default for TelnetParser {
    fn default() -> Self {
        Self::new()
    }
}

impl TelnetParser {
    pub fn new() -> Self {
        Self {
            state: State::Data,
            subnegotiation_len: 0,
        }
    }

    pub fn parse(&mut self, input: &[u8]) -> Vec<TelnetEvent> {
        let mut events = Vec::new();
        for &b in input {
            if let Some(ev) = self.feed(b) {
                events.push(ev);
            }
        }
        events
    }

    pub fn feed(&mut self, byte: u8) -> Option<TelnetEvent> {
        match self.state {
            State::Data => {
                if byte == 0xFF {
                    self.state = State::Iac;
                    Some(TelnetEvent::Iac)
                } else {
                    Some(TelnetEvent::Data(byte))
                }
            }

            State::Iac => {
                self.state = State::Data;
                match byte {
                    0xFF => Some(TelnetEvent::Data(0xFF)), // RFC 854
                    0xFA => {
                        self.state = State::Subnegotiation;
                        self.subnegotiation_len = 0;
                        Some(TelnetEvent::SubnegotiationStart)
                    }
                    0xFB => Some(TelnetEvent::Will(0)),
                    0xFC => Some(TelnetEvent::Wont(0)),
                    0xFD => Some(TelnetEvent::Do(0)),
                    0xFE => Some(TelnetEvent::Dont(0)),
                    _ => Some(TelnetEvent::Iac), // invalid but safe
                }
            }

            State::Subnegotiation => {
                self.subnegotiation_len += 1;

                if self.subnegotiation_len > 1024 {
                    self.state = State::Data;
                    return Some(TelnetEvent::OversizedSubnegotiation);
                }

                if byte == 0xFF {
                    self.state = State::Iac;
                }

                None
            }
        }
    }
}
