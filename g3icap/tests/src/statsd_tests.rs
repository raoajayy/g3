/*
 * SPDX-License-Identifier: Apache-2.0
 * Copyright 2023-2025 ByteDance and/or its affiliates.
 */

//! Tests for G3StatsD integration

use g3icap::stats::IcapStats;
use g3_statsd_client::StatsdClientConfig;
use std::sync::Arc;
use std::time::Duration;

#[cfg(test)]
mod statsd_integration_tests {
    use super::*;

    #[test]
    fn test_statsd_client_creation() {
        let config = StatsdClientConfig {
            server: "127.0.0.1".to_string(),
            port: 8125,
            prefix: "g3icap".to_string(),
            buffer_size: 1024,
            udp_enabled: true,
            tcp_enabled: false,
            ..Default::default()
        };

        let stats = IcapStats::new_with_statsd(&config);
        assert!(stats.is_ok());
        
        let stats = stats.unwrap();
        assert!(stats.statsd_client.is_some());
    }

    #[test]
    fn test_statsd_metrics_emission() {
        let config = StatsdClientConfig {
            server: "127.0.0.1".to_string(),
            port: 8125,
            prefix: "g3icap".to_string(),
            buffer_size: 1024,
            udp_enabled: true,
            tcp_enabled: false,
            ..Default::default()
        };

        let stats = IcapStats::new_with_statsd(&config).unwrap();
        
        // Increment some counters
        stats.increment_requests();
        stats.increment_reqmod_requests();
        stats.increment_successful_responses();
        stats.add_connection();
        stats.add_bytes(1024);
        stats.add_processing_time(150); // 150 microseconds
        
        // Emit stats (should not panic)
        stats.emit_stats();
        
        // Verify counters were incremented
        assert_eq!(stats.total_requests(), 1);
        assert_eq!(stats.reqmod_requests(), 1);
        assert_eq!(stats.successful_responses(), 1);
        assert_eq!(stats.get_total_connections(), 1);
        assert_eq!(stats.total_bytes(), 1024);
        assert_eq!(stats.get_total_processing_time(), 150);
    }

    #[test]
    fn test_statsd_without_client() {
        let stats = IcapStats::new();
        
        // Should not panic when emitting without StatsD client
        stats.emit_stats();
        
        // Verify basic functionality still works
        stats.increment_requests();
        assert_eq!(stats.total_requests(), 1);
    }

    #[test]
    fn test_statsd_metrics_format() {
        let config = StatsdClientConfig {
            server: "127.0.0.1".to_string(),
            port: 8125,
            prefix: "g3icap".to_string(),
            buffer_size: 1024,
            udp_enabled: true,
            tcp_enabled: false,
            ..Default::default()
        };

        let stats = IcapStats::new_with_statsd(&config).unwrap();
        
        // Set up some test data
        stats.increment_requests();
        stats.increment_reqmod_requests();
        stats.increment_respmod_requests();
        stats.increment_options_requests();
        stats.increment_successful_responses();
        stats.increment_error_responses();
        stats.increment_blocked_requests();
        stats.add_connection();
        stats.add_bytes(2048);
        stats.add_processing_time(300);
        
        // Emit stats
        stats.emit_stats();
        
        // Verify all metrics are properly tracked
        assert_eq!(stats.total_requests(), 1);
        assert_eq!(stats.reqmod_requests(), 1);
        assert_eq!(stats.respmod_requests(), 1);
        assert_eq!(stats.options_requests(), 1);
        assert_eq!(stats.successful_responses(), 1);
        assert_eq!(stats.error_responses(), 1);
        assert_eq!(stats.blocked_requests(), 1);
        assert_eq!(stats.get_total_connections(), 1);
        assert_eq!(stats.total_bytes(), 2048);
        assert_eq!(stats.get_total_processing_time(), 300);
    }

    #[test]
    fn test_statsd_average_processing_time() {
        let config = StatsdClientConfig {
            server: "127.0.0.1".to_string(),
            port: 8125,
            prefix: "g3icap".to_string(),
            buffer_size: 1024,
            udp_enabled: true,
            tcp_enabled: false,
            ..Default::default()
        };

        let stats = IcapStats::new_with_statsd(&config).unwrap();
        
        // Add multiple requests with different processing times
        stats.increment_requests();
        stats.add_processing_time(100);
        
        stats.increment_requests();
        stats.add_processing_time(200);
        
        stats.increment_requests();
        stats.add_processing_time(300);
        
        // Calculate average
        let avg_time = stats.get_avg_processing_time();
        assert_eq!(avg_time, 200); // (100 + 200 + 300) / 3 = 200
        
        // Emit stats
        stats.emit_stats();
    }

