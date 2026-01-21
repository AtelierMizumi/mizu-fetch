use clap::Parser;
use crossterm::event::{self, Event, KeyCode, MouseEventKind};
use mizu::app::{App, AppTab};
use mizu::tui::{init, restore};
use mizu::ui::render;
use std::path::PathBuf;
use std::time::{Duration, Instant};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    live: bool,

    #[arg(short, long)]
    theme: Option<String>,

    #[arg(short, long)]
    ascii: Option<PathBuf>,

    #[arg(short, long)]
    image: Option<PathBuf>,
}

fn main() -> std::io::Result<()> {
    let _args = Args::parse();

    // Initialize terminal
    // We use ratatui with crossterm backend for rendering.
    // 'init' sets up the alternate screen, raw mode, and mouse capture.
    let mut terminal = init()?;

    // Create app state
    // 'App::new()' loads configuration and initializes system info handles.
    let mut app = App::new();

    let mut last_tick = Instant::now();

    loop {
        // Draw Interface
        // This closure is called every frame to render the UI based on current app state.
        terminal.draw(|f| render(&app, f))?;

        let timeout = Duration::from_millis(app.refresh_rate_ms)
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        // Handle Input Events
        if event::poll(timeout)? {
            match event::read()? {
                // Keyboard Input
                Event::Key(key) => {
                    // Global keys
                    match key.code {
                        KeyCode::Char('?') => app.toggle_help(),
                        KeyCode::Char('q') => app.should_quit = true,
                        // Close help with Esc
                        KeyCode::Esc => {
                            if app.show_help {
                                app.show_help = false;
                            }
                        }
                        KeyCode::Char('s') => {
                            if !app.show_help && matches!(app.current_tab, AppTab::Processes) {
                                app.process_sort = app.process_sort.next();
                            }
                        }
                        _ => {
                            // Only process other keys if help is NOT shown
                            if !app.show_help {
                                match key.code {
                                    KeyCode::Tab => app.next_tab(),
                                    KeyCode::BackTab => app.previous_tab(),
                                    KeyCode::Char('1') => app.current_tab = AppTab::Dashboard,
                                    KeyCode::Char('2') => app.current_tab = AppTab::Processes,
                                    KeyCode::Char('3') => app.current_tab = AppTab::Network,
                                    KeyCode::Char('4') => app.current_tab = AppTab::Settings,
                                    KeyCode::Down | KeyCode::Char('j') => match app.current_tab {
                                        AppTab::Processes => app.scroll_down(),
                                        AppTab::Settings => app.settings_next(),
                                        _ => {}
                                    },
                                    KeyCode::Up | KeyCode::Char('k') => match app.current_tab {
                                        AppTab::Processes => app.scroll_up(),
                                        AppTab::Settings => app.settings_previous(),
                                        _ => {}
                                    },
                                    KeyCode::Enter => {
                                        if let AppTab::Settings = app.current_tab {
                                            app.settings_toggle();
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                }
                // Mouse Input
                Event::Mouse(mouse_event) => {
                    if !app.show_help {
                        match mouse_event.kind {
                            MouseEventKind::ScrollDown => {
                                if let AppTab::Processes = app.current_tab {
                                    app.scroll_down();
                                }
                            }
                            MouseEventKind::ScrollUp => {
                                if let AppTab::Processes = app.current_tab {
                                    app.scroll_up();
                                }
                            }
                            // TODO: Handle tab clicking if we track area rects
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }

        // Update Data
        if last_tick.elapsed() >= Duration::from_millis(app.refresh_rate_ms) {
            app.on_tick();
            last_tick = Instant::now();
        }

        // Check Exit Condition
        if app.should_quit {
            break;
        }
    }

    // Restore Terminal
    // Cleans up alternate screen, raw mode, and mouse capture.
    restore()?;
    Ok(())
}
