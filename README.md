# Nano Vault

An offline, hardware-encrypted personal ledger built specifically for the Ledger Nano X.

## Why I wrote this

I wanted an actual useful custom app that I could sideload onto my Ledger Nano X. 
This is a secure, completely offline vault for tracking personal finances, secret stashes, and emergency reserves.  By running this directly on the Ledger's Secure Element, the data is hardware-encrypted and never touches an internet-connected device.

## TODO List

- [x] Basic project setup (Rust, `no_std`, `Makefile`)
- [x] Set up state management and cryptography skeleton
- [x] Integrate with Ledger SDK and compile to `.hex`
- [ ] Implement on-device UI screens using Ledger's graphics library
- [ ] Add APDU handlers for computer/companion app communication
- [ ] Implement persistent storage (NVM) to save entries across reboots
- [ ] Bind vault encryption securely to the Ledger's 24-word seed phrase

## Building

If you have Docker/Orbstack installed, you can build the application using the official Ledger toolchain:

```bash
make docker-build
```

## Sideloading & Testing

The Ledger Nano X firmware strictly prohibits sideloading custom, unsigned applications via APDU/developer tools (returning a `0x5120` Instruction Not Supported error). The Custom CA backdoor has been removed from recent Nano X firmware updates.

To test this application, you must use one of the following methods:

1. **Speculos Emulator:** The official Ledger hardware emulator. It runs locally and provides a virtual display.
2. **Physical Nano S Plus:** Ledger recommends using a Ledger Nano S Plus for physical hardware testing, as its firmware still supports the developer sideloading backdoor.

### Running with Speculos (Docker)

You can run the application locally using the Speculos emulator via Docker. It will host a web interface on port 5000 so you can view and interact with the virtual device.

```bash
docker run --rm -it -v "$(pwd):/app" -p 5000:5000 ghcr.io/ledgerhq/speculos --model nanox --display headless /app/target/nanox/release/nano-vault
```
