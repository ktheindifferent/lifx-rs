# TODO List - lifx-rs

## Immediate Tasks

### Testing
- [ ] Create unit tests for `Light` struct methods
- [ ] Create unit tests for `State` struct and builder methods
- [ ] Create unit tests for `Scene` operations
- [ ] Create unit tests for effect structures (BreatheEffect, PulseEffect, etc.)
- [ ] Create unit tests for `string_vec_to_params` function
- [ ] Add integration tests with mock HTTP server
- [ ] Test async methods separately
- [ ] Add error handling tests

### Code Improvements (from TODO comments)
- [ ] Support unlimited api_endpoints (currently has TODO comment)
- [ ] Use multithreaded timeout to detect primary API failures faster
- [ ] Add retry logic for failed API calls
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