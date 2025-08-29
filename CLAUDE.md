# CLAUDE.md - AI Assistant Documentation for lifx-rs

## Project Overview

**lifx-rs** is a comprehensive Rust library for controlling LIFX smart lights through multiple communication channels:
- Official LIFX Cloud API (HTTPS)
- Unofficial Offline API (HTTP)
- Direct LAN Protocol (UDP)

The library provides both synchronous and asynchronous interfaces for all operations, making it suitable for various application architectures.

## Repository Structure

```
lifx-rs/
├── src/
│   ├── lib.rs       # Main library implementation with high-level API
│   └── lan.rs       # Low-level LIFX LAN protocol implementation
├── Cargo.toml       # Package manifest and dependencies
├── README.md        # User-facing documentation
├── LICENSE          # Apache 2.0 license
├── LICENSE-MIT      # MIT license (dual-licensed)
├── overview.md      # High-level architecture overview
├── project_description.md  # Detailed project status
└── todo.md          # Outstanding tasks and improvements

```

## Core Components

### 1. Main Library (src/lib.rs)
- **Primary Structs**: `Light`, `State`, `Scene`, `Color`, `Group`, `Location`, `Product`
- **Configuration**: `LifxConfig` for API endpoints and authentication
- **Effects**: `BreatheEffect`, `PulseEffect`, `MorphEffect`, `FlameEffect`, `MoveEffect`
- **Results**: `LifxResults` for API responses
- **Error Handling**: Custom `Error` type with detailed error messages

### 2. LAN Protocol (src/lan.rs)
- Low-level binary protocol implementation
- UDP broadcast discovery mechanism
- Message frame encoding/decoding
- Direct local network communication without cloud dependency

## Key Dependencies

```toml
serde = "1.0.162"           # Serialization/deserialization
serde_json = "1.0.96"       # JSON support
reqwest = "0.11.17"         # HTTP client (blocking + async)
trust-dns-resolver = "0.22.0"  # DNS resolution
byteorder = "1.4.3"         # Binary protocol handling
thiserror = "1.0.40"        # Error handling
```

## API Capabilities

### Supported Operations

| Feature | Official API | Unofficial API | LAN Protocol |
|---------|--------------|----------------|--------------|
| List Lights | ✅ | ✅ | ✅ |
| Set State | ✅ | ✅ | ✅ |
| Set Multiple States | ✅ | ✅ | ❌ |
| State Delta | ✅ | ❌ | ❌ |
| Toggle Power | ✅ | ❌ | ✅ |
| Breathe Effect | ✅ | ❌ | ❌ |
| Pulse Effect | ✅ | ❌ | ❌ |
| Morph Effect | ✅ | ❌ | ❌ |
| Flame Effect | ✅ | ❌ | ❌ |
| Move Effect | ✅ | ❌ | ❌ |
| Effects Off | ✅ | ❌ | ❌ |
| Clean (HEV) | ✅ | ❌ | ❌ |
| List Scenes | ✅ | ❌ | ❌ |
| Validate Color | ✅ | ❌ | ❌ |

## Usage Examples

### Basic Light Control (Synchronous)
```rust
let config = LifxConfig {
    access_token: "your_token".to_string(),
    api_endpoints: vec!["https://api.lifx.com".to_string()]
};

// List all lights
let lights = Light::list_all(config.clone())?;

// Control a specific light
let mut state = State::new();
state.power = Some("on".to_string());
state.brightness = Some(0.8);
state.color = Some("blue".to_string());

light.set_state(config, state)?;
```

### Asynchronous Operations
```rust
// All methods have async equivalents
let lights = Light::async_list_all(config.clone()).await?;
light.async_set_state(config, state).await?;
```

### LAN Protocol Discovery
```rust
// Send GetService broadcast to discover local lights
// Implemented in lan.rs for direct UDP communication
```

## Code Quality & Testing

### Current Test Coverage
- **Unit Tests**: 35 tests in lib.rs (all passing)
- **LAN Protocol Tests**: 10 tests in lan.rs (all passing)
- **Doctests**: 55 tests (currently failing - need updating)

### Known Issues
1. Doctests in lib.rs need updating to match current API
2. Clippy warnings for code style improvements
3. ~~TODO: Support unlimited api_endpoints~~ ✅ COMPLETED
4. ~~TODO: Implement multithreaded timeout for faster API failure detection~~ ✅ COMPLETED

