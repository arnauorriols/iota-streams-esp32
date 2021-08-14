# IOTA-Streams in ESP32
This repository is a PoC demonstrating the capacity to use the IOTA-Streams-Channels protocol from an ESP32 device
using the [Rust client implementation].

[espressif/rust-esp32-example] has been used as boilerplate. To compile, we need a fork of LLVM and a fork of Rustc.
This compilation environment is provided in the Docker image [espressif/idf-rust][docker espressif/idf-rust].
To run the binary an ESP32 QEMU is available in the Docker image [mluis/qemu-esp32][docker mluis/qemu-esp32].

# Status
At the time of writing (13-08-2021), this PoC is capable of running in a QEMU ESP32 emulator, performing:
- [x] Create bucket transport (ie store messages in heap memory using HashMap)
- [x] Create instance of Author
  - Generate ed25519 and x25519 key pairs
- [ ] Create instance of Subscriber
  - Generate ed25519 and x25519 key pairs
- [x] Author send announcement message
	- Sign with ed25519
- [ ] Subscriber send subscribe message
	- Generate random unsubscription key
	- Perform Diffie-Hellman exchange
	- Wrap unsubscription key with DH shared secret
- [ ] Author send Keyload message (note: without subscribers this does kind of nothing)
	- [x] Generate random encryption key
	- [ ] Perform Diffie-Hellman exchange
	- [ ] Wrap encryption key with DH shared secret
- [ ] Subscriber receive keyload message
  - Perform Diffie-Hellman exchange
  - Unwrap encryption key with DH shared secret
- [x] Author send signed packet (x2)
  - Encrypt masked payload with encryption key
  - Sign with ed25519
- [ ] Subscriber receives signed packets
- [x] Call esp-idf C lib from Rust

## Limitations
- For still uninvestigated (but seamingly obvious) reasons, channel address is 32 bytes long, the rest gets padded with 0.
- `rand::rngs::ThreadRng` is not supported (linker error) and there isn't room for future implementation, therefore Streams
  and its dependencies need to use `rand::rngs::StdRng` or `rand::rngs::OsRng` instead. This is not a practical issue
	in the current scope of the PoC. See [Github issue][github-issue-thread_rng] for more details.
- ESP-IDF task watchdog must be deactivated, or increased an uninvestigated amount.
- `getrandom` does not yet support `espidf` targets and requires an small patch. Likely to accept a PR once espidf targets make nightly or stable.
- Several subscriber actions are currently generating undeterministic errors (seemingly stack overflows or other heap corruptions):
	- receive announcement msg
		- Activating the `comprehensive` heap corruption detection of esp-idf somehow, sometimes mitigates the problem.
		  Haven't tried with `light impact` nor have discarded that it's an emulator issue
	- fetch_next_msgs
- Currently Streams requires `std` for generating random keys and nonces.
	- Technically `rand::rngs::StdRng` or `rand::rngs::OsRng` work in `no_std` scenarios, provided that `getrandom` supports the target.
	- `rand` is being used only when sending keyload and subscribe messages. Therefore, if those don't need to be sent, Streams can be used
	  without `std` feature.
- `chrono` has compatibility issues with this target, and will require a patch. Streams dependency over `chrono` was a legacy and has been
  removed, thus no further investigation has been performed. All I can say at this moment is that it has to do with [`struct tm` not including
	`tm_gmtoff` in `newlib` variant of `libc`][tm_gmtoff missing]
- opt-level 'z' does not work for some reason; the program freezes few seconds after starting, deterministically. Executable size is not much
	smaller than opt-level 's' anyhow. However, remains to be seen if using opt-level 1-3 solves the heap corruption issues mentioned above.

## Memory Usage
See [memory usage output].

## Runtime Output
See [runtime output].

## Upstream
- [espressif/rust-esp32-example]: boilerplate used for this PoC. Aparently the initial integration project
- [ivmarkov/rust-esp32-std-hello]: Alternative, pure-Rust example integration. Pending to be tested
- [esp-rs]: "The esp-rs organization has been formed to develop runtime, pac and hal crates for the Espressif chips (bare-metal as well as ESP-IDF based)"
- [esp-rs/rust]: Rust fork
- [espressif/llvm-project]: LLVM fork
- [rust-lang/rust#87666]: PR to merge libstd fork upstream. **Merged 12-08-2021**
- [rust-lang/libc#2310]: PR to merge libc fork upstream. **Merged 3-08-2021**

# Quickstart

```bash
./scripts/docker/compile.sh && ./scripts/docker/run-qemu.sh
```

Rust source code is in `components/rustlib`. Main function is in `main/main.c`. C lib is in `components/clib`.

# TODO
- [ ] Test complete flow with SingleBranching
- [ ] Fix issues in subscriber to test the complete flow
- [ ] Test in real ESP32 device
- [ ] Test agaist Tangle using `sync-client`


[Rust client implementation]: https://github.com/iotaledger/streams
[espressif/rust-esp32-example]: https://github.com/espressif/rust-esp32-example
[ivmarkov/rust-esp32-std-hello]: https://github.com/ivmarkov/rust-esp32-std-hello
[esp-rs]: https://github.com/esp-rs
[esp-rs/rust]: https://github.com/esp-rs/rust
[espressif/llvm-project]: https://github.com/espressif/llvm-project
[rust-lang/rust#87666]: https://github.com/rust-lang/rust/pull/87666
[rust-lang/libc#2310]: https://github.com/rust-lang/libc/pull/2310
[memory usage output]: MEMORY_USAGE.md
[runtime output]: RUNTIME_OUTPUT.md
[github-issue-thread_rng]: https://github.com/espressif/rust-esp32-example/issues/23
[docker espressif/idf-rust]: https://hub.docker.com/r/espressif/idf-rust
[docker mluis/qemu-esp32]: https://hub.docker.com/r/mluis/qemu-esp32
[tm_gmtoff missing]: https://github.com/rust-lang/libc/blob/e1eb9721dc4534cea84e9e3bf591e7cb257e679c/src/unix/newlib/mod.rs#L105-L115