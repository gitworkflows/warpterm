use super::*;
use crate::error::WarpError;
use std::collections::VecDeque;
use std::sync::Arc;
use tokio::sync::{Mutex, mpsc};
use sysinfo::{System, SystemExt, ProcessExt, CpuExt};

pub struct EventCollector {
    event_queue: Arc<Mutex<VecDeque<AnalyticsEvent>>>,
    system_monitor: Arc<Mutex<System>>,
    performance_tracker: Arc<Mutex<PerformanceTracker>>,
    event_sender: mpsc::UnboundedSender<AnalyticsEvent>,
    event_receiver: Arc<Mutex<mpsc::UnboundedReceiver<AnalyticsEvent>>>,
    session_id: String,
}

pub struct PerformanceTracker {
    item_metrics: HashMap<String, ItemPerformanceMetrics>,
    system_baseline: SystemBaseline,
    monitoring_active: bool,
}

#[derive(Debug, Clone)]
pub struct ItemPerformanceMetrics {
    pub item_id: String,
    pub start_time: DateTime<Utc>,
    pub cpu_samples: VecDeque<f32>,
    pub memory_samples: VecDeque<u64>,
    pub network_samples: VecDeque<NetworkSample>,
    pub error_count: u32,
    pub crash_count: u32,
    pub load_times: VecDeque<u64>,
    pub response_times: VecDeque<u64>,
}

#[derive(Debug, Clone)]
pub struct NetworkSample {
    pub timestamp: DateTime<Utc>,
    pub bytes_sent: u64,
    pub bytes_received: u64,
}

#[derive(Debug, Clone)]
pub struct SystemBaseline {
    pub baseline_cpu: f32,
    pub baseline_memory: u64,
    pub baseline_network: u64,
    pub established_at: DateTime<Utc>,
}

impl EventCollector {
    pub async fn new() -> Result<Self, WarpError> {
        let (event_sender, event_receiver) = mpsc::unbounded_channel();
        let session_id = uuid::Uuid::new_v4().to_string();
        
        let mut system = System::new_all();
        system.refresh_all();
        
        let baseline = SystemBaseline {
            baseline_cpu: system.global_cpu_info().cpu_usage(),
            baseline_memory: system.used_memory(),
            baseline_network: 0, // Would be calculated from network interfaces
            established_at: Utc::now(),
        };

        Ok(Self {
            event_queue: Arc::new(Mutex::new(VecDeque::new())),
            system_monitor: Arc::new(Mutex::new(system)),
            performance_tracker: Arc::new(Mutex::new(PerformanceTracker {
                item_metrics: HashMap::new(),
                system_baseline: baseline,
                monitoring_active: true,
            })),
            event_sender,
            event_receiver: Arc::new(Mutex::new(event_receiver)),
            session_id,
        })
    }

    pub async fn collect_event(&self, event: AnalyticsEvent) -> Result<(), WarpError> {
        // Add to queue
        {
            let mut queue = self.event_queue.lock().await;
            queue.push_back(event.clone());
            
            // Limit queue size
            if queue.len() > 10000 {
                queue.pop_front();
            }
        }

        // Send to processing pipeline
        self.event_sender.send(event)
            .map_err(|e| WarpError::ConfigError(format!("Failed to send event: {}", e)))?;

        Ok(())
    }

    pub async fn track_item_activation(&self, item_id: &str) -> Result<(), WarpError> {
        let event = AnalyticsEvent {
            id: uuid::Uuid::new_v4().to_string(),
            event_type: EventType::ItemActivation,
            timestamp: Utc::now(),
            user_id: None, // Would be set from user context
            session_id: self.session_id.clone(),
            item_id: Some(item_id.to_string()),
            metadata: HashMap::new(),
            performance_data: None,
        };

        // Start performance monitoring for this item
        self.start_item_monitoring(item_id).await?;
        
        self.collect_event(event).await
    }

    pub async fn track_item_deactivation(&self, item_id: &str) -> Result<(), WarpError> {
        // Stop performance monitoring and collect final metrics
        let performance_data = self.stop_item_monitoring(item_id).await?;
        
        let event = AnalyticsEvent {
            id: uuid::Uuid::new_v4().to_string(),
            event_type: EventType::ItemDeactivation,
            timestamp: Utc::now(),
            user_id: None,
            session_id: self.session_id.clone(),
            item_id: Some(item_id.to_string()),
            metadata: HashMap::new(),
            performance_data,
        };

        self.collect_event(event).await
    }

