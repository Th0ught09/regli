use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode},
    layout::{Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, ToSpan},
    widgets::{Block, Borders, Paragraph},
};
use regex::Regex;
use std::io;
use tui_input::Input;
use tui_input::backend::crossterm::EventHandler;

pub mod io_util;
pub mod parser;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let result = App::default().run(&mut terminal);
    ratatui::restore();
    result
}

/// App holds the state of the application
#[derive(Debug, Default)]
struct App<'a> {
    /// Current value of the input box
    input: Input,
    /// Current input mode
    input_mode: InputMode,
    /// matched strings
    matches: Vec<Paragraph<'a>>,
    /// non matched strings
    non_matches: Vec<Paragraph<'a>>,
    /// user input
    message: String,
    /// files being searched
    files: Vec<String>,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum InputMode {
    #[default]
    Normal,
    Editing,
}

impl App<'_> {
    fn run(mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        self.files = parser::parse_args();
        loop {
            let event = event::read()?;
            if let Event::Key(key) = event {
                match self.input_mode {
                    InputMode::Normal => match key.code {
                        KeyCode::Char('e') => self.start_editing(),
                        KeyCode::Char('q') => return Ok(()), // exit
                        _ => {}
                    },
                    InputMode::Editing => match key.code {
                        KeyCode::Enter => self.get_message(),
                        KeyCode::Esc => self.stop_editing(),
                        _ => {
                            self.input.handle_event(&event);
                        }
                    },
                }
            }
            terminal.draw(|frame| self.render(frame))?;
        }
    }

    fn start_editing(&mut self) {
        self.input_mode = InputMode::Editing
    }

    fn stop_editing(&mut self) {
        self.input_mode = InputMode::Normal
    }

    fn get_message(&mut self) {
        self.message = self.input.value_and_reset();
        self.matches = Vec::new();
        self.non_matches = Vec::new();
    }

    fn render(&mut self, frame: &mut Frame) {
        let verticals = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Min(1),
        ]);

        let horizontals = Layout::horizontal([Constraint::Min(1), Constraint::Min(1)]);

        let [header_area, input_area, output_area] = verticals.areas(frame.area());

        let matching_areas = Layout::horizontal([Constraint::Min(1), Constraint::Min(1)])
            .split(output_area)
            .to_vec();

        if let [matching_area, non_matching_area] = &matching_areas[..] {
            self.render_help_message(frame, header_area);
            self.render_input(frame, input_area);
            self.render_messages(frame, *matching_area, *non_matching_area);
        }
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

    fn render_messages(&mut self, frame: &mut Frame, area: Rect, non_matching_area: Rect) {
        if self.message != "" {
            let re = Regex::new(&self.message).unwrap();
            let message = io_util::read_file(&self.files);
            if re.is_match(message.as_str()) {
                let message =
                    Paragraph::new(message.clone()).block(Block::new().borders(Borders::ALL));
            } else {
                // let message = Paragraph::new(query).block(Block::new().borders(Borders::ALL));
                self.non_matches.push(Paragraph::new(message.clone()))
            }
            frame.render_widget(&message, non_matching_area);
            frame.render_widget(&message, area);
        }
    }
}
