#[allow(dead_code)]
use chip8::utils::{Event, Events};
use std::{error::Error, io};
use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders},
    Terminal,
};

fn main() -> Result<(), Box<dyn Error>> {
    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    // Setup event handlers
    let events = Events::new();

    loop {
        terminal.draw(|mut f| {
            // Wrapping block for a group
            // Just draw the block and the group on the same area and build the group
            // with at least a margin of 1
            let size = f.size();
            let block = Block::default()
                .borders(Borders::ALL)
                .title(" CHIP8 Emulator ")
                .border_type(BorderType::Rounded);
            f.render_widget(block, size);
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints([
                    Constraint::Percentage(70), 
                    Constraint::Percentage(30)].as_ref())
                .split(f.size());

            let block = Block::default()
                .title_style(Style::default().fg(Color::Yellow))
                .style(Style::default().bg(Color::Green));
            f.render_widget(block, chunks[0]);

            let block = Block::default().title(" Debug ").borders(Borders::ALL);
            f.render_widget(block, chunks[1]);
        })?;

        match events.next()? {
            Event::Input(key) => {
                if key == Key::Char('q') {
                    break;
                }
            }
            _ => {}
        }
    }
    Ok(())
}

// use chip8::Chip8;
// use std::fs;

// fn main() {
//     let game_data = match fs::read("./games/MAZE") {
//         Ok(data) => data,
//         Err(error) => {
//             println!("{}", error);
//             vec![]
//         }
//     };

//     print_file(&game_data);

//     let mut chip = Chip8::new_with_memory(game_data);
//     chip.main_loop();
// }

// fn print_file(data: &Vec<u8>) {
//     let mut i = 0;
//     while i < data.len() {
//         print!("{:02x}", data[i]);
//         println!("{:02x}", data[i + 1]);
//         i += 2;
//     }
// }