    pub async fn track_item_usage(&self, item_id: &str, action: &str, metadata: HashMap<String, serde_json::Value>) -> Result<(), WarpError> {
        let mut event_metadata = metadata;
        event_metadata.insert("action".to_string(), serde_json::Value::String(action.to_string()));

        let event = AnalyticsEvent {
            id: uuid::Uuid::new_v4().to_string(),
            event_type: EventType::ItemUsage,
            timestamp: Utc::now(),
            user_id: None,
            session_id: self.session_id.clone(),
            item_id: Some(item_id.to_string()),
            metadata: event_metadata,
            performance_data: self.get_current_performance_data(item_id).await?,
        };

        self.collect_event(event).await
    }

    pub async fn track_item_error(&self, item_id: &str, error: &str, stack_trace: Option<&str>) -> Result<(), WarpError> {
        let mut metadata = HashMap::new();
        metadata.insert("error".to_string(), serde_json::Value::String(error.to_string()));
        if let Some(stack) = stack_trace {
            metadata.insert("stack_trace".to_string(), serde_json::Value::String(stack.to_string()));
        }

        // Update error count
        {
            let mut tracker = self.performance_tracker.lock().await;
            if let Some(metrics) = tracker.item_metrics.get_mut(item_id) {
                metrics.error_count += 1;
            }
        }

        let event = AnalyticsEvent {
            id: uuid::Uuid::new_v4().to_string(),
            event_type: EventType::ItemError,
            timestamp: Utc::now(),
            user_id: None,
            session_id: self.session_id.clone(),
            item_id: Some(item_id.to_string()),
            metadata,
            performance_data: self.get_current_performance_data(item_id).await?,
        };

        self.collect_event(event).await
    }

    pub async fn track_item_crash(&self, item_id: &str, crash_info: &str) -> Result<(), WarpError> {
        let mut metadata = HashMap::new();
        metadata.insert("crash_info".to_string(), serde_json::Value::String(crash_info.to_string()));

        // Update crash count
        {
            let mut tracker = self.performance_tracker.lock().await;
            if let Some(metrics) = tracker.item_metrics.get_mut(item_id) {
                metrics.crash_count += 1;
            }
        }

        let event = AnalyticsEvent {
            id: uuid::Uuid::new_v4().to_string(),
            event_type: EventType::ItemCrash,
            timestamp: Utc::now(),
            user_id: None,
            session_id: self.session_id.clone(),
            item_id: Some(item_id.to_string()),
            metadata,
            performance_data: self.get_current_performance_data(item_id).await?,
        };

        self.collect_event(event).await
    }

    pub async fn track_load_time(&self, item_id: &str, load_time_ms: u64) -> Result<(), WarpError> {
        // Record load time
        {
            let mut tracker = self.performance_tracker.lock().await;
            if let Some(metrics) = tracker.item_metrics.get_mut(item_id) {
                metrics.load_times.push_back(load_time_ms);
                if metrics.load_times.len() > 100 {
                    metrics.load_times.pop_front();
                }
            }
        }

        let mut metadata = HashMap::new();
        metadata.insert("load_time_ms".to_string(), serde_json::Value::Number(serde_json::Number::from(load_time_ms)));

        let event = AnalyticsEvent {
            id: uuid::Uuid::new_v4().to_string(),
            event_type: EventType::ItemLoadTime,
            timestamp: Utc::now(),
            user_id: None,
            session_id: self.session_id.clone(),
            item_id: Some(item_id.to_string()),
            metadata,
            performance_data: None,
        };

        self.collect_event(event).await
    }

    async fn start_item_monitoring(&self, item_id: &str) -> Result<(), WarpError> {
        let mut tracker = self.performance_tracker.lock().await;
        
        let metrics = ItemPerformanceMetrics {
            item_id: item_id.to_string(),
            start_time: Utc::now(),
            cpu_samples: VecDeque::new(),
            memory_samples: VecDeque::new(),
            network_samples: VecDeque::new(),
            error_count: 0,
            crash_count: 0,
            load_times: VecDeque::new(),
            response_times: VecDeque::new(),
        };

        tracker.item_metrics.insert(item_id.to_string(), metrics);
        
        Ok(())
    }

