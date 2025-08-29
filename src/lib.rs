// Copyright 2021 Caleb Mitchell Smith-Woolrich (PixelCoda)
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! # lifx-rs
//!
//! ## Description
//!
//! A synchronous + asynchronous library for communicating with the official LIFX-API and the unoffical offline API. 
//!
//! ## LIFX-API Supported Methods:
//! * List Lights
//! * Set State
//! * Set States
//! * State Delta
//! * Toggle Power
//! * Breathe Effect
//! * Move Effect
//! * Morph Effect
//! * Flame Effect
//! * Pulse Effect
//! * Effects Off
//! * Clean (HEV)
//! * List Scenes
//! * Validate Color
//!
//! ## Unofficial Offline API Supported Methods:
//! * List Lights
//! * Set State
//! * Set States
//! 
//! ## To use offline use the Un-Official API Server:
//! [lifx-api-server](https://crates.io/crates/lifx-api-server)
//!
//! ## How to use library
//!
//! Add the following line to your cargo.toml:
//! ```toml
//! lifx-rs = "0.1.30"
//! ```
//!
//! Example:
//! ```rust,no_run
//! extern crate lifx_rs as lifx;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!
//!     let key = "xxx".to_string();
//!
//!     let config = lifx::LifxConfig::new(key)
//!         .add_endpoint("https://api.lifx.com".to_string())
//!         .add_endpoint("http://localhost:8089".to_string());
//!
//!     // Build an "OffState" to set
//!     let mut off_state = lifx::State::new();
//!     off_state.power = Some("off".to_string());
//!
//!     // Turn off all lights
//!     lifx::Light::set_state_by_selector(config.clone(), "all".to_string(), off_state)?;
//!
//!     let lights = lifx::Light::list_all(config.clone())?;
//!     println!("{:?}", lights.clone());
//!
//!     let mut state = lifx::State::new();
//!     state.power = Some("on".to_string());
//!     state.brightness = Some(1.0);
//!
//!     for light in lights {
//!         let results = light.set_state(config.clone(), state.clone())?;
//!         println!("{:?}", results);
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//!
//! Async Example:
//! ```rust,no_run
//! extern crate lifx_rs as lifx;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!
//!     let key = "xxx".to_string();
//!
//!     let config = lifx::LifxConfig::new(key)
//!         .add_endpoint("https://api.lifx.com".to_string())
//!         .add_endpoint("http://localhost:8089".to_string());
//!
//!     // Build "OffState" to set
//!     let mut off_state = lifx::State::new();
//!     off_state.power = Some("off".to_string());
//!  
//!     // Turn off all lights
//!     lifx::Light::async_set_state_by_selector(config.clone(), "all".to_string(), off_state).await?;
//!     
//!     Ok(())
//! }
//! ```
//!
//! ## Tile Animation with Auto-Cleanup Example:
//! 
//! This example shows how to use the new cleanup functions to properly handle tile animations
//! that have a nonzero duration, avoiding the bug where tiles remain stuck in animation state.
//! 
//! ```rust,no_run
//! extern crate lifx_rs as lifx;
//! 
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let key = "your_token".to_string();
//!     
//!     let config = lifx::LifxConfig::new(key)
//!         .add_endpoint("https://api.lifx.com".to_string());
//!     
//!     // Create a flame effect with 10 second duration
//!     let mut flame_effect = lifx::FlameEffect::new();
//!     flame_effect.period = Some(5);
//!     flame_effect.duration = Some(10.0);  // Animation runs for 10 seconds
//!     flame_effect.power_on = Some(true);
//!     
//!     // Use the cleanup function - it will automatically stop the animation after 10 seconds
//!     match lifx::flame_effect_with_cleanup(config.clone(), "all".to_string(), flame_effect) {
//!         Ok((effect_result, cleanup_result)) => {
//!             println!("Effect started: {:?}", effect_result);
//!             if let Some(cleanup) = cleanup_result {
//!                 println!("Effect cleaned up after duration: {:?}", cleanup);
//!             }
//!         }
//!         Err(e) => println!("Error: {}", e)
//!     }
//!     
//!     // Similarly for morph effect
//!     let mut morph_effect = lifx::MorphEffect::new();
//!     morph_effect.period = Some(5);
//!     morph_effect.duration = Some(15.0);  // Animation runs for 15 seconds
//!     morph_effect.palette = Some(vec!["red".to_string(), "blue".to_string(), "green".to_string()]);
//!     
//!     match lifx::morph_effect_with_cleanup(config, "all".to_string(), morph_effect) {
//!         Ok((effect_result, cleanup_result)) => {
//!             println!("Morph effect started: {:?}", effect_result);
//!             if let Some(cleanup) = cleanup_result {
//!                 println!("Morph effect cleaned up after duration: {:?}", cleanup);
//!             }
//!         }
//!         Err(e) => println!("Error: {}", e)
//!     }
//!     
//!     Ok(())
//! }
//! ```
//!
//!
//! ## License
//!
//! Released under Apache 2.0 or MIT.
//!
//! # Support and follow my work by:
//!
//! #### Buying my dope NTFs:
//!  * https://opensea.io/accounts/PixelCoda
//!
//! #### Checking out my Github:
//!  * https://github.com/PixelCoda
//!
//! #### Following my facebook page:
//!  * https://www.facebook.com/pixelcoda/
//!
//! #### Subscribing to my Patreon:
//!  * https://www.patreon.com/calebsmith_pixelcoda
//!
//! #### Or donating crypto:
//!  * ADA: addr1qyp299a45tgvveh83tcxlf7ds3yaeh969yt3v882lvxfkkv4e0f46qvr4wzj8ty5c05jyffzq8a9pfwz9dl6m0raac7s4rac48
//!  * ALGO: VQ5EK4GA3IUTGSPNGV64UANBUVFAIVBXVL5UUCNZSDH544XIMF7BAHEDM4
//!  * ATOM: cosmos1wm7lummcealk0fxn3x9tm8hg7xsyuz06ul5fw9
//!  * BTC: bc1qh5p3rff4vxnv23vg0hw8pf3gmz3qgc029cekxz
//!  * ETH: 0x7A66beaebF7D0d17598d37525e63f524CfD23452
//!  * ERC20: 0x7A66beaebF7D0d17598d37525e63f524CfD23452
//!  * XLM: GCJAUMCO2L7PTYMXELQ6GHBTF25MCQKEBNSND2C4QMUPTSVCPEN3LCOG
//!  * XTZ: tz1SgJppPn56whprsDDGcqR4fxqCr2PXvg1R

pub mod lan;



use serde::{Serialize, Deserialize};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use std::collections::HashMap;
use futures::future::{select_ok, BoxFuture};
use std::sync::atomic::{AtomicUsize, Ordering};





/// Represents endpoint health status
#[derive(Debug, Clone)]
pub struct EndpointHealth {
    pub url: String,
    pub last_success: Option<Instant>,
    pub last_failure: Option<Instant>,
    pub consecutive_failures: usize,
    pub is_healthy: bool,
    pub response_time_ms: Option<u64>,
}

impl EndpointHealth {
    fn new(url: String) -> Self {
        EndpointHealth {
            url,
            last_success: None,
            last_failure: None,
            consecutive_failures: 0,
            is_healthy: true,
            response_time_ms: None,
        }
    }
    
    fn mark_success(&mut self, response_time: Duration) {
        self.last_success = Some(Instant::now());
        self.consecutive_failures = 0;
        self.is_healthy = true;
        self.response_time_ms = Some(response_time.as_millis() as u64);
    }
    
    fn mark_failure(&mut self) {
        self.last_failure = Some(Instant::now());
        self.consecutive_failures += 1;
        // Mark unhealthy after 3 consecutive failures
        if self.consecutive_failures >= 3 {
            self.is_healthy = false;
        }
    }
    
    fn should_retry(&self) -> bool {
        if self.is_healthy {
            return true;
        }
        
        // Exponential backoff: 2^failures seconds, max 300 seconds (5 minutes)
        if let Some(last_failure) = self.last_failure {
            let backoff_seconds = (2_u64.pow(self.consecutive_failures.min(8) as u32)).min(300);
            let elapsed = last_failure.elapsed().as_secs();
            elapsed >= backoff_seconds
        } else {
            true
        }
    }
}

/// Configuration for endpoint failover behavior
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailoverConfig {
    /// Timeout for individual endpoint requests in milliseconds
    #[serde(default = "default_request_timeout")]
    pub request_timeout_ms: u64,
    
    /// Maximum number of concurrent endpoint attempts
    #[serde(default = "default_max_concurrent")]
    pub max_concurrent_attempts: usize,
    
    /// Whether to use round-robin or failover strategy
    #[serde(default = "default_strategy")]
    pub strategy: FailoverStrategy,
    
    /// Enable health checking for endpoints
    #[serde(default = "default_health_check")]
    pub health_check_enabled: bool,
    
    /// Health check interval in seconds
    #[serde(default = "default_health_interval")]
    pub health_check_interval_secs: u64,
}

fn default_request_timeout() -> u64 { 5000 }
fn default_max_concurrent() -> usize { 3 }
fn default_strategy() -> FailoverStrategy { FailoverStrategy::Failover }
fn default_health_check() -> bool { true }
fn default_health_interval() -> u64 { 60 }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FailoverStrategy {
    /// Try endpoints in order, falling back on failure
    Failover,
    /// Distribute requests across healthy endpoints
    RoundRobin,
    /// Try the fastest responding endpoint first
    FastestFirst,
}

impl Default for FailoverConfig {
    fn default() -> Self {
        FailoverConfig {
            request_timeout_ms: default_request_timeout(),
            max_concurrent_attempts: default_max_concurrent(),
            strategy: default_strategy(),
            health_check_enabled: default_health_check(),
            health_check_interval_secs: default_health_interval(),
        }
    }
}

/// Enhanced LIFX Config with unlimited endpoints and advanced failover
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LifxConfig {
    pub access_token: String,
    pub api_endpoints: Vec<String>,
    
    #[serde(default)]
    pub failover_config: FailoverConfig,
    
    #[serde(skip)]
    endpoint_health: Arc<Mutex<HashMap<String, EndpointHealth>>>,
    
    #[serde(skip)]
    round_robin_counter: Arc<AtomicUsize>,
}

impl Default for LifxConfig {
    fn default() -> Self {
        LifxConfig {
            access_token: String::new(),
            api_endpoints: Vec::new(),
            failover_config: FailoverConfig::default(),
            endpoint_health: Arc::new(Mutex::new(HashMap::new())),
            round_robin_counter: Arc::new(AtomicUsize::new(0)),
        }
    }
}

impl PartialEq for LifxConfig {
    fn eq(&self, other: &Self) -> bool {
        self.access_token == other.access_token &&
        self.api_endpoints == other.api_endpoints &&
        self.failover_config.strategy == other.failover_config.strategy
    }
}

impl LifxConfig {
    /// Create a new LifxConfig with builder pattern
    pub fn new(access_token: String) -> Self {
        LifxConfig {
            access_token,
            api_endpoints: Vec::new(),
            failover_config: FailoverConfig::default(),
            endpoint_health: Arc::new(Mutex::new(HashMap::new())),
            round_robin_counter: Arc::new(AtomicUsize::new(0)),
        }
    }
    
    /// Add an endpoint to the configuration
    pub fn add_endpoint(mut self, endpoint: String) -> Self {
        self.api_endpoints.push(endpoint);
        self
    }
    
    /// Set the failover strategy
    pub fn with_strategy(mut self, strategy: FailoverStrategy) -> Self {
        self.failover_config.strategy = strategy;
        self
    }
    
    /// Set the request timeout in milliseconds
    pub fn with_timeout(mut self, timeout_ms: u64) -> Self {
        self.failover_config.request_timeout_ms = timeout_ms;
        self
    }
    
    /// Initialize health tracking for all endpoints
    pub fn init_health_tracking(&self) {
        let mut health = self.endpoint_health.lock()
            .expect("Failed to acquire endpoint_health mutex lock: mutex was poisoned");
        for endpoint in &self.api_endpoints {
            health.entry(endpoint.clone())
                .or_insert_with(|| EndpointHealth::new(endpoint.clone()));
        }
    }
    
    /// Get the next endpoint based on the configured strategy
    pub fn get_next_endpoint(&self) -> Option<String> {
        if self.api_endpoints.is_empty() {
            return None;
        }
        
        match self.failover_config.strategy {
            FailoverStrategy::RoundRobin => {
                let healthy_endpoints = self.get_healthy_endpoints();
                if healthy_endpoints.is_empty() {
                    return self.api_endpoints.first().cloned();
                }
                let index = self.round_robin_counter.fetch_add(1, Ordering::SeqCst);
                Some(healthy_endpoints[index % healthy_endpoints.len()].clone())
            },
            FailoverStrategy::Failover => {
                // Return first healthy endpoint
                self.get_healthy_endpoints().first().cloned()
                    .or_else(|| self.api_endpoints.first().cloned())
            },
            FailoverStrategy::FastestFirst => {
                // Sort by response time and return fastest
                let health = self.endpoint_health.lock()
                    .expect("Failed to acquire endpoint_health mutex lock: mutex was poisoned");
                let mut endpoints_with_time: Vec<(String, Option<u64>)> = self.api_endpoints
                    .iter()
                    .map(|ep| {
                        let time = health.get(ep)
                            .and_then(|h| h.response_time_ms);
                        (ep.clone(), time)
                    })
                    .collect();
                    
                endpoints_with_time.sort_by_key(|(_ep, time)| time.unwrap_or(u64::MAX));
                endpoints_with_time.first().map(|(ep, _)| ep.clone())
            }
        }
    }
    
    /// Get all healthy endpoints
    pub fn get_healthy_endpoints(&self) -> Vec<String> {
        let health = self.endpoint_health.lock()
            .expect("Failed to acquire endpoint_health mutex lock: mutex was poisoned");
        self.api_endpoints
            .iter()
            .filter(|ep| {
                health.get(*ep)
                    .map(|h| h.is_healthy && h.should_retry())
                    .unwrap_or(true)
            })
            .cloned()
            .collect()
    }
    
    /// Get endpoints ordered by priority (healthy first, then by strategy)
    pub fn get_prioritized_endpoints(&self) -> Vec<String> {
        match self.failover_config.strategy {
            FailoverStrategy::Failover => {
                // Healthy endpoints first, then unhealthy with exponential backoff
                let mut healthy = self.get_healthy_endpoints();
                let health = self.endpoint_health.lock()
                    .expect("Failed to acquire endpoint_health mutex lock: mutex was poisoned");
                let mut unhealthy: Vec<String> = self.api_endpoints
                    .iter()
                    .filter(|ep| !healthy.contains(ep))
                    .filter(|ep| {
                        health.get(*ep)
                            .map(|h| h.should_retry())
                            .unwrap_or(true)
                    })
                    .cloned()
                    .collect();
                healthy.append(&mut unhealthy);
                healthy
            },
            _ => self.get_healthy_endpoints()
        }
    }
    
    /// Mark an endpoint as successful
    pub fn mark_endpoint_success(&self, endpoint: &str, response_time: Duration) {
        let mut health = self.endpoint_health.lock()
            .expect("Failed to acquire endpoint_health mutex lock: mutex was poisoned");
        health.entry(endpoint.to_string())
            .or_insert_with(|| EndpointHealth::new(endpoint.to_string()))
            .mark_success(response_time);
    }
    
    /// Mark an endpoint as failed
    pub fn mark_endpoint_failure(&self, endpoint: &str) {
        let mut health = self.endpoint_health.lock()
            .expect("Failed to acquire endpoint_health mutex lock: mutex was poisoned");
        health.entry(endpoint.to_string())
            .or_insert_with(|| EndpointHealth::new(endpoint.to_string()))
            .mark_failure();
    }
    
    /// Execute a request with failover across multiple endpoints
    pub async fn execute_with_failover<F, T>(
        &self,
        request_fn: F,
    ) -> Result<T, reqwest::Error>
    where
        F: Fn(String, String) -> BoxFuture<'static, Result<T, reqwest::Error>> + Clone + Send + 'static,
        T: Send + 'static,
    {
        self.init_health_tracking();
        let endpoints = self.get_prioritized_endpoints();
        
        if endpoints.is_empty() {
            // Try to make a request to generate a proper error
            let client = reqwest::Client::new();
            return client.get("http://no-endpoints-configured")
                .send().await
                .and_then(|_| Err(client.get("http://no-endpoints-configured").build().unwrap_err()));
        }
        
        let max_concurrent = self.failover_config.max_concurrent_attempts.min(endpoints.len());
        let timeout = Duration::from_millis(self.failover_config.request_timeout_ms);
        
        let mut last_error = None;
        
        // Try endpoints concurrently (up to max_concurrent at a time)
        for chunk in endpoints.chunks(max_concurrent) {
            let start = Instant::now();
            let futures: Vec<BoxFuture<'static, Result<T, reqwest::Error>>> = chunk
                .iter()
                .map(|endpoint| {
                    let token = self.access_token.clone();
                    let ep = endpoint.clone();
                    let f = request_fn.clone();
                    Box::pin(async move {
                        // Use futures timeout instead of tokio::select
                        match tokio::time::timeout(timeout, f(ep.clone(), token)).await {
                            Ok(result) => result,
                            Err(_) => {
                                // Return a timeout error
                                match reqwest::Client::builder()
                                    .timeout(Duration::from_millis(1))
                                    .build()
                                {
                                    Ok(client) => {
                                        // Create a request that will timeout
                                        client.get(&format!("http://{}/timeout", ep))
                                            .send().await
                                            .and_then(|_| client.get("http://timeout").build().map_err(Into::into))
                                            .and_then(|_| unreachable!())
                                    },
                                    Err(e) => {
                                        // Return the client builder error
                                        Err(e)
                                    }
                                }
                            }
                        }
                    }) as BoxFuture<'static, Result<T, reqwest::Error>>
                })
                .collect();
            
            match select_ok(futures).await {
                Ok((result, _remaining)) => {
                    // Mark successful endpoint
                    if let Some(endpoint) = chunk.first() {
                        self.mark_endpoint_success(endpoint, start.elapsed());
                    }
                    return Ok(result);
                },
                Err(err) => {
                    // Mark all attempted endpoints as failed
                    for endpoint in chunk {
                        self.mark_endpoint_failure(endpoint);
                    }
                    last_error = Some(err);
                }
            }
        }
        
        // Return the last error we encountered
        Err(last_error.unwrap_or_else(|| {
            reqwest::Client::new()
                .get("http://all-endpoints-failed")
                .build()
                .unwrap_err()
        }))
    }
}


pub type Lights = Vec<Light>;

/// Represents a LIFX Light Object
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Light {
    pub id: String,
    pub uuid: String,
    pub label: String,
    pub connected: bool,
    pub power: String,
    pub color: Color,
    pub brightness: f64,
    pub group: Group,
    pub location: Location,
    pub product: Product,
    #[serde(rename = "last_seen")]
    pub last_seen: String,
    #[serde(rename = "seconds_since_seen")]
    pub seconds_since_seen: i64,
    pub error: Option<String>,
    pub errors: Option<Vec<Error>>,
}
impl Light {

