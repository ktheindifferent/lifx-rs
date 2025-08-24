//! Example demonstrating the new unlimited endpoint failover capabilities

use lifx_rs::{LifxConfig, FailoverStrategy};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Example 1: Simple failover configuration with multiple endpoints
    let config = LifxConfig::new("your_access_token".to_string())
        .add_endpoint("https://api.lifx.com".to_string())
        .add_endpoint("http://192.168.1.100:8089".to_string())  // Local server
        .add_endpoint("http://backup.example.com:8089".to_string());  // Backup server
    
    println!("Configuration with {} endpoints", config.api_endpoints.len());
    
    // Example 2: Round-robin load balancing across endpoints
    let round_robin_config = LifxConfig::new("your_access_token".to_string())
        .add_endpoint("https://api1.lifx.com".to_string())
        .add_endpoint("https://api2.lifx.com".to_string())
        .add_endpoint("https://api3.lifx.com".to_string())
        .with_strategy(FailoverStrategy::RoundRobin)
        .with_timeout(3000);  // 3 second timeout per request
    
    println!("Round-robin configuration with {} ms timeout", 
             round_robin_config.failover_config.request_timeout_ms);
    
    // Example 3: Fastest-first strategy for optimal performance
    let performance_config = LifxConfig::new("your_access_token".to_string())
        .add_endpoint("https://api.lifx.com".to_string())
        .add_endpoint("https://eu.api.lifx.com".to_string())
        .add_endpoint("https://ap.api.lifx.com".to_string())
        .with_strategy(FailoverStrategy::FastestFirst);
    
    // Example 4: Unlimited endpoints - add as many as needed
    let mut unlimited_config = LifxConfig::new("your_access_token".to_string());
    
    // Add primary cloud endpoints
    for region in &["us", "eu", "ap", "au"] {
        unlimited_config = unlimited_config
            .add_endpoint(format!("https://{}.api.lifx.com", region));
    }
    
    // Add local network endpoints
    for i in 1..=10 {
        unlimited_config = unlimited_config
            .add_endpoint(format!("http://192.168.1.{}:8089", 100 + i));
    }
    
    println!("Unlimited configuration with {} endpoints", 
             unlimited_config.api_endpoints.len());
    
    // Example 5: Custom failover configuration with explicit settings
    let mut custom_config = LifxConfig::new("your_access_token".to_string())
        .add_endpoint("https://api.lifx.com".to_string())
        .add_endpoint("http://localhost:8089".to_string());
    
    // Customize the failover configuration
    custom_config.failover_config.request_timeout_ms = 2000;  // 2 second timeout
    custom_config.failover_config.max_concurrent_attempts = 2;  // Try 2 endpoints simultaneously
    custom_config.failover_config.strategy = FailoverStrategy::Failover;
    custom_config.failover_config.health_check_enabled = true;
    custom_config.failover_config.health_check_interval_secs = 30;  // Health check every 30 seconds
    
    // Example 6: Using the configuration with actual API calls
    // Note: This requires a valid access token and reachable endpoints
    /*
    let lights = lifx_rs::Light::async_list_all(performance_config).await?;
    println!("Found {} lights", lights.len());
    
    for light in lights {
        println!("Light: {} - Power: {}", light.label, light.power);
    }
    */
    
    // Example 7: Health tracking demonstration
    unlimited_config.init_health_tracking();
    
    // Simulate endpoint failures for demonstration
    unlimited_config.mark_endpoint_failure("https://us.api.lifx.com");
    unlimited_config.mark_endpoint_failure("https://us.api.lifx.com");
    unlimited_config.mark_endpoint_failure("https://us.api.lifx.com");
    
    // After 3 failures, the endpoint is marked unhealthy
    let healthy = unlimited_config.get_healthy_endpoints();
    println!("Healthy endpoints after failures: {}", healthy.len());
    
    // Simulate successful response to restore health
    unlimited_config.mark_endpoint_success(
        "https://eu.api.lifx.com", 
        std::time::Duration::from_millis(150)
    );
    
    // Get prioritized list based on strategy
    let prioritized = unlimited_config.get_prioritized_endpoints();
    println!("Prioritized endpoints: {:?}", prioritized.iter().take(3).collect::<Vec<_>>());
    
    println!("\nFailover system demonstration complete!");
    println!("Key features:");
    println!("- Unlimited API endpoints supported");
    println!("- Concurrent request attempts with configurable limits");
    println!("- Multiple failover strategies (Failover, RoundRobin, FastestFirst)");
    println!("- Automatic health tracking with exponential backoff");
    println!("- Configurable timeouts and retry policies");
    
    Ok(())
}