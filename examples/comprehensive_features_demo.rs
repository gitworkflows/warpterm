use std::collections::HashMap;
use std::path::PathBuf;
use warp_terminal::{
    ab_testing::{
        ABTestingFramework, AllocationStrategy, Experiment, ExperimentStatus, MetricGoal,
        MetricType, TargetMetric, Variant, VariantConfiguration,
    },
    custom_metrics::{
        CollectionMethod, CustomMetricsManager, MetricDataPoint, MetricDefinition,
        MetricType as CustomMetricType, MetricValue,
    },
    dev_tools::{DevToolsManager, TestCase, TestExpectation, TestSuite, TestType},
    error::WarpError,
    export::{DataSource, ExportDestination, ExportFormat, ExportManager, ExportRequest},
    ml_insights::{MLInsightsEngine, PredictionType},
};

#[tokio::main]
async fn main() -> Result<(), WarpError> {
    println!("🚀 Warp Terminal - Comprehensive Features Demo");
    println!("===============================================");

    // Initialize all systems
    let dev_tools = DevToolsManager::new().await?;
    let ab_testing = ABTestingFramework::new().await?;
    let ml_insights = MLInsightsEngine::new().await?;
    let export_manager = ExportManager::new().await?;
    let custom_metrics = CustomMetricsManager::new().await?;

    // Demo 1: Development Tools
    println!("\n📋 Demo 1: Development Tools");
    println!("----------------------------");

    // Create a test suite
    let test_suite = TestSuite {
        name: "Theme Validation Tests".to_string(),
        tests: vec![
            TestCase {
                name: "Color Contrast Test".to_string(),
                description: "Verify theme colors meet accessibility standards".to_string(),
                test_type: TestType::Unit,
                code: "assert_contrast_ratio(theme.foreground, theme.background) >= 4.5"
                    .to_string(),
                expected_result: TestExpectation::Success,
                timeout: 5000,
                tags: vec!["accessibility".to_string(), "theme".to_string()],
            },
            TestCase {
                name: "Performance Test".to_string(),
                description: "Ensure theme loads within acceptable time".to_string(),
                test_type: TestType::Performance,
                code: "measure_theme_load_time()".to_string(),
                expected_result: TestExpectation::Performance {
                    max_time_ms: 100,
                    max_memory_mb: 10,
                },
                timeout: 10000,
                tags: vec!["performance".to_string()],
            },
        ],
        setup: Some("initialize_theme_engine()".to_string()),
        teardown: Some("cleanup_theme_resources()".to_string()),
        timeout: 30000,
        parallel: true,
    };

    // Run tests
    let test_results = dev_tools.run_tests("catppuccin-theme", &test_suite).await?;
    println!("✅ Test Results:");
    for result in &test_results {
        println!(
            "  - {}: {:?} ({}ms)",
            result.test_name,
            result.status,
            result.duration.as_millis()
        );
    }

    // Start debugging session
    let debug_session = dev_tools.start_debug_session("catppuccin-theme").await?;
    println!("🐛 Debug session started: {}", debug_session);

    // Set breakpoint
    let breakpoint_id = dev_tools
        .set_breakpoint(
            &debug_session,
            "theme.rs",
            42,
            Some("color_count > 16".to_string()),
        )
        .await?;
    println!("🔴 Breakpoint set: {}", breakpoint_id);

    // Start profiling
    let profile_id = dev_tools.start_profiling("catppuccin-theme").await?;
    println!("📊 Profiling started: {}", profile_id);

    // Demo 2: A/B Testing Framework
    println!("\n🧪 Demo 2: A/B Testing Framework");
    println!("--------------------------------");

    // Create an experiment
    let experiment = Experiment {
        id: "theme_recommendation_algo".to_string(),
        name: "Theme Recommendation Algorithm Test".to_string(),
        description: "Test new ML-based theme recommendation vs. popularity-based".to_string(),
        status: ExperimentStatus::Draft,
        variants: vec![
            Variant {
                id: "control".to_string(),
                name: "Popularity-based Recommendations".to_string(),
                description: "Current algorithm based on download count and ratings".to_string(),
                allocation_percentage: 50.0,
                configuration: VariantConfiguration::Algorithm {
                    algorithm_id: "popularity_recommender".to_string(),
                    parameters: HashMap::from([
                        ("weight_downloads".to_string(), 0.7),
                        ("weight_ratings".to_string(), 0.3),
                    ]),
                },
                is_control: true,
            },
            Variant {
                id: "treatment".to_string(),
                name: "ML-based Recommendations".to_string(),
                description: "New machine learning algorithm using user behavior patterns"
                    .to_string(),
                allocation_percentage: 50.0,
                configuration: VariantConfiguration::Algorithm {
                    algorithm_id: "ml_recommender".to_string(),
                    parameters: HashMap::from([
                        ("model_version".to_string(), 2.1),
                        ("confidence_threshold".to_string(), 0.8),
                    ]),
                },
                is_control: false,
            },
        ],
        allocation_strategy: AllocationStrategy::Random,
        target_metrics: vec![
            TargetMetric {
                name: "theme_install_rate".to_string(),
                metric_type: MetricType::Conversion,
                goal: MetricGoal::Increase,
                baseline_value: Some(0.15),
                minimum_detectable_effect: 0.02,
            },
            TargetMetric {
                name: "user_engagement".to_string(),
                metric_type: MetricType::Engagement,
                goal: MetricGoal::Increase,
                baseline_value: Some(0.65),
                minimum_detectable_effect: 0.05,
            },
        ],
        start_date: chrono::Utc::now(),
        end_date: Some(chrono::Utc::now() + chrono::Duration::days(14)),
        sample_size: 10000,
        confidence_level: 0.95,
        minimum_effect_size: 0.02,
        traffic_allocation: 0.5,
        filters: vec![],
        metadata: HashMap::new(),
    };

    // Create and start experiment
    let experiment_id = ab_testing.create_experiment(experiment).await?;
    ab_testing.start_experiment(&experiment_id).await?;
    println!("🧪 A/B Test started: {}", experiment_id);

    // Allocate users to variants
    let user_properties = HashMap::from([
        (
            "user_type".to_string(),
            serde_json::Value::String("premium".to_string()),
        ),
        (
            "region".to_string(),
            serde_json::Value::String("US".to_string()),
        ),
    ]);

    for i in 0..10 {
        let user_id = format!("user_{}", i);
        let variant = ab_testing
            .allocate_user(&user_id, &experiment_id, user_properties.clone())
            .await?;
        println!("👤 User {} allocated to variant: {}", user_id, variant);

        // Track conversion
        if i % 3 == 0 {
            ab_testing
                .track_conversion(&user_id, &experiment_id, "theme_install_rate", 1.0)
                .await?;
            println!("📈 Conversion tracked for user {}", user_id);
        }
    }

    // Demo 3: Machine Learning Insights
    println!("\n🤖 Demo 3: Machine Learning Insights");
    println!("------------------------------------");

    // Predict user behavior
    let prediction_types = vec![
        PredictionType::ChurnProbability,
        PredictionType::LifetimeValue,
        PredictionType::FeatureAdoption,
    ];

    let user_prediction = ml_insights
        .predict_user_behavior("user_123", prediction_types)
        .await?;
    println!("🔮 User Behavior Predictions:");
    for (prediction_type, result) in &user_prediction.predictions {
        println!(
            "  - {}: {:.2} (confidence: {:.2})",
            prediction_type, result.value, result.confidence
        );
    }

    // Generate personalized recommendations
    let context = HashMap::from([
        (
            "current_theme".to_string(),
            serde_json::Value::String("dark".to_string()),
        ),
        (
            "usage_pattern".to_string(),
            serde_json::Value::String("developer".to_string()),
        ),
    ]);

    let recommendations = ml_insights
        .generate_personalized_recommendations("user_123", context)
        .await?;
    println!("💡 Personalized Recommendations:");
    for rec in &recommendations.recommendations {
        println!(
            "  - {} (score: {:.2}): {:?}",
            rec.item_id, rec.score, rec.reasoning
        );
    }

    // Detect anomalies
    let anomalies = ml_insights
        .detect_anomalies(chrono::Duration::hours(24))
        .await?;
    println!("⚠️  Anomalies Detected:");
    for anomaly in &anomalies.anomalies {
        println!(
            "  - {:?}: {} (severity: {:?})",
            anomaly.anomaly_type, anomaly.description, anomaly.severity
        );
    }

    // Analyze trends
    let trend_analysis = ml_insights
        .analyze_trends("daily_active_users", chrono::Duration::days(30))
        .await?;
    println!("📊 Trend Analysis for Daily Active Users:");
    println!("  - Direction: {:?}", trend_analysis.trend_direction);
    println!("  - Strength: {:.2}", trend_analysis.trend_strength);
    println!("  - Forecast points: {}", trend_analysis.forecast.len());

    // Demo 4: Export Capabilities
    println!("\n📤 Demo 4: Export Capabilities");
    println!("------------------------------");

    // Export analytics data to CSV
    let csv_export = ExportRequest {
        request_id: uuid::Uuid::new_v4().to_string(),
        format: ExportFormat::CSV,
        data_source: DataSource::Analytics,
        filters: vec![],
        columns: Some(vec![
            "date".to_string(),
            "users".to_string(),
            "sessions".to_string(),
            "revenue".to_string(),
        ]),
        time_range: Some(crate::export::TimeRange {
            start: chrono::Utc::now() - chrono::Duration::days(30),
            end: chrono::Utc::now(),
            timezone: Some("UTC".to_string()),
        }),
        template: None,
        destination: ExportDestination::LocalFile {
            path: PathBuf::from("/tmp/analytics_export.csv"),
        },
        compression: Some(crate::export::CompressionType::Gzip),
        encryption: None,
        metadata: HashMap::new(),
    };

    let csv_result = export_manager.export_data(csv_export).await?;
    println!("📊 CSV Export completed: {:?}", csv_result.file_path);
    println!("  - Status: {:?}", csv_result.status);
    println!("  - Rows: {:?}", csv_result.row_count);
    println!("  - Size: {:?} bytes", csv_result.file_size);

    // Export A/B test results to Excel
    let excel_export = ExportRequest {
        request_id: uuid::Uuid::new_v4().to_string(),
        format: ExportFormat::Excel,
        data_source: DataSource::ABTests,
        filters: vec![],
        columns: None,
        time_range: None,
        template: None,
        destination: ExportDestination::LocalFile {
            path: PathBuf::from("/tmp/ab_test_results.xlsx"),
        },
        compression: None,
        encryption: None,
        metadata: HashMap::new(),
    };

    let excel_result = export_manager.export_data(excel_export).await?;
    println!("📈 Excel Export completed: {:?}", excel_result.file_path);

    // Export user behavior data to JSON
    let json_export = ExportRequest {
        request_id: uuid::Uuid::new_v4().to_string(),
        format: ExportFormat::JSON,
        data_source: DataSource::UserBehavior,
        filters: vec![],
        columns: None,
        time_range: Some(crate::export::TimeRange {
            start: chrono::Utc::now() - chrono::Duration::days(7),
            end: chrono::Utc::now(),
            timezone: Some("UTC".to_string()),
        }),
        template: None,
        destination: ExportDestination::LocalFile {
            path: PathBuf::from("/tmp/user_behavior.json"),
        },
        compression: None,
        encryption: None,
        metadata: HashMap::new(),
    };

    let json_result = export_manager.export_data(json_export).await?;
    println!("📋 JSON Export completed: {:?}", json_result.file_path);

    // Demo 5: Custom Metrics
    println!("\n📏 Demo 5: Custom Metrics");
    println!("-------------------------");

    // Define a custom metric for theme switching frequency
    let theme_switch_metric = MetricDefinition {
        id: "theme_switch_frequency".to_string(),
        name: "Theme Switch Frequency".to_string(),
        description: "How often users switch between themes".to_string(),
        metric_type: CustomMetricType::Counter,
        data_type: crate::custom_metrics::MetricDataType::Integer,
        collection_method: CollectionMethod::Event,
        aggregation_rules: vec![crate::custom_metrics::AggregationRule {
            aggregation_type: crate::custom_metrics::AggregationType::Sum,
            time_window: chrono::Duration::hours(1),
            group_by: vec!["user_id".to_string()],
            filter: None,
        }],
        validation_rules: vec![],
        retention_policy: crate::custom_metrics::RetentionPolicy {
            raw_data_retention: chrono::Duration::days(30),
            aggregated_data_retention: HashMap::new(),
            compression_enabled: true,
            archival_storage: None,
        },
        tags: HashMap::from([
            ("category".to_string(), "user_behavior".to_string()),
            ("priority".to_string(), "high".to_string()),
        ]),
        dimensions: vec![
            crate::custom_metrics::MetricDimension {
                name: "user_id".to_string(),
                dimension_type: crate::custom_metrics::DimensionType::String,
                cardinality_limit: Some(100000),
                default_value: None,
            },
            crate::custom_metrics::MetricDimension {
                name: "from_theme".to_string(),
                dimension_type: crate::custom_metrics::DimensionType::Categorical,
                cardinality_limit: Some(100),
                default_value: Some("unknown".to_string()),
            },
            crate::custom_metrics::MetricDimension {
                name: "to_theme".to_string(),
                dimension_type: crate::custom_metrics::DimensionType::Categorical,
                cardinality_limit: Some(100),
                default_value: Some("unknown".to_string()),
            },
        ],
        alerts: vec![crate::custom_metrics::MetricAlert {
            alert_id: "high_switch_rate".to_string(),
            name: "High Theme Switch Rate".to_string(),
            condition: crate::custom_metrics::AlertCondition::GreaterThan,
            threshold: crate::custom_metrics::AlertThreshold {
                value: 10.0,
                duration: chrono::Duration::minutes(5),
                severity: crate::custom_metrics::AlertSeverity::Warning,
            },
            notification_channels: vec![crate::custom_metrics::NotificationChannel::Email {
                recipients: vec!["admin@warp.dev".to_string()],
            }],
            cooldown_period: chrono::Duration::minutes(15),
            enabled: true,
        }],
        created_by: "system".to_string(),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        enabled: true,
    };

    // Define the custom metric
    let metric_id = custom_metrics.define_metric(theme_switch_metric).await?;
    println!("📏 Custom metric defined: {}", metric_id);

    // Record some metric data points
    for i in 0..5 {
        let data_point = MetricDataPoint {
            metric_id: metric_id.clone(),
            value: MetricValue::Integer(1),
            dimensions: HashMap::from([
                ("user_id".to_string(), format!("user_{}", i)),
                ("from_theme".to_string(), "dark".to_string()),
                ("to_theme".to_string(), "light".to_string()),
            ]),
            timestamp: chrono::Utc::now(),
            source: "theme_engine".to_string(),
            metadata: HashMap::new(),
        };

        custom_metrics.record_metric(data_point).await?;
        println!("📊 Recorded theme switch for user_{}", i);
    }

    // Query the metric
    let query = crate::custom_metrics::MetricQuery {
        metric_id: metric_id.clone(),
        time_range: crate::custom_metrics::TimeRange {
            start: chrono::Utc::now() - chrono::Duration::hours(1),
            end: chrono::Utc::now(),
            interval: Some(chrono::Duration::minutes(10)),
        },
        aggregation: Some(crate::custom_metrics::AggregationType::Sum),
        group_by: vec!["user_id".to_string()],
        filters: vec![],
        limit: Some(10),
        offset: None,
    };

    let query_result = custom_metrics.query_metric(query).await?;
    println!("🔍 Query results:");
    println!("  - Total data points: {}", query_result.total_count);
    println!(
        "  - Query duration: {}ms",
        query_result.query_duration.as_millis()
    );

    // Get metric status
    let metric_status = custom_metrics.get_metric_status(&metric_id).await?;
    println!("📈 Metric status: {:?}", metric_status.status);
    println!("  - Collection count: {}", metric_status.collection_count);
    println!("  - Last updated: {}", metric_status.last_updated);

    // Define a performance metric
    let performance_metric = MetricDefinition {
        id: "plugin_load_time".to_string(),
        name: "Plugin Load Time".to_string(),
        description: "Time taken to load marketplace plugins".to_string(),
        metric_type: CustomMetricType::Timer,
        data_type: crate::custom_metrics::MetricDataType::Float,
        collection_method: CollectionMethod::Push,
        aggregation_rules: vec![
            crate::custom_metrics::AggregationRule {
                aggregation_type: crate::custom_metrics::AggregationType::Average,
                time_window: chrono::Duration::minutes(5),
                group_by: vec!["plugin_id".to_string()],
                filter: None,
            },
            crate::custom_metrics::AggregationRule {
                aggregation_type: crate::custom_metrics::AggregationType::Percentile(95.0),
                time_window: chrono::Duration::minutes(5),
                group_by: vec!["plugin_id".to_string()],
                filter: None,
            },
        ],
        validation_rules: vec![crate::custom_metrics::ValidationRule {
            rule_type: crate::custom_metrics::ValidationRuleType::Range {
                min: 0.0,
                max: 10000.0,
            },
            parameters: HashMap::new(),
            error_action: crate::custom_metrics::ValidationErrorAction::Log,
        }],
        retention_policy: crate::custom_metrics::RetentionPolicy {
            raw_data_retention: chrono::Duration::days(7),
            aggregated_data_retention: HashMap::from([
                (
                    crate::custom_metrics::AggregationType::Average,
                    chrono::Duration::days(90),
                ),
                (
                    crate::custom_metrics::AggregationType::Percentile(95.0),
                    chrono::Duration::days(90),
                ),
            ]),
            compression_enabled: true,
            archival_storage: Some(crate::custom_metrics::ArchivalConfig {
                storage_type: crate::custom_metrics::ArchivalStorageType::S3,
                compression_algorithm: crate::custom_metrics::CompressionAlgorithm::Zstd,
                encryption_enabled: true,
            }),
        },
        tags: HashMap::from([
            ("category".to_string(), "performance".to_string()),
            ("component".to_string(), "plugin_system".to_string()),
        ]),
        dimensions: vec![
            crate::custom_metrics::MetricDimension {
                name: "plugin_id".to_string(),
                dimension_type: crate::custom_metrics::DimensionType::String,
                cardinality_limit: Some(1000),
                default_value: None,
            },
            crate::custom_metrics::MetricDimension {
                name: "plugin_version".to_string(),
                dimension_type: crate::custom_metrics::DimensionType::String,
                cardinality_limit: Some(100),
                default_value: Some("unknown".to_string()),
            },
        ],
        alerts: vec![crate::custom_metrics::MetricAlert {
            alert_id: "slow_plugin_load".to_string(),
            name: "Slow Plugin Load Time".to_string(),
            condition: crate::custom_metrics::AlertCondition::GreaterThan,
            threshold: crate::custom_metrics::AlertThreshold {
                value: 1000.0, // 1 second
                duration: chrono::Duration::minutes(2),
                severity: crate::custom_metrics::AlertSeverity::Critical,
            },
            notification_channels: vec![crate::custom_metrics::NotificationChannel::Slack {
                webhook_url: "https://hooks.slack.com/services/...".to_string(),
                channel: "#performance-alerts".to_string(),
            }],
            cooldown_period: chrono::Duration::minutes(10),
            enabled: true,
        }],
        created_by: "performance_team".to_string(),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        enabled: true,
    };

    let perf_metric_id = custom_metrics.define_metric(performance_metric).await?;
    println!("⚡ Performance metric defined: {}", perf_metric_id);

    // Record some performance data
    let plugin_ids = ["git-enhanced", "docker-helper", "ai-assistant"];
    for (i, plugin_id) in plugin_ids.iter().enumerate() {
        let load_time = 200.0 + (i as f64 * 150.0); // Simulate different load times

        let data_point = MetricDataPoint {
            metric_id: perf_metric_id.clone(),
            value: MetricValue::Float(load_time),
            dimensions: HashMap::from([
                ("plugin_id".to_string(), plugin_id.to_string()),
                ("plugin_version".to_string(), "1.0.0".to_string()),
            ]),
            timestamp: chrono::Utc::now(),
            source: "plugin_loader".to_string(),
            metadata: HashMap::new(),
        };

        custom_metrics.record_metric(data_point).await?;
        println!(
            "⏱️  Recorded load time for {}: {:.1}ms",
            plugin_id, load_time
        );
    }

    // Start collection for all metrics
    custom_metrics.start_collection().await?;
    println!("🔄 Started metric collection");

    // Summary
    println!("\n🎉 Demo Summary");
    println!("===============");
    println!("✅ Development Tools:");
    println!("  - Test suite executed with {} tests", test_results.len());
    println!("  - Debug session active: {}", debug_session);
    println!("  - Profiling session: {}", profile_id);

    println!("✅ A/B Testing:");
    println!("  - Experiment created and running: {}", experiment_id);
    println!("  - 10 users allocated to variants");
    println!("  - Conversion tracking active");

    println!("✅ Machine Learning Insights:");
    println!("  - User behavior predictions generated");
    println!(
        "  - Personalized recommendations: {} items",
        recommendations.recommendations.len()
    );
    println!(
        "  - Anomaly detection: {} anomalies found",
        anomalies.anomalies.len()
    );
    println!("  - Trend analysis completed");

    println!("✅ Export Capabilities:");
    println!("  - CSV export: {:?}", csv_result.status);
    println!("  - Excel export: {:?}", excel_result.status);
    println!("  - JSON export: {:?}", json_result.status);

    println!("✅ Custom Metrics:");
    println!("  - Theme switch metric: {} data points", 5);
    println!(
        "  - Performance metric: {} plugins monitored",
        plugin_ids.len()
    );
    println!("  - Real-time collection active");

    println!("\n🚀 All systems operational! The Warp terminal now has:");
    println!("  📋 Comprehensive development tools with debugging, testing, and profiling");
    println!("  🧪 Advanced A/B testing framework for data-driven decisions");
    println!("  🤖 Machine learning insights for user behavior prediction and personalization");
    println!("  📤 Flexible export system supporting multiple formats and destinations");
    println!("  📏 Custom metrics platform with real-time monitoring and alerting");

    Ok(())
}
