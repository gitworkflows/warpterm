use std::collections::HashMap;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use crate::error::WarpError;

pub mod formats;
pub mod generators;
pub mod schedulers;
pub mod templates;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportManager {
    generators: HashMap<ExportFormat, Box<dyn ExportGenerator>>,
    schedulers: Vec<ExportScheduler>,
    templates: HashMap<String, ExportTemplate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExportFormat {
    CSV,
    JSON,
    XML,
    Excel,
    PDF,
    HTML,
    Parquet,
    SQLDump,
    PowerBI,
    Tableau,
    Grafana,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportRequest {
    pub request_id: String,
    pub format: ExportFormat,
    pub data_source: DataSource,
    pub filters: Vec<ExportFilter>,
    pub columns: Option<Vec<String>>,
    pub time_range: Option<TimeRange>,
    pub template: Option<String>,
    pub destination: ExportDestination,
    pub compression: Option<CompressionType>,
    pub encryption: Option<EncryptionConfig>,
    pub metadata: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataSource {
    Analytics,
    UserBehavior,
    Performance,
    ABTests,
    Marketplace,
    CustomMetrics,
    RawEvents,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportFilter {
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
    Between,
    In,
    NotIn,
    Contains,
    StartsWith,
    EndsWith,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeRange {
    pub start: chrono::DateTime<chrono::Utc>,
    pub end: chrono::DateTime<chrono::Utc>,
    pub timezone: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExportDestination {
    LocalFile { path: PathBuf },
    S3 { bucket: String, key: String, region: String },
    GCS { bucket: String, object: String },
    Azure { container: String, blob: String },
    FTP { host: String, path: String, credentials: FTPCredentials },
    Email { recipients: Vec<String>, subject: String },
    Webhook { url: String, headers: HashMap<String, String> },
    Database { connection_string: String, table: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FTPCredentials {
    pub username: String,
    pub password: String,
    pub port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CompressionType {
    Gzip,
    Zip,
    Bzip2,
    Lz4,
    Zstd,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionConfig {
    pub algorithm: EncryptionAlgorithm,
    pub key: String,
    pub iv: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EncryptionAlgorithm {
    AES256,
    ChaCha20,
    RSA,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportResult {
    pub request_id: String,
    pub status: ExportStatus,
    pub file_path: Option<PathBuf>,
    pub file_size: Option<u64>,
    pub row_count: Option<u64>,
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub error_message: Option<String>,
    pub download_url: Option<String>,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExportStatus {
    Queued,
    Processing,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportScheduler {
    pub schedule_id: String,
    pub name: String,
    pub description: String,
    pub cron_expression: String,
    pub export_request: ExportRequest,
    pub enabled: bool,
    pub last_run: Option<chrono::DateTime<chrono::Utc>>,
    pub next_run: Option<chrono::DateTime<chrono::Utc>>,
    pub run_count: u64,
    pub failure_count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportTemplate {
    pub template_id: String,
    pub name: String,
    pub description: String,
    pub format: ExportFormat,
    pub columns: Vec<ColumnDefinition>,
    pub styling: Option<TemplateStyle>,
    pub transformations: Vec<DataTransformation>,
    pub aggregations: Vec<DataAggregation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColumnDefinition {
    pub name: String,
    pub display_name: String,
    pub data_type: DataType,
    pub format: Option<String>,
    pub width: Option<u32>,
    pub alignment: Option<Alignment>,
    pub visible: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataType {
    String,
    Integer,
    Float,
    Boolean,
    Date,
    DateTime,
    Currency,
    Percentage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Alignment {
    Left,
    Center,
    Right,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateStyle {
    pub header_style: StyleConfig,
    pub data_style: StyleConfig,
    pub alternating_rows: bool,
    pub borders: bool,
    pub color_scheme: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StyleConfig {
    pub font_family: String,
    pub font_size: u32,
    pub font_weight: FontWeight,
    pub color: String,
    pub background_color: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FontWeight {
    Normal,
    Bold,
    Light,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataTransformation {
    pub transformation_type: TransformationType,
    pub source_column: String,
    pub target_column: String,
    pub parameters: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransformationType {
    Rename,
    Format,
    Calculate,
    Lookup,
    Conditional,
    Aggregate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataAggregation {
    pub aggregation_type: AggregationType,
    pub column: String,
    pub group_by: Vec<String>,
    pub alias: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AggregationType {
    Sum,
    Average,
    Count,
    Min,
    Max,
    Median,
    StandardDeviation,
}

pub trait ExportGenerator: Send + Sync {
    async fn generate(&self, request: &ExportRequest, data: &[HashMap<String, serde_json::Value>]) -> Result<Vec<u8>, WarpError>;
    fn supported_format(&self) -> ExportFormat;
    fn max_row_limit(&self) -> Option<u64>;
}

impl ExportManager {
    pub async fn new() -> Result<Self, WarpError> {
        let mut generators: HashMap<ExportFormat, Box<dyn ExportGenerator>> = HashMap::new();
        
        // Register format generators
        generators.insert(ExportFormat::CSV, Box::new(formats::CSVGenerator::new()));
        generators.insert(ExportFormat::JSON, Box::new(formats::JSONGenerator::new()));
        generators.insert(ExportFormat::Excel, Box::new(formats::ExcelGenerator::new()));
        generators.insert(ExportFormat::PDF, Box::new(formats::PDFGenerator::new()));
        generators.insert(ExportFormat::HTML, Box::new(formats::HTMLGenerator::new()));
        generators.insert(ExportFormat::Parquet, Box::new(formats::ParquetGenerator::new()));

        Ok(Self {
            generators,
            schedulers: Vec::new(),
            templates: HashMap::new(),
        })
    }

    pub async fn export_data(&self, request: ExportRequest) -> Result<ExportResult, WarpError> {
        let mut result = ExportResult {
            request_id: request.request_id.clone(),
            status: ExportStatus::Processing,
            file_path: None,
            file_size: None,
            row_count: None,
            started_at: chrono::Utc::now(),
            completed_at: None,
            error_message: None,
            download_url: None,
            expires_at: None,
        };

        // Get data from source
        let data = self.fetch_data(&request).await?;
        
        // Apply filters
        let filtered_data = self.apply_filters(&data, &request.filters)?;
        
        // Apply template transformations if specified
        let processed_data = if let Some(template_name) = &request.template {
            self.apply_template(&filtered_data, template_name)?
        } else {
            filtered_data
        };

        // Generate export
        if let Some(generator) = self.generators.get(&request.format) {
            match generator.generate(&request, &processed_data).await {
                Ok(export_data) => {
                    // Save to destination
                    let file_path = self.save_to_destination(&request.destination, &export_data).await?;
                    
                    result.status = ExportStatus::Completed;
                    result.file_path = Some(file_path);
                    result.file_size = Some(export_data.len() as u64);
                    result.row_count = Some(processed_data.len() as u64);
                    result.completed_at = Some(chrono::Utc::now());
                    
                    // Set expiration for temporary files
                    if matches!(request.destination, ExportDestination::LocalFile { .. }) {
                        result.expires_at = Some(chrono::Utc::now() + chrono::Duration::days(7));
                    }
                }
                Err(e) => {
                    result.status = ExportStatus::Failed;
                    result.error_message = Some(e.to_string());
                }
            }
        } else {
            result.status = ExportStatus::Failed;
            result.error_message = Some(format!("Unsupported export format: {:?}", request.format));
        }

        Ok(result)
    }

    pub async fn schedule_export(&mut self, scheduler: ExportScheduler) -> Result<String, WarpError> {
        let schedule_id = scheduler.schedule_id.clone();
        self.schedulers.push(scheduler);
        Ok(schedule_id)
    }

    pub async fn create_template(&mut self, template: ExportTemplate) -> Result<String, WarpError> {
        let template_id = template.template_id.clone();
        self.templates.insert(template_id.clone(), template);
        Ok(template_id)
    }

    pub async fn get_export_status(&self, request_id: &str) -> Result<ExportStatus, WarpError> {
        // In a real implementation, this would query the export status from storage
        Ok(ExportStatus::Completed)
    }

    pub async fn cancel_export(&self, request_id: &str) -> Result<(), WarpError> {
        // In a real implementation, this would cancel the running export
        log::info!("Cancelling export: {}", request_id);
        Ok(())
    }

    pub async fn list_exports(&self, user_id: Option<&str>) -> Result<Vec<ExportResult>, WarpError> {
        // In a real implementation, this would query exports from storage
        let _ = user_id;
        Ok(Vec::new())
    }

    async fn fetch_data(&self, request: &ExportRequest) -> Result<Vec<HashMap<String, serde_json::Value>>, WarpError> {
        match &request.data_source {
            DataSource::Analytics => {
                // Fetch analytics data
                self.fetch_analytics_data(request).await
            }
            DataSource::UserBehavior => {
                // Fetch user behavior data
                self.fetch_user_behavior_data(request).await
            }
            DataSource::Performance => {
                // Fetch performance data
                self.fetch_performance_data(request).await
            }
            DataSource::ABTests => {
                // Fetch A/B test data
                self.fetch_ab_test_data(request).await
            }
            DataSource::Marketplace => {
                // Fetch marketplace data
                self.fetch_marketplace_data(request).await
            }
            DataSource::CustomMetrics => {
                // Fetch custom metrics data
                self.fetch_custom_metrics_data(request).await
            }
            DataSource::RawEvents => {
                // Fetch raw events data
                self.fetch_raw_events_data(request).await
            }
        }
    }

    async fn fetch_analytics_data(&self, _request: &ExportRequest) -> Result<Vec<HashMap<String, serde_json::Value>>, WarpError> {
        // Mock analytics data
        let mut data = Vec::new();
        for i in 0..100 {
            let mut row = HashMap::new();
            row.insert("date".to_string(), serde_json::Value::String(format!("2024-01-{:02}", i % 30 + 1)));
            row.insert("users".to_string(), serde_json::Value::Number(serde_json::Number::from(1000 + i * 10)));
            row.insert("sessions".to_string(), serde_json::Value::Number(serde_json::Number::from(1500 + i * 15)));
            row.insert("revenue".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(5000.0 + i as f64 * 50.0).unwrap()));
            data.push(row);
        }
        Ok(data)
    }

    async fn fetch_user_behavior_data(&self, _request: &ExportRequest) -> Result<Vec<HashMap<String, serde_json::Value>>, WarpError> {
        // Mock user behavior data
        let mut data = Vec::new();
        for i in 0..50 {
            let mut row = HashMap::new();
            row.insert("user_id".to_string(), serde_json::Value::String(format!("user_{}", i)));
            row.insert("action".to_string(), serde_json::Value::String("click".to_string()));
            row.insert("timestamp".to_string(), serde_json::Value::String(chrono::Utc::now().to_rfc3339()));
            row.insert("duration".to_string(), serde_json::Value::Number(serde_json::Number::from(i * 100)));
            data.push(row);
        }
        Ok(data)
    }

    async fn fetch_performance_data(&self, _request: &ExportRequest) -> Result<Vec<HashMap<String, serde_json::Value>>, WarpError> {
        // Mock performance data
        let mut data = Vec::new();
        for i in 0..200 {
            let mut row = HashMap::new();
            row.insert("timestamp".to_string(), serde_json::Value::String(chrono::Utc::now().to_rfc3339()));
            row.insert("cpu_usage".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(50.0 + (i as f64 * 0.1).sin() * 20.0).unwrap()));
            row.insert("memory_usage".to_string(), serde_json::Value::Number(serde_json::Number::from(1024 + i * 10)));
            row.insert("response_time".to_string(), serde_json::Value::Number(serde_json::Number::from(100 + i % 50)));
            data.push(row);
        }
        Ok(data)
    }

    async fn fetch_ab_test_data(&self, _request: &ExportRequest) -> Result<Vec<HashMap<String, serde_json::Value>>, WarpError> {
        // Mock A/B test data
        let mut data = Vec::new();
        for i in 0..75 {
            let mut row = HashMap::new();
            row.insert("experiment_id".to_string(), serde_json::Value::String(format!("exp_{}", i % 5)));
            row.insert("variant_id".to_string(), serde_json::Value::String(format!("variant_{}", i % 2)));
            row.insert("user_id".to_string(), serde_json::Value::String(format!("user_{}", i)));
            row.insert("conversion".to_string(), serde_json::Value::Bool(i % 3 == 0));
            row.insert("revenue".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(if i % 3 == 0 { 25.99 } else { 0.0 }).unwrap()));
            data.push(row);
        }
        Ok(data)
    }

    async fn fetch_marketplace_data(&self, _request: &ExportRequest) -> Result<Vec<HashMap<String, serde_json::Value>>, WarpError> {
        // Mock marketplace data
        let mut data = Vec::new();
        for i in 0..30 {
            let mut row = HashMap::new();
            row.insert("item_id".to_string(), serde_json::Value::String(format!("item_{}", i)));
            row.insert("name".to_string(), serde_json::Value::String(format!("Item {}", i)));
            row.insert("category".to_string(), serde_json::Value::String(["Themes", "Plugins", "AI Models"][i % 3].to_string()));
            row.insert("downloads".to_string(), serde_json::Value::Number(serde_json::Number::from(1000 + i * 100)));
            row.insert("rating".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(4.0 + (i as f64 % 10) / 10.0).unwrap()));
            data.push(row);
        }
        Ok(data)
    }

    async fn fetch_custom_metrics_data(&self, _request: &ExportRequest) -> Result<Vec<HashMap<String, serde_json::Value>>, WarpError> {
        // Mock custom metrics data
        let mut data = Vec::new();
        for i in 0..150 {
            let mut row = HashMap::new();
            row.insert("metric_name".to_string(), serde_json::Value::String(format!("custom_metric_{}", i % 10)));
            row.insert("value".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(i as f64 * 1.5).unwrap()));
            row.insert("timestamp".to_string(), serde_json::Value::String(chrono::Utc::now().to_rfc3339()));
            row.insert("tags".to_string(), serde_json::Value::String(format!("tag1,tag2,tag{}", i % 5)));
            data.push(row);
        }
        Ok(data)
    }

    async fn fetch_raw_events_data(&self, _request: &ExportRequest) -> Result<Vec<HashMap<String, serde_json::Value>>, WarpError> {
        // Mock raw events data
        let mut data = Vec::new();
        for i in 0..500 {
            let mut row = HashMap::new();
            row.insert("event_id".to_string(), serde_json::Value::String(uuid::Uuid::new_v4().to_string()));
            row.insert("event_type".to_string(), serde_json::Value::String(["click", "view", "download", "install"][i % 4].to_string()));
            row.insert("user_id".to_string(), serde_json::Value::String(format!("user_{}", i % 100)));
            row.insert("timestamp".to_string(), serde_json::Value::String(chrono::Utc::now().to_rfc3339()));
            row.insert("properties".to_string(), serde_json::Value::String(format!("{{\"prop1\": {}}}", i)));
            data.push(row);
        }
        Ok(data)
    }

    fn apply_filters(&self, data: &[HashMap<String, serde_json::Value>], filters: &[ExportFilter]) -> Result<Vec<HashMap<String, serde_json::Value>>, WarpError> {
        let mut filtered_data = data.to_vec();

        for filter in filters {
            filtered_data.retain(|row| {
                if let Some(field_value) = row.get(&filter.field) {
                    match filter.operator {
                        FilterOperator::Equals => field_value == &filter.value,
                        FilterOperator::NotEquals => field_value != &filter.value,
                        FilterOperator::GreaterThan => {
                            if let (Some(field_num), Some(filter_num)) = (field_value.as_f64(), filter.value.as_f64()) {
                                field_num > filter_num
                            } else {
                                false
                            }
                        }
                        FilterOperator::LessThan => {
                            if let (Some(field_num), Some(filter_num)) = (field_value.as_f64(), filter.value.as_f64()) {
                                field_num < filter_num
                            } else {
                                false
                            }
                        }
                        FilterOperator::Between => {
                            if let Some(range) = filter.value.as_array() {
                                if range.len() == 2 {
                                    if let (Some(field_num), Some(min), Some(max)) = (
                                        field_value.as_f64(),
                                        range[0].as_f64(),
                                        range[1].as_f64()
                                    ) {
                                        field_num >= min && field_num <= max
                                    } else {
                                        false
                                    }
                                } else {
                                    false
                                }
                            } else {
                                false
                            }
                        }
                        FilterOperator::In => {
                            if let Some(values) = filter.value.as_array() {
                                values.contains(field_value)
                            } else {
                                false
                            }
                        }
                        FilterOperator::NotIn => {
                            if let Some(values) = filter.value.as_array() {
                                !values.contains(field_value)
                            } else {
                                true
                            }
                        }
                        FilterOperator::Contains => {
                            if let (Some(field_str), Some(filter_str)) = (field_value.as_str(), filter.value.as_str()) {
                                field_str.contains(filter_str)
                            } else {
                                false
                            }
                        }
                        FilterOperator::StartsWith => {
                            if let (Some(field_str), Some(filter_str)) = (field_value.as_str(), filter.value.as_str()) {
                                field_str.starts_with(filter_str)
                            } else {
                                false
                            }
                        }
                        FilterOperator::EndsWith => {
                            if let (Some(field_str), Some(filter_str)) = (field_value.as_str(), filter.value.as_str()) {
                                field_str.ends_with(filter_str)
                            } else {
                                false
                            }
                        }
                    }
                } else {
                    false
                }
            });
        }

        Ok(filtered_data)
    }

    fn apply_template(&self, data: &[HashMap<String, serde_json::Value>], template_name: &str) -> Result<Vec<HashMap<String, serde_json::Value>>, WarpError> {
        if let Some(template) = self.templates.get(template_name) {
            let mut processed_data = Vec::new();

            for row in data {
                let mut processed_row = HashMap::new();

                // Apply transformations
                for transformation in &template.transformations {
                    match transformation.transformation_type {
                        TransformationType::Rename => {
                            if let Some(value) = row.get(&transformation.source_column) {
                                processed_row.insert(transformation.target_column.clone(), value.clone());
                            }
                        }
                        TransformationType::Format => {
                            if let Some(value) = row.get(&transformation.source_column) {
                                let formatted_value = self.format_value(value, &transformation.parameters)?;
                                processed_row.insert(transformation.target_column.clone(), formatted_value);
                            }
                        }
                        TransformationType::Calculate => {
                            let calculated_value = self.calculate_value(row, &transformation.parameters)?;
                            processed_row.insert(transformation.target_column.clone(), calculated_value);
                        }
                        _ => {
                            // Handle other transformation types
                            if let Some(value) = row.get(&transformation.source_column) {
                                processed_row.insert(transformation.target_column.clone(), value.clone());
                            }
                        }
                    }
                }

                // If no transformations, copy original data
                if processed_row.is_empty() {
                    processed_row = row.clone();
                }

                processed_data.push(processed_row);
            }

            // Apply aggregations if specified
            if !template.aggregations.is_empty() {
                processed_data = self.apply_aggregations(&processed_data, &template.aggregations)?;
            }

            Ok(processed_data)
        } else {
            Err(WarpError::ConfigError(format!("Template not found: {}", template_name)))
        }
    }

    fn format_value(&self, value: &serde_json::Value, parameters: &HashMap<String, serde_json::Value>) -> Result<serde_json::Value, WarpError> {
        if let Some(format_str) = parameters.get("format").and_then(|v| v.as_str()) {
            match format_str {
                "currency" => {
                    if let Some(num) = value.as_f64() {
                        Ok(serde_json::Value::String(format!("${:.2}", num)))
                    } else {
                        Ok(value.clone())
                    }
                }
                "percentage" => {
                    if let Some(num) = value.as_f64() {
                        Ok(serde_json::Value::String(format!("{:.1}%", num * 100.0)))
                    } else {
                        Ok(value.clone())
                    }
                }
                "date" => {
                    if let Some(date_str) = value.as_str() {
                        // Parse and reformat date
                        if let Ok(parsed_date) = chrono::DateTime::parse_from_rfc3339(date_str) {
                            Ok(serde_json::Value::String(parsed_date.format("%Y-%m-%d").to_string()))
                        } else {
                            Ok(value.clone())
                        }
                    } else {
                        Ok(value.clone())
                    }
                }
                _ => Ok(value.clone()),
            }
        } else {
            Ok(value.clone())
        }
    }

    fn calculate_value(&self, row: &HashMap<String, serde_json::Value>, parameters: &HashMap<String, serde_json::Value>) -> Result<serde_json::Value, WarpError> {
        if let Some(expression) = parameters.get("expression").and_then(|v| v.as_str()) {
            // Simple expression evaluation (in a real implementation, use a proper expression parser)
            if expression.contains("+") {
                let parts: Vec<&str> = expression.split('+').collect();
                if parts.len() == 2 {
                    let left_val = row.get(parts[0].trim()).and_then(|v| v.as_f64()).unwrap_or(0.0);
                    let right_val = row.get(parts[1].trim()).and_then(|v| v.as_f64()).unwrap_or(0.0);
                    return Ok(serde_json::Value::Number(serde_json::Number::from_f64(left_val + right_val).unwrap()));
                }
            }
        }
        
        Ok(serde_json::Value::Number(serde_json::Number::from(0)))
    }

    fn apply_aggregations(&self, data: &[HashMap<String, serde_json::Value>], aggregations: &[DataAggregation]) -> Result<Vec<HashMap<String, serde_json::Value>>, WarpError> {
        let mut result = Vec::new();
        
        for aggregation in aggregations {
            let mut groups: HashMap<String, Vec<&HashMap<String, serde_json::Value>>> = HashMap::new();
            
            // Group data
            for row in data {
                let group_key = if aggregation.group_by.is_empty() {
                    "all".to_string()
                } else {
                    aggregation.group_by.iter()
                        .map(|col| row.get(col).and_then(|v| v.as_str()).unwrap_or(""))
                        .collect::<Vec<_>>()
                        .join("|")
                };
                
                groups.entry(group_key).or_insert_with(Vec::new).push(row);
            }
            
            // Calculate aggregations
            for (group_key, group_rows) in groups {
                let mut agg_row = HashMap::new();
                
                // Add group by columns
                if !aggregation.group_by.is_empty() {
                    let group_values: Vec<&str> = group_key.split('|').collect();
                    for (i, col) in aggregation.group_by.iter().enumerate() {
                        if let Some(value) = group_values.get(i) {
                            agg_row.insert(col.clone(), serde_json::Value::String(value.to_string()));
                        }
                    }
                }
                
                // Calculate aggregation value
                let agg_value = match aggregation.aggregation_type {
                    AggregationType::Count => {
                        serde_json::Value::Number(serde_json::Number::from(group_rows.len()))
                    }
                    AggregationType::Sum => {
                        let sum: f64 = group_rows.iter()
                            .filter_map(|row| row.get(&aggregation.column).and_then(|v| v.as_f64()))
                            .sum();
                        serde_json::Value::Number(serde_json::Number::from_f64(sum).unwrap())
                    }
                    AggregationType::Average => {
                        let values: Vec<f64> = group_rows.iter()
                            .filter_map(|row| row.get(&aggregation.column).and_then(|v| v.as_f64()))
                            .collect();
                        let avg = if values.is_empty() { 0.0 } else { values.iter().sum::<f64>() / values.len() as f64 };
                        serde_json::Value::Number(serde_json::Number::from_f64(avg).unwrap())
                    }
                    AggregationType::Min => {
                        let min = group_rows.iter()
                            .filter_map(|row| row.get(&aggregation.column).and_then(|v| v.as_f64()))
                            .fold(f64::INFINITY, f64::min);
                        serde_json::Value::Number(serde_json::Number::from_f64(if min.is_infinite() { 0.0 } else { min }).unwrap())
                    }
                    AggregationType::Max => {
                        let max = group_rows.iter()
                            .filter_map(|row| row.get(&aggregation.column).and_then(|v| v.as_f64()))
                            .fold(f64::NEG_INFINITY, f64::max);
                        serde_json::Value::Number(serde_json::Number::from_f64(if max.is_infinite() { 0.0 } else { max }).unwrap())
                    }
                    _ => serde_json::Value::Number(serde_json::Number::from(0)),
                };
                
                agg_row.insert(aggregation.alias.clone(), agg_value);
                result.push(agg_row);
            }
        }
        
        Ok(result)
    }

    async fn save_to_destination(&self, destination: &ExportDestination, data: &[u8]) -> Result<PathBuf, WarpError> {
        match destination {
            ExportDestination::LocalFile { path } => {
                tokio::fs::write(path, data).await?;
                Ok(path.clone())
            }
            ExportDestination::S3 { bucket, key, region: _ } => {
                // In a real implementation, upload to S3
                let local_path = PathBuf::from(format!("/tmp/export_{}_{}", bucket, key));
                tokio::fs::write(&local_path, data).await?;
                Ok(local_path)
            }
            ExportDestination::Email { recipients, subject: _ } => {
                // In a real implementation, send email with attachment
                let local_path = PathBuf::from(format!("/tmp/export_email_{}.dat", recipients.join("_")));
                tokio::fs::write(&local_path, data).await?;
                Ok(local_path)
            }
            _ => {
                // For other destinations, save locally as fallback
                let local_path = PathBuf::from("/tmp/export_fallback.dat");
                tokio::fs::write(&local_path, data).await?;
                Ok(local_path)
            }
        }
    }
}
