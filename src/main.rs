use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::widgets::{Block, Borders, Paragraph};
use std::io;
use tui_textarea::{Input, Key, TextArea};

fn main() -> io::Result<()> {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    enable_raw_mode()?;
    crossterm::execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut term = Terminal::new(backend)?;

    let mut textarea = TextArea::default();
    textarea.set_placeholder_text("Please be nice in the chat!");
    textarea.set_block(Block::default().borders(Borders::ALL).title("Chat room"));

    let mut chat_history = String::new();

    loop {
        term.draw(|f| {
            let chat =
                Paragraph::new(chat_history.as_str()).block(Block::default().borders(Borders::ALL));

            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(20), Constraint::Fill(1)])
                .split(f.area());

            let sidebar = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(3), Constraint::Fill(1)])
                .split(chunks[0]);

            f.render_widget(&textarea, sidebar[0]);
            f.render_widget(&chat, sidebar[1]);
        })?;

        match ratatui::crossterm::event::read()?.into() {
            Input { key: Key::Esc, .. } => break,
            Input {
                key: Key::Enter, ..
            } => {
                chat_history.push_str(&(textarea.lines().join("\n") + "\n"));

                textarea = TextArea::default();
                textarea.set_placeholder_text("Please be nice in the chat!");
                textarea.set_block(Block::default().borders(Borders::ALL).title("Chat room"));
            }
            input => {
                textarea.input(input);
            }
        }
    }

    disable_raw_mode()?;
    crossterm::execute!(
        term.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    term.show_cursor()?;

    println!("Lines: {:?}", textarea.lines());
    Ok(())
}
