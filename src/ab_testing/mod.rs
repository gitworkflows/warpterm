use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use serde::{Deserialize, Serialize};
use crate::error::WarpError;
use chrono::{DateTime, Utc, Duration};

pub mod experiment;
pub mod variant;
pub mod allocation;
pub mod metrics;
pub mod analysis;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ABTestingFramework {
    experiments: Arc<Mutex<HashMap<String, Experiment>>>,
    user_allocations: Arc<Mutex<HashMap<String, UserAllocation>>>,
    metrics_collector: Arc<metrics::MetricsCollector>,
    analyzer: Arc<analysis::StatisticalAnalyzer>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Experiment {
    pub id: String,
    pub name: String,
    pub description: String,
    pub status: ExperimentStatus,
    pub variants: Vec<Variant>,
    pub allocation_strategy: AllocationStrategy,
    pub target_metrics: Vec<TargetMetric>,
    pub start_date: DateTime<Utc>,
    pub end_date: Option<DateTime<Utc>>,
    pub sample_size: u32,
    pub confidence_level: f64,
    pub minimum_effect_size: f64,
    pub traffic_allocation: f64,
    pub filters: Vec<ExperimentFilter>,
    pub metadata: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExperimentStatus {
    Draft,
    Running,
    Paused,
    Completed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Variant {
    pub id: String,
    pub name: String,
    pub description: String,
    pub allocation_percentage: f64,
    pub configuration: VariantConfiguration,
    pub is_control: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VariantConfiguration {
    FeatureFlag { enabled: bool },
    ConfigValue { key: String, value: serde_json::Value },
    UIComponent { component_id: String, props: HashMap<String, serde_json::Value> },
    Algorithm { algorithm_id: String, parameters: HashMap<String, f64> },
    Theme { theme_id: String, customizations: HashMap<String, String> },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AllocationStrategy {
    Random,
    Deterministic { seed: u64 },
    Weighted { weights: HashMap<String, f64> },
    Cohort { cohort_field: String },
    Geographic { regions: Vec<String> },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetMetric {
    pub name: String,
    pub metric_type: MetricType,
    pub goal: MetricGoal,
    pub baseline_value: Option<f64>,
    pub minimum_detectable_effect: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MetricType {
    Conversion,
    Revenue,
    Engagement,
    Retention,
    Performance,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MetricGoal {
    Increase,
    Decrease,
    Maintain,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperimentFilter {
    pub field: String,
    pub operator: FilterOperator,
    pub value: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FilterOperator {
    Equals,
    NotEquals,
    GreaterThan,
    LessThan,
    Contains,
    In,
    NotIn,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAllocation {
    pub user_id: String,
    pub experiment_id: String,
    pub variant_id: String,
    pub allocated_at: DateTime<Utc>,
    pub session_id: String,
    pub user_properties: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperimentResult {
    pub experiment_id: String,
    pub variant_results: HashMap<String, VariantResult>,
    pub statistical_significance: StatisticalSignificance,
    pub recommendations: Vec<Recommendation>,
    pub confidence_intervals: HashMap<String, ConfidenceInterval>,
    pub sample_sizes: HashMap<String, u32>,
    pub duration: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariantResult {
    pub variant_id: String,
    pub metrics: HashMap<String, MetricResult>,
    pub sample_size: u32,
    pub conversion_rate: f64,
    pub revenue_per_user: f64,
    pub engagement_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricResult {
    pub metric_name: String,
    pub value: f64,
    pub improvement: f64,
    pub p_value: f64,
    pub confidence_interval: ConfidenceInterval,
    pub statistical_power: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatisticalSignificance {
    pub is_significant: bool,
    pub p_value: f64,
    pub confidence_level: f64,
    pub effect_size: f64,
    pub statistical_power: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfidenceInterval {
    pub lower_bound: f64,
    pub upper_bound: f64,
    pub confidence_level: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recommendation {
    pub recommendation_type: RecommendationType,
    pub title: String,
    pub description: String,
    pub confidence: f64,
    pub impact: RecommendationImpact,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecommendationType {
    LaunchVariant,
    ContinueTesting,
    StopExperiment,
    IncreaseTraffic,
    DecreaseTraffic,
    ExtendDuration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecommendationImpact {
    High,
    Medium,
    Low,
}

impl ABTestingFramework {
    pub async fn new() -> Result<Self, WarpError> {
        Ok(Self {
            experiments: Arc::new(Mutex::new(HashMap::new())),
            user_allocations: Arc::new(Mutex::new(HashMap::new())),
            metrics_collector: Arc::new(metrics::MetricsCollector::new().await?),
            analyzer: Arc::new(analysis::StatisticalAnalyzer::new().await?),
        })
    }

    pub async fn create_experiment(&self, experiment: Experiment) -> Result<String, WarpError> {
        let experiment_id = experiment.id.clone();
        
        // Validate experiment configuration
        self.validate_experiment(&experiment).await?;
        
        let mut experiments = self.experiments.lock().await;
        experiments.insert(experiment_id.clone(), experiment);
        
        Ok(experiment_id)
    }

    pub async fn start_experiment(&self, experiment_id: &str) -> Result<(), WarpError> {
        let mut experiments = self.experiments.lock().await;
        if let Some(experiment) = experiments.get_mut(experiment_id) {
            experiment.status = ExperimentStatus::Running;
            experiment.start_date = Utc::now();
        }
        Ok(())
    }

    pub async fn stop_experiment(&self, experiment_id: &str) -> Result<ExperimentResult, WarpError> {
        let mut experiments = self.experiments.lock().await;
        if let Some(experiment) = experiments.get_mut(experiment_id) {
            experiment.status = ExperimentStatus::Completed;
            experiment.end_date = Some(Utc::now());
            
            // Generate final results
            return self.analyze_experiment(experiment_id).await;
        }
        
        Err(WarpError::ConfigError(format!("Experiment not found: {}", experiment_id)))
    }

    pub async fn allocate_user(&self, user_id: &str, experiment_id: &str, user_properties: HashMap<String, serde_json::Value>) -> Result<String, WarpError> {
        let experiments = self.experiments.lock().await;
        let experiment = experiments.get(experiment_id)
            .ok_or_else(|| WarpError::ConfigError(format!("Experiment not found: {}", experiment_id)))?;

        // Check if user matches experiment filters
        if !self.user_matches_filters(user_id, &user_properties, &experiment.filters).await? {
            return Err(WarpError::ConfigError("User does not match experiment filters".to_string()));
        }

        // Allocate user to variant
        let variant_id = self.allocate_to_variant(user_id, experiment).await?;
        
        let allocation = UserAllocation {
            user_id: user_id.to_string(),
            experiment_id: experiment_id.to_string(),
            variant_id: variant_id.clone(),
            allocated_at: Utc::now(),
            session_id: uuid::Uuid::new_v4().to_string(),
            user_properties,
        };

        let mut allocations = self.user_allocations.lock().await;
        allocations.insert(format!("{}:{}", user_id, experiment_id), allocation);

        Ok(variant_id)
    }

    pub async fn get_user_variant(&self, user_id: &str, experiment_id: &str) -> Result<Option<String>, WarpError> {
        let allocations = self.user_allocations.lock().await;
        let key = format!("{}:{}", user_id, experiment_id);
        
        Ok(allocations.get(&key).map(|allocation| allocation.variant_id.clone()))
    }

    pub async fn track_conversion(&self, user_id: &str, experiment_id: &str, metric_name: &str, value: f64) -> Result<(), WarpError> {
        self.metrics_collector.track_conversion(user_id, experiment_id, metric_name, value).await
    }

    pub async fn analyze_experiment(&self, experiment_id: &str) -> Result<ExperimentResult, WarpError> {
        let experiments = self.experiments.lock().await;
        let experiment = experiments.get(experiment_id)
            .ok_or_else(|| WarpError::ConfigError(format!("Experiment not found: {}", experiment_id)))?;

        self.analyzer.analyze_experiment(experiment).await
    }

    pub async fn get_experiment_status(&self, experiment_id: &str) -> Result<ExperimentStatus, WarpError> {
        let experiments = self.experiments.lock().await;
        let experiment = experiments.get(experiment_id)
            .ok_or_else(|| WarpError::ConfigError(format!("Experiment not found: {}", experiment_id)))?;

        Ok(experiment.status.clone())
    }

    pub async fn list_experiments(&self) -> Result<Vec<Experiment>, WarpError> {
        let experiments = self.experiments.lock().await;
        Ok(experiments.values().cloned().collect())
    }

    async fn validate_experiment(&self, experiment: &Experiment) -> Result<(), WarpError> {
        // Validate allocation percentages sum to 100%
        let total_allocation: f64 = experiment.variants.iter()
            .map(|v| v.allocation_percentage)
            .sum();
        
        if (total_allocation - 100.0).abs() > 0.01 {
            return Err(WarpError::ConfigError("Variant allocation percentages must sum to 100%".to_string()));
        }

        // Validate at least one control variant
        if !experiment.variants.iter().any(|v| v.is_control) {
            return Err(WarpError::ConfigError("Experiment must have at least one control variant".to_string()));
        }

        // Validate target metrics
        if experiment.target_metrics.is_empty() {
            return Err(WarpError::ConfigError("Experiment must have at least one target metric".to_string()));
        }

        Ok(())
    }

    async fn user_matches_filters(&self, _user_id: &str, user_properties: &HashMap<String, serde_json::Value>, filters: &[ExperimentFilter]) -> Result<bool, WarpError> {
        for filter in filters {
            if let Some(property_value) = user_properties.get(&filter.field) {
                let matches = match filter.operator {
                    FilterOperator::Equals => property_value == &filter.value,
                    FilterOperator::NotEquals => property_value != &filter.value,
                    FilterOperator::GreaterThan => {
                        if let (Some(prop_num), Some(filter_num)) = (property_value.as_f64(), filter.value.as_f64()) {
                            prop_num > filter_num
                        } else {
                            false
                        }
                    }
                    FilterOperator::LessThan => {
                        if let (Some(prop_num), Some(filter_num)) = (property_value.as_f64(), filter.value.as_f64()) {
                            prop_num < filter_num
                        } else {
                            false
                        }
                    }
                    FilterOperator::Contains => {
                        if let (Some(prop_str), Some(filter_str)) = (property_value.as_str(), filter.value.as_str()) {
                            prop_str.contains(filter_str)
                        } else {
                            false
                        }
                    }
                    FilterOperator::In => {
                        if let Some(filter_array) = filter.value.as_array() {
                            filter_array.contains(property_value)
                        } else {
                            false
                        }
                    }
                    FilterOperator::NotIn => {
                        if let Some(filter_array) = filter.value.as_array() {
                            !filter_array.contains(property_value)
                        } else {
                            true
                        }
                    }
                };

                if !matches {
                    return Ok(false);
                }
            } else {
                return Ok(false);
            }
        }

        Ok(true)
    }

    async fn allocate_to_variant(&self, user_id: &str, experiment: &Experiment) -> Result<String, WarpError> {
        match &experiment.allocation_strategy {
            AllocationStrategy::Random => {
                let mut hasher = std::collections::hash_map::DefaultHasher::new();
                use std::hash::{Hash, Hasher};
                user_id.hash(&mut hasher);
                experiment.id.hash(&mut hasher);
                let hash = hasher.finish();
                
                let random_value = (hash % 10000) as f64 / 100.0;
                let mut cumulative = 0.0;
                
                for variant in &experiment.variants {
                    cumulative += variant.allocation_percentage;
                    if random_value < cumulative {
                        return Ok(variant.id.clone());
                    }
                }
                
                // Fallback to first variant
                Ok(experiment.variants[0].id.clone())
            }
            AllocationStrategy::Deterministic { seed } => {
                let mut hasher = std::collections::hash_map::DefaultHasher::new();
                use std::hash::{Hash, Hasher};
                user_id.hash(&mut hasher);
                seed.hash(&mut hasher);
                let hash = hasher.finish();
                
                let variant_index = (hash as usize) % experiment.variants.len();
                Ok(experiment.variants[variant_index].id.clone())
            }
            AllocationStrategy::Weighted { weights } => {
                let total_weight: f64 = weights.values().sum();
                let mut hasher = std::collections::hash_map::DefaultHasher::new();
                use std::hash::{Hash, Hasher};
                user_id.hash(&mut hasher);
                let hash = hasher.finish();
                
                let random_value = (hash % 10000) as f64 / 10000.0 * total_weight;
                let mut cumulative = 0.0;
                
                for variant in &experiment.variants {
                    if let Some(weight) = weights.get(&variant.id) {
                        cumulative += weight;
                        if random_value < cumulative {
                            return Ok(variant.id.clone());
                        }
                    }
                }
                
                Ok(experiment.variants[0].id.clone())
            }
            _ => {
                // For other strategies, use random allocation as fallback
                self.allocate_to_variant(user_id, &Experiment {
                    allocation_strategy: AllocationStrategy::Random,
                    ..experiment.clone()
                }).await
            }
        }
    }
}
