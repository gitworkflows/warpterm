use super::*;
use crate::error::WarpError;
use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph, Tabs},
    Frame,
};
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct MarketplaceUI {
    marketplace: Arc<Marketplace>,
    state: MarketplaceUIState,
    search_results: Vec<MarketplaceItem>,
    selected_item: Option<MarketplaceItem>,
    current_tab: MarketplaceTab,
    list_state: ListState,
    search_query: String,
}

#[derive(Debug, Clone)]
pub enum MarketplaceUIState {
    Browse,
    Search,
    ItemDetails,
    Installing,
    Reviews,
    MyItems,
}

#[derive(Debug, Clone)]
pub enum MarketplaceTab {
    Featured,
    Themes,
    Plugins,
    AIModels,
    Installed,
    Updates,
}

impl MarketplaceUI {
    pub async fn new(marketplace: Arc<Marketplace>) -> Result<Self, WarpError> {
        let mut ui = Self {
            marketplace,
            state: MarketplaceUIState::Browse,
            search_results: Vec::new(),
            selected_item: None,
            current_tab: MarketplaceTab::Featured,
            list_state: ListState::default(),
            search_query: String::new(),
        };
        
        // Load initial content
        ui.load_featured_items().await?;
        
        Ok(ui)
    }

    pub async fn render<B: Backend>(&mut self, f: &mut Frame<B>) -> Result<(), WarpError> {
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

        // Render main content based on state
        match self.state {
            MarketplaceUIState::Browse => self.render_browse(f, chunks[1]).await?,
            MarketplaceUIState::Search => self.render_search(f, chunks[1]).await?,
            MarketplaceUIState::ItemDetails => self.render_item_details(f, chunks[1]).await?,
            MarketplaceUIState::Installing => self.render_installing(f, chunks[1]).await?,
            MarketplaceUIState::Reviews => self.render_reviews(f, chunks[1]).await?,
            MarketplaceUIState::MyItems => self.render_my_items(f, chunks[1]).await?,
        }

        // Render status bar
        self.render_status_bar(f, chunks[2]);

        Ok(())
    }

