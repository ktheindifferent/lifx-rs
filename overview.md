# lifx-rs - High Level Overview

## Project Purpose
A comprehensive Rust library for controlling LIFX smart lights through multiple APIs, providing both synchronous and asynchronous interfaces.

## Key Components

### 1. Main Library (`lib.rs`)
The core library providing high-level interfaces for:
- **Light Control**: Power on/off, brightness, color changes
- **Effects**: Breathe, pulse, morph, flame, and move effects
- **Scene Management**: List and activate predefined scenes
- **Group Operations**: Control multiple lights simultaneously
- **State Management**: Get and set light states with validation

### 2. LAN Protocol (`lan.rs`)
Low-level implementation of the LIFX LAN protocol:
- Direct UDP communication with lights on local network
- Discovery mechanism for finding lights
- Binary protocol encoding/decoding
- Message frame construction and parsing

## API Support Matrix

| Feature | Official API | Unofficial API | LAN Protocol |
|---------|--------------|----------------|--------------|
| List Lights | ✅ | ✅ | ✅ |
| Set State | ✅ | ✅ | ✅ |
| Set States | ✅ | ✅ | ❌ |
| State Delta | ✅ | ❌ | ❌ |
| Toggle Power | ✅ | ❌ | ✅ |
| Effects | ✅ | ❌ | ❌ |
| Scenes | ✅ | ❌ | ❌ |
| Clean (HEV) | ✅ | ❌ | ❌ |
| Color Validation | ✅ | ❌ | ❌ |

## Usage Patterns

### Synchronous Usage
```rust
let config = LifxConfig { /* ... */ };
let lights = Light::list_all(config.clone())?;
light.set_state(config, state)?;
```

### Asynchronous Usage
```rust
let lights = Light::async_list_all(config.clone()).await?;
light.async_set_state(config, state).await?;
```

## Key Design Decisions

1. **Dual API Support**: Supports both official cloud API and local network control
2. **Sync/Async Duality**: Every operation available in both blocking and async variants
3. **Builder Pattern**: State objects use builder pattern for flexible configuration
4. **Error Handling**: Comprehensive error types with thiserror for better diagnostics
5. **Zero-Copy Protocol**: Efficient binary protocol handling in LAN module

## Testing Strategy

- **Unit Tests**: Protocol encoding/decoding in `lan.rs`
- **Integration Tests**: Needed for API interactions
- **Mock Testing**: Required for HTTP client testing
- **LAN Protocol Tests**: UDP packet construction and parsing

## Dependencies Philosophy

- Minimal dependencies for core functionality
- Optional features for TLS support
- Standard ecosystem crates (serde, reqwest, thiserror)
- No unsafe code in main library

## Target Use Cases

1. **Home Automation**: Integrate LIFX lights into smart home systems
2. **Desktop Applications**: Control lights from desktop software
3. **IoT Projects**: Embedded systems controlling lights locally
4. **Web Services**: Backend services managing lighting through cloud API