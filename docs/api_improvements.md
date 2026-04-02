# lifx-rs API Improvements & Recommendations

## Executive Summary

This document outlines recommended API enhancements for the lifx-rs library to improve performance, reliability, and developer experience. Based on code review and best practices for networked systems, the following recommendations prioritize connection pooling, retry logic, rate limiting, and documentation improvements.

---

## 1. Connection Pooling (High Priority)

### Problem
Currently, each API call creates a new HTTP client instance via `reqwest`. This leads to:
- Repeated TCP connection establishment overhead
- Inefficient resource utilization
- Poor performance for rapid-fire requests
- Connection limit exhaustion in high-throughput scenarios

### Recommended Solution
Implement a shared connection pool using `reqwest::Client` as a singleton.

#### Implementation Strategy
```rust
// Add to LifxConfig
pub struct LifxConfig {
    access_token: String,
    api_endpoints: Vec<String>,
    http_client: Arc<reqwest::Client>,  // Reuse across requests
    // ... existing fields
}

impl LifxConfig {
    pub fn new(access_token: String) -> Self {
        let client = reqwest::Client::builder()
            .pool_connections(10)  // Connection pool size
            .pool_idle_timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to build HTTP client");
        
        LifxConfig {
            access_token,
            api_endpoints: Vec::new(),
            http_client: Arc::new(client),
            // ...
        }
    }
}
```

#### Benefits
- **Performance**: 3-5x faster consecutive requests
- **Resource Efficiency**: Reduced memory and file descriptor usage
- **Scalability**: Support for higher request throughput
- **Connection Reuse**: TCP handshake amortized across requests

#### Implementation Notes
- Make `http_client` public or provide accessor method
- Update all internal request methods to use pooled client
- Add configuration options for pool size and timeout
- Document pool behavior in API docs

---

## 2. Retry Logic with Exponential Backoff (High Priority)

### Problem
Network failures (transient errors, timeouts) are not automatically retried:
- Temporary network blips cause immediate failures
- No exponential backoff to avoid overwhelming the API
- Rate limiting failures not handled gracefully

### Recommended Solution
Implement a robust retry mechanism with exponential backoff.

#### Implementation Strategy
```rust
pub struct RetryConfig {
    max_retries: u32,
    initial_backoff_ms: u64,
    max_backoff_ms: u64,
    backoff_multiplier: f64,
    retry_on: RetryPolicy,
}

pub enum RetryPolicy {
    Standard,  // Retry on 429, 500-599, timeout, connection errors
    Aggressive,  // Also retry on 408 (timeout)
    Conservative,  // Only retry on 429 and connection errors
}

impl LifxConfig {
    pub fn with_retry_config(mut self, config: RetryConfig) -> Self {
        self.retry_config = Some(config);
        self
    }
}
```

#### Usage Example
```rust
let config = LifxConfig::new(token)
    .add_endpoint("https://api.lifx.com".to_string())
    .with_retry_config(RetryConfig {
        max_retries: 3,
        initial_backoff_ms: 100,
        max_backoff_ms: 5000,
        backoff_multiplier: 2.0,
        retry_on: RetryPolicy::Standard,
    });
```

#### Implementation Details
- Retry on:
  - 429 (Rate Limited) - with Retry-After header support
  - 500-599 (Server Errors)
  - Connection timeouts
  - Connection refused
  - DNS resolution failures
- Do NOT retry on:
  - 400, 401, 403, 404 (Client errors are permanent)
  - 413 (Payload too large)
- Include jitter in backoff to prevent thundering herd

#### Benefits
- **Reliability**: Temporary failures handled transparently
- **Rate Limit Respect**: Honors 429 responses with Retry-After
- **User Experience**: Better fault tolerance without user intervention

---

## 3. Rate Limiting (Medium Priority)

### Problem
API has undocumented rate limits; no built-in client-side protection:
- Potential for excessive requests violating API terms
- Cascading failures when rate limits hit
- No visibility into usage patterns

### Recommended Solution
Implement client-side rate limiting with token bucket algorithm.