    fn render_tabs<B: Backend>(&self, f: &mut Frame<B>, area: Rect) {
        let titles = vec![
            "Featured",
            "Themes", 
            "Plugins",
            "AI Models",
            "Installed",
            "Updates",
        ];
        
        let selected_tab = match self.current_tab {
            MarketplaceTab::Featured => 0,
            MarketplaceTab::Themes => 1,
            MarketplaceTab::Plugins => 2,
            MarketplaceTab::AIModels => 3,
            MarketplaceTab::Installed => 4,
            MarketplaceTab::Updates => 5,
        };

        let tabs = Tabs::new(titles)
            .block(Block::default().borders(Borders::ALL).title("Warp Marketplace"))
            .select(selected_tab)
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));

        f.render_widget(tabs, area);
    }

    async fn render_browse<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect) -> Result<(), WarpError> {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        // Left panel: Item list
        self.render_item_list(f, chunks[0]);

        // Right panel: Item preview
        if let Some(item) = &self.selected_item {
            self.render_item_preview(f, chunks[1], item);
        } else {
            let placeholder = Paragraph::new("Select an item to view details")
                .block(Block::default().borders(Borders::ALL).title("Preview"))
                .style(Style::default().fg(Color::Gray));
            f.render_widget(placeholder, chunks[1]);
        }

        Ok(())
    }

    fn render_item_list<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect) {
        let items: Vec<ListItem> = self.search_results
            .iter()
            .map(|item| {
                let price_text = match &item.price {
                    Price::Free => "Free".to_string(),
                    Price::Paid { amount, currency } => format!("${}.{:02}", amount / 100, amount % 100),
                    Price::PayWhatYouWant { suggested, currency } => format!("PWYW (${}.{:02})", suggested / 100, suggested % 100),
                    Price::Subscription { monthly, currency, .. } => format!("${}.{:02}/mo", monthly / 100, monthly % 100),
                };

                let spans = vec![
                    Span::styled(&item.name, Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
                    Span::raw(" - "),
                    Span::styled(&price_text, Style::default().fg(Color::Green)),
                    Span::raw(" "),
                    Span::styled(format!("⭐ {:.1}", item.rating.average), Style::default().fg(Color::Yellow)),
                ];

                ListItem::new(Spans::from(spans))
            })
            .collect();

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Items"))
            .highlight_style(Style::default().bg(Color::DarkGray))
            .highlight_symbol("▶ ");

        f.render_stateful_widget(list, area, &mut self.list_state);
    }

    fn render_item_preview<B: Backend>(&self, f: &mut Frame<B>, area: Rect, item: &MarketplaceItem) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(6),  // Header
                Constraint::Min(0),     // Description
                Constraint::Length(4),  // Actions
            ])
            .split(area);

        // Header
        let header_text = vec![
            Spans::from(vec![
                Span::styled(&item.name, Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
                Span::raw(" v"),
                Span::styled(&item.version, Style::default().fg(Color::Cyan)),
            ]),
            Spans::from(vec![
                Span::raw("by "),
                Span::styled(&item.author.display_name, Style::default().fg(Color::Green)),
                if item.author.verified { Span::styled(" ✓", Style::default().fg(Color::Blue)) } else { Span::raw("") },
            ]),
            Spans::from(vec![
                Span::styled(format!("⭐ {:.1}", item.rating.average), Style::default().fg(Color::Yellow)),
                Span::raw(" "),
                Span::styled(format!("({} reviews)", item.rating.count), Style::default().fg(Color::Gray)),
                Span::raw(" • "),
                Span::styled(format!("{} downloads", item.downloads), Style::default().fg(Color::Gray)),
            ]),
        ];

        let header = Paragraph::new(header_text)
            .block(Block::default().borders(Borders::ALL).title("Details"));
        f.render_widget(header, chunks[0]);

        // Description
        let description = Paragraph::new(item.description.as_str())
            .block(Block::default().borders(Borders::ALL).title("Description"))
            .wrap(ratatui::widgets::Wrap { trim: true });
        f.render_widget(description, chunks[1]);

        // Actions
        let actions_text = vec![
            Spans::from(vec![
                Span::styled("[I]", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
                Span::raw(" Install  "),
                Span::styled("[R]", Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD)),
                Span::raw(" Reviews  "),
                Span::styled("[Enter]", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                Span::raw(" Details"),
            ]),
        ];

        let actions = Paragraph::new(actions_text)
            .block(Block::default().borders(Borders::ALL).title("Actions"));
        f.render_widget(actions, chunks[2]);
    }

    async fn render_search<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect) -> Result<(), WarpError> {
        // Similar to browse but with search input
        self.render_browse(f, area).await
    }

    async fn render_item_details<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect) -> Result<(), WarpError> {
        if let Some(item) = &self.selected_item {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(8),  // Header
                    Constraint::Min(0),     // README/Details
                    Constraint::Length(4),  // Actions
                ])
                .split(area);

            // Detailed header
            let header_text = vec![
                Spans::from(vec![
                    Span::styled(&item.name, Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
                    Span::raw(" v"),
                    Span::styled(&item.version, Style::default().fg(Color::Cyan)),
                ]),
                Spans::from(vec![
                    Span::raw("Category: "),
                    Span::styled(format!("{:?}", item.category), Style::default().fg(Color::Magenta)),
                ]),
                Spans::from(vec![
                    Span::raw("License: "),
                    Span::styled(&item.license.name, Style::default().fg(Color::Green)),
                ]),
                Spans::from(vec![
                    Span::raw("Tags: "),
                    Span::styled(item.tags.join(", "), Style::default().fg(Color::Gray)),
                ]),
            ];

            let header = Paragraph::new(header_text)
                .block(Block::default().borders(Borders::ALL).title("Item Details"));
            f.render_widget(header, chunks[0]);

            // README
            let readme = Paragraph::new(item.readme.as_str())
                .block(Block::default().borders(Borders::ALL).title("README"))
                .wrap(ratatui::widgets::Wrap { trim: true });
            f.render_widget(readme, chunks[1]);

            // Actions
            let actions_text = vec![
                Spans::from(vec![
                    Span::styled("[I]", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
                    Span::raw(" Install  "),
                    Span::styled("[R]", Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD)),
                    Span::raw(" Reviews  "),
                    Span::styled("[Esc]", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
                    Span::raw(" Back"),
                ]),
            ];

            let actions = Paragraph::new(actions_text)
                .block(Block::default().borders(Borders::ALL).title("Actions"));
            f.render_widget(actions, chunks[2]);
        }

        Ok(())
    }

    async fn render_installing<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect) -> Result<(), WarpError> {
        let installing_text = vec![
            Spans::from(vec![Span::styled("Installing...", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))]),
            Spans::from(vec![Span::raw("Please wait while the item is being installed.")]),
        ];

        let installing = Paragraph::new(installing_text)
            .block(Block::default().borders(Borders::ALL).title("Installation"))
            .alignment(ratatui::layout::Alignment::Center);

        // Center the widget
        let popup_area = centered_rect(50, 20, area);
        f.render_widget(Clear, popup_area);
        f.render_widget(installing, popup_area);

        Ok(())
    }

    async fn render_reviews<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect) -> Result<(), WarpError> {
        // This would show reviews for the selected item
        let placeholder = Paragraph::new("Reviews will be displayed here")
            .block(Block::default().borders(Borders::ALL).title("Reviews"));
        f.render_widget(placeholder, area);

        Ok(())
    }

    async fn render_my_items<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect) -> Result<(), WarpError> {
        // This would show installed items
        let installed_items = self.marketplace.get_installed_items().await?;
        
        let items: Vec<ListItem> = installed_items
            .iter()
            .map(|item| {
                let spans = vec![
                    Span::styled(&item.name, Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
                    Span::raw(" v"),
                    Span::styled(&item.version, Style::default().fg(Color::Cyan)),
                    Span::raw(" - Installed"),
                ];
                ListItem::new(Spans::from(spans))
            })
            .collect();

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("My Items"))
            .highlight_style(Style::default().bg(Color::DarkGray))
            .highlight_symbol("▶ ");

        f.render_stateful_widget(list, area, &mut self.list_state);

        Ok(())
    }

    fn render_status_bar<B: Backend>(&self, f: &mut Frame<B>, area: Rect) {
        let status_text = match self.state {
            MarketplaceUIState::Browse => "Browse marketplace items • Use ↑↓ to navigate, Enter for details",
            MarketplaceUIState::Search => "Search results • Type to search, Enter to select",
            MarketplaceUIState::ItemDetails => "Item details • I to install, R for reviews, Esc to go back",
            MarketplaceUIState::Installing => "Installing item...",
            MarketplaceUIState::Reviews => "Item reviews • Esc to go back",
            MarketplaceUIState::MyItems => "Your installed items • Enter for details",
        };

        let status = Paragraph::new(status_text)
            .block(Block::default().borders(Borders::ALL))
            .style(Style::default().fg(Color::Gray));

        f.render_widget(status, area);
    }

    async fn load_featured_items(&mut self) -> Result<(), WarpError> {
        // This would load featured items from the marketplace
        // For now, use discovery engine recommendations
        self.search_results = self.marketplace.get_recommendations().await?;
        
        if !self.search_results.is_empty() {
            self.list_state.select(Some(0));
            self.selected_item = Some(self.search_results[0].clone());
        }
        
        Ok(())
    }

    pub async fn handle_input(&mut self, key: crossterm::event::KeyCode) -> Result<(), WarpError> {
        match key {
            crossterm::event::KeyCode::Up => {
                if let Some(selected) = self.list_state.selected() {
                    if selected > 0 {
                        self.list_state.select(Some(selected - 1));
                        self.selected_item = Some(self.search_results[selected - 1].clone());
                    }
                }
            }
            crossterm::event::KeyCode::Down => {
                if let Some(selected) = self.list_state.selected() {
                    if selected < self.search_results.len() - 1 {
                        self.list_state.select(Some(selected + 1));
                        self.selected_item = Some(self.search_results[selected + 1].clone());
                    }
                }
            }
            crossterm::event::KeyCode::Enter => {
                self.state = MarketplaceUIState::ItemDetails;
            }
            crossterm::event::KeyCode::Char('i') | crossterm::event::KeyCode::Char('I') => {
                if let Some(item) = &self.selected_item {
                    self.install_selected_item().await?;
                }
            }
            crossterm::event::KeyCode::Char('r') | crossterm::event::KeyCode::Char('R') => {
                self.state = MarketplaceUIState::Reviews;
            }
            crossterm::event::KeyCode::Esc => {
                self.state = MarketplaceUIState::Browse;
            }
            crossterm::event::KeyCode::Tab => {
                self.switch_tab();
            }
            _ => {}
        }

        Ok(())
    }

    async fn install_selected_item(&mut self) -> Result<(), WarpError> {
        if let Some(item) = &self.selected_item {
            self.state = MarketplaceUIState::Installing;
            
            // Install the item
            match self.marketplace.install_item(&item.id).await {
                Ok(_) => {
                    // Installation successful
                    self.state = MarketplaceUIState::Browse;
                }
                Err(e) => {
                    // Handle installation error
                    log::error!("Installation failed: {}", e);
                    self.state = MarketplaceUIState::Browse;
                }
            }
        }

        Ok(())
    }

    fn switch_tab(&mut self) {
        self.current_tab = match self.current_tab {
            MarketplaceTab::Featured => MarketplaceTab::Themes,
            MarketplaceTab::Themes => MarketplaceTab::Plugins,
            MarketplaceTab::Plugins => MarketplaceTab::AIModels,
            MarketplaceTab::AIModels => MarketplaceTab::Installed,
            MarketplaceTab::Installed => MarketplaceTab::Updates,
            MarketplaceTab::Updates => MarketplaceTab::Featured,
        };
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
