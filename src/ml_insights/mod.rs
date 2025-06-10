use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use serde::{Deserialize, Serialize};
use crate::error::WarpError;

pub mod models;
pub mod features;
pub mod predictions;
pub mod recommendations;
pub mod clustering;
pub mod anomaly_detection;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MLInsightsEngine {
    models: Arc<Mutex<HashMap<String, models::MLModel>>>,
    feature_store: Arc<features::FeatureStore>,
    predictor: Arc<predictions::Predictor>,
    recommender: Arc<recommendations::RecommendationEngine>,
    clusterer: Arc<clustering::UserClusterer>,
    anomaly_detector: Arc<anomaly_detection::AnomalyDetector>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserBehaviorPrediction {
    pub user_id: String,
    pub predictions: HashMap<String, PredictionResult>,
    pub confidence_scores: HashMap<String, f64>,
    pub generated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictionResult {
    pub prediction_type: PredictionType,
    pub value: f64,
    pub confidence: f64,
    pub factors: Vec<PredictionFactor>,
    pub time_horizon: chrono::Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PredictionType {
    ChurnProbability,
    LifetimeValue,
    NextPurchaseTime,
    FeatureAdoption,
    UsagePattern,
    ConversionProbability,
    EngagementScore,
    RetentionProbability,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictionFactor {
    pub feature_name: String,
    pub importance: f64,
    pub direction: FactorDirection,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FactorDirection {
    Positive,
    Negative,
    Neutral,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalizedRecommendation {
    pub user_id: String,
    pub recommendations: Vec<RecommendationItem>,
    pub explanation: String,
    pub confidence: f64,
    pub generated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendationItem {
    pub item_id: String,
    pub item_type: RecommendationItemType,
    pub score: f64,
    pub reasoning: Vec<String>,
    pub expected_impact: ExpectedImpact,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecommendationItemType {
    MarketplaceItem,
    Feature,
    Setting,
    Workflow,
    Tutorial,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpectedImpact {
    pub engagement_lift: f64,
    pub retention_improvement: f64,
    pub satisfaction_increase: f64,
    pub productivity_gain: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserCluster {
    pub cluster_id: String,
    pub name: String,
    pub description: String,
    pub characteristics: Vec<ClusterCharacteristic>,
    pub size: u32,
    pub representative_users: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterCharacteristic {
    pub feature: String,
    pub value: f64,
    pub importance: f64,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnomalyDetectionResult {
    pub anomalies: Vec<Anomaly>,
    pub overall_score: f64,
    pub detection_timestamp: chrono::DateTime<chrono::Utc>,
    pub model_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Anomaly {
    pub anomaly_type: AnomalyType,
    pub severity: AnomalySeverity,
    pub score: f64,
    pub description: String,
    pub affected_entities: Vec<String>,
    pub suggested_actions: Vec<String>,
    pub context: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnomalyType {
    UsageSpike,
    UsageDrop,
    PerformanceDegradation,
    ErrorRateIncrease,
    UnusualUserBehavior,
    SecurityThreat,
    SystemOverload,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnomalySeverity {
    Critical,
    High,
    Medium,
    Low,
    Info,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrendAnalysis {
    pub metric_name: String,
    pub trend_direction: TrendDirection,
    pub trend_strength: f64,
    pub seasonality: Option<SeasonalityPattern>,
    pub forecast: Vec<ForecastPoint>,
    pub confidence_intervals: Vec<ConfidenceInterval>,
    pub change_points: Vec<ChangePoint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TrendDirection {
    Increasing,
    Decreasing,
    Stable,
    Cyclical,
    Volatile,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeasonalityPattern {
    pub pattern_type: SeasonalityType,
    pub period: chrono::Duration,
    pub amplitude: f64,
    pub phase: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SeasonalityType {
    Daily,
    Weekly,
    Monthly,
    Quarterly,
    Yearly,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForecastPoint {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub value: f64,
    pub lower_bound: f64,
    pub upper_bound: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangePoint {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub magnitude: f64,
    pub confidence: f64,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfidenceInterval {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub lower_bound: f64,
    pub upper_bound: f64,
    pub confidence_level: f64,
}

impl MLInsightsEngine {
    pub async fn new() -> Result<Self, WarpError> {
        Ok(Self {
            models: Arc::new(Mutex::new(HashMap::new())),
            feature_store: Arc::new(features::FeatureStore::new().await?),
            predictor: Arc::new(predictions::Predictor::new().await?),
            recommender: Arc::new(recommendations::RecommendationEngine::new().await?),
            clusterer: Arc::new(clustering::UserClusterer::new().await?),
            anomaly_detector: Arc::new(anomaly_detection::AnomalyDetector::new().await?),
        })
    }

    pub async fn predict_user_behavior(&self, user_id: &str, prediction_types: Vec<PredictionType>) -> Result<UserBehaviorPrediction, WarpError> {
        let user_features = self.feature_store.get_user_features(user_id).await?;
        let mut predictions = HashMap::new();
        let mut confidence_scores = HashMap::new();

        for prediction_type in prediction_types {
            let result = self.predictor.predict(&prediction_type, &user_features).await?;
            confidence_scores.insert(format!("{:?}", prediction_type), result.confidence);
            predictions.insert(format!("{:?}", prediction_type), result);
        }

        Ok(UserBehaviorPrediction {
            user_id: user_id.to_string(),
            predictions,
            confidence_scores,
            generated_at: chrono::Utc::now(),
        })
    }

    pub async fn generate_personalized_recommendations(&self, user_id: &str, context: HashMap<String, serde_json::Value>) -> Result<PersonalizedRecommendation, WarpError> {
        let user_features = self.feature_store.get_user_features(user_id).await?;
        let user_cluster = self.clusterer.get_user_cluster(user_id).await?;
        
        let recommendations = self.recommender.generate_recommendations(
            user_id,
            &user_features,
            &user_cluster,
            &context,
        ).await?;

        Ok(recommendations)
    }

    pub async fn detect_anomalies(&self, time_window: chrono::Duration) -> Result<AnomalyDetectionResult, WarpError> {
        self.anomaly_detector.detect_anomalies(time_window).await
    }

    pub async fn analyze_trends(&self, metric_name: &str, time_range: chrono::Duration) -> Result<TrendAnalysis, WarpError> {
        let historical_data = self.feature_store.get_metric_history(metric_name, time_range).await?;
        
        // Perform trend analysis
        let trend_direction = self.calculate_trend_direction(&historical_data);
        let trend_strength = self.calculate_trend_strength(&historical_data);
        let seasonality = self.detect_seasonality(&historical_data);
        let forecast = self.generate_forecast(&historical_data, chrono::Duration::days(30)).await?;
        let confidence_intervals = self.calculate_confidence_intervals(&forecast);
        let change_points = self.detect_change_points(&historical_data);

        Ok(TrendAnalysis {
            metric_name: metric_name.to_string(),
            trend_direction,
            trend_strength,
            seasonality,
            forecast,
            confidence_intervals,
            change_points,
        })
    }

    pub async fn cluster_users(&self) -> Result<Vec<UserCluster>, WarpError> {
        self.clusterer.perform_clustering().await
    }

    pub async fn get_feature_importance(&self, model_name: &str) -> Result<Vec<PredictionFactor>, WarpError> {
        let models = self.models.lock().await;
        if let Some(model) = models.get(model_name) {
            model.get_feature_importance().await
        } else {
            Err(WarpError::ConfigError(format!("Model not found: {}", model_name)))
        }
    }

    pub async fn retrain_model(&self, model_name: &str) -> Result<(), WarpError> {
        let mut models = self.models.lock().await;
        if let Some(model) = models.get_mut(model_name) {
            let training_data = self.feature_store.get_training_data(model_name).await?;
            model.retrain(&training_data).await?;
        }
        Ok(())
    }

    pub async fn evaluate_model_performance(&self, model_name: &str) -> Result<models::ModelPerformance, WarpError> {
        let models = self.models.lock().await;
        if let Some(model) = models.get(model_name) {
            let test_data = self.feature_store.get_test_data(model_name).await?;
            model.evaluate(&test_data).await
        } else {
            Err(WarpError::ConfigError(format!("Model not found: {}", model_name)))
        }
    }

    // Helper methods for trend analysis
    fn calculate_trend_direction(&self, data: &[(chrono::DateTime<chrono::Utc>, f64)]) -> TrendDirection {
        if data.len() < 2 {
            return TrendDirection::Stable;
        }

        let first_half_avg = data[..data.len()/2].iter().map(|(_, v)| v).sum::<f64>() / (data.len()/2) as f64;
        let second_half_avg = data[data.len()/2..].iter().map(|(_, v)| v).sum::<f64>() / (data.len() - data.len()/2) as f64;
        
        let change_ratio = (second_half_avg - first_half_avg) / first_half_avg;
        
        if change_ratio > 0.05 {
            TrendDirection::Increasing
        } else if change_ratio < -0.05 {
            TrendDirection::Decreasing
        } else {
            TrendDirection::Stable
        }
    }

    fn calculate_trend_strength(&self, data: &[(chrono::DateTime<chrono::Utc>, f64)]) -> f64 {
        if data.len() < 2 {
            return 0.0;
        }

        // Calculate linear regression slope as trend strength
        let n = data.len() as f64;
        let sum_x: f64 = (0..data.len()).map(|i| i as f64).sum();
        let sum_y: f64 = data.iter().map(|(_, v)| v).sum();
        let sum_xy: f64 = data.iter().enumerate().map(|(i, (_, v))| i as f64 * v).sum();
        let sum_x2: f64 = (0..data.len()).map(|i| (i as f64).powi(2)).sum();

        let slope = (n * sum_xy - sum_x * sum_y) / (n * sum_x2 - sum_x.powi(2));
        slope.abs()
    }

    fn detect_seasonality(&self, _data: &[(chrono::DateTime<chrono::Utc>, f64)]) -> Option<SeasonalityPattern> {
        // Simplified seasonality detection
        // In a real implementation, this would use FFT or autocorrelation
        Some(SeasonalityPattern {
            pattern_type: SeasonalityType::Weekly,
            period: chrono::Duration::days(7),
            amplitude: 0.1,
            phase: 0.0,
        })
    }

    async fn generate_forecast(&self, data: &[(chrono::DateTime<chrono::Utc>, f64)], horizon: chrono::Duration) -> Result<Vec<ForecastPoint>, WarpError> {
        // Simplified forecasting using linear extrapolation
        if data.is_empty() {
            return Ok(vec![]);
        }

        let last_point = data.last().unwrap();
        let trend_strength = self.calculate_trend_strength(data);
        let last_value = last_point.1;
        
        let mut forecast = Vec::new();
        let steps = horizon.num_hours() / 24; // Daily forecast points
        
        for i in 1..=steps {
            let timestamp = last_point.0 + chrono::Duration::days(i);
            let predicted_value = last_value + (trend_strength * i as f64);
            let uncertainty = 0.1 * predicted_value * (i as f64).sqrt(); // Increasing uncertainty
            
            forecast.push(ForecastPoint {
                timestamp,
                value: predicted_value,
                lower_bound: predicted_value - uncertainty,
                upper_bound: predicted_value + uncertainty,
            });
        }

        Ok(forecast)
    }

    fn calculate_confidence_intervals(&self, forecast: &[ForecastPoint]) -> Vec<ConfidenceInterval> {
        forecast.iter().map(|point| {
            ConfidenceInterval {
                timestamp: point.timestamp,
                lower_bound: point.lower_bound,
                upper_bound: point.upper_bound,
                confidence_level: 0.95,
            }
        }).collect()
    }

    fn detect_change_points(&self, data: &[(chrono::DateTime<chrono::Utc>, f64)]) -> Vec<ChangePoint> {
        // Simplified change point detection
        let mut change_points = Vec::new();
        
        if data.len() < 10 {
            return change_points;
        }

        let window_size = data.len() / 5;
        for i in window_size..(data.len() - window_size) {
            let before_avg = data[(i-window_size)..i].iter().map(|(_, v)| v).sum::<f64>() / window_size as f64;
            let after_avg = data[i..(i+window_size)].iter().map(|(_, v)| v).sum::<f64>() / window_size as f64;
            
            let change_magnitude = (after_avg - before_avg).abs() / before_avg;
            
            if change_magnitude > 0.2 { // 20% change threshold
                change_points.push(ChangePoint {
                    timestamp: data[i].0,
                    magnitude: change_magnitude,
                    confidence: 0.8,
                    description: format!("Significant change detected: {:.1}% change", change_magnitude * 100.0),
                });
            }
        }

        change_points
    }
}
