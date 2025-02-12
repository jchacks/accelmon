use crate::backends::Backend;
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};
use std::io;
mod backends;
mod widgets;

struct App {
    backend: Backend,
    exit: bool,
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" GPU Stat ".bold());
        let instructions = Line::from(vec![" Quit ".into(), "<Q> ".blue().bold()]);

        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        let text: Text<'_> = Text::from(vec![Line::from(vec![
            "Value: ".into(),
            self.backend.get_text().yellow(),
        ])]);

        Paragraph::new(text)
            .centered()
            .block(block.clone())
            .render(area, buf);

        let text: Text<'_> = Text::from(vec![Line::from(vec![
            "Value: ".into(),
            format!("{:?}", self.backend.get_memory_samples()).yellow(),
        ])]);

        Paragraph::new(text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}

impl App {
    pub fn new() -> Self {
        Self {
            backend: Backend::nvidia(),
            exit: false,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
            self.backend.update_memory_samples();
        }
        Ok(())
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            _ => {}
        }
    }
}

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();

    let mut app = App::new();
    let app_result = app.run(&mut terminal);

    ratatui::restore();
    app_result
}
