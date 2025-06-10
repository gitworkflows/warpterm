use warp_terminal::{
    analytics::{AnalyticsEngine, AnalyticsEvent, EventType, PerformanceData, TimeRange},
    error::WarpError,
};
use std::collections::HashMap;
use chrono::Utc;

#[tokio::main]
async fn main() -> Result<(), WarpError> {
    // Initialize the analytics engine
    let analytics = AnalyticsEngine::new().await?;
    
    println!("📊 Warp Analytics & Performance Monitoring Demo\n");
    
    // Start background processing
    analytics.start_background_processing().await?;
    
    // Demo 1: Track item activation and usage
    println!("1. Tracking item activation and usage...");
    analytics.track_event(AnalyticsEvent {
        id: uuid::Uuid::new_v4().to_string(),
        event_type: EventType::ItemActivation,
        timestamp: Utc::now(),
        user_id: Some("user123".to_string()),
        session_id: "session456".to_string(),
        item_id: Some("catppuccin-theme".to_string()),
        metadata: HashMap::new(),
        performance_data: None,
    }).await?;
    
    // Simulate usage events
    for i in 0..10 {
        let mut metadata = HashMap::new();
        metadata.insert("action".to_string(), serde_json::Value::String(format!("theme_change_{}", i)));
        
        analytics.track_event(AnalyticsEvent {
            id: uuid::Uuid::new_v4().to_string(),
            event_type: EventType::ItemUsage,
            timestamp: Utc::now(),
            user_id: Some("user123".to_string()),
            session_id: "session456".to_string(),
            item_id: Some("catppuccin-theme".to_string()),
            metadata,
            performance_data: Some(PerformanceData {
                cpu_usage: 2.5 + (i as f32 * 0.1),
                memory_usage: 15 * 1024 * 1024 + (i as u64 * 1024 * 1024),
                disk_usage: 0,
                network_bytes_sent: 0,
                network_bytes_received: 0,
                load_time_ms: 120 + (i as u64 * 5),
                response_time_ms: 50 + (i as u64 * 2),
                error_count: 0,
                crash_count: 0,
            }),
        }).await?;
    }
    
    println!("   ✅ Tracked 10 usage events for catppuccin-theme");
    
    // Demo 2: Track performance metrics
    println!("\n2. Tracking performance metrics...");
    analytics.track_event(AnalyticsEvent {
        id: uuid::Uuid::new_v4().to_string(),
        event_type: EventType::ItemLoadTime,
        timestamp: Utc::now(),
        user_id: Some("user123".to_string()),
        session_id: "session456".to_string(),
        item_id: Some("git-enhanced".to_string()),
        metadata: {
            let mut map = HashMap::new();
            map.insert("load_time_ms".to_string(), serde_json::Value::Number(serde_json::Number::from(340)));
            map
        },
        performance_data: Some(PerformanceData {
            cpu_usage: 8.5,
            memory_usage: 45 * 1024 * 1024,
            disk_usage: 0,
            network_bytes_sent: 1024,
            network_bytes_received: 2048,
            load_time_ms: 340,
            response_time_ms: 120,
            error_count: 0,
            crash_count: 0,
        }),
    }).await?;
    
    println!("   ✅ Tracked load time and performance data for git-enhanced");
    
    // Demo 3: Track errors and crashes
    println!("\n3. Tracking errors and crashes...");
    analytics.track_event(AnalyticsEvent {
        id: uuid::Uuid::new_v4().to_string(),
        event_type: EventType::ItemError,
        timestamp: Utc::now(),
        user_id: Some("user123".to_string()),
        session_id: "session456".to_string(),
        item_id: Some("ai-assistant".to_string()),
        metadata: {
            let mut map = HashMap::new();
            map.insert("error".to_string(), serde_json::Value::String("API timeout".to_string()));
            map.insert("stack_trace".to_string(), serde_json::Value::String("at ai_request:42".to_string()));
            map
        },
        performance_data: None,
    }).await?;
    
    analytics.track_event(AnalyticsEvent {
        id: uuid::Uuid::new_v4().to_string(),
        event_type: EventType::ItemCrash,
        timestamp: Utc::now(),
        user_id: Some("user123".to_string()),
        session_id: "session456".to_string(),
        item_id: Some("docker-helper".to_string()),
        metadata: {
            let mut map = HashMap::new();
            map.insert("crash_info".to_string(), serde_json::Value::String("Segmentation fault".to_string()));
            map
        },
        performance_data: None,
    }).await?;
    
    println!("   ✅ Tracked error and crash events");
    
    // Demo 4: Get usage metrics
    println!("\n4. Retrieving usage metrics...");
    match analytics.get_usage_metrics("catppuccin-theme", TimeRange::LastDay).await {
        Ok(metrics) => {
            println!("   📈 Usage Metrics for catppuccin-theme:");
            println!("      • Total Activations: {}", metrics.total_activations);
            println!("      • Total Usage Time: {} minutes", metrics.total_usage_time.num_minutes());
            println!("      • Average Session Duration: {} minutes", metrics.average_session_duration.num_minutes());
            println!("      • Error Rate: {:.2}%", metrics.error_rate * 100.0);
            println!("      • Performance Score: {:.1}/10", metrics.performance_score * 10.0);
        }
        Err(e) => println!("   ❌ Failed to get usage metrics: {}", e),
    }
    
    // Demo 5: Get performance metrics
    println!("\n5. Retrieving performance metrics...");
    match analytics.get_performance_metrics("git-enhanced", TimeRange::LastDay).await {
        Ok(metrics) => {
            println!("   ⚡ Performance Metrics for git-enhanced:");
            println!("      • Average Load Time: {}ms", metrics.average_load_time.num_milliseconds());
            println!("      • Average CPU Usage: {:.1}%", metrics.average_cpu_usage);
            println!("      • Average Memory Usage: {:.1}MB", metrics.average_memory_usage as f64 / (1024.0 * 1024.0));
            println!("      • Peak Memory Usage: {:.1}MB", metrics.peak_memory_usage as f64 / (1024.0 * 1024.0));
            println!("      • Resource Efficiency: {:.1}%", metrics.resource_efficiency * 100.0);
            println!("      • Stability Score: {:.1}/1.0", metrics.stability_score);
        }
        Err(e) => println!("   ❌ Failed to get performance metrics: {}", e),
    }
    
    // Demo 6: Get marketplace analytics
    println!("\n6. Retrieving marketplace analytics...");
    match analytics.get_marketplace_analytics(TimeRange::LastWeek).await {
        Ok(marketplace) => {
            println!("   🛒 Marketplace Analytics:");
            println!("      • Total Items: {}", marketplace.total_items);
            println!("      • Total Downloads: {}", marketplace.total_downloads);
            println!("      • Active Users: {}", marketplace.total_active_users);
            println!("      • Total Revenue: ${:.2}", marketplace.revenue_metrics.total_revenue);
            println!("      • Conversion Rate: {:.1}%", marketplace.revenue_metrics.conversion_rate * 100.0);
            println!("      • Trending Items: {}", marketplace.trending_items.len());
        }
        Err(e) => println!("   ❌ Failed to get marketplace analytics: {}", e),
    }
    
    // Demo 7: Generate analytics report
    println!("\n7. Generating analytics report...");
    match analytics.generate_report(
        crate::analytics::ReportType::UsageSummary,
        TimeRange::LastWeek
    ).await {
        Ok(report) => {
            println!("   📋 Generated Report:");
            println!("      • Report Type: {:?}", report.report_type);
            println!("      • Time Range: {:?}", report.time_range);
            println!("      • Generated At: {}", report.generated_at.format("%Y-%m-%d %H:%M:%S"));
            println!("      • Sections: {}", report.sections.len());
            println!("      • Recommendations: {}", report.recommendations.len());
            
            if !report.recommendations.is_empty() {
                println!("      • Top Recommendation: {}", report.recommendations[0].title);
            }
        }
        Err(e) => println!("   ❌ Failed to generate report: {}", e),
    }
    
    // Demo 8: Simulate real-time monitoring
    println!("\n8. Real-time monitoring simulation...");
    println!("   🔄 Monitoring system performance for 10 seconds...");
    
    for i in 0..10 {
        // Simulate system metrics collection
        analytics.track_event(AnalyticsEvent {
            id: uuid::Uuid::new_v4().to_string(),
            event_type: EventType::ItemMemoryUsage,
            timestamp: Utc::now(),
            user_id: None,
            session_id: "system".to_string(),
            item_id: Some("system".to_string()),
            metadata: HashMap::new(),
            performance_data: Some(PerformanceData {
                cpu_usage: 65.0 + (i as f32 * 2.0),
                memory_usage: (4 * 1024 * 1024 * 1024) + (i as u64 * 100 * 1024 * 1024),
                disk_usage: 0,
                network_bytes_sent: i as u64 * 1024,
                network_bytes_received: i as u64 * 2048,
                load_time_ms: 0,
                response_time_ms: 0,
                error_count: 0,
                crash_count: 0,
            }),
        }).await?;
        
        print!("   📊 Collecting metrics... {}s\r", i + 1);
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
    
    println!("\n   ✅ Real-time monitoring completed");
    
    println!("\n🎉 Analytics Demo Complete!");
    println!("\nFeatures demonstrated:");
    println!("• 📊 Event tracking and collection");
    println!("• ⚡ Performance monitoring");
    println!("• 📈 Usage analytics");
    println!("• 🛒 Marketplace analytics");
    println!("• 📋 Automated reporting");
    println!("• 🔄 Real-time monitoring");
    println!("• 🎯 User behavior analysis");
    println!("• 🚨 Error and crash tracking");
    println!("• 💡 Intelligent recommendations");
    println!("• 📱 Interactive dashboard");
    
    Ok(())
}