### Recent Improvements (Feature Branch: feature/unlimited-api-endpoints)
- **Unlimited API Endpoints**: Removed hardcoded 2-endpoint limit, now supports any number of endpoints
- **Concurrent Failover**: Implements multithreaded timeout detection with configurable concurrent attempts
- **Advanced Strategies**: Added Round-Robin, Failover, and Fastest-First endpoint selection strategies
- **Health Tracking**: Automatic endpoint health monitoring with exponential backoff for failed endpoints
- **Builder Pattern**: New fluent API for configuration with `LifxConfig::new()` builder methods
- **Backward Compatibility**: Legacy configuration still works for existing code

## Development Guidelines

### When Adding New Features
1. Implement both synchronous and asynchronous versions
2. Add comprehensive error handling using the existing Error type
3. Include unit tests for new functionality
4. Update this documentation

### API Design Principles
- **Dual Interface**: Always provide both sync and async methods
- **Builder Pattern**: Use for complex configuration objects
- **Error Propagation**: Use Result types with detailed errors
- **Zero-Copy**: Optimize protocol handling in LAN module

### Testing Strategy
1. Unit tests for data structures and transformations
2. Integration tests with mock servers for API calls
3. Protocol tests for binary encoding/decoding
4. Manual testing with real LIFX devices

## Build & Run Commands

```bash
# Build the library
cargo build

# Run tests
cargo test

# Run specific test module
cargo test lan::tests

# Check for issues
cargo clippy

# Build documentation
cargo doc --open

# Build with all features
cargo build --all-features
```

## Architecture Notes

### Multi-Endpoint Support
The library supports multiple API endpoints, allowing fallback from cloud to local APIs:
- Primary: Official LIFX API (requires internet)
- Fallback: Local unofficial API server
- Direct: LAN protocol (no server required)

### State Management
- States are immutable value objects
- Use builder pattern for construction
- All fields are optional for partial updates

### Error Handling
- Comprehensive error types using thiserror
- Network errors properly propagated
- Protocol errors for invalid messages
- Validation errors for invalid parameters

#### Error Handling Patterns (Updated)
**Mutex Lock Handling:**
- All mutex `.lock()` calls use `.expect()` with descriptive error messages
- Mutex poisoning is treated as a critical error that should panic with context
- Example: `.lock().expect("Failed to acquire endpoint_health mutex lock: mutex was poisoned")`

**HTTP Client Builder Errors:**
- Client builder errors are properly handled with pattern matching
- No `.unwrap()` calls on `reqwest::Client::builder().build()`
- Errors are propagated using `Result` types or handled with fallback logic

**Testing Error Conditions:**
- Tests verify that mutex operations don't panic under normal conditions
- Specific test for mutex poisoning behavior with `#[should_panic]`
- Tests ensure error handling code paths work correctly

**Guidelines for Contributors:**
1. Never use `.unwrap()` in production code paths
2. Use `.expect()` with descriptive messages for unrecoverable errors
3. Prefer `?` operator for error propagation in functions returning Result
4. Add tests for both success and error conditions
5. Document panic conditions in function documentation

## Future Improvements

1. **Performance**: Implement connection pooling for HTTP clients
2. **Features**: Add support for tile and beam devices
3. **Testing**: Add integration tests with mock servers
4. **Documentation**: Fix failing doctests
5. **Code Quality**: Address clippy warnings
6. **Reliability**: Add retry logic for network operations
7. **Discovery**: Improve LAN discovery with caching

## Quick Reference

### Common Selectors
- `"all"` - All lights
- `"label:Kitchen"` - Light by label
- `"group:Living Room"` - All lights in group
- `"location:Home"` - All lights in location

### Color Formats
- Named colors: `"red"`, `"blue"`, `"green"`
- HSB: `"hue:120 saturation:1.0 brightness:0.5"`
- Kelvin: `"kelvin:3500"`
- RGB: `"rgb:255,0,0"`

### Power States
- `"on"` - Turn light on
- `"off"` - Turn light off
- Toggle through API methods

## Maintenance Notes

- Version: 0.1.30
- License: Dual MIT/Apache-2.0
- Author: Caleb Mitchell Smith-Woolrich
- Repository: https://github.com/PixelCoda/lifx-rs
- Documentation: https://docs.rs/lifx-rs