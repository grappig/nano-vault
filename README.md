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
