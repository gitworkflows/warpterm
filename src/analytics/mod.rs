use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc, Duration};
use crate::error::WarpError;

pub mod collector;
pub mod aggregator;
pub mod reporter;
pub mod dashboard;
pub mod metrics;
pub mod storage;
pub mod privacy;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsEvent {
    pub id: String,
    pub event_type: EventType,
    pub timestamp: DateTime<Utc>,
    pub user_id: Option<String>,
    pub session_id: String,
    pub item_id: Option<String>,
    pub metadata: HashMap<String, serde_json::Value>,
    pub performance_data: Option<PerformanceData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventType {
    // Marketplace Events
    ItemView,
    ItemInstall,
    ItemUninstall,
    ItemUpdate,
    ItemRating,
    ItemSearch,
    ItemDownload,
    
    // Usage Events
    ItemActivation,
    ItemDeactivation,
    ItemUsage,
    ItemError,
    ItemCrash,
    
    // Performance Events
    ItemLoadTime,
    ItemMemoryUsage,
    ItemCpuUsage,
    ItemNetworkUsage,
    
    // User Interaction Events
    UserLogin,
    UserLogout,
    UserPreferenceChange,
    UserFeedback,
    
    // System Events
    SystemStartup,
    SystemShutdown,
    SystemError,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceData {
    pub cpu_usage: f32,
    pub memory_usage: u64,
    pub disk_usage: u64,
    pub network_bytes_sent: u64,
    pub network_bytes_received: u64,
    pub load_time_ms: u64,
    pub response_time_ms: u64,
    pub error_count: u32,
    pub crash_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageMetrics {
    pub item_id: String,
    pub total_activations: u64,
    pub total_usage_time: Duration,
    pub average_session_duration: Duration,
    pub daily_active_users: u32,
    pub weekly_active_users: u32,
    pub monthly_active_users: u32,
    pub retention_rate: f32,
    pub crash_rate: f32,
    pub error_rate: f32,
    pub performance_score: f32,
    pub user_satisfaction: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub item_id: String,
    pub average_load_time: Duration,
    pub p95_load_time: Duration,
    pub p99_load_time: Duration,
    pub average_memory_usage: u64,
    pub peak_memory_usage: u64,
    pub average_cpu_usage: f32,
    pub peak_cpu_usage: f32,
    pub network_efficiency: f32,
    pub stability_score: f32,
    pub resource_efficiency: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserBehaviorMetrics {
    pub item_id: String,
    pub feature_usage: HashMap<String, u64>,
    pub user_flows: Vec<UserFlow>,
    pub drop_off_points: Vec<DropOffPoint>,
    pub engagement_score: f32,
    pub feature_adoption_rate: HashMap<String, f32>,
    pub user_journey_analysis: UserJourneyAnalysis,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserFlow {
    pub flow_id: String,
    pub steps: Vec<String>,
    pub completion_rate: f32,
    pub average_duration: Duration,
    pub drop_off_rate: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DropOffPoint {
    pub step: String,
    pub drop_off_rate: f32,
    pub reasons: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserJourneyAnalysis {
    pub onboarding_completion_rate: f32,
    pub time_to_first_value: Duration,
    pub feature_discovery_rate: f32,
    pub user_progression_stages: Vec<ProgressionStage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressionStage {
    pub stage_name: String,
    pub users_reached: u32,
    pub average_time_to_reach: Duration,
    pub completion_rate: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketplaceAnalytics {
    pub total_items: u32,
    pub total_downloads: u64,
    pub total_active_users: u32,
    pub category_distribution: HashMap<String, u32>,
    pub top_items: Vec<TopItem>,
    pub trending_items: Vec<TrendingItem>,
    pub revenue_metrics: RevenueMetrics,
    pub user_acquisition: UserAcquisitionMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopItem {
    pub item_id: String,
    pub name: String,
    pub downloads: u64,
    pub rating: f32,
    pub revenue: f64,
    pub growth_rate: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrendingItem {
    pub item_id: String,
    pub name: String,
    pub growth_rate: f32,
    pub velocity: f32,
    pub momentum_score: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevenueMetrics {
    pub total_revenue: f64,
    pub monthly_recurring_revenue: f64,
    pub average_revenue_per_user: f64,
    pub conversion_rate: f32,
    pub churn_rate: f32,
    pub lifetime_value: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAcquisitionMetrics {
    pub new_users: u32,
    pub acquisition_cost: f64,
    pub acquisition_channels: HashMap<String, u32>,
    pub conversion_funnel: ConversionFunnel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionFunnel {
    pub visitors: u32,
    pub signups: u32,
    pub activations: u32,
    pub first_purchase: u32,
    pub retention: u32,
}

pub struct AnalyticsEngine {
    collector: Arc<collector::EventCollector>,
    aggregator: Arc<Mutex<aggregator::MetricsAggregator>>,
    reporter: Arc<reporter::AnalyticsReporter>,
    storage: Arc<Mutex<storage::AnalyticsStorage>>,
    privacy_manager: Arc<privacy::PrivacyManager>,
    dashboard: Arc<Mutex<dashboard::AnalyticsDashboard>>,
}

impl AnalyticsEngine {
    pub async fn new() -> Result<Self, WarpError> {
        let collector = Arc::new(collector::EventCollector::new().await?);
        let aggregator = Arc::new(Mutex::new(aggregator::MetricsAggregator::new().await?));
        let reporter = Arc::new(reporter::AnalyticsReporter::new().await?);
        let storage = Arc::new(Mutex::new(storage::AnalyticsStorage::new().await?));
        let privacy_manager = Arc::new(privacy::PrivacyManager::new().await?);
        let dashboard = Arc::new(Mutex::new(dashboard::AnalyticsDashboard::new().await?));

        Ok(Self {
            collector,
            aggregator,
            reporter,
            storage,
            privacy_manager,
            dashboard,
        })
    }

    pub async fn track_event(&self, event: AnalyticsEvent) -> Result<(), WarpError> {
        // Privacy check
        if !self.privacy_manager.should_track_event(&event).await? {
            return Ok(());
        }

        // Collect the event
        self.collector.collect_event(event.clone()).await?;

        // Store the event
        let mut storage = self.storage.lock().await;
        storage.store_event(event).await?;

        Ok(())
    }

    pub async fn get_usage_metrics(&self, item_id: &str, time_range: TimeRange) -> Result<UsageMetrics, WarpError> {
        let aggregator = self.aggregator.lock().await;
        aggregator.get_usage_metrics(item_id, time_range).await
    }

    pub async fn get_performance_metrics(&self, item_id: &str, time_range: TimeRange) -> Result<PerformanceMetrics, WarpError> {
        let aggregator = self.aggregator.lock().await;
        aggregator.get_performance_metrics(item_id, time_range).await
    }

    pub async fn get_user_behavior_metrics(&self, item_id: &str, time_range: TimeRange) -> Result<UserBehaviorMetrics, WarpError> {
        let aggregator = self.aggregator.lock().await;
        aggregator.get_user_behavior_metrics(item_id, time_range).await
    }

    pub async fn get_marketplace_analytics(&self, time_range: TimeRange) -> Result<MarketplaceAnalytics, WarpError> {
        let aggregator = self.aggregator.lock().await;
        aggregator.get_marketplace_analytics(time_range).await
    }

    pub async fn generate_report(&self, report_type: ReportType, time_range: TimeRange) -> Result<AnalyticsReport, WarpError> {
        self.reporter.generate_report(report_type, time_range).await
    }

    pub async fn start_background_processing(&self) -> Result<(), WarpError> {
        // Start aggregation tasks
        let aggregator = self.aggregator.clone();
        tokio::spawn(async move {
            loop {
                if let Err(e) = Self::run_aggregation_cycle(aggregator.clone()).await {
                    log::error!("Aggregation cycle failed: {}", e);
                }
                tokio::time::sleep(tokio::time::Duration::from_secs(300)).await; // 5 minutes
            }
        });

        // Start reporting tasks
        let reporter = self.reporter.clone();
        tokio::spawn(async move {
            loop {
                if let Err(e) = reporter.generate_scheduled_reports().await {
                    log::error!("Scheduled reporting failed: {}", e);
                }
                tokio::time::sleep(tokio::time::Duration::from_secs(3600)).await; // 1 hour
            }
        });

        Ok(())
    }

    async fn run_aggregation_cycle(aggregator: Arc<Mutex<aggregator::MetricsAggregator>>) -> Result<(), WarpError> {
        let mut agg = aggregator.lock().await;
        agg.process_pending_events().await?;
        agg.update_real_time_metrics().await?;
        agg.cleanup_old_data().await?;
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TimeRange {
    LastHour,
    LastDay,
    LastWeek,
    LastMonth,
    LastYear,
    Custom { start: DateTime<Utc>, end: DateTime<Utc> },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReportType {
    UsageSummary,
    PerformanceReport,
    UserBehaviorAnalysis,
    MarketplaceOverview,
    ItemComparison,
    TrendAnalysis,
    CustomReport { metrics: Vec<String> },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsReport {
    pub report_type: ReportType,
    pub time_range: TimeRange,
    pub generated_at: DateTime<Utc>,
    pub summary: ReportSummary,
    pub sections: Vec<ReportSection>,
    pub recommendations: Vec<Recommendation>,
    pub export_formats: Vec<ExportFormat>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportSummary {
    pub key_metrics: HashMap<String, serde_json::Value>,
    pub trends: Vec<Trend>,
    pub alerts: Vec<Alert>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportSection {
    pub title: String,
    pub content: SectionContent,
    pub charts: Vec<Chart>,
    pub tables: Vec<Table>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SectionContent {
    Text(String),
    Metrics(HashMap<String, serde_json::Value>),
    Analysis(String),
    Comparison(ComparisonData),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComparisonData {
    pub items: Vec<String>,
    pub metrics: Vec<String>,
    pub data: HashMap<String, HashMap<String, f64>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chart {
    pub chart_type: ChartType,
    pub title: String,
    pub data: ChartData,
    pub options: ChartOptions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChartType {
    Line,
    Bar,
    Pie,
    Scatter,
    Heatmap,
    Funnel,
    Gauge,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartData {
    pub labels: Vec<String>,
    pub datasets: Vec<Dataset>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dataset {
    pub label: String,
    pub data: Vec<f64>,
    pub color: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartOptions {
    pub responsive: bool,
    pub animation: bool,
    pub legend: bool,
    pub tooltip: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Table {
    pub title: String,
    pub headers: Vec<String>,
    pub rows: Vec<Vec<String>>,
    pub sortable: bool,
    pub filterable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trend {
    pub metric: String,
    pub direction: TrendDirection,
    pub magnitude: f32,
    pub significance: TrendSignificance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TrendDirection {
    Up,
    Down,
    Stable,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TrendSignificance {
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alert {
    pub alert_type: AlertType,
    pub severity: AlertSeverity,
    pub message: String,
    pub item_id: Option<String>,
    pub threshold: f64,
    pub current_value: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertType {
    PerformanceDegradation,
    HighErrorRate,
    LowUsage,
    SecurityIssue,
    ResourceExhaustion,
    UserExperienceIssue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertSeverity {
    Critical,
    Warning,
    Info,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recommendation {
    pub recommendation_type: RecommendationType,
    pub title: String,
    pub description: String,
    pub impact: RecommendationImpact,
    pub effort: RecommendationEffort,
    pub actions: Vec<RecommendationAction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecommendationType {
    Performance,
    UserExperience,
    Monetization,
    Growth,
    Retention,
    Technical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecommendationImpact {
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecommendationEffort {
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendationAction {
    pub action: String,
    pub priority: u8,
    pub estimated_impact: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExportFormat {
    PDF,
    CSV,
    JSON,
    Excel,
    HTML,
}
