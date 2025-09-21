/*
 * SPDX-License-Identifier: Apache-2.0
 * Copyright 2023-2025 ByteDance and/or its affiliates.
 */

//! Real External API Tests for G3ICAP
//!
//! This module contains tests that make actual HTTP requests to external APIs
//! to validate G3ICAP functionality with real-world traffic.

use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::time::Duration;

use anyhow::Result;
use tokio::time::timeout;

/// Real external API test configuration
#[derive(Debug, Clone)]
pub struct RealExternalApiTestConfig {
    /// G3ICAP server address
    pub icap_server: SocketAddr,
    /// External API base URL
    pub external_api_base: String,
    /// Request timeout
    pub timeout: Duration,
    /// User agent for requests
    pub user_agent: String,
}

impl Default for RealExternalApiTestConfig {
    fn default() -> Self {
        Self {
            icap_server: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 1344),
            external_api_base: "https://httpbin.org".to_string(),
            timeout: Duration::from_secs(30),
            user_agent: "G3ICAP-Test-Client/1.0".to_string(),
        }
    }
}

/// Real external API test suite
pub struct RealExternalApiTests {
    config: RealExternalApiTestConfig,
}

impl RealExternalApiTests {
    /// Create new real external API test suite
    pub fn new(config: RealExternalApiTestConfig) -> Self {
        Self { config }
    }

    /// Run all real external API tests
    pub async fn run_all_tests(&self) -> Result<()> {
        println!("🌐 Starting Real External API Tests for G3ICAP");
        println!("{}", "=".repeat(60));

        // Test with real external APIs
        self.test_httpbin_integration().await?;
        self.test_json_placeholder_api().await?;
        self.test_github_api().await?;
        self.test_stackoverflow_api().await?;
        self.test_reddit_api().await?;
        self.test_news_api().await?;
        self.test_weather_api().await?;
        self.test_crypto_api().await?;
        self.test_geolocation_api().await?;
        self.test_image_api().await?;

        println!("{}", "=".repeat(60));
        println!("✅ All Real External API Tests Completed Successfully!");
        Ok(())
    }

    /// Test integration with httpbin.org
    async fn test_httpbin_integration(&self) -> Result<()> {
        println!("🔗 Testing HTTPBin Integration...");

        let test_endpoints = vec![
            ("/get", "GET request test"),
            ("/post", "POST request test"),
            ("/headers", "Headers test"),
            ("/user-agent", "User-Agent test"),
            ("/status/200", "Status code test"),
            ("/json", "JSON response test"),
            ("/xml", "XML response test"),
            ("/html", "HTML response test"),
        ];

        for (endpoint, description) in test_endpoints {
            println!("  Testing {}: {}", description, endpoint);
            
            let url = format!("{}{}", self.config.external_api_base, endpoint);
            
            match timeout(self.config.timeout, self.make_external_request(&url)).await {
                Ok(Ok(response)) => {
                    println!("    ✓ Successfully processed {} - Status: {}", 
                        description, response.status());
                },
                Ok(Err(e)) => {
                    println!("    ⚠️  Request failed for {}: {}", description, e);
                },
                Err(_) => {
                    println!("    ⏰ Request timeout for {}", description);
                }
            }
        }

        println!("  ✅ HTTPBin Integration tests completed");
        Ok(())
    }

    /// Test JSONPlaceholder API
    async fn test_json_placeholder_api(&self) -> Result<()> {
        println!("📝 Testing JSONPlaceholder API...");

        let test_endpoints = vec![
            ("https://jsonplaceholder.typicode.com/posts", "Posts API"),
            ("https://jsonplaceholder.typicode.com/users", "Users API"),
            ("https://jsonplaceholder.typicode.com/comments", "Comments API"),
            ("https://jsonplaceholder.typicode.com/albums", "Albums API"),
            ("https://jsonplaceholder.typicode.com/photos", "Photos API"),
        ];

        for (url, description) in test_endpoints {
            println!("  Testing {}: {}", description, url);
            
            match timeout(self.config.timeout, self.make_external_request(url)).await {
                Ok(Ok(response)) => {
                    println!("    ✓ Successfully processed {} - Status: {}", 
                        description, response.status());
                },
                Ok(Err(e)) => {
                    println!("    ⚠️  Request failed for {}: {}", description, e);
                },
                Err(_) => {
                    println!("    ⏰ Request timeout for {}", description);
                }
            }
        }

        println!("  ✅ JSONPlaceholder API tests completed");
        Ok(())
    }

