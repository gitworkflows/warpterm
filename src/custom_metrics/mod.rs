use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use serde::{Deserialize, Serialize};
use crate::error::WarpError;

pub mod definitions;
pub mod collectors;
pub mod processors;
pub mod validators;
pub mod aggregators;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomMetricsManager {
    metric_definitions: Arc<Mutex<HashMap<String, MetricDefinition>>>,
    collectors: Arc<Mutex<HashMap<String, Box<dyn MetricCollector>>>>,
    processors: Arc<processors::MetricProcessor>,
    validators: Arc<validators::MetricValidator>,
    aggregators: Arc<aggregators::MetricAggregator>,
    active_metrics: Arc<Mutex<HashMap<String, ActiveMetric>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricDefinition {
    pub id: String,
    pub name: String,
    pub description: String,
    pub metric_type: MetricType,
    pub data_type: MetricDataType,
    pub collection_method: CollectionMethod,
    pub aggregation_rules: Vec<AggregationRule>,
    pub validation_rules: Vec<ValidationRule>,
    pub retention_policy: RetentionPolicy,
    pub tags: HashMap<String, String>,
    pub dimensions: Vec<MetricDimension>,
    pub alerts: Vec<MetricAlert>,
    pub created_by: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MetricType {
    Counter,
    Gauge,
    Histogram,
    Timer,
    Rate,
    Percentage,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MetricDataType {
    Integer,
    Float,
    Boolean,
    String,
    JSON,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CollectionMethod {
    Push,
    Pull,
    Event,
    Calculated,
    External { endpoint: String, interval: chrono::Duration },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregationRule {
    pub aggregation_type: AggregationType,
    pub time_window: chrono::Duration,
    pub group_by: Vec<String>,
    pub filter: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AggregationType {
    Sum,
    Average,
    Count,
    Min,
    Max,
    Median,
    Percentile(f64),
    StandardDeviation,
    Variance,
    Rate,
    Delta,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationRule {
    pub rule_type: ValidationRuleType,
    pub parameters: HashMap<String, serde_json::Value>,
    pub error_action: ValidationErrorAction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValidationRuleType {
    Range { min: f64, max: f64 },
    Pattern { regex: String },
    Required,
    DataType,
    Custom { function: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValidationErrorAction {
    Reject,
    Log,
    Transform,
    Alert,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetentionPolicy {
    pub raw_data_retention: chrono::Duration,
    pub aggregated_data_retention: HashMap<AggregationType, chrono::Duration>,
    pub compression_enabled: bool,
    pub archival_storage: Option<ArchivalConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchivalConfig {
    pub storage_type: ArchivalStorageType,
    pub compression_algorithm: CompressionAlgorithm,
    pub encryption_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ArchivalStorageType {
    S3,
    GCS,
    Azure,
    Local,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CompressionAlgorithm {
    Gzip,
    Lz4,
    Zstd,
    Snappy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricDimension {
    pub name: String,
    pub dimension_type: DimensionType,
    pub cardinality_limit: Option<u32>,
    pub default_value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DimensionType {
    String,
    Categorical,
    Numeric,
    Boolean,
    Timestamp,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricAlert {
    pub alert_id: String,
    pub name: String,
    pub condition: AlertCondition,
    pub threshold: AlertThreshold,
    pub notification_channels: Vec<NotificationChannel>,
    pub cooldown_period: chrono::Duration,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertCondition {
    GreaterThan,
    LessThan,
    Equals,
    NotEquals,
    PercentageChange,
    AnomalyDetection,
    Custom { expression: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertThreshold {
    pub value: f64,
    pub duration: chrono::Duration,
    pub severity: AlertSeverity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertSeverity {
    Critical,
    Warning,
    Info,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NotificationChannel {
    Email { recipients: Vec<String> },
    Slack { webhook_url: String, channel: String },
    Discord { webhook_url: String },
    Webhook { url: String, headers: HashMap<String, String> },
    SMS { phone_numbers: Vec<String> },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveMetric {
    pub metric_id: String,
    pub current_value: MetricValue,
    pub last_updated: chrono::DateTime<chrono::Utc>,
    pub collection_count: u64,
    pub error_count: u64,
    pub status: MetricStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MetricValue {
    Integer(i64),
    Float(f64),
    Boolean(bool),
    String(String),
    JSON(serde_json::Value),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MetricStatus {
    Active,
    Inactive,
    Error(String),
    Paused,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricDataPoint {
    pub metric_id: String,
    pub value: MetricValue,
    pub dimensions: HashMap<String, String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub source: String,
    pub metadata: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricQuery {
    pub metric_id: String,
    pub time_range: TimeRange,
    pub aggregation: Option<AggregationType>,
    pub group_by: Vec<String>,
    pub filters: Vec<MetricFilter>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeRange {
    pub start: chrono::DateTime<chrono::Utc>,
    pub end: chrono::DateTime<chrono::Utc>,
    pub interval: Option<chrono::Duration>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricFilter {
    pub dimension: String,
    pub operator: FilterOperator,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FilterOperator {
    Equals,
    NotEquals,
    Contains,
    StartsWith,
    EndsWith,
    In,
    NotIn,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricQueryResult {
    pub metric_id: String,
    pub data_points: Vec<AggregatedDataPoint>,
    pub total_count: u64,
    pub query_duration: std::time::Duration,
    pub metadata: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregatedDataPoint {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub value: f64,
    pub dimensions: HashMap<String, String>,
    pub sample_count: u64,
}

pub trait MetricCollector: Send + Sync {
    async fn collect(&self, metric_id: &str) -> Result<Vec<MetricDataPoint>, WarpError>;
    fn collection_interval(&self) -> chrono::Duration;
    fn supports_metric_type(&self, metric_type: &MetricType) -> bool;
}

impl CustomMetricsManager {
    pub async fn new() -> Result<Self, WarpError> {
        Ok(Self {
            metric_definitions: Arc::new(Mutex::new(HashMap::new())),
            collectors: Arc::new(Mutex::new(HashMap::new())),
            processors: Arc::new(processors::MetricProcessor::new().await?),
            validators: Arc::new(validators::MetricValidator::new().await?),
            aggregators: Arc::new(aggregators::MetricAggregator::new().await?),
            active_metrics: Arc::new(Mutex::new(HashMap::new())),
        })
    }

    pub async fn define_metric(&self, definition: MetricDefinition) -> Result<String, WarpError> {
        // Validate the metric definition
        self.validators.validate_definition(&definition).await?;

        let metric_id = definition.id.clone();
        
        // Store the definition
        {
            let mut definitions = self.metric_definitions.lock().await;
            definitions.insert(metric_id.clone(), definition.clone());
        }

        // Initialize active metric
        {
            let mut active_metrics = self.active_metrics.lock().await;
            active_metrics.insert(metric_id.clone(), ActiveMetric {
                metric_id: metric_id.clone(),
                current_value: MetricValue::Float(0.0),
                last_updated: chrono::Utc::now(),
                collection_count: 0,
                error_count: 0,
                status: MetricStatus::Active,
            });
        }

        // Set up collection if needed
        if definition.enabled {
            self.setup_collection(&definition).await?;
        }

        Ok(metric_id)
    }

    pub async fn update_metric_definition(&self, metric_id: &str, definition: MetricDefinition) -> Result<(), WarpError> {
        self.validators.validate_definition(&definition).await?;

        let mut definitions = self.metric_definitions.lock().await;
        if definitions.contains_key(metric_id) {
            definitions.insert(metric_id.to_string(), definition);
            Ok(())
        } else {
            Err(WarpError::ConfigError(format!("Metric not found: {}", metric_id)))
        }
    }

    pub async fn delete_metric(&self, metric_id: &str) -> Result<(), WarpError> {
        // Remove from definitions
        {
            let mut definitions = self.metric_definitions.lock().await;
            definitions.remove(metric_id);
        }

        // Remove from active metrics
        {
            let mut active_metrics = self.active_metrics.lock().await;
            active_metrics.remove(metric_id);
        }

        // Remove collector
        {
            let mut collectors = self.collectors.lock().await;
            collectors.remove(metric_id);
        }

        Ok(())
    }

    pub async fn record_metric(&self, data_point: MetricDataPoint) -> Result<(), WarpError> {
        // Validate the data point
        self.validators.validate_data_point(&data_point).await?;

        // Process the data point
        let processed_point = self.processors.process_data_point(data_point).await?;

        // Update active metric
        {
            let mut active_metrics = self.active_metrics.lock().await;
            if let Some(active_metric) = active_metrics.get_mut(&processed_point.metric_id) {
                active_metric.current_value = processed_point.value.clone();
                active_metric.last_updated = processed_point.timestamp;
                active_metric.collection_count += 1;
            }
        }

        // Store the data point
        self.aggregators.store_data_point(processed_point).await?;

        Ok(())
    }

    pub async fn query_metric(&self, query: MetricQuery) -> Result<MetricQueryResult, WarpError> {
        let start_time = std::time::Instant::now();
        
        // Execute the query
        let data_points = self.aggregators.query_data_points(&query).await?;
        
        let query_duration = start_time.elapsed();
        
        Ok(MetricQueryResult {
            metric_id: query.metric_id,
            data_points,
            total_count: data_points.len() as u64,
            query_duration,
            metadata: HashMap::new(),
        })
    }

    pub async fn get_metric_definition(&self, metric_id: &str) -> Result<MetricDefinition, WarpError> {
        let definitions = self.metric_definitions.lock().await;
        definitions.get(metric_id)
            .cloned()
            .ok_or_else(|| WarpError::ConfigError(format!("Metric not found: {}", metric_id)))
    }

    pub async fn list_metrics(&self) -> Result<Vec<MetricDefinition>, WarpError> {
        let definitions = self.metric_definitions.lock().await;
        Ok(definitions.values().cloned().collect())
    }

    pub async fn get_metric_status(&self, metric_id: &str) -> Result<ActiveMetric, WarpError> {
        let active_metrics = self.active_metrics.lock().await;
        active_metrics.get(metric_id)
            .cloned()
            .ok_or_else(|| WarpError::ConfigError(format!("Active metric not found: {}", metric_id)))
    }

    pub async fn enable_metric(&self, metric_id: &str) -> Result<(), WarpError> {
        let mut definitions = self.metric_definitions.lock().await;
        if let Some(definition) = definitions.get_mut(metric_id) {
            definition.enabled = true;
            self.setup_collection(definition).await?;
        }
        Ok(())
    }

    pub async fn disable_metric(&self, metric_id: &str) -> Result<(), WarpError> {
        let mut definitions = self.metric_definitions.lock().await;
        if let Some(definition) = definitions.get_mut(metric_id) {
            definition.enabled = false;
            
            // Remove collector
            let mut collectors = self.collectors.lock().await;
            collectors.remove(metric_id);
        }
        Ok(())
    }

    pub async fn register_collector(&self, metric_id: &str, collector: Box<dyn MetricCollector>) -> Result<(), WarpError> {
        let mut collectors = self.collectors.lock().await;
        collectors.insert(metric_id.to_string(), collector);
        Ok(())
    }

    pub async fn start_collection(&self) -> Result<(), WarpError> {
        let collectors = self.collectors.lock().await;
        
        for (metric_id, collector) in collectors.iter() {
            let metric_id = metric_id.clone();
            let interval = collector.collection_interval();
            
            // Start collection task
            tokio::spawn(async move {
                let mut interval_timer = tokio::time::interval(interval.to_std().unwrap_or(std::time::Duration::from_secs(60)));
                
                loop {
                    interval_timer.tick().await;
                    
                    // Collect metrics
                    if let Ok(data_points) = collector.collect(&metric_id).await {
                        for data_point in data_points {
                            // In a real implementation, send to processing pipeline
                            log::debug!("Collected metric data point: {:?}", data_point);
                        }
                    }
                }
            });
        }
        
        Ok(())
    }

    async fn setup_collection(&self, definition: &MetricDefinition) -> Result<(), WarpError> {
        match &definition.collection_method {
            CollectionMethod::Pull => {
                // Set up pull-based collection
                let collector = collectors::PullCollector::new(&definition.id).await?;
                self.register_collector(&definition.id, Box::new(collector)).await?;
            }
            CollectionMethod::Event => {
                // Set up event-based collection
                let collector = collectors::EventCollector::new(&definition.id).await?;
                self.register_collector(&definition.id, Box::new(collector)).await?;
            }
            CollectionMethod::External { endpoint, interval } => {
                // Set up external collection
                let collector = collectors::ExternalCollector::new(&definition.id, endpoint, *interval).await?;
                self.register_collector(&definition.id, Box::new(collector)).await?;
            }
            _ => {
                // For push and calculated methods, no automatic collection setup needed
            }
        }
        
        Ok(())
    }

    pub async fn calculate_derived_metrics(&self) -> Result<(), WarpError> {
        let definitions = self.metric_definitions.lock().await;
        
        for definition in definitions.values() {
            if matches!(definition.collection_method, CollectionMethod::Calculated) {
                // Calculate derived metric value
                let calculated_value = self.calculate_metric_value(definition).await?;
                
                let data_point = MetricDataPoint {
                    metric_id: definition.id.clone(),
                    value: calculated_value,
                    dimensions: HashMap::new(),
                    timestamp: chrono::Utc::now(),
                    source: "calculated".to_string(),
                    metadata: HashMap::new(),
                };
                
                self.record_metric(data_point).await?;
            }
        }
        
        Ok(())
    }

    async fn calculate_metric_value(&self, _definition: &MetricDefinition) -> Result<MetricValue, WarpError> {
        // In a real implementation, this would evaluate the calculation expression
        // For now, return a mock calculated value
        Ok(MetricValue::Float(42.0))
    }

    pub async fn trigger_alerts(&self) -> Result<(), WarpError> {
        let definitions = self.metric_definitions.lock().await;
        let active_metrics = self.active_metrics.lock().await;
        
        for definition in definitions.values() {
            if let Some(active_metric) = active_metrics.get(&definition.id) {
                for alert in &definition.alerts {
                    if alert.enabled {
                        let should_trigger = self.evaluate_alert_condition(alert, active_metric).await?;
                        
                        if should_trigger {
                            self.send_alert_notifications(alert, &definition.name, &active_metric.current_value).await?;
                        }
                    }
                }
            }
        }
        
        Ok(())
    }

    async fn evaluate_alert_condition(&self, alert: &MetricAlert, active_metric: &ActiveMetric) -> Result<bool, WarpError> {
        let current_value = match &active_metric.current_value {
            MetricValue::Float(v) => *v,
            MetricValue::Integer(v) => *v as f64,
            _ => return Ok(false),
        };
        
        match alert.condition {
            AlertCondition::GreaterThan => Ok(current_value > alert.threshold.value),
            AlertCondition::LessThan => Ok(current_value < alert.threshold.value),
            AlertCondition::Equals => Ok((current_value - alert.threshold.value).abs() < f64::EPSILON),
            AlertCondition::NotEquals => Ok((current_value - alert.threshold.value).abs() > f64::EPSILON),
            _ => Ok(false), // Other conditions would be implemented
        }
    }

    async fn send_alert_notifications(&self, alert: &MetricAlert, metric_name: &str, current_value: &MetricValue) -> Result<(), WarpError> {
        for channel in &alert.notification_channels {
            match channel {
                NotificationChannel::Email { recipients } => {
                    log::info!("Sending email alert to {:?} for metric {} with value {:?}", recipients, metric_name, current_value);
                }
                NotificationChannel::Slack { webhook_url, channel } => {
                    log::info!("Sending Slack alert to {} ({}) for metric {} with value {:?}", channel, webhook_url, metric_name, current_value);
                }
                _ => {
                    log::info!("Sending alert notification for metric {} with value {:?}", metric_name, current_value);
                }
            }
        }
        
        Ok(())
    }
}
