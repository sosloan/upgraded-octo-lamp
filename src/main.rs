// BET Architecture - Main Entry Point
// Terminal GUI with ANSI escape codes, Modal keyboard system (Vim-style), Menu navigation

use std::io::{self, Write};
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute, queue,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{self, Clear, ClearType},
};

use bet_architecture::{
    elixir_check::run_elixir_check,
    monad_lambda::demonstrate_monad_system,
    storm::StormTopology,
    swin_transformer::SwinTransformer,
    trading_dag::TradingWorkflow,
    trading_system::TradingSystem,
};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Mode {
    Normal,
    Command,
    Insert,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy, PartialEq)]
enum MenuItem {
    TradingSystem,
    StormTopologies,
    MonadLambda,
    ADAG,
    SwinTransformer,
    ElixirCheck,
    Quit,
}

struct App {
    mode: Mode,
    selected_menu_item: MenuItem,
    search_query: String,
    trading_system: TradingSystem,
    storm: StormTopology,
    swin: SwinTransformer,
    trading_workflow: TradingWorkflow,
}

impl App {
    fn new() -> Self {
        App {
            mode: Mode::Normal,
            selected_menu_item: MenuItem::TradingSystem,
            search_query: String::new(),
            trading_system: TradingSystem::new(1_000_000.0),
            storm: StormTopology::new(),
            swin: SwinTransformer::with_16_heads(),
            trading_workflow: TradingWorkflow::new(),
        }
    }

    fn get_menu_items(&self) -> Vec<MenuItem> {
        let all_items = vec![
            MenuItem::TradingSystem,
            MenuItem::StormTopologies,
            MenuItem::MonadLambda,
            MenuItem::ADAG,
            MenuItem::SwinTransformer,
            MenuItem::ElixirCheck,
            MenuItem::Quit,
        ];

        if self.search_query.is_empty() {
            all_items
        } else {
            all_items
                .into_iter()
                .filter(|item| {
                    let name = format!("{:?}", item).to_lowercase();
                    name.contains(&self.search_query.to_lowercase())
                })
                .collect()
        }
    }

    fn next_menu_item(&mut self) {
        let items = self.get_menu_items();
        if let Some(current_idx) = items.iter().position(|&item| item == self.selected_menu_item) {
            let next_idx = (current_idx + 1) % items.len();
            self.selected_menu_item = items[next_idx];
        }
    }

    fn prev_menu_item(&mut self) {
        let items = self.get_menu_items();
        if let Some(current_idx) = items.iter().position(|&item| item == self.selected_menu_item) {
            let prev_idx = if current_idx == 0 {
                items.len() - 1
            } else {
                current_idx - 1
            };
            self.selected_menu_item = items[prev_idx];
        }
    }
}

fn main() -> io::Result<()> {
    // Enable raw mode for terminal
    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide)?;

    let result = run_app(&mut stdout);

    // Restore terminal
    execute!(
        stdout,
        terminal::LeaveAlternateScreen,
        cursor::Show,
        ResetColor
    )?;
    terminal::disable_raw_mode()?;

    result
}

fn run_app<W: Write>(stdout: &mut W) -> io::Result<()> {
    let mut app = App::new();

    loop {
        draw_ui(stdout, &app)?;

        if let Event::Key(key) = event::read()? {
            match app.mode {
                Mode::Normal => {
                    if handle_normal_mode(&mut app, key) {
                        break;
                    }
                }
                Mode::Command => {
                    if handle_command_mode(&mut app, key) {
                        break;
                    }
                }
                Mode::Insert => {
                    if handle_insert_mode(&mut app, key) {
                        break;
                    }
                }
            }
        }
    }

    Ok(())
}

fn handle_normal_mode(app: &mut App, key: KeyEvent) -> bool {
    match key.code {
        KeyCode::Char('q') => return true,
        KeyCode::Char('j') | KeyCode::Down => app.next_menu_item(),
        KeyCode::Char('k') | KeyCode::Up => app.prev_menu_item(),
        KeyCode::Char(':') => app.mode = Mode::Command,
        KeyCode::Char('/') => {
            app.mode = Mode::Insert;
            app.search_query.clear();
        }
        KeyCode::Enter => {
            // Enter is handled by showing the selected item
        }
        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => return true,
        _ => {}
    }
    false
}

fn handle_command_mode(app: &mut App, key: KeyEvent) -> bool {
    match key.code {
        KeyCode::Char('q') => return true,
        KeyCode::Esc => app.mode = Mode::Normal,
        KeyCode::Enter => app.mode = Mode::Normal,
        _ => {}
    }
    false
}

fn handle_insert_mode(app: &mut App, key: KeyEvent) -> bool {
    match key.code {
        KeyCode::Esc => {
            app.mode = Mode::Normal;
            app.search_query.clear();
        }
        KeyCode::Enter => app.mode = Mode::Normal,
        KeyCode::Char(c) => app.search_query.push(c),
        KeyCode::Backspace => {
            app.search_query.pop();
        }
        _ => {}
    }
    false
}

fn draw_ui<W: Write>(stdout: &mut W, app: &App) -> io::Result<()> {
    queue!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0))?;

    // Draw header
    draw_header(stdout)?;

    // Draw mode indicator
    queue!(stdout, cursor::MoveTo(0, 2))?;
    let mode_text = match app.mode {
        Mode::Normal => "-- NORMAL --",
        Mode::Command => "-- COMMAND --",
        Mode::Insert => "-- INSERT (SEARCH) --",
    };
    queue!(
        stdout,
        SetForegroundColor(Color::Yellow),
        Print(format!("Mode: {}", mode_text)),
        ResetColor,
        Print("\n\n")
    )?;

    // Draw search query
    if !app.search_query.is_empty() || app.mode == Mode::Insert {
        queue!(
            stdout,
            SetForegroundColor(Color::Cyan),
            Print(format!("Search: {}", app.search_query)),
            ResetColor,
            Print("\n\n")
        )?;
    }

    // Draw menu
    draw_menu(stdout, app)?;

    // Draw content for selected item
    queue!(stdout, Print("\n"))?;
    draw_content(stdout, app)?;

    // Draw footer
    draw_footer(stdout)?;

    stdout.flush()?;
    Ok(())
}