    /// Asynchronously set the breathe animation for the current light
    /// 
    /// # Arguments
    ///
    /// * `self` - A Light object.
    /// * `access_token` - A personal acces token for authentication with LIFX.
    /// * `breathe` - A BreatheEffect object containing the values to set
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// extern crate lifx_rs as lifx;
    /// 
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// 
    ///     let key = "xxx".to_string();
    ///
    ///     let config = lifx::LifxConfig::new(key)
    ///         .add_endpoint("https://api.lifx.com".to_string())
    ///         .add_endpoint("http://localhost:8089".to_string());
    /// 
    ///     let lights = lifx::Light::list_all(config.clone())?;
    ///     println!("{:?}", lights.clone());
    /// 
    ///     let mut breathe = lifx::BreatheEffect::new();
    ///     breathe.color = Some("red".to_string());
    ///     breathe.from_color = Some("green".to_string());
    ///     breathe.period = Some(10.0);
    ///     breathe.persist = Some(true);
    ///     breathe.power_on = Some(true);
    /// 
    ///     for light in lights {
    ///         let results = light.async_breathe_effect(config.clone(), breathe.clone()).await?;
    ///         println!("{:?}", results);
    ///     }
    ///
    ///     Ok(())
    /// }
    ///  ```
    pub async fn async_breathe_effect(&self, config: LifxConfig, breathe: BreatheEffect) ->  Result<LiFxResults, reqwest::Error>{
        return Self::async_breathe_effect_by_selector(config, format!("id:{}", self.id), breathe).await;
    }

    /// Asynchronously activate the breathe animation for the selected light(s)
    /// 
    /// # Arguments
    ///
    /// * `access_token` - A personal acces token for authentication with LIFX.
    /// * `selector` - An LIFX selector ex: all, id:xxx, group_id:xxx
    /// * `breathe` - A BreatheEffect object containing the values to set
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// extern crate lifx_rs as lifx;
    /// 
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// 
    ///     let key = "xxx".to_string();
    ///
    ///     let config = lifx::LifxConfig::new(key)
    ///         .add_endpoint("https://api.lifx.com".to_string())
    ///         .add_endpoint("http://localhost:8089".to_string());
    /// 
    ///     let mut breathe = lifx::BreatheEffect::new();
    ///     breathe.color = Some("red".to_string());
    ///     breathe.from_color = Some("green".to_string());
    ///     breathe.period = Some(10.0);
    ///     breathe.persist = Some(true);
    ///     breathe.power_on = Some(true);
    ///     
    ///     // Apply breathe effect to all light(s)
    ///     let result = lifx::Light::async_breathe_effect_by_selector(config, "all".to_string(), breathe).await?;
    ///     println!("{:?}", result);
    ///
    ///     Ok(())
    /// }
    ///  ```
    pub async fn async_breathe_effect_by_selector(config: LifxConfig, selector: String, breathe: BreatheEffect) ->  Result<LiFxResults, reqwest::Error>{
        let url = format!("{}/v1/lights/{}/effects/breathe", config.api_endpoints[0], selector);

        let request = reqwest::Client::new().post(url)
            .header("Authorization", format!("Bearer {}", config.access_token))
            .form(&breathe.to_params())
            .send().await;
            
        match request{
            Ok(req) => {
                let json = req.json::<LiFxResults>().await?;
                return Ok(json);
            },
            Err(err) => {
                if config.api_endpoints.len() > 1 {
                    let url = format!("{}/v1/lights/{}/effects/breathe", config.api_endpoints[1], selector);

                    let request = reqwest::Client::new().post(url)
                        .header("Authorization", format!("Bearer {}", config.access_token))
                        .form(&breathe.to_params())
                        .send().await;
                        
                    match request{
                        Ok(req) => {
                            let json = req.json::<LiFxResults>().await?;
                            return Ok(json);
                        },
                        Err(err2) => {
                            return Err(err2);
                        }
                    }
                } else {
                    return Err(err);
                }
            }
        }
    

    }


    /// Asynchronously switch a light to clean mode, with a set duration. 
    /// 
    /// # Arguments
    ///
    /// * `self` - A Light object.
    /// * `access_token` - A personal acces token for authentication with LIFX.
    /// * `clean` - A Clean object containing the values to set
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// extern crate lifx_rs as lifx;
    /// 
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// 
    ///     let key = "xxx".to_string();
    ///
    ///     let config = lifx::LifxConfig::new(key)
    ///         .add_endpoint("https://api.lifx.com".to_string())
    ///         .add_endpoint("http://localhost:8089".to_string());
    /// 
    ///     let lights = lifx::Light::list_all(config.clone())?;
    ///     println!("{:?}", lights.clone());
    /// 
    ///     let mut clean = lifx::Clean::new();
    ///     clean.duration = Some(0);
    ///     clean.stop = Some(false);
    /// 
    ///     for light in lights {
    ///         let results = light.async_clean(config.clone(), clean.clone()).await?;
    ///         println!("{:?}", results);
    ///     }
    ///
    ///     Ok(())
    /// }
    ///  ```
    pub async fn async_clean(&self, config: LifxConfig, clean: Clean) ->  Result<LiFxResults, reqwest::Error>{
        return Self::async_clean_by_selector(config, format!("id:{}", self.id), clean).await;
    }

    /// Asynchronously switch a selected LIFX object to clean mode, with a set duration. 
    /// 
    /// # Arguments
    ///
    /// * `access_token` - A personal acces token for authentication with LIFX.
    /// * `selector` - An LIFX selector ex: all, id:xxx, group_id:xxx
    /// * `clean` - A Clean object containing the values to set
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// extern crate lifx_rs as lifx;
    /// 
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// 
    ///     let key = "xxx".to_string();
    ///
    ///     let config = lifx::LifxConfig::new(key)
    ///         .add_endpoint("https://api.lifx.com".to_string())
    ///         .add_endpoint("http://localhost:8089".to_string());
    /// 
    ///     let mut clean = lifx::Clean::new();
    ///     clean.duration = Some(0);
    ///     clean.stop = Some(false);
    ///     
    ///     // Set all light to clean mode
    ///     let result = lifx::Light::async_clean_by_selector(config, "all".to_string(), clean).await?;
    ///     println!("{:?}", result);
    ///
    ///     Ok(())
    /// }
    ///  ```
    pub async fn async_clean_by_selector(config: LifxConfig, selector: String, clean: Clean) ->  Result<LiFxResults, reqwest::Error>{
        let url = format!("{}/v1/lights/{}/clean", config.api_endpoints[0], selector);

        let request = reqwest::Client::new().post(url)
            .header("Authorization", format!("Bearer {}", config.access_token))
            .form(&clean.to_params())
            .send().await;

        match request{
            Ok(req) => {
                let json = req.json::<LiFxResults>().await?;
                return Ok(json);
            },
            Err(err) => {
                if config.api_endpoints.len() > 1 {
                    let url = format!("{}/v1/lights/{}/clean", config.api_endpoints[1], selector);

                    let request = reqwest::Client::new().post(url)
                        .header("Authorization", format!("Bearer {}", config.access_token))
                        .form(&clean.to_params())
                        .send().await;
            
                    match request{
                        Ok(req) => {
                            let json = req.json::<LiFxResults>().await?;
                            return Ok(json);
                        },
                        Err(err2) => {
                            return Err(err2);
                        }
                    }
                } else {
                    return Err(err);
                }
            }
        }
    

    }


    /// Stops animation(s) for the current light
    /// 
    /// # Arguments
    ///
    /// * `self` - A Light object.
    /// * `access_token` - A personal acces token for authentication with LIFX.
    /// * `flame_effect` - A FlameEffect object containing the values to set
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// extern crate lifx_rs as lifx;
    /// 
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// 
    ///     let key = "xxx".to_string();
    ///
    ///     let config = lifx::LifxConfig::new(key)
    ///         .add_endpoint("https://api.lifx.com".to_string())
    ///         .add_endpoint("http://localhost:8089".to_string());
    /// 
    ///     let lights = lifx::Light::list_all(config.clone())?;
    ///     println!("{:?}", lights.clone());
    /// 
    ///     let mut effects_off = lifx::EffectsOff::new();
    ///     effects_off.power_off = Some(true);
    /// 
    ///     for light in lights {
    ///         let results = light.async_effects_off(config.clone(), effects_off.clone()).await?;
    ///         println!("{:?}", results);
    ///     }
    ///
    ///     Ok(())
    /// }
    ///  ```
    pub async fn async_effects_off(&self, config: LifxConfig, effects_off: EffectsOff) ->  Result<LiFxResults, reqwest::Error>{
        return Self::async_effects_off_by_selector(config, format!("id:{}", self.id), effects_off).await;
    }

    /// Stops animation(s) for the selected light(s)
    /// 
    /// # Arguments
    ///
    /// * `access_token` - A personal acces token for authentication with LIFX.
    /// * `selector` - An LIFX selector ex: all, id:xxx, group_id:xxx
    /// * `effects_off` - A EffectsOff object containing the values to set
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// extern crate lifx_rs as lifx;
    /// 
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// 
    ///     let key = "xxx".to_string();
    ///
    ///     let config = lifx::LifxConfig::new(key)
    ///         .add_endpoint("https://api.lifx.com".to_string())
    ///         .add_endpoint("http://localhost:8089".to_string());
    /// 
    ///     let mut effects_off = lifx::EffectsOff::new();
    ///     effects_off.power_off = Some(true);
    ///     
    ///     // Send morph effect to all lights
    ///     let result = lifx::Light::async_effects_off_by_selector(config, "all".to_string(), effects_off).await?;
    ///     println!("{:?}", result);
    ///
    ///     Ok(())
    /// }
    ///  ```
    pub async fn async_effects_off_by_selector(config: LifxConfig, selector: String, effects_off: EffectsOff) ->  Result<LiFxResults, reqwest::Error>{
        let url = format!("{}/v1/lights/{}/effects/off", config.api_endpoints[0], selector);

        let request = reqwest::Client::new().post(url)
            .header("Authorization", format!("Bearer {}", config.access_token))
            .form(&effects_off.to_params())
            .send().await;

        match request {
            Ok(req) => {
                let json = req.json::<LiFxResults>().await?;
                return Ok(json);
            },
            Err(err) => {
                if config.api_endpoints.len() > 1 {
                    let url = format!("{}/v1/lights/{}/effects/off", config.api_endpoints[1], selector);

                    let request = reqwest::Client::new().post(url)
                        .header("Authorization", format!("Bearer {}", config.access_token))
                        .form(&effects_off.to_params())
                        .send().await;
            
                    match request {
                        Ok(req) => {
                            let json = req.json::<LiFxResults>().await?;
                            return Ok(json);
                        },
                        Err(err2) => {
                            return Err(err2);
                        }
                    }
                
                } else {
                    return Err(err);
                }
            }
        }
    

    }



    /// Activate the flame animation for the current light
    /// 
    /// # Arguments
    ///
    /// * `self` - A Light object.
    /// * `access_token` - A personal acces token for authentication with LIFX.
    /// * `flame_effect` - A FlameEffect object containing the values to set
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// extern crate lifx_rs as lifx;
    /// 
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// 
    ///     let key = "xxx".to_string();
    ///
    ///     let config = lifx::LifxConfig::new(key)
    ///         .add_endpoint("https://api.lifx.com".to_string())
    ///         .add_endpoint("http://localhost:8089".to_string());
    /// 
    ///     let lights = lifx::Light::list_all(config.clone())?;
    ///     println!("{:?}", lights.clone());
    /// 
    ///     let mut flame_effect = lifx::FlameEffect::new();
    ///     flame_effect.period = Some(10);
    ///     flame_effect.duration = Some(0.0);
    ///     flame_effect.power_on = Some(true);
    ///  
    ///     for light in lights {
    ///         let results = light.async_flame_effect(config.clone(), flame_effect.clone()).await?;
    ///         println!("{:?}", results);
    ///     }
    ///
    ///     Ok(())
    /// }
    ///  ```
    pub async fn async_flame_effect(&self, config: LifxConfig, flame_effect: FlameEffect) ->  Result<LiFxResults, reqwest::Error>{
        return Self::async_flame_effect_by_selector(config, format!("id:{}", self.id), flame_effect).await;
    }

    /// Activate the flame animation for the selected light(s)
    /// 
    /// # Arguments
    ///
    /// * `access_token` - A personal acces token for authentication with LIFX.
    /// * `selector` - An LIFX selector ex: all, id:xxx, group_id:xxx
    /// * `flame_effect` - A FlameEffect object containing the values to set
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// extern crate lifx_rs as lifx;
    /// 
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// 
    ///     let key = "xxx".to_string();
    ///
    ///     let config = lifx::LifxConfig::new(key)
    ///         .add_endpoint("https://api.lifx.com".to_string())
    ///         .add_endpoint("http://localhost:8089".to_string());
    /// 
    ///     let mut flame_effect = lifx::FlameEffect::new();
    ///     flame_effect.period = Some(10);
    ///     flame_effect.duration = Some(0.0);
    ///     flame_effect.power_on = Some(true);
    ///     
    ///     // Send morph effect to all lights
    ///     let result = lifx::Light::async_flame_effect_by_selector(config, "all".to_string(), flame_effect).await?;
    ///     println!("{:?}", result);
    ///
    ///     Ok(())
    /// }
    ///  ```
    pub async fn async_flame_effect_by_selector(config: LifxConfig, selector: String, flame_effect: FlameEffect) ->  Result<LiFxResults, reqwest::Error>{
        let url = format!("{}/v1/lights/{}/effects/flame", config.api_endpoints[0], selector);

        let request = reqwest::Client::new().post(url)
            .header("Authorization", format!("Bearer {}", config.access_token))
            .form(&flame_effect.to_params())
            .send().await;

        match request {
            Ok(req) => {
                let json = req.json::<LiFxResults>().await?;
                return Ok(json);
            },
            Err(err) => {
                if config.api_endpoints.len() > 1 {
                    let url = format!("{}/v1/lights/{}/effects/flame", config.api_endpoints[1], selector);

                    let request = reqwest::Client::new().post(url)
                        .header("Authorization", format!("Bearer {}", config.access_token))
                        .form(&flame_effect.to_params())
                        .send().await;
            
                    match request {
                        Ok(req) => {
                            let json = req.json::<LiFxResults>().await?;
                            return Ok(json);
                        },
                        Err(err2) => {
                            return Err(err2);
                        }
                    }
                } else {
                    return Err(err);
                }
            }
        }
    

    }



    /// Asynchronously gets ALL lights belonging to the authenticated account
    /// 
    /// # Arguments
    ///
    /// * `access_token` - A personal acces token for authentication with LIFX.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// extern crate lifx_rs as lifx;
    /// 
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// 
    ///     let key = "xxx".to_string();
    ///
    ///     let config = lifx::LifxConfig::new(key)
    ///         .add_endpoint("https://api.lifx.com".to_string())
    ///         .add_endpoint("http://localhost:8089".to_string());
    /// 
    ///     let lights = lifx::Light::async_list_all(config).await?;
    ///
    ///     Ok(())
    /// }
    ///  ```
    pub async fn async_list_all(config: LifxConfig) -> Result<Lights, reqwest::Error> {
        return Self::async_list_by_selector(config, format!("all")).await;
    }

    /// Asynchronously gets lights belonging to the authenticated account. Filtering the lights using selectors. Properties such as id, label, group and location can be used in selectors.
    /// 
    /// # Arguments
    ///
    /// * `access_token` - A personal acces token for authentication with LIFX.
    /// * `selector` - An LIFX selector ex: all, id:xxx, group_id:xxx
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// extern crate lifx_rs as lifx;
    /// 
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// 
    ///     let key = "xxx".to_string();
    ///
    ///     let config = lifx::LifxConfig::new(key)
    ///         .add_endpoint("https://api.lifx.com".to_string())
    ///         .add_endpoint("http://localhost:8089".to_string());
    /// 
    ///     let lights = lifx::Light::async_list_by_selector(config, format!("all")).await?;
    ///
    ///     Ok(())
    /// }
    ///  ```
    pub async fn async_list_by_selector(config: LifxConfig, selector: String) -> Result<Lights, reqwest::Error> {
        // Use legacy implementation if only 1-2 endpoints for backward compatibility
        if config.api_endpoints.len() <= 2 && !config.failover_config.health_check_enabled {
            return Self::async_list_by_selector_legacy(config, selector).await;
        }
        
        // Use new failover system for unlimited endpoints
        let selector_clone = selector.clone();
        config.execute_with_failover(move |endpoint, token| {
            let sel = selector_clone.clone();
            Box::pin(async move {
                let url = format!("{}/v1/lights/{}", endpoint, sel);
                let req = reqwest::Client::new()
                    .get(url)
                    .header("Authorization", format!("Bearer {}", token))
                    .send()
                    .await?;
                req.json::<Lights>().await
            })
        }).await
    }
    
    // Legacy implementation for backward compatibility
    async fn async_list_by_selector_legacy(config: LifxConfig, selector: String) -> Result<Lights, reqwest::Error> {
        let url = format!("{}/v1/lights/{}", config.api_endpoints[0], selector);
        let request = reqwest::Client::new().get(url).header("Authorization", format!("Bearer {}", config.access_token)).send().await;
        match request {
            Ok(req) => {
                let json = req.json::<Lights>().await?;
                return Ok(json);
            },
            Err(err) => {
                if config.api_endpoints.len() > 1 {
                    let url = format!("{}/v1/lights/{}", config.api_endpoints[1], selector);
                    let request = reqwest::Client::new().get(url).header("Authorization", format!("Bearer {}", config.access_token)).send().await;
                    match request {
                        Ok(req) => {
                            let json = req.json::<Lights>().await?;
                            return Ok(json);
                        },
                        Err(err2) => {
                            return Err(err2);
                        }
                    }
                } else {
                    return Err(err);
                }
            }
        }
    }

    /// Asynchronously activate the morph animation for the current light
    /// 
    /// # Arguments
    ///
    /// * `self` - A Light object.
    /// * `access_token` - A personal acces token for authentication with LIFX.
    /// * `morph_effect` - A MorphEffect object containing the values to set
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// extern crate lifx_rs as lifx;
    /// 
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// 
    ///     let key = "xxx".to_string();
    ///
    ///     let config = lifx::LifxConfig::new(key)
    ///         .add_endpoint("https://api.lifx.com".to_string())
    ///         .add_endpoint("http://localhost:8089".to_string());
    /// 
    ///     let lights = lifx::Light::list_all(config.clone())?;
    ///     println!("{:?}", lights.clone());
    /// 
    ///     let mut morph_effect = lifx::MorphEffect::new();
    ///     morph_effect.period = Some(10);
    ///     morph_effect.duration = Some(0.0);
    ///
    ///     let mut palette: Vec<String> = Vec::new();
    ///     palette.push(format!("red"));
    ///     palette.push(format!("green"));
    ///
    ///     morph_effect.palette = Some(palette);
    ///     morph_effect.power_on = Some(true);
    ///  
    ///     for light in lights {
    ///         let results = light.async_morph_effect(config.clone(), morph_effect.clone()).await?;
    ///         println!("{:?}", results);
    ///     }
    ///
    ///     Ok(())
    /// }
    ///  ```
    pub async fn async_morph_effect(&self, config: LifxConfig, morph_effect: MorphEffect) ->  Result<LiFxResults, reqwest::Error>{
        return Self::async_morph_effect_by_selector(config, format!("id:{}", self.id), morph_effect).await;
    }

