use clap::Parser;
use env_logger::Target;
use log::{LevelFilter, debug, error, info, trace, warn};
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{self, Line, Span, ToSpan},
    widgets::{Block, List, ListItem, ListState, Paragraph},
};
use std::fs::OpenOptions;
use std::{env, io, path::PathBuf};
use tui_input::Input;
use tui_input::backend::crossterm::EventHandler;
pub mod const_utils;
pub mod io_util;
pub mod matching_utils;
pub mod parser;
pub mod shell_utils;
pub mod vec_utils;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let result = App::default().run(&mut terminal);
    ratatui::restore();
    result
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// files to be inputted
    files: Vec<String>,
    /// Directory to search
    #[arg(short, long)]
    dir: String,
    /// Whether to use extension filtering
    #[arg(short, long)]
    use_extensions: bool,
    /// Extensions to parse
    #[arg(short, long)]
    given_extensions: Vec<String>,
    #[arg(
        long,
        help = "Set logging level (debug, info, warn, error). Default is 'warn'"
    )]
    log: Option<log::Level>,
}
/// App holds the state of the application
///
#[derive(Debug, Default)]
struct App {
    /// Current value of the input box
    input: Input,
    /// Current input mode
    input_mode: InputMode,
    /// What search mode we're in
    search_mode: SearchMode,
    /// which object is selected
    selected: Selected,
    /// matched strings
    matches: StatefulList<String>,
    /// non matched strings
    misses: StatefulList<String>,
    /// user input
    message: String,
    /// files being searched
    files: Vec<String>,
    /// the list of items to be matched
    items: Vec<String>,
}

