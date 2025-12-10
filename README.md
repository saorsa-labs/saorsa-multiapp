# Saorsa Multi-App

A quantum-resistant desktop application for the Saorsa decentralized network.

## Features

- **Wallet Management** - BIP39 seed phrases with ML-DSA-65/ML-KEM-768 post-quantum keys
- **File Sync** - Dropbox-like synchronization with conflict resolution
- **Media Player** - Music and video streaming from the network
- **Data Migration** - Migrate data from Autonomi network to Saorsa

## Architecture

```
saorsa-multiapp/
├── crates/
│   ├── saorsa-desktop/           # Main Dioxus UI application
│   ├── saorsa-desktop-core/      # Business logic and state management
│   ├── saorsa-desktop-wallet/    # Wallet and key management
│   ├── saorsa-desktop-sync/      # File synchronization engine
│   ├── saorsa-desktop-media/     # Music and video playback
│   └── saorsa-desktop-migration/ # Autonomi migration tools
├── assets/                       # Static assets
└── tests/                        # Integration tests
```

## Requirements

- Rust 1.85+
- saorsa-node (sibling directory)

## Building

```bash
cargo build --release
```

## Running

```bash
cargo run -p saorsa-desktop
```

## License

GPL-3.0