    #[test]
    fn test_statsd_connection_lifecycle() {
        let config = StatsdClientConfig {
            server: "127.0.0.1".to_string(),
            port: 8125,
            prefix: "g3icap".to_string(),
            buffer_size: 1024,
            udp_enabled: true,
            tcp_enabled: false,
            ..Default::default()
        };

        let stats = IcapStats::new_with_statsd(&config).unwrap();
        
        // Test connection lifecycle
        assert_eq!(stats.get_total_connections(), 0);
        assert_eq!(stats.active_connections(), 0);
        
        // Add connection
        stats.add_connection();
        assert_eq!(stats.get_total_connections(), 1);
        assert_eq!(stats.active_connections(), 1);
        
        // Add another connection
        stats.add_connection();
        assert_eq!(stats.get_total_connections(), 2);
        assert_eq!(stats.active_connections(), 2);
        
        // Remove one connection
        stats.remove_connection();
        assert_eq!(stats.get_total_connections(), 2);
        assert_eq!(stats.active_connections(), 1);
        
        // Remove another connection
        stats.remove_connection();
        assert_eq!(stats.get_total_connections(), 2);
        assert_eq!(stats.active_connections(), 0);
        
        // Emit stats
        stats.emit_stats();
    }

    #[test]
    fn test_statsd_error_handling() {
        let config = StatsdClientConfig {
            server: "127.0.0.1".to_string(),
            port: 8125,
            prefix: "g3icap".to_string(),
            buffer_size: 1024,
            udp_enabled: true,
            tcp_enabled: false,
            ..Default::default()
        };

        let stats = IcapStats::new_with_statsd(&config).unwrap();
        
        // Add some errors
        stats.add_connection_error();
        stats.add_connection_error();
        stats.increment_error_responses();
        
        assert_eq!(stats.get_connection_errors(), 2);
        assert_eq!(stats.error_responses(), 1);
        
        // Emit stats
        stats.emit_stats();
    }
}

#[cfg(test)]
mod statsd_performance_tests {
    use super::*;

    #[test]
    fn test_statsd_high_volume_emission() {
        let config = StatsdClientConfig {
            server: "127.0.0.1".to_string(),
            port: 8125,
            prefix: "g3icap".to_string(),
            buffer_size: 1024,
            udp_enabled: true,
            tcp_enabled: false,
            ..Default::default()
        };

        let stats = IcapStats::new_with_statsd(&config).unwrap();
        
        // Simulate high volume
        for i in 0..1000 {
            stats.increment_requests();
            stats.add_bytes(i as u64);
            stats.add_processing_time(i as u64);
        }
        
        // Emit stats multiple times
        for _ in 0..100 {
            stats.emit_stats();
        }
        
        // Verify final state
        assert_eq!(stats.total_requests(), 1000);
        assert_eq!(stats.total_bytes(), 499500); // Sum of 0..999
        assert_eq!(stats.get_total_processing_time(), 499500);
    }

    #[test]
    fn test_statsd_concurrent_emission() {
        use std::thread;
        
        let config = StatsdClientConfig {
            server: "127.0.0.1".to_string(),
            port: 8125,
            prefix: "g3icap".to_string(),
            buffer_size: 1024,
            udp_enabled: true,
            tcp_enabled: false,
            ..Default::default()
        };

        let stats = Arc::new(IcapStats::new_with_statsd(&config).unwrap());
        let mut handles = vec![];
        
        // Spawn multiple threads to emit stats concurrently
        for i in 0..10 {
            let stats_clone = Arc::clone(&stats);
            let handle = thread::spawn(move || {
                for _ in 0..100 {
                    stats_clone.increment_requests();
                    stats_clone.add_bytes(i as u64);
                    stats_clone.emit_stats();
                }
            });
            handles.push(handle);
        }
        
        // Wait for all threads to complete
        for handle in handles {
            handle.join().unwrap();
        }
        
        // Verify final state
        assert_eq!(stats.total_requests(), 1000);
    }
}
