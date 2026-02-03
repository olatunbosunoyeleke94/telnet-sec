# Telnet-Sec 🛡️
**Defensive Telnet parsing, sanitization, and proxying in Rust**

This project provides a hardened Telnet parsing and sanitization layer designed
to mitigate known and emerging Telnet protocol vulnerabilities, including
memory exhaustion, state desynchronization, and command injection issues.

It was developed in response to modern Telnet exploitation research,
including issues similar in class to **CVE-2026-24061 (GNU inetutils telnet)**.

---

## ⚠️ Threat Model

Telnet is a legacy, plaintext protocol with a complex control grammar
(IAC, WILL/WONT/DO/DONT, subnegotiation). Many historical Telnet
implementations:

- Trust malformed control sequences
- Allocate unbounded subnegotiation buffers
- Desynchronize protocol state
- Execute unintended commands

This project assumes **all Telnet control traffic is hostile**.

---

## 🧱 Architecture

```

┌──────────────┐
│ Telnet Client│
└──────┬───────┘
       │
       ▼
┌─────────────────────┐
│ telnet-sanitizer │ ← Drops all Telnet control logic
│ (policy layer) │
└──────┬──────────────┘
       │ 
sanitized bytes only
       ▼
┌─────────────────────┐
│ safe-telnet-parser │ ← Hardened state machine
└──────┬──────────────┘
       │
       ▼
┌──────────────┐
│ Backend Host │
└──────────────┘

```

The sanitizer operates as a strict policy boundary: Telnet control bytes
are removed *before* parsing, ensuring protocol state cannot influence
application-layer behavior.


telnet-sanitizer depends on safe-telnet-parser internally.

End users typically only need telnet-sanitizer; 
the parser crate is exposed separately for advanced use cases and research.

## 🔒 Security Properties

✔ Panic-free  
✔ No unsafe code  
✔ Bounded subnegotiation buffers  
✔ Explicit state machine  
✔ No Telnet option negotiation allowed  
✔ Suitable for proxies, gateways, and sandboxes  

---

## 📦 Crates

### `safe-telnet-parser`
Low-level, no_std-compatible Telnet parser that converts raw bytes
into structured protocol events while defending against malformed input.

### `telnet-sanitizer`
Opinionated sanitization layer that **drops all Telnet control sequences**
and forwards only plain user data.

---

## 🧪 Testing

- Unit tests for malformed sequences
- Oversized subnegotiation detection
- Invalid command handling
- Fuzz testing (see below)
- Regression test covering Telnet user-flag injection
  (e.g. `USER='-f root'`-style attacks)

## Examples

- Tiny Parsing Example (safe-telnet-parser)

```
use safe_telnet_parser::{TelnetParser, TelnetEvent};

let mut parser = TelnetParser::new();

// Raw Telnet bytes (includes IAC control byte 0xFF)
let input = b"hello\xFF\xFAworld";

let events = parser.parse(input);

for event in events {
    if let TelnetEvent::Data(byte) = event {
        print!("{}", byte as char);
    }
}

```

output: 
```
hellowworld
```

- Tiny Sanitizer Example (telnet-sanitizer)

```
use telnet_sanitizer::TelnetSanitizer;

let mut sanitizer = TelnetSanitizer::new();

// Malicious Telnet payload (IAC-based injection attempt)
let input = b"-f root\xFF\xFA";

let clean = sanitizer.sanitize(input);

assert_eq!(clean, b"-f root");

```

- Live CVE-style malicious input stripped

Input bytes (attacker-controlled)
```
[0x2d, 0x66, 0x20, 0x72, 0x6f, 0x6f, 0x74, 0xff, 0xff]

```

Sanitized output:

```
"-f root"

```  

---

## 🚨 CVE Context

This project mitigates entire classes of Telnet vulnerabilities similar to:

- CVE-2026-24061 (GNU inetutils Telnet)
- Subnegotiation buffer overflows
- IAC state desynchronization
- Option negotiation injection

Rather than patching a single bug, this project **removes the attack surface**.

---

## ⚠️ Disclaimer

This software is intended for defensive and research purposes.
Do not expose Telnet services to the internet unless absolutely necessary.