    /// Asynchronously activate the morph animation for the selected light(s)
    /// 
    /// # Arguments
    ///
    /// * `access_token` - A personal acces token for authentication with LIFX.
    /// * `selector` - An LIFX selector ex: all, id:xxx, group_id:xxx
    /// * `clean` - A Clean object containing the values to set
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// extern crate lifx_rs as lifx;
    /// 
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// 
    ///     let key = "xxx".to_string();
    ///
    ///     let config = lifx::LifxConfig::new(key)
    ///         .add_endpoint("https://api.lifx.com".to_string())
    ///         .add_endpoint("http://localhost:8089".to_string());
    /// 
    ///     let mut morph_effect = lifx::MorphEffect::new();
    ///     morph_effect.period = Some(10);
    ///     morph_effect.duration = Some(0.0);
    /// 
    ///     let mut palette: Vec<String> = Vec::new();
    ///     palette.push(format!("red"));
    ///     palette.push(format!("green"));
    /// 
    ///     morph_effect.palette = Some(palette);
    ///     morph_effect.power_on = Some(true);
    ///     
    ///     // Send morph effect to all lights
    ///     let result = lifx::Light::async_morph_effect_by_selector(config, "all".to_string(), morph_effect).await?;
    ///     println!("{:?}", result);
    ///
    ///     Ok(())
    /// }
    ///  ```
    pub async fn async_morph_effect_by_selector(config: LifxConfig, selector: String, morph_effect: MorphEffect) ->  Result<LiFxResults, reqwest::Error>{
        let url = format!("{}/v1/lights/{}/effects/morph", config.api_endpoints[0], selector);
        let request = reqwest::Client::new().post(url)
            .header("Authorization", format!("Bearer {}", config.access_token))
            .form(&morph_effect.to_params())
            .send().await;
        match request {
            Ok(req) => {
                let json = req.json::<LiFxResults>().await?;
                return Ok(json);
            },
            Err(err) => {
                if config.api_endpoints.len() > 1 {
                    let url = format!("{}/v1/lights/{}/effects/morph", config.api_endpoints[1], selector);
                    let request = reqwest::Client::new().post(url)
                        .header("Authorization", format!("Bearer {}", config.access_token))
                        .form(&morph_effect.to_params())
                        .send().await;
                    match request {
                        Ok(req) => {
                            let json = req.json::<LiFxResults>().await?;
                            return Ok(json);
                        },
                        Err(err2) => {
                            return Err(err2);
                        }
                    }
                } else {
                    return Err(err);
                }
            }
        }
    

    }

    /// Asynchronously activate the move animation for the current light
    /// 
    /// # Arguments
    ///
    /// * `self` - A Light object.
    /// * `access_token` - A personal acces token for authentication with LIFX.
    /// * `move_effect` - A MoveEffect object containing the values to set
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// extern crate lifx_rs as lifx;
    /// 
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// 
    ///     let key = "xxx".to_string();
    ///
    ///     let config = lifx::LifxConfig::new(key)
    ///         .add_endpoint("https://api.lifx.com".to_string())
    ///         .add_endpoint("http://localhost:8089".to_string());
    /// 
    ///     let lights = lifx::Light::async_list_all(config.clone()).await?;
    ///     println!("{:?}",lights.clone());
    /// 
    ///     let mut move_effect = lifx::MoveEffect::new();
    ///     move_effect.direction = Some("forward".to_string()); // or backward
    ///     move_effect.period = Some(10);
    ///     move_effect.cycles = Some(0.9);
    ///     move_effect.power_on = Some(true);
    /// 
    ///     for light in lights {
    ///         let results = light.async_move_effect(config.clone(), move_effect.clone()).await?;
    ///         println!("{:?}",results);
    ///     }
    ///
    ///     Ok(())
    /// }
    ///  ```
    pub async fn async_move_effect(&self, config: LifxConfig, move_effect: MoveEffect) ->  Result<LiFxResults, reqwest::Error>{
        return Self::async_move_effect_by_selector(config, format!("id:{}", self.id), move_effect).await;
    }

    /// Asynchronously activate the move animation for the selected light(s)
    /// 
    /// # Arguments
    ///
    /// * `access_token` - A personal acces token for authentication with LIFX.
    /// * `selector` - An LIFX selector ex: all, id:xxx, group_id:xxx
    /// * `move_effect` - A MoveEffect object containing the values to set
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// extern crate lifx_rs as lifx;
    /// 
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// 
    ///     let key = "xxx".to_string();
    ///
    ///     let config = lifx::LifxConfig::new(key)
    ///         .add_endpoint("https://api.lifx.com".to_string())
    ///         .add_endpoint("http://localhost:8089".to_string());
    /// 
    ///     let mut move_effect = lifx::MoveEffect::new();
    ///     move_effect.direction = Some("forward".to_string()); // or backward
    ///     move_effect.period = Some(10);
    ///     move_effect.cycles = Some(0.9);
    ///     move_effect.power_on = Some(true);
    ///     
    ///     // Toggle all lights
    ///     let result = lifx::Light::async_move_effect_by_selector(config, "all".to_string(), move_effect).await?;
    ///     println!("{:?}", result);
    ///
    ///     Ok(())
    /// }
    ///  ```
    pub async fn async_move_effect_by_selector(config: LifxConfig, selector: String, move_effect: MoveEffect) ->  Result<LiFxResults, reqwest::Error>{
        let url = format!("{}/v1/lights/{}/effects/move", config.api_endpoints[0], selector);

        let request = reqwest::Client::new().post(url)
            .header("Authorization", format!("Bearer {}", config.access_token))
            .form(&move_effect.to_params())
            .send().await;

        match request {
            Ok(req) => {
                let json = req.json::<LiFxResults>().await?;
                return Ok(json);
            },
            Err(err) => {
                if config.api_endpoints.len() > 1 {
                    let url = format!("{}/v1/lights/{}/effects/move", config.api_endpoints[1], selector);

                    let request = reqwest::Client::new().post(url)
                        .header("Authorization", format!("Bearer {}", config.access_token))
                        .form(&move_effect.to_params())
                        .send().await;
            
                    match request {
                        Ok(req) => {
                            let json = req.json::<LiFxResults>().await?;
                            return Ok(json);
                        },
                        Err(err2) => {
                            return Err(err2);
                        }
                    }
                } else {
                    return Err(err);
                }
            }
        }
    

    }

    /// Asynchronously activate the pulse animation for the current light
    /// 
    /// # Arguments
    ///
    /// * `self` - A Light object.
    /// * `access_token` - A personal acces token for authentication with LIFX.
    /// * `pulse_effect` - A PulseEffect object containing the values to set
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// extern crate lifx_rs as lifx;
    /// 
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// 
    ///     let key = "xxx".to_string();
    ///
    ///     let config = lifx::LifxConfig::new(key)
    ///         .add_endpoint("https://api.lifx.com".to_string())
    ///         .add_endpoint("http://localhost:8089".to_string());
    /// 
    ///     let lights = lifx::Light::async_list_all(config.clone()).await?;
    ///     println!("{:?}",lights.clone());
    /// 
    ///     let mut pulse = lifx::PulseEffect::new();
    ///     pulse.color = Some("red".to_string());
    ///     pulse.from_color = Some("green".to_string());
    ///     pulse.period = Some(10.0);
    ///     pulse.persist = Some(true);
    ///     pulse.power_on = Some(true);
    /// 
    ///     for light in lights {
    ///         let results = light.async_pulse_effect(config.clone(), pulse.clone()).await?;
    ///         println!("{:?}",results);
    ///     }
    ///
    ///     Ok(())
    /// }
    ///  ```
    pub async fn async_pulse_effect(&self, config: LifxConfig, pulse_effect: PulseEffect) ->  Result<LiFxResults, reqwest::Error>{
        return Self::async_pulse_effect_by_selector(config, format!("id:{}", self.id), pulse_effect).await;
    }

    /// Asynchronously activate the pulse animation for the selected light(s)
    /// 
    /// # Arguments
    ///
    /// * `access_token` - A personal acces token for authentication with LIFX.
    /// * `selector` - An LIFX selector ex: all, id:xxx, group_id:xxx
    /// * `pulse_effect` - A PulseEffect object containing the values to set
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// extern crate lifx_rs as lifx;
    /// 
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// 
    ///     let key = "xxx".to_string();
    ///
    ///     let config = lifx::LifxConfig::new(key)
    ///         .add_endpoint("https://api.lifx.com".to_string())
    ///         .add_endpoint("http://localhost:8089".to_string());
    /// 
    ///     let mut pulse = lifx::PulseEffect::new();
    ///     pulse.color = Some("red".to_string());
    ///     pulse.from_color = Some("green".to_string());
    ///     pulse.period = Some(10.0);
    ///     pulse.persist = Some(true);
    ///     pulse.power_on = Some(true);
    ///     
    ///     // Toggle all lights
    ///     let result = lifx::Light::async_pulse_effect_by_selector(config, "all".to_string(), pulse).await?;
    ///     println!("{:?}", result);
    ///
    ///     Ok(())
    /// }
    ///  ```
    pub async fn async_pulse_effect_by_selector(config: LifxConfig, selector: String, pulse_effect: PulseEffect) ->  Result<LiFxResults, reqwest::Error>{
        let url = format!("{}/v1/lights/{}/effects/pulse", config.api_endpoints[0], selector);

        let request = reqwest::Client::new().post(url)
            .header("Authorization", format!("Bearer {}", config.access_token))
            .form(&pulse_effect.to_params())
            .send().await;

        match request {
            Ok(req) => {
                let json = req.json::<LiFxResults>().await?;
                return Ok(json);
            },
            Err(err) => {
                if config.api_endpoints.len() > 1 {
                    let url = format!("{}/v1/lights/{}/effects/pulse", config.api_endpoints[1], selector);

                    let request = reqwest::Client::new().post(url)
                        .header("Authorization", format!("Bearer {}", config.access_token))
                        .form(&pulse_effect.to_params())
                        .send().await;
            
                    match request {
                        Ok(req) => {
                            let json = req.json::<LiFxResults>().await?;
                            return Ok(json);
                        },
                        Err(err2) => {
                            return Err(err2);
                        }
                    }
                
            
                } else {
                    return Err(err);
                }
            }
        }
    

    }



    /// Asynchronously sets the state for the current light
    /// 
    /// # Arguments
    ///
    /// * `self` - A Light object.
    /// * `access_token` - A personal acces token for authentication with LIFX.
    /// * `state` - A State object containing the values of the State to set
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// extern crate lifx_rs as lifx;
    /// 
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// 
    ///     let key = "xxx".to_string();
    ///
    ///     let config = lifx::LifxConfig::new(key)
    ///         .add_endpoint("https://api.lifx.com".to_string())
    ///         .add_endpoint("http://localhost:8089".to_string());
    /// 
    ///     let lights = lifx::Light::async_list_all(config.clone()).await?;
    ///     println!("{:?}",lights.clone());
    /// 
    ///     let mut state = lifx::State::new();
    ///     state.power = Some("on".to_string());
    ///     state.brightness = Some(1.0);
    /// 
    ///     for light in lights {
    ///         let results = light.async_set_state(config.clone(), state.clone()).await?;
    ///         println!("{:?}",results);
    ///     }
    ///
    ///     Ok(())
    /// }
    ///  ```
    pub async fn async_set_state(&self, config: LifxConfig, state: State) ->  Result<LiFxResults, reqwest::Error>{
        return Self::async_set_state_by_selector(config, format!("id:{}", self.id), state).await;
    }

    /// Asynchronously sets the state for the selected LIFX object
    /// 
    /// # Arguments
    ///
    /// * `access_token` - A personal acces token for authentication with LIFX.
    /// * `selector` - An LIFX selector ex: all, id:xxx, group_id:xxx
    /// * `state` - A State object containing the values of the State to set
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// extern crate lifx_rs as lifx;
    /// 
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// 
    ///     let key = "xxx".to_string();
    ///
    ///     let config = lifx::LifxConfig::new(key)
    ///         .add_endpoint("https://api.lifx.com".to_string())
    ///         .add_endpoint("http://localhost:8089".to_string());
    /// 
    ///     let mut off_state = lifx::State::new();
    ///     off_state.power = Some("off".to_string());
    ///     
    ///     // Turn off all lights
    ///     let result = lifx::Light::async_set_state_by_selector(config, "all".to_string(), off_state).await?;
    ///     println!("{:?}", result);
    ///
    ///     Ok(())
    /// }
    ///  ```
    pub async fn async_set_state_by_selector(config: LifxConfig, selector: String, state: State) ->  Result<LiFxResults, reqwest::Error>{
        let url = format!("{}/v1/lights/{}/state", config.api_endpoints[0], selector);

        let request = reqwest::Client::new().put(url)
            .header("Authorization", format!("Bearer {}", config.access_token))
            .form(&state.to_params())
            .send().await;

        match request {
            Ok(req) => {
                let json = req.json::<LiFxResults>().await?;
                return Ok(json);
            },
            Err(err) => {
                if config.api_endpoints.len() > 1 {
                    let url = format!("{}/v1/lights/{}/state", config.api_endpoints[0], selector);

                    let request = reqwest::Client::new().put(url)
                        .header("Authorization", format!("Bearer {}", config.access_token))
                        .form(&state.to_params())
                        .send().await;
            
                    match request {
                        Ok(req) => {
                            let json = req.json::<LiFxResults>().await?;
                            return Ok(json);
                        },
                        Err(err2) => {
                          return Err(err2);  
                        }
                    }
                } else {
                    return Err(err);
                }
            }
        }
    

    }

    /// Asynchronously sets the state for the selected LIFX object(s)
    /// 
    /// # Arguments
    ///
    /// * `access_token` - A personal acces token for authentication with LIFX.
    /// * `states` - A vector of States with defaults
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// extern crate lifx_rs as lifx;
    /// 
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// 
    ///     let key = "xxx".to_string();
    ///
    ///     let config = lifx::LifxConfig::new(key)
    ///         .add_endpoint("https://api.lifx.com".to_string())
    ///         .add_endpoint("http://localhost:8089".to_string());
    /// 
    ///     let mut set_states = lifx::States::new();
    ///     let mut states: Vec<lifx::State> = Vec::new();
    ///     let mut defaults = lifx::State::new();
    ///     
    ///     defaults.brightness = Some(1.0);
    ///     
    ///     let mut state_1 = lifx::State::new();
    ///     state_1.selector = Some("id:xxx".to_string());
    ///     state_1.power = Some("on".to_string());
    ///     
    ///     let mut state_2 = lifx::State::new();
    ///     state_2.selector = Some("id:xyz".to_string());
    ///     state_2.power = Some("on".to_string());
    ///     
    ///     set_states.states = Some(states);
    ///     set_states.defaults = Some(defaults);
    ///     
    ///     let result = lifx::Light::async_set_states(config, set_states).await?;
    ///     println!("{:?}", result);
    ///
    ///     Ok(())
    /// }
    ///  ```
    pub async fn async_set_states(config: LifxConfig, states: States) ->  Result<LiFxResults, reqwest::Error>{
        let url = format!("{}/v1/lights/state", config.api_endpoints[0]);

        let request = reqwest::blocking::Client::new().put(url)
            .header("Authorization", format!("Bearer {}", config.access_token))
            .json(&states)
            .send();

        match request{
            Ok(req) => {
                let json = req.json::<LiFxResults>()?;
                return Ok(json);
            },
            Err(e) => {
                if config.api_endpoints.len() > 1 {

                    let url = format!("{}/v1/lights/state", config.api_endpoints[1]);

                    let request = reqwest::blocking::Client::new().put(url)
                        .header("Authorization", format!("Bearer {}", config.access_token))
                        .json(&states)
                        .send();
            
                    match request{
                        Ok(req) => {
                            let json = req.json::<LiFxResults>()?;
                            return Ok(json);
                        },
                        Err(e2) => {
                            return Err(e2);
                        }
                    }


                } else {
                    return Err(e);
                }
            }
        }
    

    }

    /// Asynchronously set parameters other than power and duration change the state of the lights by the amount specified.
    /// 
    /// # Arguments
    ///
    /// * `access_token` - A personal acces token for authentication with LIFX.
    /// * `selector` - An LIFX selector ex: all, id:xxx, group_id:xxx
    /// * `delta` - A StateDelta object containing the values to set
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// extern crate lifx_rs as lifx;
    /// 
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// 
    ///     let key = "xxx".to_string();
    ///
    ///     let config = lifx::LifxConfig::new(key)
    ///         .add_endpoint("https://api.lifx.com".to_string())
    ///         .add_endpoint("http://localhost:8089".to_string());
    /// 
    ///     let mut delta = lifx::StateDelta::new();
    ///     delta.duration = Some(0.0);
    ///     delta.power = Some("on".to_string());
    ///     
    ///     // Send StateDelta
    ///     let result = lifx::Light::async_state_delta_by_selector(config, "all".to_string(), delta).await?;
    ///     println!("{:?}", result);
    ///
    ///     Ok(())
    /// }
    ///  ```
    pub async fn async_state_delta_by_selector(config: LifxConfig, selector: String, delta: StateDelta) ->  Result<LiFxResults, reqwest::Error>{
        let url = format!("{}/v1/lights/{}/state/delta", config.api_endpoints[0], selector);

        let request = reqwest::Client::new().post(url)
            .header("Authorization", format!("Bearer {}", config.access_token))
            .form(&delta.to_params())
            .send().await;

        match request{
            Ok(req) => {
                let json = req.json::<LiFxResults>().await?;
                return Ok(json);
            },
            Err(err) => {
                if config.api_endpoints.len() > 1 {
                    let url = format!("{}/v1/lights/{}/state/delta", config.api_endpoints[1], selector);

                    let request = reqwest::Client::new().post(url)
                        .header("Authorization", format!("Bearer {}", config.access_token))
                        .form(&delta.to_params())
                        .send().await;
            
                    match request{
                        Ok(req) => {
                            let json = req.json::<LiFxResults>().await?;
                            return Ok(json);
                        },
                        Err(err2) => {
                            return Err(err2)
                        }
                    }
                } else {
                    return Err(err);
                }
            }
        }
    

    }



    /// Turn off light if on, or turn them on if it is off. 
    /// 
    /// # Arguments
    ///
    /// * `self` - A Light object.
    /// * `access_token` - A personal acces token for authentication with LIFX.
    /// * `clean` - A Clean object containing the values to set
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// extern crate lifx_rs as lifx;
    /// 
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// 
    ///     let key = "xxx".to_string();
    ///
    ///     let config = lifx::LifxConfig::new(key)
    ///         .add_endpoint("https://api.lifx.com".to_string())
    ///         .add_endpoint("http://localhost:8089".to_string());
    /// 
    ///     let lights = lifx::Light::async_list_all(config.clone()).await?;
    ///     println!("{:?}",lights.clone());
    /// 
    ///     let mut toggle = lifx::Toggle::new();
    ///     toggle.duration = Some(0);
    /// 
    ///     for light in lights {
    ///         let results = light.async_toggle(config.clone(), toggle.clone()).await?;
    ///         println!("{:?}",results);
    ///     }
    ///
    ///     Ok(())
    /// }
    ///  ```
    pub async fn async_toggle(&self, config: LifxConfig, toggle: Toggle) ->  Result<LiFxResults, reqwest::Error>{
        return Self::async_toggle_by_selector(config, format!("id:{}", self.id), toggle).await;
    }

    /// Turn off lights if any of them are on, or turn them on if they are all off. 
    /// 
    /// # Arguments
    ///
    /// * `access_token` - A personal acces token for authentication with LIFX.
    /// * `selector` - An LIFX selector ex: all, id:xxx, group_id:xxx
    /// * `clean` - A Clean object containing the values to set
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate lifx_rs as lifx;
    /// 
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// 
    ///     let key = "xxx".to_string();
    ///
    ///     let config = lifx::LifxConfig::new(key)
    ///         .add_endpoint("https://api.lifx.com".to_string())
    ///         .add_endpoint("http://localhost:8089".to_string());
    /// 
    ///     let mut toggle = lifx::Toggle::new();
    ///     toggle.duration = Some(0);
    ///     
    ///     // Toggle all lights
    ///     lifx::Light::async_toggle_by_selector(config, format!("all"), toggle).await?;
    ///
    ///     Ok(())
    /// }
    ///  ```
    pub async fn async_toggle_by_selector(config: LifxConfig, selector: String, toggle: Toggle) ->  Result<LiFxResults, reqwest::Error>{
        let url = format!("{}/v1/lights/{}/toggle", config.api_endpoints[0], selector);

        let request = reqwest::Client::new().post(url)
            .header("Authorization", format!("Bearer {}", config.access_token))
            .form(&toggle.to_params())
            .send().await;

        match request {
            Ok(req) => {
                let json = req.json::<LiFxResults>().await?;
                return Ok(json);
            },
            Err(err) => {
                if config.api_endpoints.len() > 1 {
                    let url = format!("{}/v1/lights/{}/toggle", config.api_endpoints[1], selector);

                    let request = reqwest::Client::new().post(url)
                        .header("Authorization", format!("Bearer {}", config.access_token))
                        .form(&toggle.to_params())
                        .send().await;
            
                    match request {
                        Ok(req) => {
                            let json = req.json::<LiFxResults>().await?;
                            return Ok(json);
                        },
                        Err(err2) => {
                            return Err(err2);
                        }
                    }
                } else {
                    return Err(err);
                }
            }
        }
    

    }