#### Implementation Strategy
```rust
pub struct RateLimitConfig {
    requests_per_second: f64,
    burst_size: u32,
}

pub struct RateLimiter {
    rate_config: RateLimitConfig,
    tokens: Arc<Mutex<f64>>,
    last_refill: Arc<Mutex<Instant>>,
}

impl RateLimiter {
    pub async fn acquire(&self) {
        // Token bucket implementation
        // Wait if necessary to maintain rate limit
    }
}
```

#### Usage
```rust
let config = LifxConfig::new(token)
    .with_rate_limit(RateLimitConfig {
        requests_per_second: 10.0,  // Conservative estimate
        burst_size: 5,
    });
```

#### Implementation Details
- Token bucket algorithm for smooth rate limiting
- Per-endpoint rate limiting (if multiple endpoints)
- Async-friendly with minimal overhead
- Observable counters for diagnostics

#### Benefits
- **Compliance**: Prevents hitting API rate limits
- **Predictability**: Smooth, bounded request patterns
- **Observability**: Can track actual vs limit usage

---

## 4. Documentation Improvements (Medium Priority)

### 4.1 Public Method Documentation

Several public methods lack doc comments:

**LifxConfig methods:**
- `mark_endpoint_success()` - Needs example of how endpoint health affects failover
- `mark_endpoint_failure()` - Document backoff behavior
- `execute_with_failover()` - Add timeout and concurrency details

**Light methods:**
- All effect methods should have examples showing color format options
- Selector methods should document common selector patterns
- Async variants should clarify concurrency semantics

### 4.2 Documentation Gaps to Fix

**Missing Examples:**
- [ ] HSB color format with saturation/brightness ranges
- [ ] Kelvin color temperature selection (range: 1500-9000K)
- [ ] RGB color conversion examples
- [ ] Common selector patterns (label:X, group:Y, location:Z)
- [ ] Effect parameter combinations and constraints

**Missing Error Documentation:**
- [ ] When each error type is returned
- [ ] Network timeout behavior
- [ ] Rate limiting error handling
- [ ] Invalid parameter errors

**Missing Struct Documentation:**
- [ ] `EndpointHealth` - What metrics are tracked
- [ ] `FailoverConfig` - How each field affects behavior
- [ ] `Color` - Field constraints (hue: 0-360, saturation: 0-1, etc.)

### 4.3 Recommended Doc Comment Format

