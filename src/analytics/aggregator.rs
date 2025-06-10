use super::*;
use crate::error::WarpError;
use std::collections::HashMap;

pub struct MetricsAggregator {
    usage_metrics: HashMap<String, UsageMetrics>,
    performance_metrics: HashMap<String, PerformanceMetrics>,
    user_behavior_metrics: HashMap<String, UserBehaviorMetrics>,
    marketplace_analytics: MarketplaceAnalytics,
    real_time_cache: HashMap<String, RealTimeMetrics>,
    pending_events: Vec<AnalyticsEvent>,
}

#[derive(Debug, Clone)]
pub struct RealTimeMetrics {
    pub active_users: u32,
    pub current_usage: u64,
    pub error_rate: f32,
    pub performance_score: f32,
    pub last_updated: DateTime<Utc>,
}

impl MetricsAggregator {
    pub async fn new() -> Result<Self, WarpError> {
        Ok(Self {
            usage_metrics: HashMap::new(),
            performance_metrics: HashMap::new(),
            user_behavior_metrics: HashMap::new(),
            marketplace_analytics: MarketplaceAnalytics {
                total_items: 0,
                total_downloads: 0,
                total_active_users: 0,
                category_distribution: HashMap::new(),
                top_items: vec![],
                trending_items: vec![],
                revenue_metrics: RevenueMetrics {
                    total_revenue: 0.0,
                    monthly_recurring_revenue: 0.0,
                    average_revenue_per_user: 0.0,
                    conversion_rate: 0.0,
                    churn_rate: 0.0,
                    lifetime_value: 0.0,
                },
                user_acquisition: UserAcquisitionMetrics {
                    new_users: 0,
                    acquisition_cost: 0.0,
                    acquisition_channels: HashMap::new(),
                    conversion_funnel: ConversionFunnel {
                        visitors: 0,
                        signups: 0,
                        activations: 0,
                        first_purchase: 0,
                        retention: 0,
                    },
                },
            },
            real_time_cache: HashMap::new(),
            pending_events: Vec::new(),
        })
    }

    pub async fn process_pending_events(&mut self) -> Result<(), WarpError> {
        for event in self.pending_events.drain(..) {
            self.process_event(event).await?;
        }
        Ok(())
    }

    pub async fn add_event(&mut self, event: AnalyticsEvent) -> Result<(), WarpError> {
        self.pending_events.push(event);
        Ok(())
    }

    async fn process_event(&mut self, event: AnalyticsEvent) -> Result<(), WarpError> {
        match event.event_type {
            EventType::ItemActivation => {
                self.process_activation_event(&event).await?;
            }
            EventType::ItemDeactivation => {
                self.process_deactivation_event(&event).await?;
            }
            EventType::ItemUsage => {
                self.process_usage_event(&event).await?;
            }
            EventType::ItemError => {
                self.process_error_event(&event).await?;
            }
            EventType::ItemCrash => {
                self.process_crash_event(&event).await?;
            }
            EventType::ItemLoadTime => {
                self.process_load_time_event(&event).await?;
            }
            EventType::ItemMemoryUsage | EventType::ItemCpuUsage => {
                self.process_performance_event(&event).await?;
            }
            EventType::ItemInstall => {
                self.process_install_event(&event).await?;
            }
            EventType::ItemRating => {
                self.process_rating_event(&event).await?;
            }
            _ => {
                // Handle other event types
            }
        }
        Ok(())
    }