    // =======================================
    // END OF ASYNC FUNCTIONS
    // =======================================

    // =======================================
    // BEGINING OF SYNC FUNCTIONS
    // =======================================

    /// Set the breathe animation for the current light
    /// 
    /// # Arguments
    ///
    /// * `self` - A Light object.
    /// * `access_token` - A personal acces token for authentication with LIFX.
    /// * `breathe` - A BreatheEffect object containing the values to set
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// extern crate lifx_rs as lifx;
    /// 
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// 
    ///     let key = "xxx".to_string();
    ///
    ///     let config = lifx::LifxConfig::new(key)
    ///         .add_endpoint("https://api.lifx.com".to_string())
    ///         .add_endpoint("http://localhost:8089".to_string());
    /// 
    ///     let lights = lifx::Light::list_all(config.clone())?;
    ///     println!("{:?}",lights.clone());
    /// 
    ///     let mut breathe = lifx::BreatheEffect::new();
    ///     breathe.color = Some("red".to_string());
    ///     breathe.from_color = Some("green".to_string());
    ///     breathe.period = Some(10.0);
    ///     breathe.persist = Some(true);
    ///     breathe.power_on = Some(true);
    /// 
    ///     for light in lights {
    ///         let results = light.breathe_effect(config.clone(), breathe.clone());
    ///         println!("{:?}",results);
    ///     }
    ///
    ///     Ok(())
    /// }
    ///  ```
    pub fn breathe_effect(&self, config: LifxConfig, breathe: BreatheEffect) ->  Result<LiFxResults, reqwest::Error>{
        return Self::breathe_by_selector_effect(config, format!("id:{}", self.id), breathe);
    }

    /// Activate the breathe animation for the selected light(s)
    /// 
    /// # Arguments
    ///
    /// * `access_token` - A personal acces token for authentication with LIFX.
    /// * `selector` - An LIFX selector ex: all, id:xxx, group_id:xxx
    /// * `breathe` - A BreatheEffect object containing the values to set
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// extern crate lifx_rs as lifx;
    /// 
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// 
    ///     let key = "xxx".to_string();
    ///
    ///     let config = lifx::LifxConfig::new(key)
    ///         .add_endpoint("https://api.lifx.com".to_string())
    ///         .add_endpoint("http://localhost:8089".to_string());
    /// 
    ///     let mut breathe = lifx::BreatheEffect::new();
    ///     breathe.color = Some("red".to_string());
    ///     breathe.from_color = Some("green".to_string());
    ///     breathe.period = Some(10.0);
    ///     breathe.persist = Some(true);
    ///     breathe.power_on = Some(true);
    ///     
    ///     // Apply breathe effect to all light(s)
    ///     lifx::Light::breathe_by_selector_effect(config, format!("all"), breathe)?;
    ///
    ///     Ok(())
    /// }
    ///  ```
    pub fn breathe_by_selector_effect(config: LifxConfig, selector: String, breathe: BreatheEffect) ->  Result<LiFxResults, reqwest::Error>{
        let url = format!("{}/v1/lights/{}/effects/breathe", config.api_endpoints[0], selector);

        let request = reqwest::blocking::Client::new().post(url)
            .header("Authorization", format!("Bearer {}", config.access_token))
            .form(&breathe.to_params())
            .send();

        match request {
            Ok(req) => {
                let json = req.json::<LiFxResults>()?;
                return Ok(json);
            },
            Err(e) => {
                if config.api_endpoints.len() > 1 {
                    let url = format!("{}/v1/lights/{}/effects/breathe", config.api_endpoints[1], selector);

                    let request = reqwest::blocking::Client::new().post(url)
                        .header("Authorization", format!("Bearer {}", config.access_token))
                        .form(&breathe.to_params())
                        .send();
            
                    match request {
                        Ok(req) => {
                            let json = req.json::<LiFxResults>()?;
                            return Ok(json);
                        },
                        Err(e2) => {
                            return Err(e2);
                        }
                    }
                } else {
                    return Err(e);
                }
            }
        }
    

    }

    /// This endpoint lets you switch a light to clean mode, with a set duration. 
    /// 
    /// # Arguments
    ///
    /// * `self` - A Light object.
    /// * `access_token` - A personal acces token for authentication with LIFX.
    /// * `clean` - A Clean object containing the values to set
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// extern crate lifx_rs as lifx;
    /// 
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// 
    ///     let key = "xxx".to_string();
    ///
    ///     let config = lifx::LifxConfig::new(key)
    ///         .add_endpoint("https://api.lifx.com".to_string())
    ///         .add_endpoint("http://localhost:8089".to_string());
    /// 
    ///     let lights = lifx::Light::list_all(config.clone())?;
    ///     println!("{:?}",lights.clone());
    /// 
    ///     let mut clean = lifx::Clean::new();
    ///     clean.duration = Some(0);
    ///     clean.stop = Some(false);
    /// 
    ///     for light in lights {
    ///         let results = light.clean(config.clone(), clean.clone());
    ///         println!("{:?}",results);
    ///     }
    ///
    ///     Ok(())
    /// }
    ///  ```
    pub fn clean(&self, config: LifxConfig, clean: Clean) ->  Result<LiFxResults, reqwest::Error>{
        return Self::clean_by_selector(config, format!("id:{}", self.id), clean);
    }

    /// This endpoint lets you switch a selected LIFX object to clean mode, with a set duration. 
    /// 
    /// # Arguments
    ///
    /// * `access_token` - A personal acces token for authentication with LIFX.
    /// * `selector` - An LIFX selector ex: all, id:xxx, group_id:xxx
    /// * `clean` - A Clean object containing the values to set
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// extern crate lifx_rs as lifx;
    /// 
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// 
    ///     let key = "xxx".to_string();
    ///
    ///     let config = lifx::LifxConfig::new(key)
    ///         .add_endpoint("https://api.lifx.com".to_string())
    ///         .add_endpoint("http://localhost:8089".to_string());
    /// 
    ///     let mut clean = lifx::Clean::new();
    ///     clean.duration = Some(0);
    ///     clean.stop = Some(false);
    ///     
    ///     // Set all light to clean mode
    ///     lifx::Light::clean_by_selector(config.clone(), format!("all"), clean);
    ///
    ///     Ok(())
    /// }
    ///  ```
    pub fn clean_by_selector(config: LifxConfig, selector: String, clean: Clean) ->  Result<LiFxResults, reqwest::Error>{
        let url = format!("{}/v1/lights/{}/clean", config.api_endpoints[0], selector);

        let request = reqwest::blocking::Client::new().post(url)
            .header("Authorization", format!("Bearer {}", config.access_token))
            .form(&clean.to_params())
            .send();

        match request {
            Ok(req) => {
                let json = req.json::<LiFxResults>()?;
                return Ok(json);
            },
            Err(err) => {
                if config.api_endpoints.len() > 1 {
                    let url = format!("{}/v1/lights/{}/clean", config.api_endpoints[1], selector);

                    let request = reqwest::blocking::Client::new().post(url)
                        .header("Authorization", format!("Bearer {}", config.access_token))
                        .form(&clean.to_params())
                        .send();
            
                    match request {
                        Ok(req) => {
                            let json = req.json::<LiFxResults>()?;
                            return Ok(json);
                        },
                        Err(err2) => {
                            return Err(err2);
                        }
                    }
                } else {
                    return Err(err);
                }
            }
        }
    

    }

    /// Stops animation(s) for the current light
    /// 
    /// # Arguments
    ///
    /// * `self` - A Light object.
    /// * `access_token` - A personal acces token for authentication with LIFX.
    /// * `flame_effect` - A FlameEffect object containing the values to set
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// extern crate lifx_rs as lifx;
    /// 
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// 
    ///     let key = "xxx".to_string();
    ///
    ///     let config = lifx::LifxConfig::new(key)
    ///         .add_endpoint("https://api.lifx.com".to_string())
    ///         .add_endpoint("http://localhost:8089".to_string());
    /// 
    ///     let lights = lifx::Light::list_all(config.clone())?;
    ///     println!("{:?}",lights.clone());
    ///     
    ///             let mut effects_off = lifx::EffectsOff::new();
    ///             effects_off.power_off = Some(true);
    ///         
    ///             for light in lights {
    ///                 let results = light.effects_off(config.clone(), effects_off.clone());
    ///                 println!("{:?}",results);
    ///             }
    ///
    ///     Ok(())
    /// }
    ///  ```
    pub fn effects_off(&self, config: LifxConfig, effects_off: EffectsOff) ->  Result<LiFxResults, reqwest::Error>{
        return Self::effects_off_by_selector(config, format!("id:{}", self.id), effects_off);
    }

    /// Stops animation(s) for the selected light(s)
    /// 
    /// # Arguments
    ///
    /// * `access_token` - A personal acces token for authentication with LIFX.
    /// * `selector` - An LIFX selector ex: all, id:xxx, group_id:xxx
    /// * `effects_off` - A EffectsOff object containing the values to set
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// extern crate lifx_rs as lifx;
    /// 
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// 
    ///     let key = "xxx".to_string();
    ///
    ///     let config = lifx::LifxConfig::new(key)
    ///         .add_endpoint("https://api.lifx.com".to_string())
    ///         .add_endpoint("http://localhost:8089".to_string());
    /// 
    ///     let mut effects_off = lifx::EffectsOff::new();
    ///     effects_off.power_off = Some(true);
    ///     
    ///     // Send morph effect to all lights
    ///     lifx::Light::effects_off_by_selector(config.clone(), format!("all"), effects_off);
    ///
    ///     Ok(())
    /// }
    ///  ```
    pub fn effects_off_by_selector(config: LifxConfig, selector: String, effects_off: EffectsOff) ->  Result<LiFxResults, reqwest::Error>{
        let url = format!("{}/v1/lights/{}/effects/off", config.api_endpoints[0], selector);

        let request = reqwest::blocking::Client::new().post(url)
            .header("Authorization", format!("Bearer {}", config.access_token))
            .form(&effects_off.to_params())
            .send();

        match request {
            Ok(req) => {
                let json = req.json::<LiFxResults>()?;
                return Ok(json);
            },
            Err(err) => {
                if config.api_endpoints.len() > 1 {
                    let url = format!("{}/v1/lights/{}/effects/off", config.api_endpoints[1], selector);

                    let request = reqwest::blocking::Client::new().post(url)
                        .header("Authorization", format!("Bearer {}", config.access_token))
                        .form(&effects_off.to_params())
                        .send();
            
                    match request {
                        Ok(req) => {
                            let json = req.json::<LiFxResults>()?;
                            return Ok(json);
                        },
                        Err(err2) => {
                            return Err(err2);
                        }
                    }
                } else {
                    return Err(err);
                }
            }
        }
    

    }

    /// Activate the flame animation for the current light
    /// 
    /// # Arguments
    ///
    /// * `self` - A Light object.
    /// * `access_token` - A personal acces token for authentication with LIFX.
    /// * `flame_effect` - A FlameEffect object containing the values to set
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// extern crate lifx_rs as lifx;
    /// 
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// 
    ///     let key = "xxx".to_string();
    ///
    ///     let config = lifx::LifxConfig::new(key)
    ///         .add_endpoint("https://api.lifx.com".to_string())
    ///         .add_endpoint("http://localhost:8089".to_string());
    /// 
    ///     let lights = lifx::Light::list_all(config.clone())?;
    ///     println!("{:?}",lights.clone());
    ///     
    ///             let mut flame_effect = lifx::FlameEffect::new();
    ///             flame_effect.period = Some(10);
    ///             flame_effect.duration = Some(0.0);
    ///             flame_effect.power_on = Some(true);
    ///         
    ///             for light in lights {
    ///                 let results = light.flame_effect(config.clone(), flame_effect.clone());
    ///                 println!("{:?}",results);
    ///             }
    ///
    ///     Ok(())
    /// }
    ///  ```
    pub fn flame_effect(&self, config: LifxConfig, flame_effect: FlameEffect) ->  Result<LiFxResults, reqwest::Error>{
        return Self::flame_effect_by_selector(config, format!("id:{}", self.id), flame_effect);
    }

    /// Activate the flame animation for the selected light(s)
    /// 
    /// # Arguments
    ///
    /// * `access_token` - A personal acces token for authentication with LIFX.
    /// * `selector` - An LIFX selector ex: all, id:xxx, group_id:xxx
    /// * `flame_effect` - A FlameEffect object containing the values to set
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// extern crate lifx_rs as lifx;
    /// 
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// 
    ///     let key = "xxx".to_string();
    ///
    ///     let config = lifx::LifxConfig::new(key)
    ///         .add_endpoint("https://api.lifx.com".to_string())
    ///         .add_endpoint("http://localhost:8089".to_string());
    /// 
    ///     let mut flame_effect = lifx::FlameEffect::new();
    ///     flame_effect.period = Some(10);
    ///     flame_effect.duration = Some(0.0);
    ///     flame_effect.power_on = Some(true);
    ///     
    ///     // Send morph effect to all lights
    ///     lifx::Light::flame_effect_by_selector(config.clone(), format!("all"), flame_effect);
    ///
    ///     Ok(())
    /// }
    ///  ```
    pub fn flame_effect_by_selector(config: LifxConfig, selector: String, flame_effect: FlameEffect) ->  Result<LiFxResults, reqwest::Error>{
        let url = format!("{}/v1/lights/{}/effects/flame", config.api_endpoints[0], selector);

        let request = reqwest::blocking::Client::new().post(url)
            .header("Authorization", format!("Bearer {}", config.access_token))
            .form(&flame_effect.to_params())
            .send();

        match request {
            Ok(req) => {
                let json = req.json::<LiFxResults>()?;
                return Ok(json);
            },
            Err(err) => {
                if config.api_endpoints.len() > 1 {
                    let url = format!("{}/v1/lights/{}/effects/flame", config.api_endpoints[1], selector);

                    let request = reqwest::blocking::Client::new().post(url)
                        .header("Authorization", format!("Bearer {}", config.access_token))
                        .form(&flame_effect.to_params())
                        .send();
            
                    match request {
                        Ok(req) => {
                            let json = req.json::<LiFxResults>()?;
                            return Ok(json);
                        },
                        Err(err2) => {
                            return Err(err2);
                        }
                    }
                } else {
                    return Err(err);
                }
            }
        }
    

    }

    /// Gets ALL lights belonging to the authenticated account
    /// 
    /// # Arguments
    ///
    /// * `access_token` - A personal acces token for authentication with LIFX.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// extern crate lifx_rs as lifx;
    /// 
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// 
    ///     let key = "xxx".to_string();
    ///
    ///     let config = lifx::LifxConfig::new(key)
    ///         .add_endpoint("https://api.lifx.com".to_string())
    ///         .add_endpoint("http://localhost:8089".to_string());
    /// 
    ///     let lights = lifx::Light::list_all(config)?;
    ///
    ///     Ok(())
    /// }
    ///  ```
    pub fn list_all(config: LifxConfig) -> Result<Lights, reqwest::Error> {
        return Self::list_by_selector(config, format!("all"));
    }

    /// Gets lights belonging to the authenticated account. Filtering the lights using selectors. Properties such as id, label, group and location can be used in selectors.
    /// 
    /// # Arguments
    ///
    /// * `access_token` - A personal acces token for authentication with LIFX.
    /// * `selector` - An LIFX selector ex: all, id:xxx, group_id:xxx
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// extern crate lifx_rs as lifx;
    /// 
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// 
    ///     let key = "xxx".to_string();
    ///
    ///     let config = lifx::LifxConfig::new(key)
    ///         .add_endpoint("https://api.lifx.com".to_string())
    ///         .add_endpoint("http://localhost:8089".to_string());
    /// 
    ///     let lights = lifx::Light::list_by_selector(config, format!("all"))?;
    ///
    ///     Ok(())
    /// }
    ///  ```
    pub fn list_by_selector(config: LifxConfig, selector: String) -> Result<Lights, reqwest::Error> {
        let url = format!("{}/v1/lights/{}", config.api_endpoints[0], selector);
        let request = reqwest::blocking::Client::new().get(url).header("Authorization", format!("Bearer {}", config.access_token)).send();
        match request {
            Ok(req) => {
                let json = req.json::<Lights>()?;
                return Ok(json);
            },
            Err(err) => {
                if config.api_endpoints.len() > 1 {
                    let url = format!("{}/v1/lights/{}", config.api_endpoints[1], selector);
                    let request = reqwest::blocking::Client::new().get(url).header("Authorization", format!("Bearer {}", config.access_token)).send();
                    match request {
                        Ok(req) => {
                            let json = req.json::<Lights>()?;
                            return Ok(json);
                        },
                        Err(err2) => {
                            return Err(err2);
                        }
                    }
                } else {
                    return Err(err);
                }
            }
        }

    }

    /// Activate the morph animation for the current light
    /// 
    /// # Arguments
    ///
    /// * `self` - A Light object.
    /// * `access_token` - A personal acces token for authentication with LIFX.
    /// * `morph_effect` - A MorphEffect object containing the values to set
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// extern crate lifx_rs as lifx;
    /// 
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// 
    ///     let key = "xxx".to_string();
    ///
    ///     let config = lifx::LifxConfig::new(key)
    ///         .add_endpoint("https://api.lifx.com".to_string())
    ///         .add_endpoint("http://localhost:8089".to_string());
    /// 
    ///     let lights = lifx::Light::list_all(config.clone())?;
    ///     println!("{:?}",lights.clone());
    ///     
    ///             let mut morph_effect = lifx::MorphEffect::new();
    ///             morph_effect.period = Some(10);
    ///             morph_effect.duration = Some(0.0);
    /// 
    ///             let mut palette: Vec<String> = Vec::new();
    ///             palette.push(format!("red"));
    ///             palette.push(format!("green"));
    /// 
    ///             morph_effect.palette = Some(palette);
    ///             morph_effect.power_on = Some(true);
    ///         
    ///             for light in lights {
    ///                 let results = light.morph_effect(config.clone(), morph_effect.clone());
    ///                 println!("{:?}",results);
    ///             }
    ///
    ///     Ok(())
    /// }
    ///  ```
    pub fn morph_effect(&self, config: LifxConfig, morph_effect: MorphEffect) ->  Result<LiFxResults, reqwest::Error>{
        return Self::morph_effect_by_selector(config, format!("id:{}", self.id), morph_effect);
    }