    async fn stop_item_monitoring(&self, item_id: &str) -> Result<Option<PerformanceData>, WarpError> {
        let mut tracker = self.performance_tracker.lock().await;
        
        if let Some(metrics) = tracker.item_metrics.remove(item_id) {
            let performance_data = PerformanceData {
                cpu_usage: metrics.cpu_samples.iter().sum::<f32>() / metrics.cpu_samples.len() as f32,
                memory_usage: metrics.memory_samples.iter().sum::<u64>() / metrics.memory_samples.len() as u64,
                disk_usage: 0, // Would be calculated
                network_bytes_sent: metrics.network_samples.iter().map(|s| s.bytes_sent).sum(),
                network_bytes_received: metrics.network_samples.iter().map(|s| s.bytes_received).sum(),
                load_time_ms: metrics.load_times.iter().sum::<u64>() / metrics.load_times.len() as u64,
                response_time_ms: metrics.response_times.iter().sum::<u64>() / metrics.response_times.len() as u64,
                error_count: metrics.error_count,
                crash_count: metrics.crash_count,
            };
            
            Ok(Some(performance_data))
        } else {
            Ok(None)
        }
    }

    async fn get_current_performance_data(&self, item_id: &str) -> Result<Option<PerformanceData>, WarpError> {
        let mut system = self.system_monitor.lock().await;
        system.refresh_all();
        
        let tracker = self.performance_tracker.lock().await;
        
        if let Some(metrics) = tracker.item_metrics.get(item_id) {
            let current_cpu = system.global_cpu_info().cpu_usage();
            let current_memory = system.used_memory();
            
            let performance_data = PerformanceData {
                cpu_usage: current_cpu,
                memory_usage: current_memory,
                disk_usage: 0, // Would be calculated from disk I/O
                network_bytes_sent: 0, // Would be calculated from network interfaces
                network_bytes_received: 0,
                load_time_ms: 0,
                response_time_ms: 0,
                error_count: metrics.error_count,
                crash_count: metrics.crash_count,
            };
            
            Ok(Some(performance_data))
        } else {
            Ok(None)
        }
    }

    pub async fn start_continuous_monitoring(&self) -> Result<(), WarpError> {
        let system_monitor = self.system_monitor.clone();
        let performance_tracker = self.performance_tracker.clone();
        let event_sender = self.event_sender.clone();
        let session_id = self.session_id.clone();

        tokio::spawn(async move {
            loop {
                if let Err(e) = Self::collect_system_metrics(
                    system_monitor.clone(),
                    performance_tracker.clone(),
                    event_sender.clone(),
                    session_id.clone(),
                ).await {
                    log::error!("System metrics collection failed: {}", e);
                }
                
                tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;
            }
        });

        Ok(())
    }

    async fn collect_system_metrics(
        system_monitor: Arc<Mutex<System>>,
        performance_tracker: Arc<Mutex<PerformanceTracker>>,
        event_sender: mpsc::UnboundedSender<AnalyticsEvent>,
        session_id: String,
    ) -> Result<(), WarpError> {
        let mut system = system_monitor.lock().await;
        system.refresh_all();
        
        let current_cpu = system.global_cpu_info().cpu_usage();
        let current_memory = system.used_memory();
        
        // Update performance metrics for all monitored items
        {
            let mut tracker = performance_tracker.lock().await;
            let now = Utc::now();
            
            for (item_id, metrics) in tracker.item_metrics.iter_mut() {
                metrics.cpu_samples.push_back(current_cpu);
                metrics.memory_samples.push_back(current_memory);
                
                // Limit sample size
                if metrics.cpu_samples.len() > 1000 {
                    metrics.cpu_samples.pop_front();
                }
                if metrics.memory_samples.len() > 1000 {
                    metrics.memory_samples.pop_front();
                }
                
                // Create performance event
                let performance_data = PerformanceData {
                    cpu_usage: current_cpu,
                    memory_usage: current_memory,
                    disk_usage: 0,
                    network_bytes_sent: 0,
                    network_bytes_received: 0,
                    load_time_ms: 0,
                    response_time_ms: 0,
                    error_count: metrics.error_count,
                    crash_count: metrics.crash_count,
                };
                
                let event = AnalyticsEvent {
                    id: uuid::Uuid::new_v4().to_string(),
                    event_type: EventType::ItemMemoryUsage,
                    timestamp: now,
                    user_id: None,
                    session_id: session_id.clone(),
                    item_id: Some(item_id.clone()),
                    metadata: HashMap::new(),
                    performance_data: Some(performance_data),
                };
                
                let _ = event_sender.send(event);
            }
        }
        
        Ok(())
    }

    pub async fn get_pending_events(&self) -> Result<Vec<AnalyticsEvent>, WarpError> {
        let mut queue = self.event_queue.lock().await;
        let events: Vec<AnalyticsEvent> = queue.drain(..).collect();
        Ok(events)
    }
}
