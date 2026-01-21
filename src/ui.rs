use crate::app::{App, AppTab, ProcessSortMode};
use crate::config::parse_color;
use crate::draw::logos::get_logo;
use crate::ui::widgets::CyberpunkBlock;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Gauge, Paragraph, Row, Table, Tabs, Wrap},
};

pub mod widgets;

pub fn render(app: &App, frame: &mut Frame) {
    let area = frame.area();

    // --- TAB BAR RENDER ---
    // Use 3 lines for tabs, rest for content
    // Use 3 lines for tabs, 1 line for footer (if enabled), rest for content
    let constraints = if app.show_hints {
        vec![
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(1),
        ]
    } else {
        vec![Constraint::Length(3), Constraint::Min(0)]
    };

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(area);

    render_tabs(app, frame, chunks[0]);

    // --- CONTENT RENDER ---
    let content_area = chunks[1];
    match app.current_tab {
        AppTab::Dashboard => render_dashboard(app, frame, content_area),
        AppTab::Processes => render_processes(app, frame, content_area),
        AppTab::Network => render_network(app, frame, content_area),
        AppTab::Settings => render_settings(app, frame, content_area),
    }

    if app.show_hints && chunks.len() > 2 {
        render_footer(app, frame, chunks[2]);
    }

    if app.show_help {
        render_help_popup(app, frame, area);
    }
}

// --- HELP POPUP ---
fn render_help_popup(app: &App, frame: &mut Frame, area: Rect) {
    let popup_area = centered_rect(60, 50, area);

    let help_text = vec![
        "Keyboard Shortcuts",
        "------------------",
        "Tab / Shift+Tab : Navigate Tabs",
        "1, 2, 3, 4      : Jump to Tab",
        "j / Down        : Scroll Down / Next Option",
        "k / Up          : Scroll Up / Prev Option",
        "Enter           : Toggle Setting",
        "?               : Toggle Help",
        "q               : Quit",
        "",
        "Mouse Support",
        "-------------",
        "Scroll          : Scroll Lists",
        // "Click           : Select Tab (Coming Soon)",
    ];

    let help_paragraph = Paragraph::new(help_text.join("\n"))
        .block(
            Block::default()
                .title(" Help ")
                .borders(Borders::ALL)
                .style(
                    Style::default()
                        .fg(parse_color(&app.config.theme.border_color))
                        .bg(Color::Black),
                ),
        )
        .style(Style::default().fg(parse_color(&app.config.theme.text_color)))
        .alignment(Alignment::Center);

    // Clear the background of the popup area to avoid transparency issues
    frame.render_widget(ratatui::widgets::Clear, popup_area);
    frame.render_widget(help_paragraph, popup_area);
}

// Helper to center the popup
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

// --- PROCESS MONITOR RENDER ---
fn render_processes(app: &App, frame: &mut Frame, area: Rect) {
    let cpu_header = if matches!(app.process_sort, ProcessSortMode::Cpu) {
        "CPU % [*]"
    } else {
        "CPU %"
    };
    let mem_header = if matches!(app.process_sort, ProcessSortMode::Memory) {
        "Mem (MB) [*]"
    } else {
        "Mem (MB)"
    };
    let pid_header = if matches!(app.process_sort, ProcessSortMode::Pid) {
        "PID [*]"
    } else {
        "PID"
    };

    let header = Row::new(vec![pid_header, "Name", cpu_header, mem_header])
        .style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
        .bottom_margin(1);

    let rows: Vec<Row> = app
        .system_info
        .processes
        .iter()
        .skip(app.process_scroll)
        .map(|p| {
            let mem_mb = p.mem as f64 / 1024.0 / 1024.0;
            let cpu_color = if p.cpu > 50.0 {
                Color::Red
            } else if p.cpu > 10.0 {
                Color::Yellow
            } else {
                Color::Green
            };

            Row::new(vec![
                p.pid.to_string(),
                p.name.clone(),
                format!("{:.1}", p.cpu),
                format!("{:.1}", mem_mb),
            ])
            .style(Style::default().fg(cpu_color))
        })
        .collect();

    let widths = [
        Constraint::Length(8),
        Constraint::Percentage(50),
        Constraint::Length(10),
        Constraint::Length(15),
    ];

    let table = Table::new(rows, widths).header(header).column_spacing(1);

    frame.render_widget(
        CyberpunkBlock::new(
            " Process Monitor ",
            parse_color(&app.config.theme.border_color),
        ),
        area,
    );

    let inner_area = Rect::new(area.x + 1, area.y + 1, area.width - 2, area.height - 2);
    frame.render_widget(table, inner_area);
}