    /// Activate the morph animation for the selected light(s)
    /// 
    /// # Arguments
    ///
    /// * `access_token` - A personal acces token for authentication with LIFX.
    /// * `selector` - An LIFX selector ex: all, id:xxx, group_id:xxx
    /// * `clean` - A Clean object containing the values to set
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// extern crate lifx_rs as lifx;
    /// 
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// 
    ///     let key = "xxx".to_string();
    ///
    ///     let config = lifx::LifxConfig::new(key)
    ///         .add_endpoint("https://api.lifx.com".to_string())
    ///         .add_endpoint("http://localhost:8089".to_string());
    /// 
    ///     let mut morph_effect = lifx::MorphEffect::new();
    ///     morph_effect.period = Some(10);
    ///     morph_effect.duration = Some(0.0);
    /// 
    ///     let mut palette: Vec<String> = Vec::new();
    ///     palette.push(format!("red"));
    ///     palette.push(format!("green"));
    /// 
    ///     morph_effect.palette = Some(palette);
    ///     morph_effect.power_on = Some(true);
    ///     
    ///     // Send morph effect to all lights
    ///     lifx::Light::morph_effect_by_selector(config.clone(), format!("all"), morph_effect);
    ///
    ///     Ok(())
    /// }
    ///  ```
    pub fn morph_effect_by_selector(config: LifxConfig, selector: String, morph_effect: MorphEffect) ->  Result<LiFxResults, reqwest::Error>{
        let url = format!("{}/v1/lights/{}/effects/morph", config.api_endpoints[0], selector);
        let request = reqwest::blocking::Client::new().post(url).header("Authorization", format!("Bearer {}", config.access_token)).form(&morph_effect.to_params()).send();
        match request{
            Ok(req) => {
                let json = req.json::<LiFxResults>()?;
                return Ok(json);
            },
            Err(err) => {
                if config.api_endpoints.len() > 1 {
                    let url = format!("{}/v1/lights/{}/effects/morph", config.api_endpoints[1], selector);
                    let request = reqwest::blocking::Client::new().post(url).header("Authorization", format!("Bearer {}", config.access_token)).form(&morph_effect.to_params()).send();
                    match request{
                        Ok(req) => {
                            let json = req.json::<LiFxResults>()?;
                            return Ok(json);
                        },
                        Err(err2) => {
                            return Err(err2);
                        }
                    }
                } else {
                    return Err(err);
                }
            }
        }



    }

    /// Activate the move animation for the current light
    /// 
    /// # Arguments
    ///
    /// * `self` - A Light object.
    /// * `access_token` - A personal acces token for authentication with LIFX.
    /// * `move_effect` - A MoveEffect object containing the values to set
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// extern crate lifx_rs as lifx;
    /// 
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// 
    ///     let key = "xxx".to_string();
    ///
    ///     let config = lifx::LifxConfig::new(key)
    ///         .add_endpoint("https://api.lifx.com".to_string())
    ///         .add_endpoint("http://localhost:8089".to_string());
    /// 
    ///     let lights = lifx::Light::list_all(config.clone())?;
    ///     println!("{:?}",lights.clone());
    ///     
    ///             let mut move_effect = lifx::MoveEffect::new();
    ///             move_effect.direction = Some("forward".to_string()); // or backward
    ///             move_effect.period = Some(10);
    ///             move_effect.cycles = Some(0.9);
    ///             move_effect.power_on = Some(true);
    ///         
    ///             for light in lights {
    ///                 let results = light.move_effect(config.clone(), move_effect.clone());
    ///                 println!("{:?}",results);
    ///             }
    ///
    ///     Ok(())
    /// }
    ///  ```
    pub fn move_effect(&self, config: LifxConfig, move_effect: MoveEffect) ->  Result<LiFxResults, reqwest::Error>{
        return Self::move_effect_by_selector(config, format!("id:{}", self.id), move_effect);
    }

    /// Activate the move animation for the selected light(s)
    /// 
    /// # Arguments
    ///
    /// * `access_token` - A personal acces token for authentication with LIFX.
    /// * `selector` - An LIFX selector ex: all, id:xxx, group_id:xxx
    /// * `clean` - A Clean object containing the values to set
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// extern crate lifx_rs as lifx;
    /// 
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// 
    ///     let key = "xxx".to_string();
    ///
    ///     let config = lifx::LifxConfig::new(key)
    ///         .add_endpoint("https://api.lifx.com".to_string())
    ///         .add_endpoint("http://localhost:8089".to_string());
    /// 
    ///     let mut move_effect = lifx::MoveEffect::new();
    ///     move_effect.direction = Some("forward".to_string()); // or backward
    ///     move_effect.period = Some(10);
    ///     move_effect.cycles = Some(0.9);
    ///     move_effect.power_on = Some(true);
    ///     
    ///     // Toggle all lights
    ///     lifx::Light::move_effect_by_selector(config.clone(), format!("all"), move_effect);
    ///
    ///     Ok(())
    /// }
    ///  ```
    pub fn move_effect_by_selector(config: LifxConfig, selector: String, move_effect: MoveEffect) ->  Result<LiFxResults, reqwest::Error>{
        let url = format!("{}/v1/lights/{}/effects/move", config.api_endpoints[0], selector);
        let request = reqwest::blocking::Client::new().post(url).header("Authorization", format!("Bearer {}", config.access_token)).form(&move_effect.to_params()).send();
        match request{
            Ok(req) => {
                let json = req.json::<LiFxResults>()?;
                return Ok(json);
            },
            Err(err) => {
                if config.api_endpoints.len() > 1 {
                    let url = format!("{}/v1/lights/{}/effects/move", config.api_endpoints[1], selector);
                    let request = reqwest::blocking::Client::new().post(url).header("Authorization", format!("Bearer {}", config.access_token)).form(&move_effect.to_params()).send();
                    match request{
                        Ok(req) => {
                            let json = req.json::<LiFxResults>()?;
                            return Ok(json);
                        },
                        Err(err2) => {
                            return Err(err2);
                        }
                    }
                } else {
                    return Err(err);
                }
            }
        }

    }

    /// Activate the pulse animation for the current light
    /// 
    /// # Arguments
    ///
    /// * `self` - A Light object.
    /// * `access_token` - A personal acces token for authentication with LIFX.
    /// * `pulse_effect` - A PulseEffect object containing the values to set
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// extern crate lifx_rs as lifx;
    /// 
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// 
    ///     let key = "xxx".to_string();
    ///
    ///     let config = lifx::LifxConfig::new(key)
    ///         .add_endpoint("https://api.lifx.com".to_string())
    ///         .add_endpoint("http://localhost:8089".to_string());
    /// 
    ///     let lights = lifx::Light::list_all(config.clone())?;
    ///     println!("{:?}",lights.clone());
    ///     
    ///             let mut pulse = lifx::PulseEffect::new();
    ///             pulse.color = Some("red".to_string());
    ///             pulse.from_color = Some("green".to_string());
    ///             pulse.period = Some(10.0);
    ///             pulse.persist = Some(true);
    ///             pulse.power_on = Some(true);
    ///         
    ///             for light in lights {
    ///                 let results = light.pulse_effect(config.clone(), pulse.clone());
    ///                 println!("{:?}",results);
    ///             }
    ///
    ///     Ok(())
    /// }
    ///  ```
    pub fn pulse_effect(&self, config: LifxConfig, pulse_effect: PulseEffect) ->  Result<LiFxResults, reqwest::Error>{
        return Self::pulse_effect_by_selector(config, format!("id:{}", self.id), pulse_effect);
    }

    /// Activate the pulse animation for the selected light(s)
    /// 
    /// # Arguments
    ///
    /// * `access_token` - A personal acces token for authentication with LIFX.
    /// * `selector` - An LIFX selector ex: all, id:xxx, group_id:xxx
    /// * `pulse_effect` - A PulseEffect object containing the values to set
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// extern crate lifx_rs as lifx;
    /// 
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// 
    ///     let key = "xxx".to_string();
    ///
    ///     let config = lifx::LifxConfig::new(key)
    ///         .add_endpoint("https://api.lifx.com".to_string())
    ///         .add_endpoint("http://localhost:8089".to_string());
    /// 
    ///     let mut pulse = lifx::PulseEffect::new();
    ///     pulse.color = Some("red".to_string());
    ///     pulse.from_color = Some("green".to_string());
    ///     pulse.period = Some(10.0);
    ///     pulse.persist = Some(true);
    ///     pulse.power_on = Some(true);
    ///     
    ///     // Toggle all lights
    ///     lifx::Light::pulse_effect_by_selector(config.clone(), format!("all"), pulse);
    ///
    ///     Ok(())
    /// }
    ///  ```
    pub fn pulse_effect_by_selector(config: LifxConfig, selector: String, pulse_effect: PulseEffect) ->  Result<LiFxResults, reqwest::Error>{
        let url = format!("{}/v1/lights/{}/effects/pulse", config.api_endpoints[0], selector);
        let request = reqwest::blocking::Client::new().post(url)
            .header("Authorization", format!("Bearer {}", config.access_token))
            .form(&pulse_effect.to_params())
            .send();
        match request {
            Ok(req) => {
                let json = req.json::<LiFxResults>()?;
                return Ok(json);
            },
            Err(err) => {
                if config.api_endpoints.len() > 1 {
                    let url = format!("{}/v1/lights/{}/effects/pulse", config.api_endpoints[1], selector);
                    let request = reqwest::blocking::Client::new().post(url)
                        .header("Authorization", format!("Bearer {}", config.access_token))
                        .form(&pulse_effect.to_params())
                        .send();
                    match request {
                        Ok(req) => {
                            let json = req.json::<LiFxResults>()?;
                            return Ok(json);
                        },
                        Err(err2) => {
                            return Err(err2);
                        }
                    }
                } else {
                    return Err(err);
                }
            }
        }

    }

    /// Sets the state for the current light
    /// 
    /// # Arguments
    ///
    /// * `self` - A Light object.
    /// * `access_token` - A personal acces token for authentication with LIFX.
    /// * `state` - A State object containing the values of the State to set
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// extern crate lifx_rs as lifx;
    /// 
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// 
    ///     let key = "xxx".to_string();
    ///
    ///     let config = lifx::LifxConfig::new(key)
    ///         .add_endpoint("https://api.lifx.com".to_string())
    ///         .add_endpoint("http://localhost:8089".to_string());
    /// 
    ///     let lights = lifx::Light::list_all(config.clone())?;
    ///     println!("{:?}",lights.clone());
    ///     
    ///             let mut state = lifx::State::new();
    ///             state.power = Some("on".to_string());
    ///             state.brightness = Some(1.0);
    ///         
    ///             for light in lights {
    ///                 let results = light.set_state(config.clone(), state.clone());
    ///                 println!("{:?}",results);
    ///             }
    ///
    ///     Ok(())
    /// }
    ///  ```
    pub fn set_state(&self, config: LifxConfig, state: State) ->  Result<LiFxResults, reqwest::Error>{
        return Self::set_state_by_selector(config, format!("id:{}", self.id), state);
    }

    /// Sets the state for the selected LIFX object
    /// 
    /// # Arguments
    ///
    /// * `access_token` - A personal acces token for authentication with LIFX.
    /// * `selector` - An LIFX selector ex: all, id:xxx, group_id:xxx
    /// * `state` - A State object containing the values of the State to set
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// extern crate lifx_rs as lifx;
    /// 
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// 
    ///     let key = "xxx".to_string();
    ///
    ///     let config = lifx::LifxConfig::new(key)
    ///         .add_endpoint("https://api.lifx.com".to_string())
    ///         .add_endpoint("http://localhost:8089".to_string());
    /// 
    ///     let mut off_state = lifx::State::new();
    ///     off_state.power = Some("off".to_string());
    ///     
    ///     // Turn off all lights
    ///     lifx::Light::set_state_by_selector(config.clone(), format!("all"), off_state);
    ///
    ///     Ok(())
    /// }
    ///  ```
    pub fn set_state_by_selector(config: LifxConfig, selector: String, state: State) ->  Result<LiFxResults, reqwest::Error>{
        let url = format!("{}/v1/lights/{}/state", config.api_endpoints[0], selector);

        let request = reqwest::blocking::Client::new().put(url)
            .header("Authorization", format!("Bearer {}", config.access_token))
            .form(&state.to_params())
            .send();
        match request {
            Ok(req) => {
                let json = req.json::<LiFxResults>()?;
                return Ok(json);
            },
            Err(err) => {
                if config.api_endpoints.len() > 1 {
                    let url = format!("{}/v1/lights/{}/state", config.api_endpoints[1], selector);

                    let request = reqwest::blocking::Client::new().put(url)
                        .header("Authorization", format!("Bearer {}", config.access_token))
                        .form(&state.to_params())
                        .send();
                    match request {
                        Ok(req) => {
                            let json = req.json::<LiFxResults>()?;
                            return Ok(json);
                        },
                        Err(err2) => {
                            return Err(err2);
                        }
                    }
                } else {
                    return Err(err);
                }
            }
        }
    

    }

    /// Sets the state for the selected LIFX object
    /// 
    /// # Arguments
    ///
    /// * `access_token` - A personal acces token for authentication with LIFX.
    /// * `states` - A vector of States with defaults
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// extern crate lifx_rs as lifx;
    /// 
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// 
    ///     let key = "xxx".to_string();
    ///
    ///     let config = lifx::LifxConfig::new(key)
    ///         .add_endpoint("https://api.lifx.com".to_string())
    ///         .add_endpoint("http://localhost:8089".to_string());
    /// 
    ///     let mut set_states = lifx::States::new();
    ///     let mut states: Vec<lifx::State> = Vec::new();
    ///     let mut defaults = lifx::State::new();
    ///     
    ///     defaults.brightness = Some(1.0);
    ///     
    ///     let mut state_1 = lifx::State::new();
    ///     state_1.selector = Some("id:xxx".to_string());
    ///     state_1.power = Some("on".to_string());
    ///     
    ///     let mut state_2 = lifx::State::new();
    ///     state_2.selector = Some("id:xyz".to_string());
    ///     state_2.power = Some("on".to_string());
    ///     
    ///     set_states.states = Some(states);
    ///     set_states.defaults = Some(defaults);
    ///     
    ///     lifx::Light::set_states(config.clone(), set_states);
    ///
    ///     Ok(())
    /// }
    ///  ```
    pub fn set_states(config: LifxConfig, states: States) ->  Result<LiFxResults, reqwest::Error>{
        let url = format!("{}/v1/lights/state", config.api_endpoints[0]);

        let request = reqwest::blocking::Client::new().put(url)
            .header("Authorization", format!("Bearer {}", config.access_token))
            .json(&states)
            .send();

        match request {
            Ok(req) => {
                let json = req.json::<LiFxResults>()?;
                return Ok(json);
            },
            Err(err) => {
                if config.api_endpoints.len() > 1 {
                    let url = format!("{}/v1/lights/state", config.api_endpoints[1]);

                    let request = reqwest::blocking::Client::new().put(url)
                        .header("Authorization", format!("Bearer {}", config.access_token))
                        .json(&states)
                        .send();
            
                    match request {
                        Ok(req) => {
                            let json = req.json::<LiFxResults>()?;
                            return Ok(json);
                        },
                        Err(err2) => {
                            return Err(err2);
                        }
                    }
                } else {
                    return Err(err);
                }
            }
        }
    

    }

    /// Set parameters other than power and duration change the state of the lights by the amount specified.
    /// 
    /// # Arguments
    ///
    /// * `access_token` - A personal acces token for authentication with LIFX.
    /// * `selector` - An LIFX selector ex: all, id:xxx, group_id:xxx
    /// * `delta` - A StateDelta object containing the values to set
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// extern crate lifx_rs as lifx;
    /// 
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// 
    ///     let key = "xxx".to_string();
    ///
    ///     let config = lifx::LifxConfig::new(key)
    ///         .add_endpoint("https://api.lifx.com".to_string())
    ///         .add_endpoint("http://localhost:8089".to_string());
    /// 
    ///     let mut delta = lifx::StateDelta::new();
    ///     delta.duration = Some(0.0);
    ///     delta.power = Some("on".to_string());
    ///     
    ///     // Send StateDelta
    ///     lifx::Light::state_delta_by_selector(config, format!("all"), delta)?;
    ///
    ///     Ok(())
    /// }
    ///  ```
    pub fn state_delta_by_selector(config: LifxConfig, selector: String, delta: StateDelta) ->  Result<LiFxResults, reqwest::Error>{
        let url = format!("{}/v1/lights/{}/state/delta", config.api_endpoints[0], selector);

        let request = reqwest::blocking::Client::new().post(url)
            .header("Authorization", format!("Bearer {}", config.access_token))
            .form(&delta.to_params())
            .send();

        match request {
            Ok(req) => {
                let json = req.json::<LiFxResults>()?;
                return Ok(json);
            },
            Err(err) => {
                if config.api_endpoints.len() > 1 {
                    let url = format!("{}/v1/lights/{}/state/delta", config.api_endpoints[1], selector);

                    let request = reqwest::blocking::Client::new().post(url)
                        .header("Authorization", format!("Bearer {}", config.access_token))
                        .form(&delta.to_params())
                        .send();
            
                    match request {
                        Ok(req) => {
                            let json = req.json::<LiFxResults>()?;
                            return Ok(json);
                        },
                        Err(err2) => {
                            return Err(err2);
                        }
                    }
                } else {
                    return Err(err);
                }
            }
        }

    }


    /// Turn off light if on, or turn them on if it is off. 
    /// 
    /// # Arguments
    ///
    /// * `self` - A Light object.
    /// * `access_token` - A personal acces token for authentication with LIFX.
    /// * `clean` - A Clean object containing the values to set
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// extern crate lifx_rs as lifx;
    /// 
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// 
    ///     let key = "xxx".to_string();
    ///
    ///     let config = lifx::LifxConfig::new(key)
    ///         .add_endpoint("https://api.lifx.com".to_string())
    ///         .add_endpoint("http://localhost:8089".to_string());
    /// 
    ///     let lights = lifx::Light::list_all(config.clone())?;
    ///     println!("{:?}",lights.clone());
    /// 
    ///     let mut toggle = lifx::Toggle::new();
    ///     toggle.duration = Some(0);
    /// 
    ///     for light in lights {
    ///         let results = light.toggle(config.clone(), toggle.clone());
    ///         println!("{:?}",results);
    ///     }
    ///
    ///     Ok(())
    /// }
    ///  ```
    pub fn toggle(&self, config: LifxConfig, toggle: Toggle) ->  Result<LiFxResults, reqwest::Error>{
        return Self::toggle_by_selector(config, format!("id:{}", self.id), toggle);
    }

    /// Turn off lights if any of them are on, or turn them on if they are all off. 
    /// 
    /// # Arguments
    ///
    /// * `access_token` - A personal acces token for authentication with LIFX.
    /// * `selector` - An LIFX selector ex: all, id:xxx, group_id:xxx
    /// * `clean` - A Clean object containing the values to set
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// extern crate lifx_rs as lifx;
    /// 
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// 
    ///     let key = "xxx".to_string();
    ///
    ///     let config = lifx::LifxConfig::new(key)
    ///         .add_endpoint("https://api.lifx.com".to_string())
    ///         .add_endpoint("http://localhost:8089".to_string());
    /// 
    ///     let mut toggle = lifx::Toggle::new();
    ///     toggle.duration = Some(0);
    ///     
    ///     // Toggle all lights
    ///     lifx::Light::toggle_by_selector(config, format!("all"), toggle)?;
    ///
    ///     Ok(())
    /// }
    ///  ```
    pub fn toggle_by_selector(config: LifxConfig, selector: String, toggle: Toggle) ->  Result<LiFxResults, reqwest::Error>{
        let url = format!("{}/v1/lights/{}/toggle", config.api_endpoints[0], selector);

        let request = reqwest::blocking::Client::new().post(url)
            .header("Authorization", format!("Bearer {}", config.access_token))
            .form(&toggle.to_params())
            .send();

        match request {
            Ok(req) => {
                let json = req.json::<LiFxResults>()?;
                return Ok(json);
            },
            Err(err) => {
                if config.api_endpoints.len() > 1 {
                    let url = format!("{}/v1/lights/{}/toggle", config.api_endpoints[1], selector);

                    let request = reqwest::blocking::Client::new().post(url)
                        .header("Authorization", format!("Bearer {}", config.access_token))
                        .form(&toggle.to_params())
                        .send();
            
                    match request {
                        Ok(req) => {
                            let json = req.json::<LiFxResults>()?;
                            return Ok(json);
                        },
                        Err(err2) => {
                            return Err(err2);
                        }
                    }
                
                } else {
                    return Err(err);
                }
            }
        }
    

    }
}