#[derive(Debug, Default)]
pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T> StatefulList<T> {
    pub fn with_items(items: Vec<T>) -> Self {
        Self {
            state: ListState::default(),
            items,
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            _ => {
                trace!("Failed selecting next");
                0
            }
        };
        trace!("Selected next object: {i}");
        trace!("Total items: {}", self.items.len());
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            _ => 0,
        };
        trace!("Selected prev object: {i}");
        self.state.select(Some(i));
    }
    pub fn clear(&mut self) {
        self.items.clear();
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum InputMode {
    #[default]
    Normal,
    Editing,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum SearchMode {
    #[default]
    Shell,
    File,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum Selected {
    #[default]
    Matches,
    Misses,
}

impl App {
    fn run(mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        let args = Cli::parse();
        let mut builder = env_logger::Builder::new();
        let log_level = if let Some(level) = args.log {
            level.to_string()
        } else {
            std::env::var("RUST_LOG").unwrap_or(String::from("warn"))
        };
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true) // Remove existing content; set to false to append
            .open("log/logging.log")
            .expect("Failed to open log file");
        let boxed_file = Box::new(file);
        builder
            .parse_filters(&log_level)
            .target(Target::Pipe(boxed_file))
            .init();
        let extensions;
        let path;
        if args.given_extensions.is_empty() && args.use_extensions {
            extensions = const_utils::get_default_extensions();
        } else {
            extensions = args.given_extensions;
        }
        self.files = args.files;
        if self.files.is_empty() {
            if args.dir.is_empty() {
                path = env::current_dir().unwrap();
            } else {
                path = PathBuf::from(args.dir);
            }
            self.items = shell_utils::start_shell_search(path, extensions);
        } else {
            self.items = io_util::read_file(&self.files)
        }
        loop {
            terminal.draw(|frame| self.render(frame))?;
            let event = event::read()?;
            if let Event::Key(key) = event {
                match self.input_mode {
                    InputMode::Normal => match key.code {
                        KeyCode::Char('e') => self.start_editing(),
                        KeyCode::Char('j') => self.on_down(),
                        KeyCode::Char('k') => self.on_up(),
                        KeyCode::Char('h') => self.on_left(),
                        KeyCode::Char('l') => self.on_right(),
                        KeyCode::Enter => self.select_current(),
                        KeyCode::Char('q') => return Ok(()), // exit
                        _ => {}
                    },
                    InputMode::Editing => match key.code {
                        KeyCode::Enter => self.get_message(),
                        KeyCode::Esc => self.stop_editing(),
                        _ => {
                            self.input.handle_event(&event);
                            self.get_message();
                        }
                    },
                }
            }
        }
    }

    fn start_editing(&mut self) {
        self.input_mode = InputMode::Editing
    }

    fn stop_editing(&mut self) {
        self.input_mode = InputMode::Normal
    }

    fn on_down(&mut self) {
        if self.selected == Selected::Matches {
            self.matches.next();
            trace!("Selected next match");
        } else {
            self.misses.next();
            trace!("Selected next miss");
        }
    }
    fn on_up(&mut self) {
        if self.selected == Selected::Matches {
            self.matches.previous();
            trace!("Selected prev match");
        } else {
            self.misses.previous();
            trace!("Selected prev miss");
        }
    }
    fn on_left(&mut self) {
        if self.selected == Selected::Misses {
            trace!("selected matches");
            self.selected = Selected::Matches
        }
    }
    fn on_right(&mut self) {
        if self.selected == Selected::Matches {
            trace!("selected misses");
            self.selected = Selected::Misses
        }
    }

    fn select_current(&mut self) {
        if let Some(i) = self.matches.state.selected() {
            trace!("{}", self.matches.items[i]);
            if self.search_mode == SearchMode::Shell {}
        }
    }

    fn get_message(&mut self) {
        self.message = self.input.value().to_string();
        self.matches = StatefulList::with_items(Vec::new());
        self.misses = StatefulList::with_items(Vec::new());
    }

    fn render(&mut self, frame: &mut Frame) {
        let verticals = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Length(1),
                Constraint::Length(3),
                Constraint::Min(1),
            ])
            .split(frame.area());

        let matching_areas = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(verticals[2]);

        self.render_help_message(frame, verticals[0]);
        self.render_input(frame, verticals[1]);
        self.render_messages(frame, matching_areas[0], matching_areas[1]);
    }

    fn render_help_message(&self, frame: &mut Frame, area: Rect) {
        let help_message = Line::from_iter(match self.input_mode {
            InputMode::Normal => [
                "Press ".to_span(),
                "q".bold(),
                " to exit, ".to_span(),
                "e".bold(),
                " to start editing.".to_span(),
            ],
            InputMode::Editing => [
                "Press ".to_span(),
                "Esc".bold(),
                " to stop editing, ".to_span(),
                "Enter".bold(),
                " to record the message".to_span(),
            ],
        });
        frame.render_widget(help_message, area);
    }

    fn render_input(&self, frame: &mut Frame, area: Rect) {
        // keep 2 for borders and 1 for cursor
        let width = area.width.max(3) - 3;
        let scroll = self.input.visual_scroll(width as usize);
        let style = match self.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Color::Yellow.into(),
        };
        let input = Paragraph::new(self.input.value())
            .style(style)
            .scroll((0, scroll as u16))
            .block(Block::bordered().title("Input"));
        frame.render_widget(input, area);

        if self.input_mode == InputMode::Editing {
            // Ratatui hides the cursor unless it's explicitly set. Position the  cursor past the
            // end of the input text and one line down from the border to the input line
            let x = self.input.visual_cursor().max(scroll) - scroll + 1;
            frame.set_cursor_position((area.x + x as u16, area.y + 1))
        }
    }

    fn render_messages(&mut self, frame: &mut Frame, matches_area: Rect, misses_area: Rect) {
        self.matches.clear();
        self.misses.clear();
        matching_utils::update_matches(
            &self.message,
            &mut self.matches.items,
            &mut self.misses.items,
            self.items.clone(),
        );
        let matches = List::new(self.matches.items.clone())
            .block(Block::bordered())
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol("> ");
        let misses = List::new(self.misses.items.clone())
            .block(Block::bordered())
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol("> ");

        frame.render_stateful_widget(matches, matches_area, &mut self.matches.state);
        frame.render_stateful_widget(misses, misses_area, &mut self.misses.state);
    }
}