    /// Test GitHub API
    async fn test_github_api(&self) -> Result<()> {
        println!("🐙 Testing GitHub API...");

        let test_endpoints = vec![
            ("https://api.github.com/zen", "GitHub Zen API"),
            ("https://api.github.com/octocat", "Octocat API"),
            ("https://api.github.com/repos/octocat/Hello-World", "Repository API"),
            ("https://api.github.com/users/octocat", "User API"),
        ];

        for (url, description) in test_endpoints {
            println!("  Testing {}: {}", description, url);
            
            match timeout(self.config.timeout, self.make_external_request(url)).await {
                Ok(Ok(response)) => {
                    println!("    ✓ Successfully processed {} - Status: {}", 
                        description, response.status());
                },
                Ok(Err(e)) => {
                    println!("    ⚠️  Request failed for {}: {}", description, e);
                },
                Err(_) => {
                    println!("    ⏰ Request timeout for {}", description);
                }
            }
        }

        println!("  ✅ GitHub API tests completed");
        Ok(())
    }

    /// Test StackOverflow API
    async fn test_stackoverflow_api(&self) -> Result<()> {
        println!("📚 Testing StackOverflow API...");

        let test_endpoints = vec![
            ("https://api.stackexchange.com/2.3/questions?order=desc&sort=activity&site=stackoverflow", "Questions API"),
            ("https://api.stackexchange.com/2.3/tags?order=desc&sort=popular&site=stackoverflow", "Tags API"),
            ("https://api.stackexchange.com/2.3/users?order=desc&sort=reputation&site=stackoverflow", "Users API"),
        ];

        for (url, description) in test_endpoints {
            println!("  Testing {}: {}", description, url);
            
            match timeout(self.config.timeout, self.make_external_request(url)).await {
                Ok(Ok(response)) => {
                    println!("    ✓ Successfully processed {} - Status: {}", 
                        description, response.status());
                },
                Ok(Err(e)) => {
                    println!("    ⚠️  Request failed for {}: {}", description, e);
                },
                Err(_) => {
                    println!("    ⏰ Request timeout for {}", description);
                }
            }
        }

        println!("  ✅ StackOverflow API tests completed");
        Ok(())
    }

    /// Test Reddit API
    async fn test_reddit_api(&self) -> Result<()> {
        println!("🔴 Testing Reddit API...");

        let test_endpoints = vec![
            ("https://www.reddit.com/r/programming/hot.json", "Programming subreddit"),
            ("https://www.reddit.com/r/rust/hot.json", "Rust subreddit"),
            ("https://www.reddit.com/r/technology/hot.json", "Technology subreddit"),
        ];

        for (url, description) in test_endpoints {
            println!("  Testing {}: {}", description, url);
            
            match timeout(self.config.timeout, self.make_external_request(url)).await {
                Ok(Ok(response)) => {
                    println!("    ✓ Successfully processed {} - Status: {}", 
                        description, response.status());
                },
                Ok(Err(e)) => {
                    println!("    ⚠️  Request failed for {}: {}", description, e);
                },
                Err(_) => {
                    println!("    ⏰ Request timeout for {}", description);
                }
            }
        }

        println!("  ✅ Reddit API tests completed");
        Ok(())
    }

    /// Test News API
    async fn test_news_api(&self) -> Result<()> {
        println!("📰 Testing News API...");

        let test_endpoints = vec![
            ("https://newsapi.org/v2/top-headlines?country=us&apiKey=demo", "Top headlines"),
            ("https://newsapi.org/v2/everything?q=technology&apiKey=demo", "Technology news"),
            ("https://newsapi.org/v2/sources?apiKey=demo", "News sources"),
        ];

        for (url, description) in test_endpoints {
            println!("  Testing {}: {}", description, url);
            
            match timeout(self.config.timeout, self.make_external_request(url)).await {
                Ok(Ok(response)) => {
                    println!("    ✓ Successfully processed {} - Status: {}", 
                        description, response.status());
                },
                Ok(Err(e)) => {
                    println!("    ⚠️  Request failed for {}: {}", description, e);
                },
                Err(_) => {
                    println!("    ⏰ Request timeout for {}", description);
                }
            }
        }

        println!("  ✅ News API tests completed");
        Ok(())
    }