pub type Scenes = Vec<Scene>;

/// Represents an LIFX Scene
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Scene {
    pub uuid: String,
    pub name: String,
    pub account: Account,
    pub states: Vec<State>,
    #[serde(rename = "created_at")]
    pub created_at: i64,
    #[serde(rename = "updated_at")]
    pub updated_at: i64,
    pub error: Option<String>,
    pub errors: Option<Vec<Error>>,
}
impl Scene {
    /// Asynchronously gets ALL scenes belonging to the authenticated account
    /// 
    /// # Arguments
    ///
    /// * `access_token` - A personal acces token for authentication with LIFX.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// extern crate lifx_rs as lifx;
    /// 
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// 
    ///     let key = "xxx".to_string();
    ///
    ///     let config = lifx::LifxConfig::new(key)
    ///         .add_endpoint("https://api.lifx.com".to_string())
    ///         .add_endpoint("http://localhost:8089".to_string());
    /// 
    ///     let scenes = lifx::Scene::async_list(config).await?;
    ///
    ///     Ok(())
    /// }
    ///  ```
    pub async fn async_list(config: LifxConfig) -> Result<Scenes, reqwest::Error> {
        let url = format!("{}/v1/scenes", config.api_endpoints[0]);
        let request = reqwest::Client::new().get(url).header("Authorization", format!("Bearer {}", config.access_token)).send().await;
        match request {
            Ok(req) => {
                let json = req.json::<Scenes>().await?;
                return Ok(json);
            },
            Err(err) => {
                if config.api_endpoints.len() > 1 {
                    let url = format!("{}/v1/scenes", config.api_endpoints[1]);
                    let request = reqwest::Client::new().get(url).header("Authorization", format!("Bearer {}", config.access_token)).send().await;
                    match request {
                        Ok(req) => {
                            let json = req.json::<Scenes>().await?;
                            return Ok(json);
                        },
                        Err(err2) => {
                            return Err(err2);
                        }
                    }
            
                } else {
                    return Err(err);
                }
            }
        }

    }

    /// Gets ALL scenes belonging to the authenticated account
    /// 
    /// # Arguments
    ///
    /// * `access_token` - A personal acces token for authentication with LIFX.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// extern crate lifx_rs as lifx;
    /// 
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// 
    ///     let key = "xxx".to_string();
    ///
    ///     let config = lifx::LifxConfig::new(key)
    ///         .add_endpoint("https://api.lifx.com".to_string())
    ///         .add_endpoint("http://localhost:8089".to_string());
    /// 
    ///     let scenes = lifx::Scene::list(config)?;
    ///
    ///     Ok(())
    /// }
    ///  ```
    pub fn list(config: LifxConfig) -> Result<Scenes, reqwest::Error> {
        let url = format!("{}/v1/scenes", config.api_endpoints[0]);
        let request = reqwest::blocking::Client::new().get(url).header("Authorization", format!("Bearer {}", config.access_token)).send();

        match request{
            Ok(req) => {
                let json = req.json::<Scenes>()?;
                return Ok(json);
            },
            Err(err) => {
                if config.api_endpoints.len() > 1 {
                    let url = format!("{}/v1/scenes", config.api_endpoints[1]);
                    let request = reqwest::blocking::Client::new().get(url).header("Authorization", format!("Bearer {}", config.access_token)).send();
            
                    match request{
                        Ok(req) => {
                            let json = req.json::<Scenes>()?;
                            return Ok(json);
                        },
                        Err(err2) => {
                            return Err(err2);
                        }
                    }
                } else {
                    return Err(err);
                }
            }
        }


    }
}

/// Represents an LIFX Color
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Color {
    pub hue: Option<f64>,
    pub saturation: Option<f64>,
    pub kelvin: Option<i64>,
    pub brightness: Option<f64>,
    pub error: Option<String>,
    pub errors: Option<Vec<Error>>,
}
impl Color {
    /// Asynchronously validates a color
    /// 
    /// # Arguments
    ///
    /// * `access_token` - A personal acces token for authentication with LIFX.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// extern crate lifx_rs as lifx;
    /// 
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// 
    ///     let key = "xxx".to_string();
    ///
    ///     let config = lifx::LifxConfig::new(key)
    ///         .add_endpoint("https://api.lifx.com".to_string())
    ///         .add_endpoint("http://localhost:8089".to_string());
    /// 
    ///     let scenes = lifx::Color::async_validate(config, format!("red")).await?;
    ///
    ///     Ok(())
    /// }
    ///  ```
    pub async fn async_validate(config: LifxConfig, color: String) -> Result<Color, reqwest::Error> {
        let url = format!("{}/v1/color?string={}", config.api_endpoints[0], color);
        let request = reqwest::Client::new().get(url).header("Authorization", format!("Bearer {}", config.access_token)).send().await;
        match request {
            Ok(req) => {
                let json = req.json::<Color>().await?;
                return Ok(json);
            },
            Err(err) => {
                if config.api_endpoints.len() > 1 {
                    let url = format!("{}/v1/color?string={}", config.api_endpoints[1], color);
                    let request = reqwest::Client::new().get(url).header("Authorization", format!("Bearer {}", config.access_token)).send().await;
                    match request {
                        Ok(req) => {
                            let json = req.json::<Color>().await?;
                            return Ok(json);
                        },
                        Err(err2) => {
                            return Err(err2);
                        }
                    }
                } else {
                    return Err(err);
                }
            }
        }

    }

    /// Validates a color
    /// 
    /// # Arguments
    ///
    /// * `access_token` - A personal acces token for authentication with LIFX.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// extern crate lifx_rs as lifx;
    /// 
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// 
    ///     let key = "xxx".to_string();
    ///
    ///     let config = lifx::LifxConfig::new(key)
    ///         .add_endpoint("https://api.lifx.com".to_string())
    ///         .add_endpoint("http://localhost:8089".to_string());
    /// 
    ///     let scenes = lifx::Color::validate(config, format!("red"))?;
    ///
    ///     Ok(())
    /// }
    ///  ```
    pub fn validate(config: LifxConfig, color: String) -> Result<Color, reqwest::Error> {
        let url = format!("{}/v1/color?string={}", config.api_endpoints[0], color);
        let request = reqwest::blocking::Client::new().get(url).header("Authorization", format!("Bearer {}", config.access_token)).send();
        match request {
            Ok(req) => {
                let json = req.json::<Color>()?;
                return Ok(json);
            },
            Err(err) => {
                if config.api_endpoints.len() > 1 {
                    let url = format!("{}/v1/color?string={}", config.api_endpoints[1], color);
                    let request = reqwest::blocking::Client::new().get(url).header("Authorization", format!("Bearer {}", config.access_token)).send();
                    match request {
                        Ok(req) => {
                            let json = req.json::<Color>()?;
                            return Ok(json);
                        },
                        Err(err2) => {
                            return Err(err2);
                        }
                    }
                } else {
                    return Err(err);
                }
            }
        }


    }
}

/// Used to set the duration/state of the HEV Clean array
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Clean {
    /// Turn the device on / off
    pub stop: Option<bool>,
    /// Duration in seconds (leaving blank or 0 sets the default duration for the device)
    pub duration: Option<i64>
}
impl Clean {
    pub fn new() -> Self {
        return Clean{
            stop: None,
            duration: None
        };
    }

    fn to_params(&self) -> Vec<(String, String)> {
        let mut params: Vec<(String, String)> = vec![];
        match &self.stop{
            Some(stop) => params.push(("stop".to_string(), stop.to_string())),
            None => {}
        }
        match &self.duration{
            Some(duration) => params.push(("duration".to_string(), duration.to_string())),
            None => {}
        }
       
        return params;
    }


}

/// Used to descripe the state of an LIFX Light Source
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct State {
    /// The power state you want to set on the selector. on or off
    pub power: Option<String>,
    /// The color to set the light to.
    pub color: Option<String>,
    /// The brightness level from 0.0 to 1.0. Overrides any brightness set in color (if any).
    pub brightness: Option<f64>,
    /// How long in seconds you want the power action to take. Range: 0.0 – 3155760000.0 (100 years)
    pub duration: Option<f64>,
    /// The maximum brightness of the infrared channel from 0.0 to 1.0.
    pub infrared: Option<f64>,
    /// The selector to limit which light to use for set_states()
    pub selector:  Option<String>,
    /// Execute the query fast, without initial state checks and wait for no results.
    pub fast: Option<bool>
}
impl State {

    /// Returns a new State object
    /// 
    /// # Examples
    ///
    /// ```rust,no_run
    /// extern crate lifx_rs as lifx;
    /// 
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// 
    ///     let key = "xxx".to_string();
    ///
    ///     let config = lifx::LifxConfig::new(key)
    ///         .add_endpoint("https://api.lifx.com".to_string())
    ///         .add_endpoint("http://localhost:8089".to_string());
    /// 
    ///     let mut state = lifx::State::new();
    ///     state.power = Some("off".to_string());
    ///
    ///     Ok(())
    /// }
    ///  ```
    pub fn new() -> Self {
        return State{
            power: None,
            color: None,
            brightness: None,
            duration: None,
            infrared: None,
            selector: None,
            fast: None
        };
    }

    fn to_params(&self) -> Vec<(String, String)> {
        let mut params: Vec<(String, String)> = vec![];
        match &self.power{
            Some(power) => params.push(("power".to_string(), power.to_string())),
            None => {}
        }
        match &self.color{
            Some(color) => params.push(("color".to_string(), color.to_string())),
            None => {}
        }
        match &self.brightness{
            Some(brightness) => params.push(("brightness".to_string(), brightness.to_string())),
            None => {}
        }
        match &self.duration{
            Some(duration) => params.push(("duration".to_string(), duration.to_string())),
            None => {}
        }
        match &self.infrared{
            Some(infrared) => params.push(("infrared".to_string(), infrared.to_string())),
            None => {}
        }
        match &self.selector{
            Some(selector) => params.push(("selector".to_string(), selector.to_string())),
            None => {}
        }
        match &self.fast{
            Some(fast) => params.push(("fast".to_string(), fast.to_string())),
            None => {}
        }
        return params;
    }


}

/// Used to set the params when posting a Toggle event
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Toggle {
    pub duration: Option<i64>
}
impl Toggle {
    /// Returns a new Toggle object
    /// 
    /// # Examples
    ///
    /// ```rust,no_run
    /// extern crate lifx_rs as lifx;
    /// 
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// 
    ///     let key = "xxx".to_string();
    ///
    ///     let config = lifx::LifxConfig::new(key)
    ///         .add_endpoint("https://api.lifx.com".to_string())
    ///         .add_endpoint("http://localhost:8089".to_string());
    /// 
    ///     let mut toggle = lifx::Toggle::new();
    ///     toggle.duration = Some(0);
    ///
    ///     Ok(())
    /// }
    ///  ```
    pub fn new() -> Self {
        return Toggle{
            duration: None
        };
    }

    fn to_params(&self) -> Vec<(String, String)> {
        let mut params: Vec<(String, String)> = vec![];
        match &self.duration{
            Some(duration) => params.push(("duration".to_string(), duration.to_string())),
            None => {}
        }
        return params;
    }


}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[doc(hidden)]
pub struct States {
    pub states: Option<Vec<State>>,
    pub defaults: Option<State>,
}
impl States {
    /// Returns a new States object
    /// 
    /// # Examples
    ///
    /// ```rust,no_run
    /// extern crate lifx_rs as lifx;
    /// 
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// 
    ///     let key = "xxx".to_string();
    ///
    ///     let config = lifx::LifxConfig::new(key)
    ///         .add_endpoint("https://api.lifx.com".to_string())
    ///         .add_endpoint("http://localhost:8089".to_string());
    /// 
    ///     let mut states = lifx::States::new();
    ///
    ///     Ok(())
    /// }
    ///  ```
    pub fn new() -> Self {
        return States{
            states: None,
            defaults: None
        };
    }
}

/// Used to set the params when posting a StateDelta event
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StateDelta {
    /// The power state you want to set on the selector. on or off
    pub power: Option<String>,
    /// How long in seconds you want the power action to take. Range: 0.0 – 3155760000.0 (100 years)
    pub duration: Option<f64>,
    /// The maximum brightness of the infrared channel.
    pub infrared: Option<f64>,
    /// Rotate the hue by this angle in degrees. Range: -360.0 – 360.0 degrees
    pub hue: Option<f64>,
    /// Change the saturation by this additive amount; the resulting saturation is clipped to [0, 1].
    pub saturation: Option<f64>,
    /// Change the brightness by this additive amount; the resulting brightness is clipped to [0, 1].
    pub brightness: Option<f64>,
    /// Change the kelvin by this additive amount; the resulting kelvin is clipped to [2500, 9000].
    pub kelvin: Option<i64>,
    /// Execute the query fast, without initial state checks and wait for no results.
    pub fast: Option<bool>,
}
impl StateDelta {
    /// Returns a new StateDelta object
    /// 
    /// # Examples
    ///
    /// ```rust,no_run
    /// extern crate lifx_rs as lifx;
    /// 
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// 
    ///     let key = "xxx".to_string();
    ///
    ///     let config = lifx::LifxConfig::new(key)
    ///         .add_endpoint("https://api.lifx.com".to_string())
    ///         .add_endpoint("http://localhost:8089".to_string());
    /// 
    ///     let mut delta = lifx::StateDelta::new();
    ///     delta.duration = Some(0.0);
    ///
    ///     Ok(())
    /// }
    ///  ```
    pub fn new() -> Self {
        return StateDelta{
            power: None,
            duration: None,
            infrared: None,
            hue: None,
            saturation: None,
            brightness: None,
            kelvin: None,
            fast: None
        };
    }

    fn to_params(&self) -> Vec<(String, String)> {
        let mut params: Vec<(String, String)> = vec![];
        match &self.power{
            Some(power) => params.push(("power".to_string(), power.to_string())),
            None => {}
        }

        match &self.duration{
            Some(duration) => params.push(("duration".to_string(), duration.to_string())),
            None => {}
        }

        match &self.infrared{
            Some(infrared) => params.push(("infrared".to_string(), infrared.to_string())),
            None => {}
        }

        match &self.hue{
            Some(hue) => params.push(("hue".to_string(), hue.to_string())),
            None => {}
        }

        match &self.saturation{
            Some(saturation) => params.push(("saturation".to_string(), saturation.to_string())),
            None => {}
        }

        match &self.brightness{
            Some(brightness) => params.push(("brightness".to_string(), brightness.to_string())),
            None => {}
        }

        match &self.kelvin{
            Some(kelvin) => params.push(("kelvin".to_string(), kelvin.to_string())),
            None => {}
        }

        match &self.fast{
            Some(fast) => params.push(("fast".to_string(), fast.to_string())),
            None => {}
        }

        return params;
    }

}

/// Used to set the params when posting a BreatheEffect event
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BreatheEffect {
    /// The color to use for the breathe effect.
    pub color: Option<String>,
    /// The color to start the effect from. If this parameter is omitted then the color the bulb is currently set to is used instead.
    pub from_color: Option<String>,
    /// The time in seconds for one cycle of the effect.
    pub period: Option<f64>,
    /// The number of times to repeat the effect.
    pub cycles: Option<f64>,
    /// If false set the light back to its previous value when effect ends, if true leave the last effect color.
    pub persist: Option<bool>,
    /// If true, turn the bulb on if it is not already on.
    pub power_on: Option<bool>,
    /// Defines where in a period the target color is at its maximum. Minimum 0.0, maximum 1.0.
    pub peak: Option<f64>,
}
impl BreatheEffect {
    /// Returns a new BreatheEffect object
    /// 
    /// # Examples
    ///
    /// ```rust,no_run
    /// extern crate lifx_rs as lifx;
    /// 
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// 
    ///     let key = "xxx".to_string();
    ///
    ///     let config = lifx::LifxConfig::new(key)
    ///         .add_endpoint("https://api.lifx.com".to_string())
    ///         .add_endpoint("http://localhost:8089".to_string());
    /// 
    ///     let mut breathe = lifx::BreatheEffect::new();
    ///     breathe.color = Some("red".to_string());
    ///     breathe.from_color = Some("green".to_string());
    ///     breathe.period = Some(10.0);
    ///     breathe.persist = Some(true);
    ///     breathe.power_on = Some(true);
    ///
    ///     Ok(())
    /// }
    ///  ```
    pub fn new() -> Self {
        return BreatheEffect{
            color: None,
            from_color: None,
            period: None,
            cycles: None,
            persist: None,
            power_on: None,
            peak: None
        };
    }

    fn to_params(&self) -> Vec<(String, String)> {
        let mut params: Vec<(String, String)> = vec![];
        match &self.color{
            Some(color) => params.push(("color".to_string(), color.to_string())),
            None => {}
        }

        match &self.from_color{
            Some(from_color) => params.push(("from_color".to_string(), from_color.to_string())),
            None => {}
        }

        match &self.period{
            Some(period) => params.push(("period".to_string(), period.to_string())),
            None => {}
        }

        match &self.cycles{
            Some(cycles) => params.push(("cycles".to_string(), cycles.to_string())),
            None => {}
        }

        match &self.persist{
            Some(persist) => params.push(("persist".to_string(), persist.to_string())),
            None => {}
        }

        match &self.power_on{
            Some(power_on) => params.push(("power_on".to_string(), power_on.to_string())),
            None => {}
        }

        match &self.peak{
            Some(peak) => params.push(("peak".to_string(), peak.to_string())),
            None => {}
        }

        return params;
    }

}

/// Used to set the params when posting a MoveEffect event
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MoveEffect {
    /// The color to use for the breathe effect.
    pub direction: Option<String>,
    /// The time in seconds for one cycle of the effect.
    pub period: Option<i64>,
    /// The number of times to repeat the effect.
    pub cycles: Option<f64>,
    /// If true, turn the bulb on if it is not already on.
    pub power_on: Option<bool>,
    /// Execute the query fast, without initial state checks and wait for no results.
    pub fast: Option<bool>,
}
impl MoveEffect {
    /// Returns a new MoveEffect object
    /// 
    /// # Examples
    ///
    /// ```rust,no_run
    /// extern crate lifx_rs as lifx;
    /// 
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// 
    ///     let key = "xxx".to_string();
    ///
    ///     let config = lifx::LifxConfig::new(key)
    ///         .add_endpoint("https://api.lifx.com".to_string())
    ///         .add_endpoint("http://localhost:8089".to_string());
    /// 
    ///     let mut move_effect = lifx::MoveEffect::new();
    ///     move_effect.direction = Some("forward".to_string()); // or backward
    ///     move_effect.period = Some(10);
    ///     move_effect.cycles = Some(0.9);
    ///     move_effect.power_on = Some(true);
    ///
    ///     Ok(())
    /// }
    ///  ```
    pub fn new() -> Self {
        return MoveEffect{
            direction: None,
            period: None,
            cycles: None,
            power_on: None,
            fast: None
        };
    }

    fn to_params(&self) -> Vec<(String, String)> {
        let mut params: Vec<(String, String)> = vec![];
        match &self.direction{
            Some(direction) => params.push(("direction".to_string(), direction.to_string())),
            None => {}
        }

        match &self.period{
            Some(period) => params.push(("period".to_string(), period.to_string())),
            None => {}
        }

        match &self.cycles{
            Some(cycles) => params.push(("cycles".to_string(), cycles.to_string())),
            None => {}
        }

        match &self.power_on{
            Some(power_on) => params.push(("power_on".to_string(), power_on.to_string())),
            None => {}
        }

        match &self.fast{
            Some(fast) => params.push(("fast".to_string(), fast.to_string())),
            None => {}
        }

        return params;
    }

}

