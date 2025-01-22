use crate::flashcard::Flashcard;
use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    layout::{Constraint, Layout}, style::Stylize, text::Line, widgets::{Block, Paragraph}, DefaultTerminal, Frame
};

#[derive(Debug, Default)]
pub struct App {
    /// Is the application running?
    running: bool,
    flashcards: Vec<Flashcard>,
    current_index: usize,
    display_answer: bool,
}

impl App {
    /// Construct a new instance of [`App`].
    pub fn new(flashcards: Vec<Flashcard>) -> Self {
        let mut app = Self::default();
        app.flashcards = flashcards;
        return app;
    }

    /// Run the application's main loop.
    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.running = true;
        while self.running {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_crossterm_events()?;
        }
        Ok(())
    }

    /// Renders the user interface.
    ///
    /// This is where you add new widgets. See the following resources for more information:
    /// - <https://docs.rs/ratatui/latest/ratatui/widgets/index.html>
    /// - <https://github.com/ratatui/ratatui/tree/master/examples>
    fn draw(&mut self, frame: &mut Frame) {
        let title = Line::from("Flashcards!")
            .bold()
            .blue()
            .centered();
        let text = self.flashcards[self.current_index].front();
        let answer_text = match self.display_answer {
            false => "",
            true => &self.flashcards[self.current_index].back(),
        };

        let layout = Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)]);
        let [question, answer] = layout.areas(frame.area());
        frame.render_widget(
            Paragraph::new(text)
                .block(Block::bordered().title(title))
                .centered(),
            question,
        );
        frame.render_widget(
            Paragraph::new(answer_text)
                .block(Block::bordered().title("Answer"))
                .centered(),
            answer,
        );
    }

    /// Reads the crossterm events and updates the state of [`App`].
    ///
    /// If your application needs to perform work in between handling events, you can use the
    /// [`event::poll`] function to check if there are any events available with a timeout.
    fn handle_crossterm_events(&mut self) -> Result<()> {
        match event::read()? {
            // it's important to check KeyEventKind::Press to avoid handling key release events
            Event::Key(key) if key.kind == KeyEventKind::Press => self.on_key_event(key),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
            _ => {}
        }
        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    fn on_key_event(&mut self, key: KeyEvent) {
        match (key.modifiers, key.code) {
            (_, KeyCode::Esc | KeyCode::Char('q'))
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.quit(),
            // Add other key handlers here.
            (_, KeyCode::Up) => self.flip_current_card(),
            (_, KeyCode::Left) => self.next_card(),
            (_, KeyCode::Right) => self.previous_card(),
            _ => {}
        }
    }

    /// Set running to false to quit the application.
    fn quit(&mut self) {
        self.running = false;
    }

    /// Set state to display the flashcard answer
    fn flip_current_card(&mut self) {
        self.display_answer = !self.display_answer;
    }

    /// Move one card forward in the deck and hide the answer
    fn next_card(&mut self) {
        self.current_index = self.current_index.saturating_add(1);
        self.display_answer = false;
    }

    /// Move one card previous into the deck and hide the answer
    fn previous_card(&mut self) {
        self.current_index = self.current_index.saturating_sub(1);
        self.display_answer = false;
    }
}