    async fn process_activation_event(&mut self, event: &AnalyticsEvent) -> Result<(), WarpError> {
        if let Some(item_id) = &event.item_id {
            let metrics = self.usage_metrics.entry(item_id.clone()).or_insert_with(|| {
                UsageMetrics {
                    item_id: item_id.clone(),
                    total_activations: 0,
                    total_usage_time: Duration::seconds(0),
                    average_session_duration: Duration::seconds(0),
                    daily_active_users: 0,
                    weekly_active_users: 0,
                    monthly_active_users: 0,
                    retention_rate: 0.0,
                    crash_rate: 0.0,
                    error_rate: 0.0,
                    performance_score: 0.0,
                    user_satisfaction: 0.0,
                }
            });

            metrics.total_activations += 1;
            
            // Update real-time metrics
            let real_time = self.real_time_cache.entry(item_id.clone()).or_insert_with(|| {
                RealTimeMetrics {
                    active_users: 0,
                    current_usage: 0,
                    error_rate: 0.0,
                    performance_score: 0.0,
                    last_updated: Utc::now(),
                }
            });
            
            real_time.active_users += 1;
            real_time.current_usage += 1;
            real_time.last_updated = Utc::now();
        }
        Ok(())
    }

    async fn process_deactivation_event(&mut self, event: &AnalyticsEvent) -> Result<(), WarpError> {
        if let Some(item_id) = &event.item_id {
            if let Some(real_time) = self.real_time_cache.get_mut(item_id) {
                if real_time.active_users > 0 {
                    real_time.active_users -= 1;
                }
                real_time.last_updated = Utc::now();
            }

            // Calculate session duration if we have performance data
            if let Some(performance_data) = &event.performance_data {
                if let Some(metrics) = self.usage_metrics.get_mut(item_id) {
                    // This would calculate actual session duration based on activation/deactivation times
                    let session_duration = Duration::minutes(5); // Placeholder
                    metrics.total_usage_time = metrics.total_usage_time + session_duration;
                    
                    // Update average session duration
                    if metrics.total_activations > 0 {
                        metrics.average_session_duration = Duration::seconds(
                            metrics.total_usage_time.num_seconds() / metrics.total_activations as i64
                        );
                    }
                }
            }
        }
        Ok(())
    }

    async fn process_usage_event(&mut self, event: &AnalyticsEvent) -> Result<(), WarpError> {
        if let Some(item_id) = &event.item_id {
            // Update user behavior metrics
            let behavior_metrics = self.user_behavior_metrics.entry(item_id.clone()).or_insert_with(|| {
                UserBehaviorMetrics {
                    item_id: item_id.clone(),
                    feature_usage: HashMap::new(),
                    user_flows: vec![],
                    drop_off_points: vec![],
                    engagement_score: 0.0,
                    feature_adoption_rate: HashMap::new(),
                    user_journey_analysis: UserJourneyAnalysis {
                        onboarding_completion_rate: 0.0,
                        time_to_first_value: Duration::seconds(0),
                        feature_discovery_rate: 0.0,
                        user_progression_stages: vec![],
                    },
                }
            });

            // Track feature usage
            if let Some(action) = event.metadata.get("action") {
                if let Some(action_str) = action.as_str() {
                    *behavior_metrics.feature_usage.entry(action_str.to_string()).or_insert(0) += 1;
                }
            }

            // Update real-time metrics
            if let Some(real_time) = self.real_time_cache.get_mut(item_id) {
                real_time.current_usage += 1;
                real_time.last_updated = Utc::now();
            }
        }
        Ok(())
    }

    async fn process_error_event(&mut self, event: &AnalyticsEvent) -> Result<(), WarpError> {
        if let Some(item_id) = &event.item_id {
            if let Some(metrics) = self.usage_metrics.get_mut(item_id) {
                // Calculate error rate
                let total_events = metrics.total_activations + 1; // +1 for this error
                metrics.error_rate = 1.0 / total_events as f32; // Simplified calculation
            }

            // Update real-time metrics
            if let Some(real_time) = self.real_time_cache.get_mut(item_id) {
                real_time.error_rate += 0.01; // Increment error rate
                real_time.last_updated = Utc::now();
            }
        }
        Ok(())
    }

