# IOTA-Streams in ESP32 
This repository is a PoC demonstrating the capacity to use the IOTA-Streams-Channels protocol from an ESP32 device
using the [Rust client implementation].

# Status
At the time of writing (13-08-2021), this PoC is capable of running in a QEMU ESP32 emulator, performing:
- [x] Create bucket transport (ie store messages in heap memory using HashMap)
- [x] Create instance of Author
  - Generate ed25519 and x25519 key pairs
- [] Create instance of Subscriber
  - Generate ed25519 and x25519 key pairs
- [x] Author send announcement message
	- Sign with ed25519
- [] Subscriber send subscribe message
	- Generate random unsubscription key
	- Perform Diffie-Hellman exchange
	- Wrap unsubscription key with DH shared secret
- [] Author send Keyload message (note: without subscribers this does kind of nothing)
	- [x] Generate random encryption key
	- [] Perform Diffie-Hellman exchange 
	- [] Wrap encryption key with DH shared secret
- [] Subscriber receive keyload message
  - Perform Diffie-Hellman exchange
	- Unwrap encryption key with DH shared secret
- [x] Author send signed packet (x2)
  - Encrypt masked payload with encryption key
	- Sign with ed25519
- [] Subscriber receives signed packets

## Limitations
- For still uninvestigated (but seamingly obvious) reasons, channel address is 32 bytes long, the rest gets padded with 0. 
- `rand::rngs::ThreadRng` is not supported, therefore Streams and its dependencies need to use `rand::rngs::StdRng` or
  `rand::rngs::OsRng` instead. This is not a practical issue in the current scope of the PoC. 
- ESP-IDF task watchdog must be deactivated, or increased an uninvestigated amount. 
- `getrandom` does not yet support `espidf` targets and requires an small patch
- main task stack size must be increased an uninvestigated amount from the default value
- Needs `comprehensive' heap corruption detection to go past `subscriber.receive_announcement()`. Haven't tried with `light impact` nor discarded it's an emulator issue

## Memory Usage
See [memory usage output].

## Runtime Output
See [runtime output].

## Upstream Status

# Quickstart

# TODO



[Rust client implementation]: https://github.com/iotaledger/streams
[memory usage output]: MEMORY_USAGE.md
[runtime output]: RUNTIME_OUTPUT.md