```rust
/// Sets the light state with optional color and brightness.
///
/// This method applies the given state to the light. All fields in the state
/// are optional; only specified fields will be modified.
///
/// # Arguments
///
/// * `config` - The LIFX configuration with API endpoint(s)
/// * `state` - The desired state (partial updates supported)
///
/// # Returns
///
/// Returns a `LiFxResults` containing operation status for this light.
/// Returns `Err` if the request fails or the light doesn't exist.
///
/// # Examples
///
/// Turn on and set to full brightness:
/// ```rust,no_run
/// # use lifx_rs::{Light, State, LifxConfig};
/// # let config = LifxConfig::new("token".to_string());
/// let mut state = State::new();
/// state.power = Some("on".to_string());
/// state.brightness = Some(1.0);
/// # let light = Light {
/// #     id: "1".to_string(),
/// #     uuid: "".to_string(),
/// #     label: "Lamp".to_string(),
/// #     connected: true,
/// #     power: "off".to_string(),
/// #     color: Default::default(),
/// #     group: Default::default(),
/// #     location: Default::default(),
/// #     product: Default::default(),
/// # };
/// light.set_state(config, state)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn set_state(&self, config: LifxConfig, state: State) -> Result<LiFxResults, reqwest::Error>
```

---

## 5. Error Handling Enhancements

### Current State
- Good use of `Result` types and `thiserror`
- Descriptive error messages for mutex operations
- Proper propagation with `?` operator

### Recommendations

1. **Add Specific Error Variants:**
   ```rust
   pub enum LifxError {
       RateLimited { retry_after: Option<Duration> },
       Timeout,
       ConnectionFailed { endpoint: String },
       InvalidSelector(String),
       InvalidColor(String),
       NotFound,
       Unauthorized,
   }
   ```

2. **Document Error Conditions in Doc Comments:**
   - When each error is likely to occur
   - How to handle each error type
   - Suggested recovery strategies

---

## 6. Performance Monitoring & Diagnostics

### Recommendation: Add Telemetry Hooks

```rust
pub trait LifxTelemetry {
    fn on_request_start(&self, endpoint: &str, method: &str);
    fn on_request_end(&self, endpoint: &str, status: u32, duration: Duration);
    fn on_retry(&self, endpoint: &str, attempt: u32);
    fn on_rate_limited(&self, endpoint: &str, retry_after: Option<Duration>);
}
```

Benefits:
- Application can implement custom logging/metrics
- No compile-time overhead if not used
- Supports distributed tracing integration

---

## 7. Configuration Best Practices

### Recommended Configuration Pattern

```rust
impl LifxConfig {
    /// Create a production-ready configuration with sensible defaults
    pub fn production(access_token: String, primary_endpoint: String) -> Self {
        Self::new(access_token)
            .add_endpoint(primary_endpoint)
            .with_strategy(FailoverStrategy::FastestFirst)
            .with_timeout(30_000)
            .with_retry_config(RetryConfig {
                max_retries: 3,
                initial_backoff_ms: 100,
                max_backoff_ms: 10_000,
                backoff_multiplier: 2.0,
                retry_on: RetryPolicy::Standard,
            })
            .with_rate_limit(RateLimitConfig {
                requests_per_second: 10.0,
                burst_size: 5,
            })
    }
    
    /// Create a quick development configuration
    pub fn dev(access_token: String) -> Self {
        Self::new(access_token)
            .add_endpoint("http://localhost:8089".to_string())
            .with_timeout(5_000)
    }
}
```

---

## 8. Implementation Priority & Effort Estimate

| Feature | Priority | Effort | Impact | Recommended Phase |
|---------|----------|--------|--------|-------------------|
| Connection Pooling | High | Medium | High | Phase 1 |
| Retry Logic | High | Medium | High | Phase 1 |
| Rate Limiting | Medium | Medium | Medium | Phase 2 |
| Doc Improvements | Medium | Low | Medium | Phase 1 |
| Error Enhancements | Medium | Low | Medium | Phase 2 |
| Telemetry | Low | Medium | Low | Phase 3 |
| Config Presets | Low | Low | Low | Phase 1 |

---

## 9. Testing Strategy

### Recommended Tests

1. **Connection Pool Tests:**
   - Verify pool reuse across multiple requests
   - Test pool size limits
   - Verify timeout behavior

2. **Retry Logic Tests:**
   - Mock transient failures and verify retries
   - Test backoff timing
   - Verify 429 Retry-After header handling
   - Verify non-retryable errors fail immediately

3. **Rate Limiting Tests:**
   - Verify token bucket behavior
   - Test burst capacity
   - Verify sustained rate limiting

4. **Integration Tests:**
   - Test with mock HTTP server
   - Simulate network failures
   - Test failover between endpoints

---

## 10. Breaking vs Non-Breaking Changes

### Non-Breaking (Additive)
- New `with_*` builder methods
- New `RetryConfig` and `RateLimitConfig` types
- New configuration presets

### Potentially Breaking
- Changing `LifxConfig` internal structure (mitigate by keeping all fields public)
- Changing error types (if external code pattern matches on them)

**Recommendation:** Version as `0.2.0` to signal API enhancements while maintaining backward compatibility where possible.

---

## Summary

These improvements focus on production-readiness:

1. **Connection Pooling** - Essential for performance
2. **Retry Logic** - Critical for reliability
3. **Rate Limiting** - Important for API compliance
4. **Documentation** - Reduces friction for users
5. **Error Handling** - Improves debuggability

All recommendations use existing Rust ecosystem patterns and libraries already in the dependency tree.