fn draw_header<W: Write>(stdout: &mut W) -> io::Result<()> {
    queue!(
        stdout,
        SetBackgroundColor(Color::Blue),
        SetForegroundColor(Color::White),
        Print("╔═══════════════════════════════════════════════════════════════════════════════╗"),
        Print("\n"),
        Print("║                         BET ARCHITECTURE SYSTEM                               ║"),
        Print("\n"),
        Print("╚═══════════════════════════════════════════════════════════════════════════════╝"),
        ResetColor,
        Print("\n")
    )?;
    Ok(())
}

fn draw_menu<W: Write>(stdout: &mut W, app: &App) -> io::Result<()> {
    queue!(
        stdout,
        SetForegroundColor(Color::Green),
        Print("MENU:\n"),
        ResetColor
    )?;

    let menu_items = app.get_menu_items();
    for item in menu_items {
        let is_selected = item == app.selected_menu_item;
        
        if is_selected {
            queue!(
                stdout,
                SetBackgroundColor(Color::White),
                SetForegroundColor(Color::Black),
                Print("  ▶ ")
            )?;
        } else {
            queue!(stdout, Print("    "))?;
        }

        let label = match item {
            MenuItem::TradingSystem => "Trading System (Biotech, P&L, CURE Foundation)",
            MenuItem::StormTopologies => "Storm Topologies (Word Count, Sum, Edison⚡, Polymath🌐)",
            MenuItem::MonadLambda => "Monad λ System (Invariants, Laws, Plumber)",
            MenuItem::ADAG => "A-DAG (OCTOTREÉ, Topological Sort, Critical Path)",
            MenuItem::SwinTransformer => "SWIN Transformer (16 Heads, Grey Eyes, 600 Shades)",
            MenuItem::ElixirCheck => "Elixir Check (Erlang/OTP Guarantees)",
            MenuItem::Quit => "Quit",
        };

        queue!(stdout, Print(label))?;

        if is_selected {
            queue!(stdout, ResetColor)?;
        }
        
        queue!(stdout, Print("\n"))?;
    }

    Ok(())
}

fn draw_content<W: Write>(stdout: &mut W, app: &App) -> io::Result<()> {
    queue!(
        stdout,
        SetForegroundColor(Color::Cyan),
        Print("═══════════════════════════════════════════════════════════════════════════════\n"),
        ResetColor
    )?;

    match app.selected_menu_item {
        MenuItem::TradingSystem => {
            queue!(
                stdout,
                SetForegroundColor(Color::Yellow),
                Print("TRADING SYSTEM\n"),
                ResetColor,
                Print(format!("{}\n", app.trading_system.display_summary()))
            )?;
        }
        MenuItem::StormTopologies => {
            queue!(
                stdout,
                SetForegroundColor(Color::Yellow),
                Print("STORM TOPOLOGIES\n"),
                ResetColor,
                Print(format!("{}\n", app.storm.display()))
            )?;
        }
        MenuItem::MonadLambda => {
            queue!(
                stdout,
                SetForegroundColor(Color::Yellow),
                Print("MONAD λ SYSTEM\n"),
                ResetColor,
                Print(format!("{}\n", demonstrate_monad_system()))
            )?;
        }
        MenuItem::ADAG => {
            queue!(
                stdout,
                SetForegroundColor(Color::Yellow),
                Print("A-DAG (ACYCLIC DIRECTED ACYCLIC GRAPH)\n"),
                ResetColor,
                Print(format!("{}\n", app.trading_workflow.display())),
                Print("Trading Workflow:\n")
            )?;
            if let Ok(order) = app.trading_workflow.get_execution_order() {
                for (i, task) in order.iter().enumerate() {
                    queue!(stdout, Print(format!("  {}. {}\n", i + 1, task)))?;
                }
            }
        }
        MenuItem::SwinTransformer => {
            queue!(
                stdout,
                SetForegroundColor(Color::Yellow),
                Print("SWIN TRANSFORMER\n"),
                ResetColor,
                Print(format!("{}\n", app.swin.display()))
            )?;
        }
        MenuItem::ElixirCheck => {
            let elixir_check = run_elixir_check();
            queue!(
                stdout,
                SetForegroundColor(Color::Yellow),
                Print("ELIXIR CHECK\n"),
                ResetColor,
                Print(format!("{}\n", elixir_check.display()))
            )?;
        }
        MenuItem::Quit => {
            queue!(
                stdout,
                SetForegroundColor(Color::Red),
                Print("Press 'q' or Ctrl+C to exit\n"),
                ResetColor
            )?;
        }
    }

    Ok(())
}

fn draw_footer<W: Write>(stdout: &mut W) -> io::Result<()> {
    queue!(
        stdout,
        SetForegroundColor(Color::DarkGrey),
        Print("\n"),
        Print("─────────────────────────────────────────────────────────────────────────────\n"),
        Print("Keys: j/k or ↑/↓ (navigate) | / (search) | : (command) | Enter (select) | q (quit)\n"),
        Print("Screen Reader: Menu items are numbered and labeled for accessibility\n"),
        ResetColor
    )?;
    Ok(())
}
