use clap::Parser;
use log::{info, trace, warn};
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, ToSpan},
    widgets::{Block, List, ListState, Paragraph},
};
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
}
/// App holds the state of the application
///
#[derive(Debug, Default)]
struct App {
    /// Current value of the input box
    input: Input,
    /// Current input mode
    input_mode: InputMode,
    /// which object is selected
    selected: Selected,
    /// matched strings
    matches: StatefulList<String>,
    /// non matched strings
    non_matches: StatefulList<String>,
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
            None => 0,
        };
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
            None => 0,
        };
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
        let extensions;
        if args.given_extensions.is_empty() && args.use_extensions {
            extensions = const_utils::get_default_extensions();
        } else {
            extensions = args.given_extensions;
        }
        self.files = args.files;
        if self.files.is_empty() {
            if args.dir.is_empty() {
                self.items =
                    shell_utils::start_shell_search(env::current_dir().unwrap(), extensions);
            } else {
                self.items = shell_utils::start_shell_search(PathBuf::from(args.dir), extensions);
            }
        } else {
            self.items = io_util::read_file(&self.files)
        }
        let mut list_state = ListState::default().with_selected(Some(0));
        terminal.draw(|frame| self.render(frame, &mut list_state))?;
        loop {
            let event = event::read()?;
            if let Event::Key(key) = event {
                match self.input_mode {
                    InputMode::Normal => match key.code {
                        KeyCode::Char('e') => self.start_editing(),
                        KeyCode::Char('j') => self.on_down(),
                        KeyCode::Char('k') => self.on_up(),
                        KeyCode::Char('l') => self.on_left(),
                        KeyCode::Char('h') => self.on_right(),
                        KeyCode::Char('q') => return Ok(()), // exit
                        _ => {}
                    },
                    InputMode::Editing => match key.code {
                        KeyCode::Enter => self.get_message(),
                        KeyCode::Esc => self.stop_editing(),
                        _ => {
                            self.input.handle_event(&event);
                            self.get_message();
                            terminal.draw(|frame| self.render(frame, &mut list_state))?;
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
        } else {
            self.non_matches.next();
        }
    }
    fn on_up(&mut self) {
        if self.selected == Selected::Matches {
            self.matches.previous();
        } else {
            self.non_matches.previous();
        }
    }
    fn on_left(&mut self) {
        if self.selected == Selected::Misses {
            self.selected = Selected::Matches
        }
    }
    fn on_right(&mut self) {
        if self.selected == Selected::Matches {
            self.selected = Selected::Misses
        }
    }

    fn get_message(&mut self) {
        self.message = self.input.value().to_string();
        self.matches = StatefulList::with_items(Vec::new());
        self.non_matches = StatefulList::with_items(Vec::new());
    }

    fn render(&mut self, frame: &mut Frame, list_state: &mut ListState) {
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
        self.render_messages(frame, list_state, matching_areas[0], matching_areas[1]);
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

    fn render_messages(
        &mut self,
        frame: &mut Frame,
        list_state: &mut ListState,
        matching_area: Rect,
        non_matching_area: Rect,
    ) {
        self.matches.clear();
        self.non_matches.clear();
        matching_utils::update_matches(
            &self.message,
            &mut self.matches.items,
            &mut self.non_matches.items,
            self.items.clone(),
        );
        frame.render_stateful_widget(
            List::new(self.matches.items.clone())
                .block(Block::bordered())
                .style(Color::White)
                .highlight_style(Modifier::REVERSED)
                .highlight_symbol("> "),
            matching_area,
            list_state,
        );
        frame.render_stateful_widget(
            List::new(self.non_matches.items.clone())
                .block(Block::bordered())
                .style(Color::White)
                .highlight_style(Modifier::REVERSED)
                .highlight_symbol("> "),
            non_matching_area,
            list_state,
        );
    }
}
