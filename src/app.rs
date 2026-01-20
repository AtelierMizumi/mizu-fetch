use crate::config::Config;
use crate::fetch::SystemInfo;

pub enum AppTab {
    Dashboard,
    Processes,
    Network,
    Settings,
}

pub enum SettingsOption {
    RefreshRate,
    ThemeColor,
}

pub struct App {
    pub should_quit: bool,
    pub current_tab: AppTab,
    pub system_info: SystemInfo,
    pub config: Config,
    pub process_scroll: usize, // New: state for process list scrolling
    pub show_help: bool,       // New: toggle for help popup

    // Settings state
    pub settings_index: usize,
    pub refresh_rate_ms: u64,
}

impl App {
    pub fn new() -> Self {
        let config = Config::load();
        Self {
            should_quit: false,
            current_tab: AppTab::Dashboard,
            system_info: SystemInfo::new(),
            process_scroll: 0,
            show_help: false,
            settings_index: 0,
            refresh_rate_ms: config.refresh_rate,
            config,
        }
    }

    pub fn on_tick(&mut self) {
        self.system_info.refresh();
    }

    pub fn next_tab(&mut self) {
        self.current_tab = match self.current_tab {
            AppTab::Dashboard => AppTab::Processes,
            AppTab::Processes => AppTab::Network,
            AppTab::Network => AppTab::Settings,
            AppTab::Settings => AppTab::Dashboard,
        };
    }

    pub fn previous_tab(&mut self) {
        self.current_tab = match self.current_tab {
            AppTab::Dashboard => AppTab::Settings,
            AppTab::Processes => AppTab::Dashboard,
            AppTab::Network => AppTab::Processes,
            AppTab::Settings => AppTab::Network,
        };
    }

    // New: Process navigation
    pub fn scroll_down(&mut self) {
        if self.process_scroll < self.system_info.processes.len().saturating_sub(1) {
            self.process_scroll += 1;
        }
    }

    pub fn scroll_up(&mut self) {
        if self.process_scroll > 0 {
            self.process_scroll -= 1;
        }
    }

    // Settings navigation
    pub fn settings_next(&mut self) {
        // We have 2 settings currently: Refresh Rate (0) and Theme Color (1)
        if self.settings_index < 1 {
            self.settings_index += 1;
        }
    }

    pub fn settings_previous(&mut self) {
        if self.settings_index > 0 {
            self.settings_index -= 1;
        }
    }

    pub fn settings_toggle(&mut self) {
        match self.settings_index {
            0 => {
                // Refresh Rate: Cycle 250 -> 500 -> 1000 -> 2000 -> 250
                self.refresh_rate_ms = match self.refresh_rate_ms {
                    250 => 500,
                    500 => 1000,
                    1000 => 2000,
                    _ => 250,
                };
                self.config.refresh_rate = self.refresh_rate_ms;
                let _ = self.config.save();
            }
            1 => {
                // Theme Color: Toggle between Cyan and Magenta
                if self.config.theme.border_color == "#00ffff" {
                    // Cyan hex
                    self.config.theme.border_color = "#ff00ff".to_string(); // Magenta hex
                    self.config.theme.key_color = "#ff00ff".to_string(); // Magenta hex
                    self.config.theme.title_color = "#ff00ff".to_string(); // Magenta hex
                    self.config.theme.value_color = "#ff00ff".to_string(); // Magenta hex
                    self.config.theme.gauge_cpu_low = "#ff00ff".to_string(); // Magenta hex
                } else {
                    self.config.theme.border_color = "#00ffff".to_string(); // Cyan hex
                    self.config.theme.key_color = "#00ffff".to_string(); // Cyan hex
                    self.config.theme.title_color = "#00ffff".to_string(); // Cyan hex
                    self.config.theme.value_color = "#00ffff".to_string(); // Cyan hex
                    self.config.theme.gauge_cpu_low = "#00ffff".to_string(); // Cyan hex
                }
                let _ = self.config.save();
            }
            _ => {}
        }
    }

    pub fn toggle_help(&mut self) {
        self.show_help = !self.show_help;
    }
}