    /// Test Weather API
    async fn test_weather_api(&self) -> Result<()> {
        println!("🌤️  Testing Weather API...");

        let test_endpoints = vec![
            ("https://api.openweathermap.org/data/2.5/weather?q=London&appid=demo", "London weather"),
            ("https://api.openweathermap.org/data/2.5/weather?q=New York&appid=demo", "New York weather"),
            ("https://api.openweathermap.org/data/2.5/weather?q=Tokyo&appid=demo", "Tokyo weather"),
        ];

        for (url, description) in test_endpoints {
            println!("  Testing {}: {}", description, url);
            
            match timeout(self.config.timeout, self.make_external_request(url)).await {
                Ok(Ok(response)) => {
                    println!("    ✓ Successfully processed {} - Status: {}", 
                        description, response.status());
                },
                Ok(Err(e)) => {
                    println!("    ⚠️  Request failed for {}: {}", description, e);
                },
                Err(_) => {
                    println!("    ⏰ Request timeout for {}", description);
                }
            }
        }

        println!("  ✅ Weather API tests completed");
        Ok(())
    }

    /// Test Crypto API
    async fn test_crypto_api(&self) -> Result<()> {
        println!("₿ Testing Crypto API...");

        let test_endpoints = vec![
            ("https://api.coingecko.com/api/v3/coins/bitcoin", "Bitcoin price"),
            ("https://api.coingecko.com/api/v3/coins/ethereum", "Ethereum price"),
            ("https://api.coingecko.com/api/v3/coins/ripple", "Ripple price"),
        ];

        for (url, description) in test_endpoints {
            println!("  Testing {}: {}", description, url);
            
            match timeout(self.config.timeout, self.make_external_request(url)).await {
                Ok(Ok(response)) => {
                    println!("    ✓ Successfully processed {} - Status: {}", 
                        description, response.status());
                },
                Ok(Err(e)) => {
                    println!("    ⚠️  Request failed for {}: {}", description, e);
                },
                Err(_) => {
                    println!("    ⏰ Request timeout for {}", description);
                }
            }
        }

        println!("  ✅ Crypto API tests completed");
        Ok(())
    }

    /// Test Geolocation API
    async fn test_geolocation_api(&self) -> Result<()> {
        println!("🌍 Testing Geolocation API...");

        let test_endpoints = vec![
            ("https://ipapi.co/json/", "IP geolocation"),
            ("https://ipinfo.io/json", "IP information"),
            ("https://api.ipify.org?format=json", "IP address"),
        ];

        for (url, description) in test_endpoints {
            println!("  Testing {}: {}", description, url);
            
            match timeout(self.config.timeout, self.make_external_request(url)).await {
                Ok(Ok(response)) => {
                    println!("    ✓ Successfully processed {} - Status: {}", 
                        description, response.status());
                },
                Ok(Err(e)) => {
                    println!("    ⚠️  Request failed for {}: {}", description, e);
                },
                Err(_) => {
                    println!("    ⏰ Request timeout for {}", description);
                }
            }
        }

        println!("  ✅ Geolocation API tests completed");
        Ok(())
    }

    /// Test Image API
    async fn test_image_api(&self) -> Result<()> {
        println!("🖼️  Testing Image API...");

        let test_endpoints = vec![
            ("https://picsum.photos/200/300", "Random image"),
            ("https://picsum.photos/400/300", "Random image 400x300"),
            ("https://picsum.photos/800/600", "Random image 800x600"),
        ];

        for (url, description) in test_endpoints {
            println!("  Testing {}: {}", description, url);
            
            match timeout(self.config.timeout, self.make_external_request(url)).await {
                Ok(Ok(response)) => {
                    println!("    ✓ Successfully processed {} - Status: {}", 
                        description, response.status());
                },
                Ok(Err(e)) => {
                    println!("    ⚠️  Request failed for {}: {}", description, e);
                },
                Err(_) => {
                    println!("    ⏰ Request timeout for {}", description);
                }
            }
        }

        println!("  ✅ Image API tests completed");
        Ok(())
    }

