use std::{io, thread, time::Duration};
use tui::{
    backend::CrosstermBackend,
    widgets::{Widget, Block, Borders},
    layout::{Layout, Constraint, Direction},
    Terminal, style::Style, text::Span
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

fn main() -> Result<(), io::Error> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|f| {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage(20),
                    Constraint::Percentage(30),
                    Constraint::Percentage(50),
                ]
            )
            .split(f.size());
        let mut block1_title = Span::styled("David's cool block blah blah blah blah blah blah blah blah blah blah blah", Style::default().bg(tui::style::Color::DarkGray));
        if block1_title.width() > layout[0].width as usize {
            let mut new_title = String::from(&block1_title.content[..layout[0].width as usize - 3]);
            new_title.push_str("...");
            block1_title = Span::styled(new_title, block1_title.style);
        }
        let block = Block::default()
            .title(block1_title)
            .borders(Borders::NONE)
            .style(Style::default().bg(tui::style::Color::Cyan));
        f.render_widget(block, layout[0]);
        let block = Block::default()
            .title("Block 2")
            .borders(Borders::ALL)
            .borders(Borders::NONE)
            .style(Style::default().bg(tui::style::Color::LightCyan));
        f.render_widget(block, layout[1]);
        let block = Block::default()
            .title("Block 3")
            .borders(Borders::ALL)
            .borders(Borders::NONE)
            .style(Style::default().bg(tui::style::Color::LightBlue));
        f.render_widget(block, layout[2]);
    })?;

    thread::sleep(Duration::from_millis(5000));

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}