// --- DASHBOARD RENDERING (Moved from old render function) ---
fn render_dashboard(app: &App, frame: &mut Frame, area: Rect) {
    // Ngưỡng xác định chế độ nhỏ gọn (Compact Mode)
    // Nếu chiều rộng < 60 ký tự hoặc chiều cao < 25 dòng
    let is_width_compact = area.width < 65;
    let is_height_compact = area.height < 25;
    let is_compact = is_width_compact || is_height_compact;

    // --- MAIN LAYOUT ---
    let main_constraints = if is_compact {
        // Chế độ gọn: Ưu tiên nội dung, giảm Hinge, Bottom cố định nhỏ
        vec![
            Constraint::Min(10),   // Top (Logo + Info) co giãn
            Constraint::Length(1), // Hinge nhỏ 1 dòng
            Constraint::Length(if is_height_compact { 6 } else { 10 }), // Bottom
        ]
    } else {
        // Chế độ thường: Dual Screen cân đối
        vec![
            Constraint::Percentage(55), // Top
            Constraint::Length(3),      // Hinge 3 dòng
            Constraint::Min(10),        // Bottom
        ]
    };

    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(main_constraints)
        .split(area);

    let top_area = main_chunks[0];
    let hinge_area = main_chunks[1];
    let bottom_area = main_chunks[2];

    // --- TOP SCREEN RENDER ---
    render_top_screen(app, frame, top_area, is_width_compact);

    // --- HINGE RENDER ---
    render_hinge(frame, hinge_area, is_compact);

    // --- BOTTOM SCREEN RENDER ---
    render_bottom_screen(app, frame, bottom_area, is_height_compact);
}

fn render_top_screen(app: &App, frame: &mut Frame, area: Rect, is_width_compact: bool) {
    if is_width_compact {
        // Layout Dọc hoặc Chỉ Info (nếu quá hẹp)
        // Kiểm tra xem có đủ chỗ hiện cả Logo không (cần khoảng 25-30 dòng cho cả hai nếu xếp dọc)
        if area.height > 20 {
            // Đủ cao: Xếp Logo trên, Info dưới
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(8), Constraint::Min(8)])
                .split(area);
            render_logo(app, frame, chunks[0]);
            render_info(app, frame, chunks[1]);
        } else {
            // Không đủ cao: Chỉ hiện Info (Quan trọng hơn)
            render_info(app, frame, area);
        }
    } else {
        // Layout Ngang (truyền thống)
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(40), // Logo
                Constraint::Percentage(60), // Info
            ])
            .split(area);
        render_logo(app, frame, chunks[0]);
        render_info(app, frame, chunks[1]);
    }
}

fn render_logo(app: &App, frame: &mut Frame, area: Rect) {
    let logo_lines = get_logo(&app.system_info.os.name);
    let logo_text = logo_lines.join("\n");

    // Tự động ẩn border nếu area quá nhỏ
    if area.width > 20 && area.height > 5 {
        let block = CyberpunkBlock::new(" System ", parse_color(&app.config.theme.border_color));
        // We need to render the block separately because Paragraph doesn't accept a Widget as block,
        // it accepts a Block struct.
        // Since CyberpunkBlock is a Widget that wraps a Block, we'll render it as a background/border
        // then render the paragraph inside.
        frame.render_widget(block, area);

        let inner_area = Rect::new(area.x + 1, area.y + 1, area.width - 2, area.height - 2);

        let logo_paragraph = Paragraph::new(logo_text)
            .style(Style::default().fg(parse_color(&app.config.theme.key_color)))
            .alignment(Alignment::Center);
        frame.render_widget(logo_paragraph, inner_area);
    } else {
        let logo_paragraph = Paragraph::new(logo_text)
            .style(Style::default().fg(parse_color(&app.config.theme.key_color)))
            .alignment(Alignment::Center);
        frame.render_widget(logo_paragraph, area);
    };
}

