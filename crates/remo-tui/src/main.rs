#![allow(dead_code, unused_imports)]
use anyhow::bail;
use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event as CEvent, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use remo::{
    self,
    cloud::{Appliance, Client},
};
use std::{
    env, io,
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph},
    Frame, Terminal,
};

const CACHE_PATH: &str = "./cache.json";

#[derive(Debug)]
pub enum State {
    Main,
    Detail,
}

pub enum Event<I> {
    Input(I),
    Tick,
}

pub struct App {
    pub client: Client,
    pub appliances: Vec<Appliance>,
    pub current_state: State,
}

pub trait Context {
    fn new() -> Self;
    fn draw<B: Backend>(&mut self, f: &mut Frame<B>, app_state: &mut App);
    fn event(&mut self, e: KeyCode, app_state: &mut App) -> Result<()>;
}

struct MainContext {
    state: ListState,
}
impl MainContext {
    fn render_home<'a>() -> Paragraph<'a> {
        let home = Paragraph::new(vec![
            Spans::from(vec![Span::raw("")]),
            Spans::from(vec![Span::raw("Welcome")]),
            Spans::from(vec![Span::raw("")]),
            Spans::from(vec![Span::raw("to")]),
            Spans::from(vec![Span::raw("")]),
            Spans::from(vec![Span::styled(
                "Nature Remo TUI",
                Style::default().fg(Color::LightBlue),
            )]),
        ])
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title("Home")
                .border_type(BorderType::Plain),
        );
        home
    }
}
impl Context for MainContext {
    fn new() -> MainContext {
        let mut state = ListState::default();
        state.select(Some(0));
        MainContext { state }
    }

    fn draw<B: Backend>(&mut self, f: &mut Frame<B>, app_state: &mut App) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
            .split(f.size());

        // Appliances
        let apps: Vec<_> = app_state
            .appliances
            .iter()
            .map(|app| {
                ListItem::new(Spans::from(vec![Span::styled(
                    app.nickname.clone(),
                    Style::default(),
                )]))
            })
            .collect();
        let block = Block::default().title("Appliances").borders(Borders::ALL);
        let list = List::new(apps).block(block).highlight_style(
            Style::default()
                .bg(Color::Yellow)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        );
        f.render_stateful_widget(list, chunks[0], &mut self.state);

        // Controls
        let app = app_state.appliances[self.state.selected().unwrap_or(0)].clone();
        // println!("{:?}", app.r#type);
        let block = Block::default()
            .title(app.nickname.to_owned())
            .borders(Borders::ALL);
        let list = match app.r#type.as_str() {
            "AC" => block,
            _ => block,
        };
        f.render_widget(list, chunks[1]);
    }

    fn event(&mut self, code: KeyCode, app_state: &mut App) -> Result<()> {
        match code {
            // Quit
            KeyCode::Char('q') => bail!("Quit"),

            // Reload
            KeyCode::Char('r') => {}

            // Return to menu
            KeyCode::Esc => {}

            // Select appliance
            KeyCode::Enter => {
                let selected = self.state.selected().unwrap_or(0);
                // self.selected_appliance = Some(app_state.appliances[selected].clone());
            }

            // Move cursor down
            KeyCode::Down => {
                if let Some(selected) = self.state.selected() {
                    let len = app_state.appliances.len();
                    if selected >= len - 1 {
                        self.state.select(Some(0));
                    } else {
                        self.state.select(Some(selected + 1));
                    }
                }
            }

            // Move cursor up
            KeyCode::Up => {
                if let Some(selected) = self.state.selected() {
                    let len = app_state.appliances.len();
                    if selected > 0 {
                        self.state.select(Some(selected - 1));
                    } else {
                        self.state.select(Some(len - 1));
                    }
                }
            }
            _ => {}
        };
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    enable_raw_mode()?;

    // setup event poll
    let (tx, rx) = mpsc::channel();
    let tick_rate = Duration::from_millis(200);
    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout).expect("poll works") {
                if let CEvent::Key(key) = event::read().expect("can read events") {
                    tx.send(Event::Input(key)).expect("can send events");
                }
            }

            if last_tick.elapsed() >= tick_rate {
                if let Ok(_) = tx.send(Event::Tick) {
                    last_tick = Instant::now();
                }
            }
        }
    });

    // setup terminal
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // setup state
    let token = env::var("NATURE_REMO_CLOUD_API_TOKEN").ok();
    let client = remo::cloud::Client::new(token);
    let appliances = client.get_appliances().await.expect("get appliance");

    let mut app_state = App {
        client,
        appliances,
        current_state: State::Main,
    };

    let mut main_ctx: MainContext = Context::new();

    loop {
        // redraw ui
        // terminal.draw(|f| draw(f, &mut app_state))?;
        terminal.draw(|f| <MainContext as Context>::draw(&mut main_ctx, f, &mut app_state))?;

        // handle events
        match rx.recv()? {
            Event::Input(event) => {
                match <MainContext as Context>::event(&mut main_ctx, event.code, &mut app_state) {
                    Ok(_) => {}
                    Err(_) => {
                        // restore terminal
                        disable_raw_mode()?;
                        execute!(
                            terminal.backend_mut(),
                            LeaveAlternateScreen,
                            DisableMouseCapture
                        )?;
                        terminal.show_cursor()?;
                        break;
                    }
                }
            }
            Event::Tick => {}
        }
    }

    Ok(())
}
