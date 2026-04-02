# TODO List - lifx-rs

## Build & QA Status (April 2, 2026) ✅
- ✅ Cargo build successful (15.31s)
- ✅ All 66 unit tests passing
- ✅ All 61 doc tests passing (2 ignored)
- ✅ Code formatted with rustfmt
- ⚠️ Clippy: 210 warnings (mostly style: needless returns, useless conversions)
- 📄 API improvements documentation created (docs/api_improvements.md)

## Immediate Tasks (Priority Order)

### Code Quality (High Priority)
- [ ] Fix all clippy warnings (needless returns, derive traits, etc.) - 2-3h effort
- [ ] Implement custom error type with thiserror crate
- [ ] Add error source chains and context
- [ ] Ensure 100% clippy clean build

### Testing (Next Phase)
- [ ] Create integration tests with mockito/wiremock
- [ ] Create unit tests for `Light` struct methods
- [ ] Create unit tests for `State` struct and builder methods
- [ ] Test async methods separately (not just compile tests)
- [ ] Add error handling tests
- [ ] Add performance benchmarks (criterion)

### Code Improvements (from TODO comments)
- [ ] Support unlimited api_endpoints (currently has TODO comment)
- [ ] Use multithreaded timeout to detect primary API failures faster
- [ ] Add retry logic for failed API calls with exponential backoff
- [ ] Implement connection pooling for better performance

### Documentation
- [ ] Add inline documentation for all public methods
- [ ] Create usage examples for each effect type
- [ ] Document error types and when they occur
- [ ] Add troubleshooting guide

### Feature Completeness
- [ ] Implement missing LAN protocol messages
- [ ] Add support for more device capabilities
- [ ] Implement device discovery caching
- [ ] Add rate limiting to respect API limits

### Code Quality
- [ ] Run clippy and fix all warnings
- [ ] Format code with rustfmt
- [ ] Add CI/CD pipeline configuration
- [ ] Set up code coverage reporting
- [ ] Review and optimize error handling

### Performance
- [ ] Benchmark sync vs async operations
- [ ] Optimize message serialization/deserialization
- [ ] Reduce allocations in hot paths
- [ ] Profile and optimize network operations

## Future Enhancements

### Advanced Features
- [ ] WebSocket support for real-time updates
- [ ] Batch operations optimization
- [ ] Local caching of light states
- [ ] Automatic reconnection logic
- [ ] Event system for state changes

### Platform Support
- [ ] Test on different OS platforms
- [ ] Add WASM support for browser usage
- [ ] Create FFI bindings for other languages
- [ ] Mobile platform considerations

### Developer Experience
- [ ] Create a CLI tool using this library
- [ ] Develop a GUI application example
- [ ] Add macro support for common patterns
- [ ] Create a testing framework for LIFX applications

## Completed Tasks
- ✅ Basic library structure implemented
- ✅ Official API support added
- ✅ Unofficial API support added
- ✅ LAN protocol basics implemented
- ✅ Synchronous and asynchronous methods
- ✅ Basic test structure in lan.rs
- ✅ Project documentation created