fn render_info(app: &App, frame: &mut Frame, area: Rect) {
    // fastfetch style icons and labels
    // We want aligned output.
    //
    // Format:
    // Icon Label: Value
    //
    // We need to handle padding manually or use a Table if strict alignment is needed,
    // but Paragraph is simpler for colorized lines.

    // We can use a simpler approach: padded keys.
    // Key width approx 15-20 chars including icon.

    let mut info_lines = vec![format!("                  -------------------")];

    for module in &app.config.modules {
        match module.as_str() {
            "os" => info_lines.push(format!(" OS:             {}", app.system_info.os.name)),
            "host" => info_lines.push(format!(" Host:           {}", app.system_info.os.hostname)),
            "kernel" => info_lines.push(format!(" Kernel:         {}", app.system_info.os.kernel)),
            "uptime" => info_lines.push(format!(
                " Uptime:         {}",
                app.system_info.get_formatted_uptime()
            )),
            "packages" => {
                info_lines.push(format!(" Packages:       {}", app.system_info.packages))
            }
            "shell" => info_lines.push(format!(" Shell:          {}", app.system_info.os.shell)),
            "display" => info_lines.push(format!(" Display:        {}", app.system_info.display)),
            "de" => info_lines.push(format!(" DE:             {}", app.system_info.os.de_wm)),
            "wm" => info_lines.push(format!(" WM:             {}", app.system_info.os.wm)),
            "wm_theme" => {
                info_lines.push(format!(" WM Theme:       {}", app.system_info.wm_theme))
            }
            "theme" => info_lines.push(format!(" kr Theme:         {}", app.system_info.theme)),
            "icons" => info_lines.push(format!(" Icons:          {}", app.system_info.icons)),
            "font" => info_lines.push(format!(" Font:           {}", app.system_info.font)),
            "cursor" => info_lines.push(format!(" Cursor:         {}", app.system_info.cursor)),
            "terminal" => {
                info_lines.push(format!(" Terminal:       {}", app.system_info.os.terminal))
            }
            "cpu" => info_lines.push(format!(
                " CPU:            {}",
                app.system_info
                    .cpu_info
                    .models
                    .first()
                    .unwrap_or(&"Unknown".to_string())
            )),
            "gpu" => info_lines.push(format!(
                "﬙ GPU:            {}",
                app.system_info.gpus.join(", ")
            )),
            "memory" => info_lines.push(format!(
                " Memory:         {:.2} GiB / {:.2} GiB ({:.0}%)",
                app.system_info.memory_used as f64 / 1024.0 / 1024.0 / 1024.0,
                app.system_info.memory_total as f64 / 1024.0 / 1024.0 / 1024.0,
                (app.system_info.memory_used as f64 / app.system_info.memory_total as f64) * 100.0
            )),
            "disk" => info_lines.push(format!(" Disk:           {}", app.system_info.disk_usage)),
            "battery" => info_lines.push(format!(" Battery:        {}", app.system_info.battery)),
            "locale" => info_lines.push(format!(" Locale:         {}", app.system_info.os.locale)),
            "local_ip" => {
                info_lines.push(format!(" IP:             {}", app.system_info.local_ip))
            }
            _ => {} // Ignore unknown modules
        }
    }

    let info_text = info_lines.join("\n");

    if area.width > 30 && area.height > 5 {
        let block =
            CyberpunkBlock::new(" Core Specs ", parse_color(&app.config.theme.border_color));
        frame.render_widget(block, area);

        let inner_area = Rect::new(area.x + 1, area.y + 1, area.width - 2, area.height - 2);

        let info_paragraph = Paragraph::new(info_text)
            .style(Style::default().fg(parse_color(&app.config.theme.text_color)))
            // .wrap(Wrap { trim: true }) // Don't trim or wrap to keep alignment if possible
            .scroll((0, 0)); // Ensure start at top

        frame.render_widget(info_paragraph, inner_area);
    } else {
        let info_paragraph = Paragraph::new(info_text)
            .style(Style::default().fg(parse_color(&app.config.theme.text_color)));
        frame.render_widget(info_paragraph, area);
    };
}

fn render_hinge(frame: &mut Frame, area: Rect, is_compact: bool) {
    if area.height == 0 {
        return;
    }

    let text = if is_compact {
        " ─ ─ ─ "
    } else {
        " ○  ○  ○ "
    };
    let paragraph = Paragraph::new(text)
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::DarkGray));
    frame.render_widget(paragraph, area);
}