    async fn process_crash_event(&mut self, event: &AnalyticsEvent) -> Result<(), WarpError> {
        if let Some(item_id) = &event.item_id {
            if let Some(metrics) = self.usage_metrics.get_mut(item_id) {
                // Calculate crash rate
                let total_events = metrics.total_activations + 1;
                metrics.crash_rate = 1.0 / total_events as f32; // Simplified calculation
            }

            // Update real-time metrics
            if let Some(real_time) = self.real_time_cache.get_mut(item_id) {
                real_time.error_rate += 0.05; // Crashes are more severe than errors
                real_time.performance_score *= 0.9; // Reduce performance score
                real_time.last_updated = Utc::now();
            }
        }
        Ok(())
    }

    async fn process_load_time_event(&mut self, event: &AnalyticsEvent) -> Result<(), WarpError> {
        if let Some(item_id) = &event.item_id {
            let perf_metrics = self.performance_metrics.entry(item_id.clone()).or_insert_with(|| {
                PerformanceMetrics {
                    item_id: item_id.clone(),
                    average_load_time: Duration::seconds(0),
                    p95_load_time: Duration::seconds(0),
                    p99_load_time: Duration::seconds(0),
                    average_memory_usage: 0,
                    peak_memory_usage: 0,
                    average_cpu_usage: 0.0,
                    peak_cpu_usage: 0.0,
                    network_efficiency: 0.0,
                    stability_score: 1.0,
                    resource_efficiency: 0.0,
                }
            });

            if let Some(load_time_value) = event.metadata.get("load_time_ms") {
                if let Some(load_time_ms) = load_time_value.as_u64() {
                    // Update average load time (simplified)
                    perf_metrics.average_load_time = Duration::milliseconds(load_time_ms as i64);
                }
            }
        }
        Ok(())
    }

    async fn process_performance_event(&mut self, event: &AnalyticsEvent) -> Result<(), WarpError> {
        if let Some(item_id) = &event.item_id {
            if let Some(performance_data) = &event.performance_data {
                let perf_metrics = self.performance_metrics.entry(item_id.clone()).or_insert_with(|| {
                    PerformanceMetrics {
                        item_id: item_id.clone(),
                        average_load_time: Duration::seconds(0),
                        p95_load_time: Duration::seconds(0),
                        p99_load_time: Duration::seconds(0),
                        average_memory_usage: 0,
                        peak_memory_usage: 0,
                        average_cpu_usage: 0.0,
                        peak_cpu_usage: 0.0,
                        network_efficiency: 0.0,
                        stability_score: 1.0,
                        resource_efficiency: 0.0,
                    }
                });

                // Update performance metrics
                perf_metrics.average_cpu_usage = (perf_metrics.average_cpu_usage + performance_data.cpu_usage) / 2.0;
                perf_metrics.average_memory_usage = (perf_metrics.average_memory_usage + performance_data.memory_usage) / 2;
                
                if performance_data.cpu_usage > perf_metrics.peak_cpu_usage {
                    perf_metrics.peak_cpu_usage = performance_data.cpu_usage;
                }
                
                if performance_data.memory_usage > perf_metrics.peak_memory_usage {
                    perf_metrics.peak_memory_usage = performance_data.memory_usage;
                }

                // Calculate resource efficiency score
                let cpu_efficiency = 1.0 - (performance_data.cpu_usage / 100.0);
                let memory_efficiency = 1.0 - (performance_data.memory_usage as f32 / (8 * 1024 * 1024 * 1024) as f32); // Assume 8GB baseline
                perf_metrics.resource_efficiency = (cpu_efficiency + memory_efficiency) / 2.0;

                // Update real-time performance score
                if let Some(real_time) = self.real_time_cache.get_mut(item_id) {
                    real_time.performance_score = perf_metrics.resource_efficiency * perf_metrics.stability_score;
                    real_time.last_updated = Utc::now();
                }
            }
        }
        Ok(())
    }