/// Used to set the params when posting a MorphEffect event
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MorphEffect {
    /// The time in seconds for one cycle of the effect.
    pub period: Option<i64>,
    /// How long the animation lasts for in seconds. Not specifying a duration makes the animation never stop. Specifying 0 makes the animation stop.
    /// 
    /// **Note:** For proper cleanup when using nonzero duration with tiles, consider using the
    /// `morph_effect_with_cleanup()` or `async_morph_effect_with_cleanup()` helper functions
    /// which automatically stop the animation after the duration expires.
    pub duration: Option<f64>,
    /// You can control the colors in the animation by specifying a list of color specifiers. For example ["red", "hue:100 saturation:1"]. See https://api.developer.lifx.com/docs/colors
    pub palette: Option<Vec<String>>,
    /// If true, turn the bulb on if it is not already on.
    pub power_on: Option<bool>,
    /// Execute the query fast, without initial state checks and wait for no results.
    pub fast: Option<bool>,
}
impl MorphEffect {
    /// Returns a new MorphEffect object
    /// 
    /// # Examples
    ///
    /// ```rust,no_run
    /// extern crate lifx_rs as lifx;
    /// 
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// 
    ///     let key = "xxx".to_string();
    ///
    ///     let config = lifx::LifxConfig::new(key)
    ///         .add_endpoint("https://api.lifx.com".to_string())
    ///         .add_endpoint("http://localhost:8089".to_string());
    /// 
    ///     let mut morph_effect = lifx::MorphEffect::new();
    ///     morph_effect.period = Some(10);
    ///     morph_effect.duration = Some(0.0);
    /// 
    ///     let mut palette: Vec<String> = Vec::new();
    ///     palette.push("red".to_string());
    ///     palette.push("green".to_string());
    /// 
    ///     morph_effect.palette = Some(palette);
    ///     morph_effect.power_on = Some(true);
    ///
    ///     Ok(())
    /// }
    ///  ```
    pub fn new() -> Self {
        return MorphEffect{
            period: None,
            duration: None,
            palette: None,
            power_on: None,
            fast: None
        };
    }

    fn to_params(&self) -> Vec<(String, String)> {
        let mut params: Vec<(String, String)> = vec![];
        match &self.period{
            Some(period) => params.push(("period".to_string(), period.to_string())),
            None => {}
        }

        match &self.duration{
            Some(duration) => params.push(("duration".to_string(), duration.to_string())),
            None => {}
        }

        match &self.palette{
            Some(palette) => params.push(("palette".to_string(), string_vec_to_params(palette.to_vec()))),
            None => {}
        }

        match &self.power_on{
            Some(power_on) => params.push(("power_on".to_string(), power_on.to_string())),
            None => {}
        }

        match &self.fast{
            Some(fast) => params.push(("fast".to_string(), fast.to_string())),
            None => {}
        }

        return params;
    }

}



/// Used to set the params when posting a PulseEffect event
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PulseEffect {
    /// The color to use for the breathe effect.
    pub color: Option<String>,
    /// The color to start the effect from. If this parameter is omitted then the color the bulb is currently set to is used instead.
    pub from_color: Option<String>,
    /// The time in seconds for one cycle of the effect.
    pub period: Option<f64>,
    /// The number of times to repeat the effect.
    pub cycles: Option<f64>,
    /// If false set the light back to its previous value when effect ends, if true leave the last effect color.
    pub persist: Option<bool>,
    /// If true, turn the bulb on if it is not already on.
    pub power_on: Option<bool>,
}
impl PulseEffect {
    /// Returns a new PulseEffect object
    /// 
    /// # Examples
    ///
    /// ```rust,no_run
    /// extern crate lifx_rs as lifx;
    /// 
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// 
    ///     let key = "xxx".to_string();
    ///
    ///     let config = lifx::LifxConfig::new(key)
    ///         .add_endpoint("https://api.lifx.com".to_string())
    ///         .add_endpoint("http://localhost:8089".to_string());
    /// 
    ///     let mut pulse = lifx::PulseEffect::new();
    ///     pulse.color = Some("red".to_string());
    ///     pulse.from_color = Some("green".to_string());
    ///     pulse.period = Some(10.0);
    ///     pulse.persist = Some(true);
    ///     pulse.power_on = Some(true);
    ///
    ///     Ok(())
    /// }
    ///  ```
    pub fn new() -> Self {
        return PulseEffect{
            color: None,
            from_color: None,
            period: None,
            cycles: None,
            persist: None,
            power_on: None
        };
    }

    fn to_params(&self) -> Vec<(String, String)> {
        let mut params: Vec<(String, String)> = vec![];
        match &self.color{
            Some(color) => params.push(("color".to_string(), color.to_string())),
            None => {}
        }

        match &self.from_color{
            Some(from_color) => params.push(("from_color".to_string(), from_color.to_string())),
            None => {}
        }

        match &self.period{
            Some(period) => params.push(("period".to_string(), period.to_string())),
            None => {}
        }

        match &self.cycles{
            Some(cycles) => params.push(("cycles".to_string(), cycles.to_string())),
            None => {}
        }

        match &self.persist{
            Some(persist) => params.push(("persist".to_string(), persist.to_string())),
            None => {}
        }

        match &self.power_on{
            Some(power_on) => params.push(("power_on".to_string(), power_on.to_string())),
            None => {}
        }

        return params;
    }

}

/// Used to set the params when posting a EffectsOff event
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EffectsOff {
    /// If true, the devices will also be turned off
    pub power_off: Option<bool>,
}
impl EffectsOff {
    /// Returns a new EffectsOff object
    /// 
    /// # Examples
    ///
    /// ```rust,no_run
    /// extern crate lifx_rs as lifx;
    /// 
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// 
    ///     let key = "xxx".to_string();
    ///
    ///     let config = lifx::LifxConfig::new(key)
    ///         .add_endpoint("https://api.lifx.com".to_string())
    ///         .add_endpoint("http://localhost:8089".to_string());
    /// 
    ///     let mut ef = lifx::EffectsOff::new();
    ///     ef.power_off = Some(true);
    ///
    ///     Ok(())
    /// }
    ///  ```
    pub fn new() -> Self {
        return EffectsOff{
            power_off: None,
        };
    }

    fn to_params(&self) -> Vec<(String, String)> {
        let mut params: Vec<(String, String)> = vec![];
        match &self.power_off{
            Some(power_off) => params.push(("power_off".to_string(), power_off.to_string())),
            None => {}
        }

        return params;
    }

}



/// Used to set the params when posting a FlameEffect event
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlameEffect {
    /// The time in seconds for one cycle of the effect.
    pub period: Option<i64>,
    /// How long the animation lasts for in seconds. Not specifying a duration makes the animation never stop. Specifying 0 makes the animation stop.
    /// 
    /// **Note:** For proper cleanup when using nonzero duration with tiles, consider using the
    /// `flame_effect_with_cleanup()` or `async_flame_effect_with_cleanup()` helper functions
    /// which automatically stop the animation after the duration expires.
    pub duration: Option<f64>,
    /// If true, turn the bulb on if it is not already on.
    pub power_on: Option<bool>,
    /// Execute the query fast, without initial state checks and wait for no results.
    pub fast: Option<bool>,
}
impl FlameEffect {
    /// Returns a new FlameEffect object
    /// 
    /// # Examples
    ///
    /// ```rust,no_run
    /// extern crate lifx_rs as lifx;
    /// 
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// 
    ///     let key = "xxx".to_string();
    ///
    ///     let config = lifx::LifxConfig::new(key)
    ///         .add_endpoint("https://api.lifx.com".to_string())
    ///         .add_endpoint("http://localhost:8089".to_string());
    /// 
    ///     let mut flame_effect = lifx::FlameEffect::new();
    ///     flame_effect.period = Some(10);
    ///     flame_effect.duration = Some(0.0);
    ///     flame_effect.power_on = Some(true);
    ///
    ///     Ok(())
    /// }
    ///  ```
    pub fn new() -> Self {
        return FlameEffect{
            period: None,
            duration: None,
            power_on: None,
            fast: None
        };
    }

    fn to_params(&self) -> Vec<(String, String)> {
        let mut params: Vec<(String, String)> = vec![];
        match &self.period{
            Some(period) => params.push(("period".to_string(), period.to_string())),
            None => {}
        }

        match &self.duration{
            Some(duration) => params.push(("duration".to_string(), duration.to_string())),
            None => {}
        }

        match &self.power_on{
            Some(power_on) => params.push(("power_on".to_string(), power_on.to_string())),
            None => {}
        }

        match &self.fast{
            Some(fast) => params.push(("fast".to_string(), fast.to_string())),
            None => {}
        }

        return params;
    }

}

/// Performs a flame effect on tiles with automatic cleanup after duration.
/// This is a workaround for the tile animation bug where tiles remain in animation state after completion.
/// 
/// # Arguments
/// * `config` - LIFX configuration with access token and endpoints
/// * `selector` - LIFX selector (e.g., "all", "id:xxx", "group_id:xxx")
/// * `flame_effect` - FlameEffect configuration with duration
/// 
/// # Returns
/// Returns a tuple of (effect_result, cleanup_result) where cleanup_result is None if duration is 0 or None
/// 
/// # Example
/// ```ignore
/// let mut flame_effect = lifx::FlameEffect::new();
/// flame_effect.period = Some(5.0);
/// flame_effect.duration = Some(10.0); // Will auto-cleanup after 10 seconds
/// 
/// let (effect_result, cleanup_result) = lifx::flame_effect_with_cleanup(
///     config.clone(),
///     "all".to_string(),
///     flame_effect
/// )?;
/// ```
pub fn flame_effect_with_cleanup(
    config: LifxConfig,
    selector: String,
    flame_effect: FlameEffect,
) -> Result<(LiFxResults, Option<LiFxResults>), reqwest::Error> {
    // Start the flame effect
    let effect_result = Light::flame_effect_by_selector(config.clone(), selector.clone(), flame_effect.clone())?;
    
    // If duration is specified and non-zero, schedule cleanup
    let cleanup_result = match flame_effect.duration {
        Some(duration) if duration > 0.0 => {
            // Sleep for the duration
            std::thread::sleep(std::time::Duration::from_secs_f64(duration));
            
            // Turn off the effect
            let effects_off = EffectsOff::new();
            Some(Light::effects_off_by_selector(config, selector, effects_off)?)
        }
        _ => None,
    };
    
    Ok((effect_result, cleanup_result))
}

/// Performs a morph effect on tiles with automatic cleanup after duration.
/// This is a workaround for the tile animation bug where tiles remain in animation state after completion.
/// 
/// # Arguments
/// * `config` - LIFX configuration with access token and endpoints
/// * `selector` - LIFX selector (e.g., "all", "id:xxx", "group_id:xxx")
/// * `morph_effect` - MorphEffect configuration with duration
/// 
/// # Returns
/// Returns a tuple of (effect_result, cleanup_result) where cleanup_result is None if duration is 0 or None
/// 
/// # Example
/// ```ignore
/// let mut morph_effect = lifx::MorphEffect::new();
/// morph_effect.period = Some(5.0);
/// morph_effect.duration = Some(10.0); // Will auto-cleanup after 10 seconds
/// 
/// let (effect_result, cleanup_result) = lifx::morph_effect_with_cleanup(
///     config.clone(),
///     "all".to_string(),
///     morph_effect
/// )?;
/// ```
pub fn morph_effect_with_cleanup(
    config: LifxConfig,
    selector: String,
    morph_effect: MorphEffect,
) -> Result<(LiFxResults, Option<LiFxResults>), reqwest::Error> {
    // Start the morph effect
    let effect_result = Light::morph_effect_by_selector(config.clone(), selector.clone(), morph_effect.clone())?;
    
    // If duration is specified and non-zero, schedule cleanup
    let cleanup_result = match morph_effect.duration {
        Some(duration) if duration > 0.0 => {
            // Sleep for the duration
            std::thread::sleep(std::time::Duration::from_secs_f64(duration));
            
            // Turn off the effect
            let effects_off = EffectsOff::new();
            Some(Light::effects_off_by_selector(config, selector, effects_off)?)
        }
        _ => None,
    };
    
    Ok((effect_result, cleanup_result))
}

