use super::*;
use crate::error::WarpError;
use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect, Alignment},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{
        Block, Borders, Chart, Dataset, Axis, GraphType, Gauge, List, ListItem, 
        Paragraph, Tabs, Table, Row, Cell, BarChart, Sparkline
    },
    symbols,
    Frame,
};
use std::collections::VecDeque;

pub struct AnalyticsDashboard {
    current_tab: DashboardTab,
    selected_item: Option<String>,
    time_range: TimeRange,
    real_time_data: HashMap<String, VecDeque<f64>>,
    refresh_interval: std::time::Duration,
    last_refresh: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub enum DashboardTab {
    Overview,
    Performance,
    Usage,
    UserBehavior,
    Marketplace,
    RealTime,
    Alerts,
}

impl AnalyticsDashboard {
    pub async fn new() -> Result<Self, WarpError> {
        Ok(Self {
            current_tab: DashboardTab::Overview,
            selected_item: None,
            time_range: TimeRange::LastDay,
            real_time_data: HashMap::new(),
            refresh_interval: std::time::Duration::from_secs(30),
            last_refresh: Utc::now(),
        })
    }

    pub async fn render<B: Backend>(
        &mut self,
        f: &mut Frame<B>,
        analytics: &AnalyticsEngine,
    ) -> Result<(), WarpError> {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),  // Header/Tabs
                Constraint::Min(0),     // Main content
                Constraint::Length(3),  // Status bar
            ])
            .split(f.size());

        // Render tabs
        self.render_tabs(f, chunks[0]);

        // Render main content based on current tab
        match self.current_tab {
            DashboardTab::Overview => self.render_overview(f, chunks[1], analytics).await?,
            DashboardTab::Performance => self.render_performance(f, chunks[1], analytics).await?,
            DashboardTab::Usage => self.render_usage(f, chunks[1], analytics).await?,
            DashboardTab::UserBehavior => self.render_user_behavior(f, chunks[1], analytics).await?,
            DashboardTab::Marketplace => self.render_marketplace(f, chunks[1], analytics).await?,
            DashboardTab::RealTime => self.render_real_time(f, chunks[1], analytics).await?,
            DashboardTab::Alerts => self.render_alerts(f, chunks[1], analytics).await?,
        }

        // Render status bar
        self.render_status_bar(f, chunks[2]);

        Ok(())
    }

    fn render_tabs<B: Backend>(&self, f: &mut Frame<B>, area: Rect) {
        let titles = vec![
            "Overview",
            "Performance",
            "Usage",
            "User Behavior",
            "Marketplace",
            "Real-time",
            "Alerts",
        ];
        
        let selected_tab = match self.current_tab {
            DashboardTab::Overview => 0,
            DashboardTab::Performance => 1,
            DashboardTab::Usage => 2,
            DashboardTab::UserBehavior => 3,
            DashboardTab::Marketplace => 4,
            DashboardTab::RealTime => 5,
            DashboardTab::Alerts => 6,
        };

        let tabs = Tabs::new(titles)
            .block(Block::default().borders(Borders::ALL).title("Analytics Dashboard"))
            .select(selected_tab)
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));

        f.render_widget(tabs, area);
    }

    async fn render_overview<B: Backend>(
        &mut self,
        f: &mut Frame<B>,
        area: Rect,
        analytics: &AnalyticsEngine,
    ) -> Result<(), WarpError> {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(8),  // Key metrics
                Constraint::Min(0),     // Charts and details
            ])
            .split(area);

        // Key metrics row
        self.render_key_metrics(f, chunks[0], analytics).await?;

        // Charts section
        let chart_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(chunks[1]);

        // Usage trend chart
        self.render_usage_trend_chart(f, chart_chunks[0], analytics).await?;

        // Performance overview chart
        self.render_performance_overview_chart(f, chart_chunks[1], analytics).await?;

        Ok(())
    }

    async fn render_key_metrics<B: Backend>(
        &mut self,
        f: &mut Frame<B>,
        area: Rect,
        analytics: &AnalyticsEngine,
    ) -> Result<(), WarpError> {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
            ])
            .split(area);

        // Get marketplace analytics
        let marketplace_analytics = analytics.get_marketplace_analytics(self.time_range.clone()).await?;

        // Total Downloads
        let downloads_text = vec![
            Spans::from(vec![
                Span::styled("Total Downloads", Style::default().fg(Color::Gray)),
            ]),
            Spans::from(vec![
                Span::styled(
                    format!("{}", marketplace_analytics.total_downloads),
                    Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
                ),
            ]),
            Spans::from(vec![
                Span::styled("↗ +12.5%", Style::default().fg(Color::Green)),
            ]),
        ];

        let downloads_widget = Paragraph::new(downloads_text)
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Center);
        f.render_widget(downloads_widget, chunks[0]);

        // Active Users
        let users_text = vec![
            Spans::from(vec![
                Span::styled("Active Users", Style::default().fg(Color::Gray)),
            ]),
            Spans::from(vec![
                Span::styled(
                    format!("{}", marketplace_analytics.total_active_users),
                    Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD),
                ),
            ]),
            Spans::from(vec![
                Span::styled("↗ +8.3%", Style::default().fg(Color::Green)),
            ]),
        ];

        let users_widget = Paragraph::new(users_text)
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Center);
        f.render_widget(users_widget, chunks[1]);

        // Revenue
        let revenue_text = vec![
            Spans::from(vec![
                Span::styled("Revenue", Style::default().fg(Color::Gray)),
            ]),
            Spans::from(vec![
                Span::styled(
                    format!("${:.2}", marketplace_analytics.revenue_metrics.total_revenue),
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
                ),
            ]),
            Spans::from(vec![
                Span::styled("↗ +15.7%", Style::default().fg(Color::Green)),
            ]),
        ];

        let revenue_widget = Paragraph::new(revenue_text)
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Center);
        f.render_widget(revenue_widget, chunks[2]);

        // Conversion Rate
        let conversion_text = vec![
            Spans::from(vec![
                Span::styled("Conversion Rate", Style::default().fg(Color::Gray)),
            ]),
            Spans::from(vec![
                Span::styled(
                    format!("{:.1}%", marketplace_analytics.revenue_metrics.conversion_rate * 100.0),
                    Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD),
                ),
            ]),
            Spans::from(vec![
                Span::styled("↘ -2.1%", Style::default().fg(Color::Red)),
            ]),
        ];

        let conversion_widget = Paragraph::new(conversion_text)
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Center);
        f.render_widget(conversion_widget, chunks[3]);

        Ok(())
    }

    async fn render_usage_trend_chart<B: Backend>(
        &mut self,
        f: &mut Frame<B>,
        area: Rect,
        _analytics: &AnalyticsEngine,
    ) -> Result<(), WarpError> {
        // Mock data for demonstration
        let data: Vec<(f64, f64)> = (0..24)
            .map(|i| (i as f64, (i as f64 * 0.5).sin() * 50.0 + 100.0))
            .collect();

        let datasets = vec![Dataset::default()
            .name("Usage")
            .marker(symbols::Marker::Dot)
            .style(Style::default().fg(Color::Cyan))
            .graph_type(GraphType::Line)
            .data(&data)];

        let chart = Chart::new(datasets)
            .block(Block::default().title("Usage Trend (24h)").borders(Borders::ALL))
            .x_axis(
                Axis::default()
                    .title("Hours")
                    .style(Style::default().fg(Color::Gray))
                    .bounds([0.0, 24.0])
                    .labels(vec![
                        Span::styled("0", Style::default().add_modifier(Modifier::BOLD)),
                        Span::styled("12", Style::default().add_modifier(Modifier::BOLD)),
                        Span::styled("24", Style::default().add_modifier(Modifier::BOLD)),
                    ]),
            )
            .y_axis(
                Axis::default()
                    .title("Usage Count")
                    .style(Style::default().fg(Color::Gray))
                    .bounds([0.0, 200.0])
                    .labels(vec![
                        Span::styled("0", Style::default().add_modifier(Modifier::BOLD)),
                        Span::styled("100", Style::default().add_modifier(Modifier::BOLD)),
                        Span::styled("200", Style::default().add_modifier(Modifier::BOLD)),
                    ]),
            );

        f.render_widget(chart, area);
        Ok(())
    }

    async fn render_performance_overview_chart<B: Backend>(
        &mut self,
        f: &mut Frame<B>,
        area: Rect,
        _analytics: &AnalyticsEngine,
    ) -> Result<(), WarpError> {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        // CPU Usage Gauge
        let cpu_usage = 65; // Mock data
        let cpu_gauge = Gauge::default()
            .block(Block::default().title("CPU Usage").borders(Borders::ALL))
            .gauge_style(Style::default().fg(Color::Yellow))
            .percent(cpu_usage)
            .label(format!("{}%", cpu_usage));
        f.render_widget(cpu_gauge, chunks[0]);

        // Memory Usage Gauge
        let memory_usage = 42; // Mock data
        let memory_gauge = Gauge::default()
            .block(Block::default().title("Memory Usage").borders(Borders::ALL))
            .gauge_style(Style::default().fg(Color::Blue))
            .percent(memory_usage)
            .label(format!("{}%", memory_usage));
        f.render_widget(memory_gauge, chunks[1]);

        Ok(())
    }

    async fn render_performance<B: Backend>(
        &mut self,
        f: &mut Frame<B>,
        area: Rect,
        analytics: &AnalyticsEngine,
    ) -> Result<(), WarpError> {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
            .split(area);

        // Performance metrics table
        self.render_performance_table(f, chunks[0], analytics).await?;

        // Performance trends
        self.render_performance_trends(f, chunks[1], analytics).await?;

        Ok(())
    }

    async fn render_performance_table<B: Backend>(
        &mut self,
        f: &mut Frame<B>,
        area: Rect,
        _analytics: &AnalyticsEngine,
    ) -> Result<(), WarpError> {
        let header = vec!["Item", "Load Time", "CPU %", "Memory", "Score"];
        let rows = vec![
            vec!["catppuccin-theme", "120ms", "2.1%", "15MB", "9.2"],
            vec!["git-enhanced", "340ms", "8.5%", "45MB", "8.7"],
            vec!["docker-helper", "280ms", "5.2%", "32MB", "8.9"],
            vec!["ai-assistant", "890ms", "15.3%", "128MB", "7.8"],
            vec!["vim-keyset", "45ms", "0.8%", "8MB", "9.8"],
        ];

        let table_rows: Vec<Row> = rows
            .iter()
            .map(|row| {
                let cells: Vec<Cell> = row.iter().map(|&cell| Cell::from(cell)).collect();
                Row::new(cells)
            })
            .collect();

        let table = Table::new(table_rows)
            .header(
                Row::new(header.iter().map(|&h| Cell::from(h)).collect::<Vec<_>>())
                    .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
                    .bottom_margin(1),
            )
            .block(Block::default().title("Performance Metrics").borders(Borders::ALL))
            .widths(&[
                Constraint::Percentage(30),
                Constraint::Percentage(15),
                Constraint::Percentage(15),
                Constraint::Percentage(15),
                Constraint::Percentage(15),
            ])
            .column_spacing(1)
            .highlight_style(Style::default().bg(Color::DarkGray))
            .highlight_symbol("▶ ");

        f.render_widget(table, area);
        Ok(())
    }

    async fn render_performance_trends<B: Backend>(
        &mut self,
        f: &mut Frame<B>,
        area: Rect,
        _analytics: &AnalyticsEngine,
    ) -> Result<(), WarpError> {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(34),
            ])
            .split(area);

        // Load time sparkline
        let load_time_data = vec![120, 115, 130, 125, 118, 122, 119, 124, 121, 117];
        let load_time_sparkline = Sparkline::default()
            .block(Block::default().title("Load Time Trend").borders(Borders::ALL))
            .data(&load_time_data)
            .style(Style::default().fg(Color::Green));
        f.render_widget(load_time_sparkline, chunks[0]);

        // Error rate sparkline
        let error_rate_data = vec![2, 1, 3, 2, 1, 0, 1, 2, 1, 1];
        let error_rate_sparkline = Sparkline::default()
            .block(Block::default().title("Error Rate").borders(Borders::ALL))
            .data(&error_rate_data)
            .style(Style::default().fg(Color::Red));
        f.render_widget(error_rate_sparkline, chunks[1]);

        // Performance score sparkline
        let perf_score_data = vec![92, 91, 89, 90, 93, 94, 93, 91, 92, 94];
        let perf_score_sparkline = Sparkline::default()
            .block(Block::default().title("Performance Score").borders(Borders::ALL))
            .data(&perf_score_data)
            .style(Style::default().fg(Color::Blue));
        f.render_widget(perf_score_sparkline, chunks[2]);

        Ok(())
    }

    async fn render_usage<B: Backend>(
        &mut self,
        f: &mut Frame<B>,
        area: Rect,
        analytics: &AnalyticsEngine,
    ) -> Result<(), WarpError> {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        // Top items by usage
        self.render_top_items(f, chunks[0], analytics).await?;

        // Usage patterns
        self.render_usage_patterns(f, chunks[1], analytics).await?;

        Ok(())
    }

    async fn render_top_items<B: Backend>(
        &mut self,
        f: &mut Frame<B>,
        area: Rect,
        analytics: &AnalyticsEngine,
    ) -> Result<(), WarpError> {
        let marketplace_analytics = analytics.get_marketplace_analytics(self.time_range.clone()).await?;
        
        let items: Vec<ListItem> = marketplace_analytics.top_items
            .iter()
            .enumerate()
            .map(|(i, item)| {
                let spans = vec![
                    Span::styled(format!("{}. ", i + 1), Style::default().fg(Color::Gray)),
                    Span::styled(&item.name, Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
                    Span::raw(" - "),
                    Span::styled(format!("{} downloads", item.downloads), Style::default().fg(Color::Green)),
                    Span::raw(" "),
                    Span::styled(format!("⭐ {:.1}", item.rating), Style::default().fg(Color::Yellow)),
                ];
                ListItem::new(Spans::from(spans))
            })
            .collect();

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Top Items by Usage"))
            .highlight_style(Style::default().bg(Color::DarkGray))
            .highlight_symbol("▶ ");

        f.render_widget(list, area);
        Ok(())
    }

    async fn render_usage_patterns<B: Backend>(
        &mut self,
        f: &mut Frame<B>,
        area: Rect,
        _analytics: &AnalyticsEngine,
    ) -> Result<(), WarpError> {
        // Mock usage pattern data
        let data = vec![
            ("Themes", 45),
            ("Plugins", 32),
            ("AI Models", 18),
            ("Keysets", 12),
            ("Workflows", 8),
        ];

        let bar_chart = BarChart::default()
            .block(Block::default().title("Usage by Category").borders(Borders::ALL))
            .data(&data)
            .bar_width(8)
            .bar_style(Style::default().fg(Color::Cyan))
            .value_style(Style::default().fg(Color::White).add_modifier(Modifier::BOLD));

        f.render_widget(bar_chart, area);
        Ok(())
    }

    async fn render_user_behavior<B: Backend>(
        &mut self,
        f: &mut Frame<B>,
        area: Rect,
        _analytics: &AnalyticsEngine,
    ) -> Result<(), WarpError> {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        // User journey funnel
        self.render_user_journey_funnel(f, chunks[0]).await?;

        // Feature adoption
        self.render_feature_adoption(f, chunks[1]).await?;

        Ok(())
    }

    async fn render_user_journey_funnel<B: Backend>(
        &mut self,
        f: &mut Frame<B>,
        area: Rect,
    ) -> Result<(), WarpError> {
        let funnel_data = vec![
            ("Visitors", 1000, 100.0),
            ("Signups", 450, 45.0),
            ("Activations", 320, 32.0),
            ("First Purchase", 128, 12.8),
            ("Retention", 96, 9.6),
        ];

        let items: Vec<ListItem> = funnel_data
            .iter()
            .map(|(stage, count, percentage)| {
                let bar_width = (*percentage / 100.0 * 30.0) as usize;
                let bar = "█".repeat(bar_width);
                let spaces = " ".repeat(30 - bar_width);
                
                let spans = vec![
                    Span::styled(format!("{:<12}", stage), Style::default().fg(Color::White)),
                    Span::styled(bar, Style::default().fg(Color::Green)),
                    Span::raw(spaces),
                    Span::styled(format!(" {} ({:.1}%)", count, percentage), Style::default().fg(Color::Gray)),
                ];
                ListItem::new(Spans::from(spans))
            })
            .collect();

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("User Journey Funnel"));

        f.render_widget(list, area);
        Ok(())
    }

    async fn render_feature_adoption<B: Backend>(
        &mut self,
        f: &mut Frame<B>,
        area: Rect,
    ) -> Result<(), WarpError> {
        let adoption_data = vec![
            ("Search", 89),
            ("Install", 76),
            ("Rate", 34),
            ("Review", 23),
            ("Share", 12),
        ];

        let bar_chart = BarChart::default()
            .block(Block::default().title("Feature Adoption %").borders(Borders::ALL))
            .data(&adoption_data)
            .bar_width(6)
            .bar_style(Style::default().fg(Color::Magenta))
            .value_style(Style::default().fg(Color::White).add_modifier(Modifier::BOLD));

        f.render_widget(bar_chart, area);
        Ok(())
    }

    async fn render_marketplace<B: Backend>(
        &mut self,
        f: &mut Frame<B>,
        area: Rect,
        analytics: &AnalyticsEngine,
    ) -> Result<(), WarpError> {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(10), // Revenue metrics
                Constraint::Min(0),     // Category distribution and trends
            ])
            .split(area);

        // Revenue metrics
        self.render_revenue_metrics(f, chunks[0], analytics).await?;

        // Category distribution and trends
        let bottom_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(chunks[1]);

        self.render_category_distribution(f, bottom_chunks[0], analytics).await?;
        self.render_trending_items(f, bottom_chunks[1], analytics).await?;

        Ok(())
    }

    async fn render_revenue_metrics<B: Backend>(
        &mut self,
        f: &mut Frame<B>,
        area: Rect,
        analytics: &AnalyticsEngine,
    ) -> Result<(), WarpError> {
        let marketplace_analytics = analytics.get_marketplace_analytics(self.time_range.clone()).await?;
        let revenue = &marketplace_analytics.revenue_metrics;

        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
            ])
            .split(area);

        // Total Revenue
        let total_revenue_text = vec![
            Spans::from(vec![Span::styled("Total Revenue", Style::default().fg(Color::Gray))]),
            Spans::from(vec![Span::styled(
                format!("${:.2}", revenue.total_revenue),
                Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
            )]),
        ];
        let total_revenue_widget = Paragraph::new(total_revenue_text)
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Center);
        f.render_widget(total_revenue_widget, chunks[0]);

        // MRR
        let mrr_text = vec![
            Spans::from(vec![Span::styled("MRR", Style::default().fg(Color::Gray))]),
            Spans::from(vec![Span::styled(
                format!("${:.2}", revenue.monthly_recurring_revenue),
                Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD),
            )]),
        ];
        let mrr_widget = Paragraph::new(mrr_text)
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Center);
        f.render_widget(mrr_widget, chunks[1]);

        // ARPU
        let arpu_text = vec![
            Spans::from(vec![Span::styled("ARPU", Style::default().fg(Color::Gray))]),
            Spans::from(vec![Span::styled(
                format!("${:.2}", revenue.average_revenue_per_user),
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
            )]),
        ];
        let arpu_widget = Paragraph::new(arpu_text)
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Center);
        f.render_widget(arpu_widget, chunks[2]);

        // Churn Rate
        let churn_text = vec![
            Spans::from(vec![Span::styled("Churn Rate", Style::default().fg(Color::Gray))]),
            Spans::from(vec![Span::styled(
                format!("{:.1}%", revenue.churn_rate * 100.0),
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            )]),
        ];
        let churn_widget = Paragraph::new(churn_text)
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Center);
        f.render_widget(churn_widget, chunks[3]);

        Ok(())
    }

    async fn render_category_distribution<B: Backend>(
        &mut self,
        f: &mut Frame<B>,
        area: Rect,
        analytics: &AnalyticsEngine,
    ) -> Result<(), WarpError> {
        let marketplace_analytics = analytics.get_marketplace_analytics(self.time_range.clone()).await?;
        
        let data: Vec<(&str, u64)> = marketplace_analytics.category_distribution
            .iter()
            .map(|(category, count)| (category.as_str(), *count as u64))
            .collect();

        let bar_chart = BarChart::default()
            .block(Block::default().title("Items by Category").borders(Borders::ALL))
            .data(&data)
            .bar_width(8)
            .bar_style(Style::default().fg(Color::Cyan))
            .value_style(Style::default().fg(Color::White).add_modifier(Modifier::BOLD));

        f.render_widget(bar_chart, area);
        Ok(())
    }

    async fn render_trending_items<B: Backend>(
        &mut self,
        f: &mut Frame<B>,
        area: Rect,
        analytics: &AnalyticsEngine,
    ) -> Result<(), WarpError> {
        let marketplace_analytics = analytics.get_marketplace_analytics(self.time_range.clone()).await?;
        
        let items: Vec<ListItem> = marketplace_analytics.trending_items
            .iter()
            .enumerate()
            .map(|(i, item)| {
                let spans = vec![
                    Span::styled(format!("{}. ", i + 1), Style::default().fg(Color::Gray)),
                    Span::styled(&item.name, Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
                    Span::raw(" "),
                    Span::styled(format!("🔥 {:.1}", item.momentum_score), Style::default().fg(Color::Red)),
                    Span::raw(" "),
                    Span::styled(format!("↗ {:.1}%", item.growth_rate * 100.0), Style::default().fg(Color::Green)),
                ];
                ListItem::new(Spans::from(spans))
            })
            .collect();

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Trending Items"))
            .highlight_style(Style::default().bg(Color::DarkGray))
            .highlight_symbol("▶ ");

        f.render_widget(list, area);
        Ok(())
    }

    async fn render_real_time<B: Backend>(
        &mut self,
        f: &mut Frame<B>,
        area: Rect,
        _analytics: &AnalyticsEngine,
    ) -> Result<(), WarpError> {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(6),  // Real-time stats
                Constraint::Min(0),     // Live charts
            ])
            .split(area);

        // Real-time stats
        self.render_real_time_stats(f, chunks[0]).await?;

        // Live activity chart
        self.render_live_activity_chart(f, chunks[1]).await?;

        Ok(())
    }

    async fn render_real_time_stats<B: Backend>(
        &mut self,
        f: &mut Frame<B>,
        area: Rect,
    ) -> Result<(), WarpError> {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
            ])
            .split(area);

        // Active Users
        let active_users_text = vec![
            Spans::from(vec![Span::styled("Active Users", Style::default().fg(Color::Gray))]),
            Spans::from(vec![Span::styled(
                "1,247",
                Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
            )]),
            Spans::from(vec![Span::styled("🟢 Live", Style::default().fg(Color::Green))]),
        ];
        let active_users_widget = Paragraph::new(active_users_text)
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Center);
        f.render_widget(active_users_widget, chunks[0]);

        // Downloads/min
        let downloads_text = vec![
            Spans::from(vec![Span::styled("Downloads/min", Style::default().fg(Color::Gray))]),
            Spans::from(vec![Span::styled(
                "23",
                Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD),
            )]),
            Spans::from(vec![Span::styled("📈 +15%", Style::default().fg(Color::Green))]),
        ];
        let downloads_widget = Paragraph::new(downloads_text)
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Center);
        f.render_widget(downloads_widget, chunks[1]);

        // Error Rate
        let error_rate_text = vec![
            Spans::from(vec![Span::styled("Error Rate", Style::default().fg(Color::Gray))]),
            Spans::from(vec![Span::styled(
                "0.12%",
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
            )]),
            Spans::from(vec![Span::styled("⚠️ Normal", Style::default().fg(Color::Yellow))]),
        ];
        let error_rate_widget = Paragraph::new(error_rate_text)
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Center);
        f.render_widget(error_rate_widget, chunks[2]);

        // System Load
        let system_load_text = vec![
            Spans::from(vec![Span::styled("System Load", Style::default().fg(Color::Gray))]),
            Spans::from(vec![Span::styled(
                "68%",
                Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD),
            )]),
            Spans::from(vec![Span::styled("⚡ Optimal", Style::default().fg(Color::Green))]),
        ];
        let system_load_widget = Paragraph::new(system_load_text)
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Center);
        f.render_widget(system_load_widget, chunks[3]);

        Ok(())
    }

    async fn render_live_activity_chart<B: Backend>(
        &mut self,
        f: &mut Frame<B>,
        area: Rect,
    ) -> Result<(), WarpError> {
        // Mock real-time data
        let data: Vec<(f64, f64)> = (0..60)
            .map(|i| {
                let time = i as f64;
                let activity = (time * 0.1).sin() * 20.0 + 50.0 + (time * 0.05).cos() * 10.0;
                (time, activity)
            })
            .collect();

        let datasets = vec![Dataset::default()
            .name("Live Activity")
            .marker(symbols::Marker::Braille)
            .style(Style::default().fg(Color::Green))
            .graph_type(GraphType::Line)
            .data(&data)];

        let chart = Chart::new(datasets)
            .block(Block::default().title("Live Activity (Last 60 minutes)").borders(Borders::ALL))
            .x_axis(
                Axis::default()
                    .title("Minutes Ago")
                    .style(Style::default().fg(Color::Gray))
                    .bounds([0.0, 60.0])
                    .labels(vec![
                        Span::styled("60", Style::default().add_modifier(Modifier::BOLD)),
                        Span::styled("30", Style::default().add_modifier(Modifier::BOLD)),
                        Span::styled("0", Style::default().add_modifier(Modifier::BOLD)),
                    ]),
            )
            .y_axis(
                Axis::default()
                    .title("Activity Level")
                    .style(Style::default().fg(Color::Gray))
                    .bounds([0.0, 100.0])
                    .labels(vec![
                        Span::styled("0", Style::default().add_modifier(Modifier::BOLD)),
                        Span::styled("50", Style::default().add_modifier(Modifier::BOLD)),
                        Span::styled("100", Style::default().add_modifier(Modifier::BOLD)),
                    ]),
            );

        f.render_widget(chart, area);
        Ok(())
    }

    async fn render_alerts<B: Backend>(
        &mut self,
        f: &mut Frame<B>,
        area: Rect,
        _analytics: &AnalyticsEngine,
    ) -> Result<(), WarpError> {
        // Mock alerts data
        let alerts = vec![
            ("🔴 Critical", "High memory usage detected in git-enhanced plugin", "2 min ago"),
            ("🟡 Warning", "Error rate increased by 15% for ai-assistant", "5 min ago"),
            ("🔵 Info", "New trending item: catppuccin-theme", "12 min ago"),
            ("🟡 Warning", "Slow response time for marketplace API", "18 min ago"),
            ("🔴 Critical", "Plugin crash detected: docker-helper", "25 min ago"),
        ];

        let items: Vec<ListItem> = alerts
            .iter()
            .map(|(severity, message, time)| {
                let spans = vec![
                    Span::raw(format!("{} ", severity)),
                    Span::styled(message, Style::default().fg(Color::White)),
                    Span::styled(format!(" ({})", time), Style::default().fg(Color::Gray)),
                ];
                ListItem::new(Spans::from(spans))
            })
            .collect();

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("System Alerts"))
            .highlight_style(Style::default().bg(Color::DarkGray))
            .highlight_symbol("▶ ");

        f.render_widget(list, area);
        Ok(())
    }

    fn render_status_bar<B: Backend>(&self, f: &mut Frame<B>, area: Rect) {
        let status_text = format!(
            "Analytics Dashboard • Tab: {:?} • Time Range: {:?} • Last Refresh: {} • Press 'r' to refresh",
            self.current_tab,
            self.time_range,
            self.last_refresh.format("%H:%M:%S")
        );

        let status = Paragraph::new(status_text)
            .block(Block::default().borders(Borders::ALL))
            .style(Style::default().fg(Color::Gray));

        f.render_widget(status, area);
    }

    pub async fn handle_input(&mut self, key: crossterm::event::KeyCode) -> Result<(), WarpError> {
        match key {
            crossterm::event::KeyCode::Tab => {
                self.switch_tab();
            }
            crossterm::event::KeyCode::Char('r') | crossterm::event::KeyCode::F5 => {
                self.refresh_data().await?;
            }
            crossterm::event::KeyCode::Char('1') => {
                self.time_range = TimeRange::LastHour;
            }
            crossterm::event::KeyCode::Char('2') => {
                self.time_range = TimeRange::LastDay;
            }
            crossterm::event::KeyCode::Char('3') => {
                self.time_range = TimeRange::LastWeek;
            }
            crossterm::event::KeyCode::Char('4') => {
                self.time_range = TimeRange::LastMonth;
            }
            _ => {}
        }

        Ok(())
    }

    fn switch_tab(&mut self) {
        self.current_tab = match self.current_tab {
            DashboardTab::Overview => DashboardTab::Performance,
            DashboardTab::Performance => DashboardTab::Usage,
            DashboardTab::Usage => DashboardTab::UserBehavior,
            DashboardTab::UserBehavior => DashboardTab::Marketplace,
            DashboardTab::Marketplace => DashboardTab::RealTime,
            DashboardTab::RealTime => DashboardTab::Alerts,
            DashboardTab::Alerts => DashboardTab::Overview,
        };
    }

    async fn refresh_data(&mut self) -> Result<(), WarpError> {
        self.last_refresh = Utc::now();
        // In a real implementation, this would trigger data refresh
        Ok(())
    }

    pub fn set_selected_item(&mut self, item_id: Option<String>) {
        self.selected_item = item_id;
    }

    pub fn set_time_range(&mut self, time_range: TimeRange) {
        self.time_range = time_range;
    }
}