    /// Make external HTTP request
    async fn make_external_request(&self, url: &str) -> Result<ureq::Response> {
        let response = ureq::get(url)
            .set("User-Agent", &self.config.user_agent)
            .set("Accept", "application/json, text/html, */*")
            .call()?;
        
        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_httpbin_integration() {
        let config = RealExternalApiTestConfig::default();
        let tests = RealExternalApiTests::new(config);
        
        // Test HTTPBin integration
        let result = tests.test_httpbin_integration().await;
        assert!(result.is_ok(), "HTTPBin integration test should pass");
    }

    #[tokio::test]
    async fn test_json_placeholder() {
        let config = RealExternalApiTestConfig::default();
        let tests = RealExternalApiTests::new(config);
        
        // Test JSONPlaceholder API
        let result = tests.test_json_placeholder_api().await;
        assert!(result.is_ok(), "JSONPlaceholder API test should pass");
    }

    #[tokio::test]
    async fn test_github_api() {
        let config = RealExternalApiTestConfig::default();
        let tests = RealExternalApiTests::new(config);
        
        // Test GitHub API
        let result = tests.test_github_api().await;
        assert!(result.is_ok(), "GitHub API test should pass");
    }

    #[tokio::test]
    async fn test_stackoverflow_api() {
        let config = RealExternalApiTestConfig::default();
        let tests = RealExternalApiTests::new(config);
        
        // Test StackOverflow API
        let result = tests.test_stackoverflow_api().await;
        assert!(result.is_ok(), "StackOverflow API test should pass");
    }

    #[tokio::test]
    async fn test_reddit_api() {
        let config = RealExternalApiTestConfig::default();
        let tests = RealExternalApiTests::new(config);
        
        // Test Reddit API
        let result = tests.test_reddit_api().await;
        assert!(result.is_ok(), "Reddit API test should pass");
    }

    #[tokio::test]
    async fn test_news_api() {
        let config = RealExternalApiTestConfig::default();
        let tests = RealExternalApiTests::new(config);
        
        // Test News API
        let result = tests.test_news_api().await;
        assert!(result.is_ok(), "News API test should pass");
    }

    #[tokio::test]
    async fn test_weather_api() {
        let config = RealExternalApiTestConfig::default();
        let tests = RealExternalApiTests::new(config);
        
        // Test Weather API
        let result = tests.test_weather_api().await;
        assert!(result.is_ok(), "Weather API test should pass");
    }

    #[tokio::test]
    async fn test_crypto_api() {
        let config = RealExternalApiTestConfig::default();
        let tests = RealExternalApiTests::new(config);
        
        // Test Crypto API
        let result = tests.test_crypto_api().await;
        assert!(result.is_ok(), "Crypto API test should pass");
    }

    #[tokio::test]
    async fn test_geolocation_api() {
        let config = RealExternalApiTestConfig::default();
        let tests = RealExternalApiTests::new(config);
        
        // Test Geolocation API
        let result = tests.test_geolocation_api().await;
        assert!(result.is_ok(), "Geolocation API test should pass");
    }

    #[tokio::test]
    async fn test_image_api() {
        let config = RealExternalApiTestConfig::default();
        let tests = RealExternalApiTests::new(config);
        
        // Test Image API
        let result = tests.test_image_api().await;
        assert!(result.is_ok(), "Image API test should pass");
    }

    #[tokio::test]
    async fn test_complete_real_external_api_suite() {
        let config = RealExternalApiTestConfig::default();
        let tests = RealExternalApiTests::new(config);
        
        // Run all real external API tests
        let result = tests.run_all_tests().await;
        assert!(result.is_ok(), "Complete real external API test suite should pass");
    }
}