    async fn process_install_event(&mut self, event: &AnalyticsEvent) -> Result<(), WarpError> {
        // Update marketplace analytics
        self.marketplace_analytics.total_downloads += 1;
        
        if let Some(item_id) = &event.item_id {
            // This would update category distribution, trending items, etc.
            // For now, just increment total downloads
        }
        
        Ok(())
    }

    async fn process_rating_event(&mut self, event: &AnalyticsEvent) -> Result<(), WarpError> {
        if let Some(item_id) = &event.item_id {
            if let Some(metrics) = self.usage_metrics.get_mut(item_id) {
                // Update user satisfaction based on rating
                if let Some(rating_value) = event.metadata.get("rating") {
                    if let Some(rating) = rating_value.as_f64() {
                        metrics.user_satisfaction = (metrics.user_satisfaction + rating as f32) / 2.0;
                    }
                }
            }
        }
        Ok(())
    }

    pub async fn get_usage_metrics(&self, item_id: &str, _time_range: TimeRange) -> Result<UsageMetrics, WarpError> {
        self.usage_metrics.get(item_id)
            .cloned()
            .ok_or_else(|| WarpError::ConfigError(format!("No usage metrics found for item: {}", item_id)))
    }

    pub async fn get_performance_metrics(&self, item_id: &str, _time_range: TimeRange) -> Result<PerformanceMetrics, WarpError> {
        self.performance_metrics.get(item_id)
            .cloned()
            .ok_or_else(|| WarpError::ConfigError(format!("No performance metrics found for item: {}", item_id)))
    }

    pub async fn get_user_behavior_metrics(&self, item_id: &str, _time_range: TimeRange) -> Result<UserBehaviorMetrics, WarpError> {
        self.user_behavior_metrics.get(item_id)
            .cloned()
            .ok_or_else(|| WarpError::ConfigError(format!("No user behavior metrics found for item: {}", item_id)))
    }

    pub async fn get_marketplace_analytics(&self, _time_range: TimeRange) -> Result<MarketplaceAnalytics, WarpError> {
        Ok(self.marketplace_analytics.clone())
    }

    pub async fn update_real_time_metrics(&mut self) -> Result<(), WarpError> {
        let now = Utc::now();
        
        // Update marketplace totals
        self.marketplace_analytics.total_active_users = self.real_time_cache.values()
            .map(|metrics| metrics.active_users)
            .sum();

        // Calculate trending items based on recent activity
        let mut trending_items = Vec::new();
        for (item_id, real_time) in &self.real_time_cache {
            if now.signed_duration_since(real_time.last_updated).num_minutes() < 60 {
                let momentum_score = real_time.current_usage as f32 * real_time.performance_score;
                trending_items.push(TrendingItem {
                    item_id: item_id.clone(),
                    name: format!("Item {}", item_id), // Would be fetched from item metadata
                    growth_rate: 0.1, // Would be calculated from historical data
                    velocity: real_time.current_usage as f32,
                    momentum_score,
                });
            }
        }

        // Sort by momentum score
        trending_items.sort_by(|a, b| b.momentum_score.partial_cmp(&a.momentum_score).unwrap_or(std::cmp::Ordering::Equal));
        trending_items.truncate(10);
        
        self.marketplace_analytics.trending_items = trending_items;

        Ok(())
    }

    pub async fn cleanup_old_data(&mut self) -> Result<(), WarpError> {
        let cutoff_time = Utc::now() - Duration::hours(24);
        
        // Remove stale real-time metrics
        self.real_time_cache.retain(|_, metrics| {
            metrics.last_updated > cutoff_time
        });

        Ok(())
    }

    pub fn get_real_time_metrics(&self, item_id: &str) -> Option<&RealTimeMetrics> {
        self.real_time_cache.get(item_id)
    }

    pub fn get_all_real_time_metrics(&self) -> &HashMap<String, RealTimeMetrics> {
        &self.real_time_cache
    }
}
