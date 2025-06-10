use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use serde::{Deserialize, Serialize};
use crate::error::WarpError;

pub mod dashboard_engine;
pub mod chart_builder;
pub mod data_processor;
pub mod interactive_widgets;
pub mod real_time_updates;
pub mod export_renderer;
pub mod theme_manager;
pub mod layout_manager;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dashboard {
    pub id: String,
    pub name: String,
    pub description: String,
    pub owner_id: String,
    pub layout: DashboardLayout,
    pub widgets: Vec<Widget>,
    pub data_sources: Vec<DataSource>,
    pub filters: Vec<Filter>,
    pub theme: DashboardTheme,
    pub settings: DashboardSettings,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub shared_with: Vec<String>,
    pub is_public: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardLayout {
    pub layout_type: LayoutType,
    pub grid_config: GridConfig,
    pub responsive_breakpoints: HashMap<String, BreakpointConfig>,
    pub auto_arrange: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LayoutType {
    Grid,
    Masonry,
    Flex,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GridConfig {
    pub columns: u32,
    pub rows: u32,
    pub gap: u32,
    pub margin: u32,
    pub min_widget_width: u32,
    pub min_widget_height: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreakpointConfig {
    pub min_width: u32,
    pub columns: u32,
    pub widget_scaling: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Widget {
    pub id: String,
    pub widget_type: WidgetType,
    pub title: String,
    pub position: WidgetPosition,
    pub size: WidgetSize,
    pub data_source_id: String,
    pub query: DataQuery,
    pub visualization_config: VisualizationConfig,
    pub interaction_config: InteractionConfig,
    pub refresh_interval: Option<u64>,
    pub last_updated: chrono::DateTime<chrono::Utc>,
    pub is_visible: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WidgetType {
    LineChart,
    BarChart,
    PieChart,
    ScatterPlot,
    Heatmap,
    Table,
    Metric,
    Gauge,
    Sparkline,
    TreeMap,
    Sankey,
    Funnel,
    Radar,
    Candlestick,
    Histogram,
    BoxPlot,
    Map,
    Network,
    Timeline,
    Calendar,
    Text,
    Image,
    Video,
    IFrame,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WidgetPosition {
    pub x: u32,
    pub y: u32,
    pub z_index: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WidgetSize {
    pub width: u32,
    pub height: u32,
    pub min_width: u32,
    pub min_height: u32,
    pub max_width: Option<u32>,
    pub max_height: Option<u32>,
    pub resizable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSource {
    pub id: String,
    pub name: String,
    pub source_type: DataSourceType,
    pub connection_config: ConnectionConfig,
    pub schema: Option<DataSchema>,
    pub refresh_interval: u64,
    pub cache_duration: u64,
    pub last_updated: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataSourceType {
    Analytics,
    Marketplace,
    Performance,
    UserBehavior,
    CICD,
    Collaboration,
    CustomMetrics,
    Database,
    API,
    File,
    RealTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionConfig {
    pub endpoint: String,
    pub authentication: AuthenticationConfig,
    pub headers: HashMap<String, String>,
    pub parameters: HashMap<String, String>,
    pub timeout: u64,
    pub retry_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthenticationConfig {
    None,
    ApiKey { key: String },
    Bearer { token: String },
    Basic { username: String, password: String },
    OAuth { client_id: String, client_secret: String, token_url: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSchema {
    pub fields: Vec<DataField>,
    pub primary_key: Option<String>,
    pub relationships: Vec<DataRelationship>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataField {
    pub name: String,
    pub field_type: DataFieldType,
    pub nullable: bool,
    pub description: Option<String>,
    pub format: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataFieldType {
    String,
    Integer,
    Float,
    Boolean,
    DateTime,
    Date,
    Time,
    JSON,
    Array,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataRelationship {
    pub from_field: String,
    pub to_table: String,
    pub to_field: String,
    pub relationship_type: RelationshipType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RelationshipType {
    OneToOne,
    OneToMany,
    ManyToOne,
    ManyToMany,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataQuery {
    pub query_type: QueryType,
    pub query_string: String,
    pub parameters: HashMap<String, serde_json::Value>,
    pub aggregations: Vec<Aggregation>,
    pub filters: Vec<QueryFilter>,
    pub sorting: Vec<SortConfig>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QueryType {
    SQL,
    GraphQL,
    REST,
    NoSQL,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Aggregation {
    pub field: String,
    pub function: AggregationFunction,
    pub alias: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AggregationFunction {
    Count,
    Sum,
    Average,
    Min,
    Max,
    Median,
    StdDev,
    Variance,
    Percentile(f32),
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryFilter {
    pub field: String,
    pub operator: FilterOperator,
    pub value: serde_json::Value,
    pub logical_operator: Option<LogicalOperator>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FilterOperator {
    Equals,
    NotEquals,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
    Contains,
    StartsWith,
    EndsWith,
    In,
    NotIn,
    IsNull,
    IsNotNull,
    Between,
    Regex,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogicalOperator {
    And,
    Or,
    Not,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SortConfig {
    pub field: String,
    pub direction: SortDirection,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SortDirection {
    Ascending,
    Descending,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualizationConfig {
    pub chart_config: ChartConfig,
    pub color_scheme: ColorScheme,
    pub styling: WidgetStyling,
    pub animations: AnimationConfig,
    pub legends: LegendConfig,
    pub axes: AxesConfig,
    pub tooltips: TooltipConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartConfig {
    pub chart_type: WidgetType,
    pub series: Vec<SeriesConfig>,
    pub options: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeriesConfig {
    pub name: String,
    pub data_field: String,
    pub color: Option<String>,
    pub line_style: Option<LineStyle>,
    pub marker_style: Option<MarkerStyle>,
    pub fill_style: Option<FillStyle>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineStyle {
    pub width: f32,
    pub dash_pattern: Option<Vec<f32>>,
    pub cap_style: LineCapStyle,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LineCapStyle {
    Round,
    Square,
    Butt,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarkerStyle {
    pub shape: MarkerShape,
    pub size: f32,
    pub color: Option<String>,
    pub border_color: Option<String>,
    pub border_width: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MarkerShape {
    Circle,
    Square,
    Triangle,
    Diamond,
    Cross,
    Plus,
    Star,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FillStyle {
    pub color: String,
    pub opacity: f32,
    pub gradient: Option<GradientConfig>,
    pub pattern: Option<PatternConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GradientConfig {
    pub gradient_type: GradientType,
    pub stops: Vec<GradientStop>,
    pub direction: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GradientType {
    Linear,
    Radial,
    Conic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GradientStop {
    pub offset: f32,
    pub color: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternConfig {
    pub pattern_type: PatternType,
    pub color: String,
    pub background_color: String,
    pub size: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PatternType {
    Dots,
    Lines,
    Diagonal,
    Grid,
    Zigzag,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorScheme {
    pub scheme_type: ColorSchemeType,
    pub colors: Vec<String>,
    pub background_color: String,
    pub text_color: String,
    pub accent_color: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ColorSchemeType {
    Categorical,
    Sequential,
    Diverging,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WidgetStyling {
    pub background_color: String,
    pub border_color: String,
    pub border_width: f32,
    pub border_radius: f32,
    pub shadow: Option<ShadowConfig>,
    pub padding: PaddingConfig,
    pub font_family: String,
    pub font_size: f32,
    pub font_weight: FontWeight,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShadowConfig {
    pub x_offset: f32,
    pub y_offset: f32,
    pub blur_radius: f32,
    pub spread_radius: f32,
    pub color: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaddingConfig {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FontWeight {
    Thin,
    Light,
    Normal,
    Medium,
    Bold,
    ExtraBold,
    Black,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationConfig {
    pub enabled: bool,
    pub duration: u64,
    pub easing: EasingFunction,
    pub delay: u64,
    pub loop_animation: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EasingFunction {
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
    Bounce,
    Elastic,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegendConfig {
    pub enabled: bool,
    pub position: LegendPosition,
    pub orientation: LegendOrientation,
    pub styling: WidgetStyling,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LegendPosition {
    Top,
    Bottom,
    Left,
    Right,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Custom { x: f32, y: f32 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LegendOrientation {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AxesConfig {
    pub x_axis: AxisConfig,
    pub y_axis: AxisConfig,
    pub secondary_y_axis: Option<AxisConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AxisConfig {
    pub enabled: bool,
    pub title: Option<String>,
    pub min_value: Option<f64>,
    pub max_value: Option<f64>,
    pub tick_interval: Option<f64>,
    pub tick_format: Option<String>,
    pub grid_lines: bool,
    pub styling: WidgetStyling,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TooltipConfig {
    pub enabled: bool,
    pub format: String,
    pub styling: WidgetStyling,
    pub trigger: TooltipTrigger,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TooltipTrigger {
    Hover,
    Click,
    Focus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractionConfig {
    pub zoom_enabled: bool,
    pub pan_enabled: bool,
    pub selection_enabled: bool,
    pub drill_down_enabled: bool,
    pub cross_filter_enabled: bool,
    pub export_enabled: bool,
    pub click_actions: Vec<ClickAction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClickAction {
    pub action_type: ClickActionType,
    pub target: String,
    pub parameters: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClickActionType {
    NavigateTo,
    OpenModal,
    UpdateFilter,
    DrillDown,
    ExportData,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Filter {
    pub id: String,
    pub name: String,
    pub field: String,
    pub filter_type: FilterType,
    pub options: Vec<FilterOption>,
    pub default_value: Option<serde_json::Value>,
    pub required: bool,
    pub multiple_selection: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FilterType {
    Dropdown,
    MultiSelect,
    DateRange,
    NumberRange,
    Text,
    Checkbox,
    Radio,
    Slider,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterOption {
    pub label: String,
    pub value: serde_json::Value,
    pub selected: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardTheme {
    pub name: String,
    pub primary_color: String,
    pub secondary_color: String,
    pub background_color: String,
    pub surface_color: String,
    pub text_color: String,
    pub accent_color: String,
    pub error_color: String,
    pub warning_color: String,
    pub success_color: String,
    pub info_color: String,
    pub font_family: String,
    pub border_radius: f32,
    pub shadow_elevation: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardSettings {
    pub auto_refresh: bool,
    pub refresh_interval: u64,
    pub enable_animations: bool,
    pub enable_tooltips: bool,
    pub enable_legends: bool,
    pub responsive_design: bool,
    pub dark_mode: bool,
    pub high_contrast: bool,
    pub reduced_motion: bool,
    pub export_formats: Vec<ExportFormat>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExportFormat {
    PNG,
    SVG,
    PDF,
    Excel,
    CSV,
    JSON,
}

pub struct VisualizationManager {
    dashboards: Arc<Mutex<HashMap<String, Dashboard>>>,
    dashboard_engine: Arc<dashboard_engine::DashboardEngine>,
    chart_builder: Arc<chart_builder::ChartBuilder>,
    data_processor: Arc<data_processor::DataProcessor>,
    interactive_widgets: Arc<interactive_widgets::InteractiveWidgetManager>,
    real_time_updates: Arc<real_time_updates::RealTimeUpdateManager>,
    export_renderer: Arc<export_renderer::ExportRenderer>,
    theme_manager: Arc<theme_manager::ThemeManager>,
    layout_manager: Arc<layout_manager::LayoutManager>,
}

impl VisualizationManager {
    pub async fn new() -> Result<Self, WarpError> {
        Ok(Self {
            dashboards: Arc::new(Mutex::new(HashMap::new())),
            dashboard_engine: Arc::new(dashboard_engine::DashboardEngine::new().await?),
            chart_builder: Arc::new(chart_builder::ChartBuilder::new().await?),
            data_processor: Arc::new(data_processor::DataProcessor::new().await?),
            interactive_widgets: Arc::new(interactive_widgets::InteractiveWidgetManager::new().await?),
            real_time_updates: Arc::new(real_time_updates::RealTimeUpdateManager::new().await?),
            export_renderer: Arc::new(export_renderer::ExportRenderer::new().await?),
            theme_manager: Arc::new(theme_manager::ThemeManager::new().await?),
            layout_manager: Arc::new(layout_manager::LayoutManager::new().await?),
        })
    }

    pub async fn create_dashboard(&self, owner_id: &str, name: &str, description: &str) -> Result<String, WarpError> {
        let dashboard_id = uuid::Uuid::new_v4().to_string();
        
        let dashboard = Dashboard {
            id: dashboard_id.clone(),
            name: name.to_string(),
            description: description.to_string(),
            owner_id: owner_id.to_string(),
            layout: DashboardLayout {
                layout_type: LayoutType::Grid,
                grid_config: GridConfig {
                    columns: 12,
                    rows: 8,
                    gap: 16,
                    margin: 16,
                    min_widget_width: 200,
                    min_widget_height: 150,
                },
                responsive_breakpoints: HashMap::from([
                    ("mobile".to_string(), BreakpointConfig { min_width: 0, columns: 1, widget_scaling: 0.8 }),
                    ("tablet".to_string(), BreakpointConfig { min_width: 768, columns: 6, widget_scaling: 0.9 }),
                    ("desktop".to_string(), BreakpointConfig { min_width: 1024, columns: 12, widget_scaling: 1.0 }),
                ]),
                auto_arrange: false,
            },
            widgets: Vec::new(),
            data_sources: Vec::new(),
            filters: Vec::new(),
            theme: self.theme_manager.get_default_theme().await?,
            settings: DashboardSettings::default(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            shared_with: Vec::new(),
            is_public: false,
        };

        let mut dashboards = self.dashboards.lock().await;
        dashboards.insert(dashboard_id.clone(), dashboard);

        Ok(dashboard_id)
    }

    pub async fn add_widget(&self, dashboard_id: &str, widget_type: WidgetType, title: &str, position: WidgetPosition, size: WidgetSize) -> Result<String, WarpError> {
        let widget_id = uuid::Uuid::new_v4().to_string();
        
        let widget = Widget {
            id: widget_id.clone(),
            widget_type: widget_type.clone(),
            title: title.to_string(),
            position,
            size,
            data_source_id: String::new(),
            query: DataQuery {
                query_type: QueryType::REST,
                query_string: String::new(),
                parameters: HashMap::new(),
                aggregations: Vec::new(),
                filters: Vec::new(),
                sorting: Vec::new(),
                limit: None,
                offset: None,
            },
            visualization_config: self.get_default_visualization_config(&widget_type).await?,
            interaction_config: InteractionConfig {
                zoom_enabled: true,
                pan_enabled: true,
                selection_enabled: true,
                drill_down_enabled: false,
                cross_filter_enabled: true,
                export_enabled: true,
                click_actions: Vec::new(),
            },
            refresh_interval: Some(30),
            last_updated: chrono::Utc::now(),
            is_visible: true,
        };

        let mut dashboards = self.dashboards.lock().await;
        if let Some(dashboard) = dashboards.get_mut(dashboard_id) {
            dashboard.widgets.push(widget);
            dashboard.updated_at = chrono::Utc::now();
            Ok(widget_id)
        } else {
            Err(WarpError::ConfigError("Dashboard not found".to_string()))
        }
    }

    pub async fn add_data_source(&self, dashboard_id: &str, name: &str, source_type: DataSourceType, connection_config: ConnectionConfig) -> Result<String, WarpError> {
        let data_source_id = uuid::Uuid::new_v4().to_string();
        
        let data_source = DataSource {
            id: data_source_id.clone(),
            name: name.to_string(),
            source_type,
            connection_config,
            schema: None,
            refresh_interval: 300, // 5 minutes
            cache_duration: 600,   // 10 minutes
            last_updated: chrono::Utc::now(),
        };

        let mut dashboards = self.dashboards.lock().await;
        if let Some(dashboard) = dashboards.get_mut(dashboard_id) {
            dashboard.data_sources.push(data_source);
            dashboard.updated_at = chrono::Utc::now();
            Ok(data_source_id)
        } else {
            Err(WarpError::ConfigError("Dashboard not found".to_string()))
        }
    }

    pub async fn render_dashboard(&self, dashboard_id: &str, format: RenderFormat) -> Result<RenderResult, WarpError> {
        let dashboards = self.dashboards.lock().await;
        if let Some(dashboard) = dashboards.get(dashboard_id) {
            self.dashboard_engine.render_dashboard(dashboard, format).await
        } else {
            Err(WarpError::ConfigError("Dashboard not found".to_string()))
        }
    }

    pub async fn update_widget_data(&self, dashboard_id: &str, widget_id: &str) -> Result<(), WarpError> {
        let dashboards = self.dashboards.lock().await;
        if let Some(dashboard) = dashboards.get(dashboard_id) {
            if let Some(widget) = dashboard.widgets.iter().find(|w| w.id == widget_id) {
                if let Some(data_source) = dashboard.data_sources.iter().find(|ds| ds.id == widget.data_source_id) {
                    let data = self.data_processor.fetch_data(data_source, &widget.query).await?;
                    self.real_time_updates.update_widget_data(dashboard_id, widget_id, data).await?;
                }
            }
        }
        Ok(())
    }

    pub async fn export_dashboard(&self, dashboard_id: &str, format: ExportFormat) -> Result<Vec<u8>, WarpError> {
        let dashboards = self.dashboards.lock().await;
        if let Some(dashboard) = dashboards.get(dashboard_id) {
            self.export_renderer.export_dashboard(dashboard, format).await
        } else {
            Err(WarpError::ConfigError("Dashboard not found".to_string()))
        }
    }

    async fn get_default_visualization_config(&self, widget_type: &WidgetType) -> Result<VisualizationConfig, WarpError> {
        Ok(VisualizationConfig {
            chart_config: ChartConfig {
                chart_type: widget_type.clone(),
                series: Vec::new(),
                options: HashMap::new(),
            },
            color_scheme: ColorScheme {
                scheme_type: ColorSchemeType::Categorical,
                colors: vec![
                    "#3498db".to_string(),
                    "#e74c3c".to_string(),
                    "#2ecc71".to_string(),
                    "#f39c12".to_string(),
                    "#9b59b6".to_string(),
                ],
                background_color: "#ffffff".to_string(),
                text_color: "#2c3e50".to_string(),
                accent_color: "#3498db".to_string(),
            },
            styling: WidgetStyling {
                background_color: "#ffffff".to_string(),
                border_color: "#ecf0f1".to_string(),
                border_width: 1.0,
                border_radius: 8.0,
                shadow: Some(ShadowConfig {
                    x_offset: 0.0,
                    y_offset: 2.0,
                    blur_radius: 4.0,
                    spread_radius: 0.0,
                    color: "rgba(0,0,0,0.1)".to_string(),
                }),
                padding: PaddingConfig {
                    top: 16.0,
                    right: 16.0,
                    bottom: 16.0,
                    left: 16.0,
                },
                font_family: "Inter, sans-serif".to_string(),
                font_size: 14.0,
                font_weight: FontWeight::Normal,
            },
            animations: AnimationConfig {
                enabled: true,
                duration: 300,
                easing: EasingFunction::EaseInOut,
                delay: 0,
                loop_animation: false,
            },
            legends: LegendConfig {
                enabled: true,
                position: LegendPosition::Bottom,
                orientation: LegendOrientation::Horizontal,
                styling: WidgetStyling {
                    background_color: "transparent".to_string(),
                    border_color: "transparent".to_string(),
                    border_width: 0.0,
                    border_radius: 0.0,
                    shadow: None,
                    padding: PaddingConfig {
                        top: 8.0,
                        right: 8.0,
                        bottom: 8.0,
                        left: 8.0,
                    },
                    font_family: "Inter, sans-serif".to_string(),
                    font_size: 12.0,
                    font_weight: FontWeight::Normal,
                },
            },
            axes: AxesConfig {
                x_axis: AxisConfig {
                    enabled: true,
                    title: None,
                    min_value: None,
                    max_value: None,
                    tick_interval: None,
                    tick_format: None,
                    grid_lines: true,
                    styling: WidgetStyling {
                        background_color: "transparent".to_string(),
                        border_color: "#ecf0f1".to_string(),
                        border_width: 1.0,
                        border_radius: 0.0,
                        shadow: None,
                        padding: PaddingConfig {
                            top: 0.0,
                            right: 0.0,
                            bottom: 0.0,
                            left: 0.0,
                        },
                        font_family: "Inter, sans-serif".to_string(),
                        font_size: 11.0,
                        font_weight: FontWeight::Normal,
                    },
                },
                y_axis: AxisConfig {
                    enabled: true,
                    title: None,
                    min_value: None,
                    max_value: None,
                    tick_interval: None,
                    tick_format: None,
                    grid_lines: true,
                    styling: WidgetStyling {
                        background_color: "transparent".to_string(),
                        border_color: "#ecf0f1".to_string(),
                        border_width: 1.0,
                        border_radius: 0.0,
                        shadow: None,
                        padding: PaddingConfig {
                            top: 0.0,
                            right: 0.0,
                            bottom: 0.0,
                            left: 0.0,
                        },
                        font_family: "Inter, sans-serif".to_string(),
                        font_size: 11.0,
                        font_weight: FontWeight::Normal,
                    },
                },
                secondary_y_axis: None,
            },
            tooltips: TooltipConfig {
                enabled: true,
                format: "{series.name}: {point.y}".to_string(),
                styling: WidgetStyling {
                    background_color: "rgba(0,0,0,0.8)".to_string(),
                    border_color: "transparent".to_string(),
                    border_width: 0.0,
                    border_radius: 4.0,
                    shadow: None,
                    padding: PaddingConfig {
                        top: 8.0,
                        right: 12.0,
                        bottom: 8.0,
                        left: 12.0,
                    },
                    font_family: "Inter, sans-serif".to_string(),
                    font_size: 12.0,
                    font_weight: FontWeight::Normal,
                },
                trigger: TooltipTrigger::Hover,
            },
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RenderFormat {
    HTML,
    Canvas,
    SVG,
    WebGL,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderResult {
    pub content: String,
    pub metadata: RenderMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderMetadata {
    pub render_time: std::time::Duration,
    pub data_points: u32,
    pub widgets_rendered: u32,
    pub cache_hits: u32,
    pub errors: Vec<String>,
}

impl Default for DashboardSettings {
    fn default() -> Self {
        Self {
            auto_refresh: true,
            refresh_interval: 30,
            enable_animations: true,
            enable_tooltips: true,
            enable_legends: true,
            responsive_design: true,
            dark_mode: false,
            high_contrast: false,
            reduced_motion: false,
            export_formats: vec![
                ExportFormat::PNG,
                ExportFormat::SVG,
                ExportFormat::PDF,
                ExportFormat::CSV,
                ExportFormat::JSON,
            ],
        }
    }
}