fn render_bottom_screen(app: &App, frame: &mut Frame, area: Rect, is_compact: bool) {
    // Nếu Compact: Bỏ viền ngoài Hardware Stats, chỉ hiện nội dung
    let inner_area = if !is_compact && area.width > 30 && area.height > 8 {
        let block = CyberpunkBlock::new(
            " Hardware Stats ",
            parse_color(&app.config.theme.border_color),
        );
        frame.render_widget(block, area);
        // CyberpunkBlock doesn't implement inner(), so we manually calculate inner area
        Rect::new(area.x + 1, area.y + 1, area.width - 2, area.height - 2)
    } else {
        area
    };

    let constraints = if is_compact {
        vec![
            Constraint::Length(1), // CPU (1 dòng)
            Constraint::Length(1), // RAM (1 dòng)
            Constraint::Length(1), // Swap (1 dòng)
            Constraint::Min(1),    // Disk + GPU
        ]
    } else {
        vec![
            Constraint::Length(3), // CPU (Gauge to)
            Constraint::Length(3), // RAM (Gauge to)
            Constraint::Length(3), // Swap (Gauge to)
            Constraint::Min(1),    // Disk + GPU
        ]
    };

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .margin(if is_compact { 0 } else { 1 }) // Bỏ margin trong chế độ compact
        .split(inner_area);

    // --- CPU GAUGE ---
    let cpu_label = format!(
        "CPU: {:.1}% ({})",
        app.system_info.cpu_usage,
        app.system_info
            .cpu_info
            .models
            .first()
            .unwrap_or(&"Unknown".to_string())
    );
    let cpu_ratio = app.system_info.cpu_usage as f64 / 100.0;
    let cpu_color = if app.system_info.cpu_usage > 80.0 {
        parse_color(&app.config.theme.gauge_cpu_high)
    } else {
        parse_color(&app.config.theme.gauge_cpu_low)
    };

    let cpu_gauge = if is_compact {
        // Line Gauge cho Compact mode
        Gauge::default()
            .gauge_style(Style::default().fg(cpu_color))
            .ratio(cpu_ratio)
            .label(cpu_label)
            .use_unicode(true)
    } else {
        // Block Gauge đầy đủ cho Normal mode
        Gauge::default()
            .block(Block::default().title("CPU Load"))
            .gauge_style(Style::default().fg(cpu_color))
            .ratio(cpu_ratio)
            .label(cpu_label)
    };
    frame.render_widget(cpu_gauge, chunks[0]);

    // --- RAM GAUGE ---
    let ram_ratio = app.system_info.memory_used as f64 / app.system_info.memory_total as f64;
    let ram_label = format!(
        "RAM: {}/{} MB",
        app.system_info.memory_used / 1024 / 1024,
        app.system_info.memory_total / 1024 / 1024
    );

    let ram_gauge = if is_compact {
        Gauge::default()
            .gauge_style(Style::default().fg(parse_color(&app.config.theme.gauge_ram)))
            .ratio(ram_ratio)
            .label(ram_label)
            .use_unicode(true)
    } else {
        Gauge::default()
            .block(Block::default().title("Memory Usage"))
            .gauge_style(Style::default().fg(parse_color(&app.config.theme.gauge_ram)))
            .ratio(ram_ratio)
            .label(ram_label)
    };
    frame.render_widget(ram_gauge, chunks[1]);

    // --- SWAP GAUGE ---
    let swap_percent = if app.system_info.swap_total > 0 {
        app.system_info.swap_used as f64 / app.system_info.swap_total as f64
    } else {
        0.0
    };
    let swap_label = format!(
        "Swap: {}/{} MB",
        app.system_info.swap_used / 1024 / 1024,
        app.system_info.swap_total / 1024 / 1024
    );
    let swap_gauge = if is_compact {
        Gauge::default()
            .gauge_style(Style::default().fg(parse_color(&app.config.theme.gauge_cpu_high)))
            .ratio(swap_percent)
            .label(swap_label)
            .use_unicode(true)
    } else {
        Gauge::default()
            .block(Block::default().title("Swap Usage"))
            .gauge_style(Style::default().fg(parse_color(&app.config.theme.gauge_cpu_high)))
            .ratio(swap_percent)
            .label(swap_label)
    };
    frame.render_widget(swap_gauge, chunks[2]);

    // --- DISK & GPU INFO ---
    let gpu_text = app.system_info.gpus.join(", ");
    let other_info = format!(
        " Disk (/): {}\n GPU(s):   {}",
        app.system_info.disk_usage, gpu_text
    );
    let other_paragraph = Paragraph::new(other_info)
        .style(Style::default().fg(parse_color(&app.config.theme.value_color)))
        .wrap(Wrap { trim: true });

    if chunks.len() > 3 {
        frame.render_widget(other_paragraph, chunks[3]);
    }
}