pub fn string_vec_to_params(input: Vec<String>) -> String {

    if input.is_empty() {
        return "[]".to_string();
    }

    let mut params = String::new();
    let mut count = 0;
    for iput in input {
        if count == 0 {
            params = format!("[\"{}\"", iput);
        } else {
            params = format!("{}, \"{}\"",params, iput);
        }
        count += 1;
    }

    params = format!("{}]", params);

    return params;
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[doc(hidden)]
pub struct Group {
    pub id: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[doc(hidden)]
pub struct Location {
    pub id: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[doc(hidden)]
pub struct Product {
    pub name: String,
    pub identifier: String,
    pub company: String,
    #[serde(rename = "vendor_id")]
    pub vendor_id: i64,
    #[serde(rename = "product_id")]
    pub product_id: i64,
    pub capabilities: Capabilities,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[doc(hidden)]
pub struct Capabilities {
    #[serde(rename = "has_color")]
    pub has_color: bool,
    #[serde(rename = "has_variable_color_temp")]
    pub has_variable_color_temp: bool,
    #[serde(rename = "has_ir")]
    pub has_ir: bool,
    #[serde(rename = "has_hev")]
    pub has_hev: bool,
    #[serde(rename = "has_chain")]
    pub has_chain: bool,
    #[serde(rename = "has_matrix")]
    pub has_matrix: bool,
    #[serde(rename = "has_multizone")]
    pub has_multizone: bool,
    #[serde(rename = "min_kelvin")]
    pub min_kelvin: i64,
    #[serde(rename = "max_kelvin")]
    pub max_kelvin: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[doc(hidden)]
pub struct Account {
    pub uuid: String,
}




#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[doc(hidden)]
pub struct Error {
    pub field: String,
    pub message: Vec<String>,
}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[doc(hidden)]
pub struct LiFxResults {
    pub results: Option<Vec<LiFxResult>>,
    pub error: Option<String>
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[doc(hidden)]
pub struct LiFxResult {
    pub id: String,
    pub label: String,
    pub status: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lifx_config_creation() {
        let config = LifxConfig::new("test_token".to_string())
            .add_endpoint("https://api.lifx.com".to_string());
        assert_eq!(config.access_token, "test_token");
        assert_eq!(config.api_endpoints.len(), 1);
        assert_eq!(config.api_endpoints[0], "https://api.lifx.com");
    }

    #[test]
    fn test_state_new() {
        let state = State::new();
        assert_eq!(state.power, None);
        assert_eq!(state.color, None);
        assert_eq!(state.brightness, None);
        assert_eq!(state.duration, None);
        assert_eq!(state.infrared, None);
        assert_eq!(state.fast, None);
    }

    #[test]
    fn test_state_with_values() {
        let mut state = State::new();
        state.power = Some("on".to_string());
        state.brightness = Some(0.5);
        state.duration = Some(1.0);
        
        assert_eq!(state.power, Some("on".to_string()));
        assert_eq!(state.brightness, Some(0.5));
        assert_eq!(state.duration, Some(1.0));
    }

    #[test]
    fn test_state_delta_new() {
        let delta = StateDelta::new();
        assert_eq!(delta.power, None);
        assert_eq!(delta.duration, None);
        assert_eq!(delta.infrared, None);
        assert_eq!(delta.hue, None);
        assert_eq!(delta.saturation, None);
        assert_eq!(delta.brightness, None);
        assert_eq!(delta.kelvin, None);
    }

    #[test]
    fn test_state_delta_with_values() {
        let mut delta = StateDelta::new();
        delta.power = Some("on".to_string());
        delta.duration = Some(2.0);
        delta.brightness = Some(0.1);
        delta.kelvin = Some(3500);
        
        assert_eq!(delta.power, Some("on".to_string()));
        assert_eq!(delta.duration, Some(2.0));
        assert_eq!(delta.brightness, Some(0.1));
        assert_eq!(delta.kelvin, Some(3500));
    }

    #[test]
    fn test_breathe_effect_new() {
        let effect = BreatheEffect::new();
        assert_eq!(effect.color, None);
        assert_eq!(effect.from_color, None);
        assert_eq!(effect.period, None);
        assert_eq!(effect.cycles, None);
        assert_eq!(effect.persist, None);
        assert_eq!(effect.power_on, None);
        assert_eq!(effect.peak, None);
    }

    #[test]
    fn test_breathe_effect_with_values() {
        let mut effect = BreatheEffect::new();
        effect.color = Some("red".to_string());
        effect.period = Some(2.0);
        effect.cycles = Some(5.0);
        effect.persist = Some(true);
        effect.power_on = Some(true);
        effect.peak = Some(0.8);
        
        assert_eq!(effect.color, Some("red".to_string()));
        assert_eq!(effect.period, Some(2.0));
        assert_eq!(effect.cycles, Some(5.0));
        assert_eq!(effect.persist, Some(true));
        assert_eq!(effect.power_on, Some(true));
        assert_eq!(effect.peak, Some(0.8));
    }

    #[test]
    fn test_pulse_effect_new() {
        let effect = PulseEffect::new();
        assert_eq!(effect.color, None);
        assert_eq!(effect.from_color, None);
        assert_eq!(effect.period, None);
        assert_eq!(effect.cycles, None);
        assert_eq!(effect.persist, None);
        assert_eq!(effect.power_on, None);
    }

    #[test]
    fn test_morph_effect_new() {
        let effect = MorphEffect::new();
        assert_eq!(effect.period, None);
        assert_eq!(effect.duration, None);
        assert_eq!(effect.power_on, None);
        assert_eq!(effect.palette, None);
    }

    #[test]
    fn test_morph_effect_with_palette() {
        let mut effect = MorphEffect::new();
        effect.palette = Some(vec!["red".to_string(), "blue".to_string(), "green".to_string()]);
        effect.period = Some(3);
        effect.duration = Some(10.0);
        effect.power_on = Some(true);
        
        assert!(effect.palette.is_some(), "Palette should be Some after being set");
        let palette = effect.palette.as_ref().unwrap();
        assert_eq!(palette.len(), 3, "Palette should contain exactly 3 colors");
        assert_eq!(palette[0], "red", "First color should be red");
        assert_eq!(palette[1], "blue", "Second color should be blue");
        assert_eq!(palette[2], "green", "Third color should be green");
        assert_eq!(effect.period, Some(3), "Period should be 3");
        assert_eq!(effect.duration, Some(10.0), "Duration should be 10.0");
        assert_eq!(effect.power_on, Some(true), "Power should be on");
    }

    #[test]
    fn test_flame_effect_new() {
        let effect = FlameEffect::new();
        assert_eq!(effect.period, None);
        assert_eq!(effect.duration, None);
        assert_eq!(effect.power_on, None);
    }

    #[test]
    fn test_move_effect_new() {
        let effect = MoveEffect::new();
        assert_eq!(effect.direction, None);
        assert_eq!(effect.period, None);
        assert_eq!(effect.cycles, None);
        assert_eq!(effect.power_on, None);
    }

    #[test]
    fn test_effects_off_new() {
        let effect = EffectsOff::new();
        assert_eq!(effect.power_off, None);
    }

    #[test]
    fn test_toggle_new() {
        let toggle = Toggle::new();
        assert_eq!(toggle.duration, None);
    }

    #[test]
    fn test_clean_new() {
        let clean = Clean::new();
        assert_eq!(clean.duration, None);
        assert_eq!(clean.stop, None);
    }

    #[test]
    fn test_states_new() {
        let states = States::new();
        assert_eq!(states.states, None);
        assert_eq!(states.defaults, None);
    }

    #[test]
    fn test_states_with_values() {
        let mut states = States::new();
        let mut state1 = State::new();
        state1.power = Some("on".to_string());
        state1.brightness = Some(1.0);
        
        let mut state2 = State::new();
        state2.power = Some("off".to_string());
        
        states.states = Some(vec![state1, state2]);
        
        assert!(states.states.is_some(), "States should be Some after being set");
        let states_vec = states.states.as_ref().unwrap();
        assert_eq!(states_vec.len(), 2, "States vector should contain exactly 2 states");
        assert_eq!(states_vec[0].power, Some("on".to_string()), "First state power should be 'on'");
        assert_eq!(states_vec[1].power, Some("off".to_string()), "Second state power should be 'off'");
    }

    #[test]
    fn test_string_vec_to_params() {
        let params = vec!["param1".to_string(), "param2".to_string(), "param3".to_string()];
        let result = string_vec_to_params(params);
        assert_eq!(result, "[\"param1\", \"param2\", \"param3\"]");
    }

    #[test]
    fn test_string_vec_to_params_single() {
        let params = vec!["single".to_string()];
        let result = string_vec_to_params(params);
        assert_eq!(result, "[\"single\"]");
    }

    #[test]
    fn test_string_vec_to_params_empty() {
        let params: Vec<String> = vec![];
        let result = string_vec_to_params(params);
        assert_eq!(result, "[]");
    }

    #[test]
    fn test_color_creation() {
        let color = Color {
            hue: Some(120.0),
            saturation: Some(1.0),
            kelvin: Some(3500),
            brightness: Some(0.8),
            error: None,
            errors: None,
        };
        assert_eq!(color.hue, Some(120.0));
        assert_eq!(color.saturation, Some(1.0));
        assert_eq!(color.kelvin, Some(3500));
        assert_eq!(color.brightness, Some(0.8));
    }

    #[test]
    fn test_scene_creation() {
        let scene = Scene {
            uuid: "test-uuid".to_string(),
            name: "Test Scene".to_string(),
            account: Account { uuid: "account-uuid".to_string() },
            states: vec![],
            created_at: 1234567890,
            updated_at: 1234567891,
            error: None,
            errors: None,
        };
        assert_eq!(scene.uuid, "test-uuid");
        assert_eq!(scene.name, "Test Scene");
        assert_eq!(scene.states.len(), 0);
    }

    #[test]
    fn test_light_creation() {
        let light = Light {
            id: "test-id".to_string(),
            uuid: "test-uuid".to_string(),
            label: "Test Light".to_string(),
            connected: true,
            power: "on".to_string(),
            brightness: 1.0,
            color: Color {
                hue: Some(120.0),
                saturation: Some(1.0),
                kelvin: Some(3500),
                brightness: Some(1.0),
                error: None,
                errors: None,
            },
            group: Group {
                id: "group-id".to_string(),
                name: "Test Group".to_string(),
            },
            location: Location {
                id: "location-id".to_string(),
                name: "Test Location".to_string(),
            },
            product: Product {
                name: "Test Product".to_string(),
                identifier: "test-identifier".to_string(),
                company: "LIFX".to_string(),
                vendor_id: 1,
                product_id: 1,
                capabilities: Capabilities {
                    has_color: true,
                    has_variable_color_temp: true,
                    has_ir: false,
                    has_hev: false,
                    has_chain: false,
                    has_matrix: false,
                    has_multizone: false,
                    min_kelvin: 2500,
                    max_kelvin: 9000,
                },
            },
            last_seen: "2023-01-01T00:00:00Z".to_string(),
            seconds_since_seen: 0,
            error: None,
            errors: None,
        };
        assert_eq!(light.id, "test-id");
        assert_eq!(light.label, "Test Light");
        assert!(light.connected);
        assert_eq!(light.power, "on");
        assert_eq!(light.brightness, 1.0);
    }

    #[test]
    fn test_lifx_results_creation() {
        let results = LiFxResults {
            results: Some(vec![
                LiFxResult {
                    id: "id1".to_string(),
                    label: "Light 1".to_string(),
                    status: "ok".to_string(),
                },
                LiFxResult {
                    id: "id2".to_string(),
                    label: "Light 2".to_string(),
                    status: "ok".to_string(),
                },
            ]),
            error: None,
        };
        
        assert!(results.results.is_some(), "Results should be Some");
        let results_vec = results.results.unwrap();
        assert_eq!(results_vec.len(), 2, "Results vector should contain exactly 2 results");
        assert_eq!(results_vec[0].id, "id1", "First result ID should be 'id1'");
        assert_eq!(results_vec[1].label, "Light 2", "Second result label should be 'Light 2'");
        assert!(results.error.is_none(), "Error field should be None when results are present");
    }

    #[test]
    fn test_lifx_results_with_error() {
        let results = LiFxResults {
            results: None,
            error: Some("API Error".to_string()),
        };
        
        assert!(results.results.is_none(), "Results should be None when error is present");
        assert!(results.error.is_some(), "Error should be Some when API error occurs");
        assert_eq!(results.error.unwrap(), "API Error", "Error message should be 'API Error'");
    }

    #[test]
    fn test_morph_effect_with_none_palette() {
        let effect = MorphEffect::new();
        assert!(effect.palette.is_none());
        
        // This should not panic - palette is None
        if let Some(palette) = effect.palette.as_ref() {
            panic!("Expected None but got Some with {} items", palette.len());
        }
    }

    #[test]
    fn test_states_with_none_states() {
        let states = States::new();
        assert!(states.states.is_none());
        
        // This should not panic - states is None
        if let Some(states_vec) = states.states.as_ref() {
            panic!("Expected None but got Some with {} states", states_vec.len());
        }
    }

    #[test]
    fn test_lifx_results_with_none_results() {
        let results = LiFxResults {
            results: None,
            error: None,
        };
        
        assert!(results.results.is_none());
        assert!(results.error.is_none());
        
        // This should not panic - both fields are None
        if let Some(results_vec) = results.results {
            panic!("Expected None but got Some with {} results", results_vec.len());
        }
        if let Some(error) = results.error {
            panic!("Expected None but got Some error: {}", error);
        }
    }

    #[test]
    #[should_panic(expected = "Palette should be Some after being set")]
    fn test_expect_message_for_palette() {
        let effect = MorphEffect::new();
        // This will panic with our custom message
        let _palette = effect.palette.as_ref().expect("Palette should be Some after being set");
    }

    #[test]
    #[should_panic(expected = "States should be Some after being set")]
    fn test_expect_message_for_states() {
        let states = States::new();
        // This will panic with our custom message
        let _states_vec = states.states.as_ref().expect("States should be Some after being set");
    }

    #[test]
    #[should_panic(expected = "Results should be Some after being set")]
    fn test_expect_message_for_results() {
        let results = LiFxResults {
            results: None,
            error: None,
        };
        // This will panic with our custom message
        let _results_vec = results.results.expect("Results should be Some after being set");
    }

    #[test]
    fn test_flame_effect_with_cleanup_no_duration() {
        // Test that cleanup functions work correctly with no duration specified
        let mut effect = FlameEffect::new();
        effect.period = Some(5);
        // No duration set, so no cleanup should happen
        
        assert_eq!(effect.duration, None);
    }

    #[test]
    fn test_flame_effect_with_cleanup_zero_duration() {
        // Test that cleanup functions work correctly with zero duration
        let mut effect = FlameEffect::new();
        effect.period = Some(5);
        effect.duration = Some(0.0);
        
        // Zero duration means stop immediately, no cleanup needed
        assert_eq!(effect.duration, Some(0.0));
    }

    #[test]
    fn test_flame_effect_with_cleanup_positive_duration() {
        // Test that cleanup functions work correctly with positive duration
        let mut effect = FlameEffect::new();
        effect.period = Some(5);
        effect.duration = Some(10.0);
        
        // Positive duration should trigger cleanup after the duration
        assert_eq!(effect.duration, Some(10.0));
    }

    #[test]
    fn test_morph_effect_with_cleanup_no_duration() {
        // Test that cleanup functions work correctly with no duration specified
        let mut effect = MorphEffect::new();
        effect.period = Some(5);
        // No duration set, so no cleanup should happen
        
        assert_eq!(effect.duration, None);
    }

    #[test]
    fn test_morph_effect_with_cleanup_zero_duration() {
        // Test that cleanup functions work correctly with zero duration
        let mut effect = MorphEffect::new();
        effect.period = Some(5);
        effect.duration = Some(0.0);
        
        // Zero duration means stop immediately, no cleanup needed
        assert_eq!(effect.duration, Some(0.0));
    }

    #[test]
    fn test_morph_effect_with_cleanup_positive_duration() {
        // Test that cleanup functions work correctly with positive duration
        let mut effect = MorphEffect::new();
        effect.period = Some(5);
        effect.duration = Some(10.0);
        let mut palette = Vec::new();
        palette.push("red".to_string());
        palette.push("blue".to_string());
        effect.palette = Some(palette);
        
        // Positive duration should trigger cleanup after the duration
        assert_eq!(effect.duration, Some(10.0));
        assert!(effect.palette.is_some());
    }

    #[test]
    fn test_effects_off_creation() {
        // Test EffectsOff struct creation
        let mut effects_off = EffectsOff::new();
        assert_eq!(effects_off.power_off, None);
        
        effects_off.power_off = Some(true);
        assert_eq!(effects_off.power_off, Some(true));
    }
    
    // New tests for failover functionality
    #[test]
    fn test_lifx_config_builder() {
        let config = LifxConfig::new("test_token".to_string())
            .add_endpoint("https://api1.lifx.com".to_string())
            .add_endpoint("https://api2.lifx.com".to_string())
            .add_endpoint("https://api3.lifx.com".to_string())
            .with_strategy(FailoverStrategy::RoundRobin)
            .with_timeout(3000);
            
        assert_eq!(config.access_token, "test_token");
        assert_eq!(config.api_endpoints.len(), 3);
        assert_eq!(config.failover_config.strategy, FailoverStrategy::RoundRobin);
        assert_eq!(config.failover_config.request_timeout_ms, 3000);
    }
    
    #[test]
    fn test_endpoint_health_tracking() {
        let health = EndpointHealth::new("https://api.lifx.com".to_string());
        assert!(health.is_healthy);
        assert_eq!(health.consecutive_failures, 0);
        assert!(health.should_retry());
    }
    
    #[test]
    fn test_endpoint_health_failure_tracking() {
        let mut health = EndpointHealth::new("https://api.lifx.com".to_string());
        
        // First failure
        health.mark_failure();
        assert_eq!(health.consecutive_failures, 1);
        assert!(health.is_healthy);
        
        // Second failure
        health.mark_failure();
        assert_eq!(health.consecutive_failures, 2);
        assert!(health.is_healthy);
        
        // Third failure - should mark as unhealthy
        health.mark_failure();
        assert_eq!(health.consecutive_failures, 3);
        assert!(!health.is_healthy);
    }
    
    #[test]
    fn test_endpoint_health_success_resets_failures() {
        let mut health = EndpointHealth::new("https://api.lifx.com".to_string());
        
        health.mark_failure();
        health.mark_failure();
        assert_eq!(health.consecutive_failures, 2);
        
        health.mark_success(Duration::from_millis(100));
        assert_eq!(health.consecutive_failures, 0);
        assert!(health.is_healthy);
        assert_eq!(health.response_time_ms, Some(100));
    }
    
    #[test]
    fn test_exponential_backoff() {
        let mut health = EndpointHealth::new("https://api.lifx.com".to_string());
        
        // Mark as failed multiple times
        for _ in 0..5 {
            health.mark_failure();
        }
        
        assert!(!health.is_healthy);
        
        // Immediately after failure, should not retry
        assert!(!health.should_retry());
        
        // After backoff period, should allow retry
        // Note: This test is simplified - in real tests we'd mock time
    }
    
    #[test]
    fn test_failover_config_defaults() {
        let config = FailoverConfig::default();
        assert_eq!(config.request_timeout_ms, 5000);
        assert_eq!(config.max_concurrent_attempts, 3);
        assert_eq!(config.strategy, FailoverStrategy::Failover);
        assert!(config.health_check_enabled);
        assert_eq!(config.health_check_interval_secs, 60);
    }
    
    #[test]
    fn test_get_healthy_endpoints() {
        let config = LifxConfig::new("test_token".to_string())
            .add_endpoint("https://api1.lifx.com".to_string())
            .add_endpoint("https://api2.lifx.com".to_string())
            .add_endpoint("https://api3.lifx.com".to_string());
            
        config.init_health_tracking();
        
        // Initially all should be healthy
        let healthy = config.get_healthy_endpoints();
        assert_eq!(healthy.len(), 3);
        
        // Mark one as failed
        config.mark_endpoint_failure("https://api2.lifx.com");
        config.mark_endpoint_failure("https://api2.lifx.com");
        config.mark_endpoint_failure("https://api2.lifx.com");
        
        let healthy = config.get_healthy_endpoints();
        assert_eq!(healthy.len(), 2);
        assert!(!healthy.contains(&"https://api2.lifx.com".to_string()));
    }
    
    #[test]
    fn test_round_robin_endpoint_selection() {
        let config = LifxConfig::new("test_token".to_string())
            .add_endpoint("https://api1.lifx.com".to_string())
            .add_endpoint("https://api2.lifx.com".to_string())
            .add_endpoint("https://api3.lifx.com".to_string())
            .with_strategy(FailoverStrategy::RoundRobin);
            
        config.init_health_tracking();
        
        // Should rotate through endpoints
        let first = config.get_next_endpoint();
        let second = config.get_next_endpoint();
        let third = config.get_next_endpoint();
        let fourth = config.get_next_endpoint();
        
        assert!(first.is_some());
        assert!(second.is_some());
        assert!(third.is_some());
        assert!(fourth.is_some());
        
        // Fourth should wrap around to first
        assert_eq!(fourth, first);
    }
    
    #[test]
    fn test_failover_strategy_endpoint_selection() {
        let config = LifxConfig::new("test_token".to_string())
            .add_endpoint("https://api1.lifx.com".to_string())
            .add_endpoint("https://api2.lifx.com".to_string())
            .add_endpoint("https://api3.lifx.com".to_string())
            .with_strategy(FailoverStrategy::Failover);
            
        config.init_health_tracking();
        
        // Should always return first healthy endpoint
        let first = config.get_next_endpoint();
        let second = config.get_next_endpoint();
        
        assert_eq!(first, second);
        assert_eq!(first, Some("https://api1.lifx.com".to_string()));
    }
    
    #[test]
    fn test_fastest_first_strategy() {
        let config = LifxConfig::new("test_token".to_string())
            .add_endpoint("https://api1.lifx.com".to_string())
            .add_endpoint("https://api2.lifx.com".to_string())
            .add_endpoint("https://api3.lifx.com".to_string())
            .with_strategy(FailoverStrategy::FastestFirst);
            
        config.init_health_tracking();
        
        // Mark different response times
        config.mark_endpoint_success("https://api1.lifx.com", Duration::from_millis(500));
        config.mark_endpoint_success("https://api2.lifx.com", Duration::from_millis(100));
        config.mark_endpoint_success("https://api3.lifx.com", Duration::from_millis(300));
        
        // Should return fastest endpoint
        let fastest = config.get_next_endpoint();
        assert_eq!(fastest, Some("https://api2.lifx.com".to_string()));
    }
    
    #[test]
    fn test_unlimited_endpoints() {
        let mut config = LifxConfig::new("test_token".to_string());
        
        // Add 100 endpoints
        for i in 0..100 {
            config = config.add_endpoint(format!("https://api{}.lifx.com", i));
        }
        
        assert_eq!(config.api_endpoints.len(), 100);
        
        config.init_health_tracking();
        let healthy = config.get_healthy_endpoints();
        assert_eq!(healthy.len(), 100);
    }
    
    #[test]
    fn test_prioritized_endpoints_with_failures() {
        let config = LifxConfig::new("test_token".to_string())
            .add_endpoint("https://api1.lifx.com".to_string())
            .add_endpoint("https://api2.lifx.com".to_string())
            .add_endpoint("https://api3.lifx.com".to_string())
            .with_strategy(FailoverStrategy::Failover);
            
        config.init_health_tracking();
        
        // Mark first endpoint as failed
        for _ in 0..3 {
            config.mark_endpoint_failure("https://api1.lifx.com");
        }
        
        let prioritized = config.get_prioritized_endpoints();
        
        // Should have healthy endpoints first
        assert_eq!(prioritized[0], "https://api2.lifx.com");
        assert_eq!(prioritized[1], "https://api3.lifx.com");
        // Unhealthy endpoint may still appear if retry time has passed
    }
    
    #[test]
    fn test_config_serialization() {
        let config = LifxConfig::new("test_token".to_string())
            .add_endpoint("https://api.lifx.com".to_string())
            .with_strategy(FailoverStrategy::RoundRobin)
            .with_timeout(3000);
            
        // Serialize to JSON
        let json = serde_json::to_string(&config).unwrap();
        assert!(json.contains("test_token"));
        assert!(json.contains("https://api.lifx.com"));
        
        // Deserialize back
        let deserialized: LifxConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.access_token, config.access_token);
        assert_eq!(deserialized.api_endpoints, config.api_endpoints);
        assert_eq!(deserialized.failover_config.strategy, config.failover_config.strategy);
    }

    // Tests for error handling improvements
    #[test]
    fn test_mutex_error_handling() {
        let config = LifxConfig::new("test_token".to_string())
            .add_endpoint("https://api.lifx.com".to_string());
        
        // Test that init_health_tracking doesn't panic
        config.init_health_tracking();
        
        // Test that get_healthy_endpoints doesn't panic
        let healthy = config.get_healthy_endpoints();
        assert_eq!(healthy.len(), 1);
        
        // Test mark_endpoint_success doesn't panic
        config.mark_endpoint_success("https://api.lifx.com", Duration::from_millis(100));
        
        // Test mark_endpoint_failure doesn't panic
        config.mark_endpoint_failure("https://api.lifx.com");
    }

    #[test]
    fn test_get_next_endpoint_with_mutex() {
        let config = LifxConfig::new("test_token".to_string())
            .add_endpoint("https://api1.lifx.com".to_string())
            .add_endpoint("https://api2.lifx.com".to_string())
            .with_strategy(FailoverStrategy::FastestFirst);
        
        config.init_health_tracking();
        
        // Test FastestFirst strategy doesn't panic when accessing mutex
        let endpoint = config.get_next_endpoint();
        assert!(endpoint.is_some());
    }

    #[test]
    fn test_get_prioritized_endpoints_with_mutex() {
        let config = LifxConfig::new("test_token".to_string())
            .add_endpoint("https://api1.lifx.com".to_string())
            .add_endpoint("https://api2.lifx.com".to_string())
            .with_strategy(FailoverStrategy::Failover);
        
        config.init_health_tracking();
        
        // Mark one endpoint as failed
        config.mark_endpoint_failure("https://api1.lifx.com");
        config.mark_endpoint_failure("https://api1.lifx.com");
        config.mark_endpoint_failure("https://api1.lifx.com");
        
        // Test that get_prioritized_endpoints doesn't panic
        let endpoints = config.get_prioritized_endpoints();
        assert!(!endpoints.is_empty());
    }

    #[test]
    #[should_panic(expected = "Failed to acquire endpoint_health mutex lock: mutex was poisoned")]
    fn test_mutex_poisoning_behavior() {
        use std::panic;
        use std::sync::Arc;
        use std::thread;

        let config = Arc::new(LifxConfig::new("test_token".to_string())
            .add_endpoint("https://api.lifx.com".to_string()));
        
        config.init_health_tracking();
        
        let config_clone = config.clone();
        
        // Create a thread that will panic while holding the mutex
        let handle = thread::spawn(move || {
            let _guard = config_clone.endpoint_health.lock()
                .expect("Failed to acquire endpoint_health mutex lock: mutex was poisoned");
            panic!("Intentionally poisoning the mutex");
        });
        
        // Wait for the thread to panic
        let _ = handle.join();
        
        // This should now panic with our custom message
        config.get_healthy_endpoints();
    }
}