// --- NETWORK MONITOR RENDER ---
fn render_network(app: &App, frame: &mut Frame, area: Rect) {
    let block = CyberpunkBlock::new(
        " Network Monitor ",
        parse_color(&app.config.theme.border_color),
    );
    frame.render_widget(block, area);

    let inner_area = Rect::new(area.x + 1, area.y + 1, area.width - 2, area.height - 2);

    // Split inner area for interface list and details
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(5), Constraint::Length(3)])
        .split(inner_area);

    // Interface List
    let header = Row::new(vec![
        "Interface",
        "IP Address",
        "RX Speed",
        "TX Speed",
        "Total RX",
        "Total TX",
    ])
    .style(
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD),
    )
    .bottom_margin(1);

    let rows: Vec<Row> = app
        .system_info
        .networks
        .iter()
        .map(|network_info| {
            // Calculate speed in KB/s
            // refresh_rate_ms is the interval, e.g., 250ms
            // rx is bytes received in that interval
            // Speed = (bytes / (interval_ms / 1000.0)) / 1024.0
            let seconds = app.refresh_rate_ms as f64 / 1000.0;
            let rx_speed = (network_info.rx as f64 / seconds) / 1024.0;
            let tx_speed = (network_info.tx as f64 / seconds) / 1024.0;

            Row::new(vec![
                network_info.name.clone(),
                network_info.ip_v4.clone(),
                format!("{:.1} KB/s", rx_speed),
                format!("{:.1} KB/s", tx_speed),
                format!("{:.1} MB", network_info.total_rx as f64 / 1024.0 / 1024.0),
                format!("{:.1} MB", network_info.total_tx as f64 / 1024.0 / 1024.0),
            ])
            .style(Style::default().fg(parse_color(&app.config.theme.text_color)))
        })
        .collect();

    let table = Table::new(
        rows,
        [
            Constraint::Length(15), // Interface
            Constraint::Length(16), // IP Address
            Constraint::Length(12), // RX Speed
            Constraint::Length(12), // TX Speed
            Constraint::Length(12), // Total RX
            Constraint::Length(12), // Total TX
        ],
    )
    .header(header)
    .column_spacing(2);

    frame.render_widget(table, chunks[0]);

    // Summary / Status
    let status_text = format!("Total Interfaces: {}", app.system_info.networks.len());
    let p = Paragraph::new(status_text)
        .style(Style::default().fg(parse_color(&app.config.theme.value_color)))
        .alignment(Alignment::Center);
    frame.render_widget(p, chunks[1]);
}

// --- SETTINGS RENDER ---
fn render_settings(app: &App, frame: &mut Frame, area: Rect) {
    let block = CyberpunkBlock::new(" Settings ", parse_color(&app.config.theme.border_color));
    frame.render_widget(block, area);

    let inner_area = Rect::new(area.x + 1, area.y + 1, area.width - 2, area.height - 2);

    let options = [
        ("Refresh Rate", format!("{} ms", app.refresh_rate_ms)),
        (
            "Theme Color",
            if app.config.theme.border_color == "#00ffff" {
                "Cyan".to_string()
            } else {
                "Magenta".to_string()
            },
        ),
        (
            "Show Hints",
            if app.show_hints {
                "Yes".to_string()
            } else {
                "No".to_string()
            },
        ),
    ];

    let rows: Vec<Row> = options
        .iter()
        .enumerate()
        .map(|(i, (label, value))| {
            let style = if i == app.settings_index {
                Style::default().fg(Color::Black).bg(Color::Cyan)
            } else {
                Style::default().fg(Color::White)
            };

            Row::new(vec![label.to_string(), value.to_string()]).style(style)
        })
        .collect();

    let table = Table::new(
        rows,
        [Constraint::Percentage(50), Constraint::Percentage(50)],
    )
    .column_spacing(1);

    frame.render_widget(table, inner_area);
}

// --- TAB BAR RENDER ---
fn render_tabs(app: &App, frame: &mut Frame, area: Rect) {
    let tabs = vec!["Dashboard", "Processes", "Network", "Settings"];
    let selected_index = match app.current_tab {
        AppTab::Dashboard => 0,
        AppTab::Processes => 1,
        AppTab::Network => 2,
        AppTab::Settings => 3,
    };

    let tabs_widget = Tabs::new(tabs)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Tabs ")
                .border_style(Style::default().fg(parse_color(&app.config.theme.border_color))),
        )
        .highlight_style(
            Style::default()
                .fg(Color::Black)
                .bg(parse_color(&app.config.theme.key_color)),
        )
        .select(selected_index)
        .divider(" | ");

    frame.render_widget(tabs_widget, area);
}

fn render_footer(app: &App, frame: &mut Frame, area: Rect) {
    let hints = match app.current_tab {
        AppTab::Dashboard => "q: Quit | ?: Help | 1-4: Tabs",
        AppTab::Processes => "j/k: Scroll | s: Sort | ?: Help",
        AppTab::Network => "?: Help",
        AppTab::Settings => "Enter: Toggle | j/k: Nav | ?: Help",
    };

    let p = Paragraph::new(hints)
        .style(Style::default().fg(Color::DarkGray))
        .alignment(Alignment::Center);
    frame.render_widget(p, area);
}
