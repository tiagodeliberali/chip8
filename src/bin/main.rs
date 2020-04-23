#[allow(dead_code)]
use chip8::utils::{Event, Events};
use chip8::Chip8;
use std::fs;
use std::{error::Error, io};
use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::TermionBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph, Text},
    Terminal,
};

fn main() -> Result<(), Box<dyn Error>> {
    // GAME
    let game_data = match fs::read("./games/MAZE") {
        Ok(data) => data,
        Err(error) => {
            println!("{}", error);
            vec![]
        }
    };

    let mut chip = Chip8::new_with_memory(game_data);

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
        chip.single_iteration();

        terminal.draw(|mut f| {
            let size = f.size();
            let block = Block::default()
                .borders(Borders::ALL)
                .title(" CHIP8 Emulator ")
                .border_type(BorderType::Rounded);
            f.render_widget(block, size);

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints([Constraint::Length(35), Constraint::Length(10)].as_ref())
                .split(f.size());

            // CHIP-8 UI
            let block = Block::default()
                .title_style(Style::default().fg(Color::Yellow))
                .style(Style::default().bg(Color::Green));

            let text: [Text; 32] = [
                Text::raw(format_line(chip.screen[0])),
                Text::raw(format_line(chip.screen[1])),
                Text::raw(format_line(chip.screen[2])),
                Text::raw(format_line(chip.screen[3])),
                Text::raw(format_line(chip.screen[4])),
                Text::raw(format_line(chip.screen[5])),
                Text::raw(format_line(chip.screen[6])),
                Text::raw(format_line(chip.screen[7])),
                Text::raw(format_line(chip.screen[8])),
                Text::raw(format_line(chip.screen[9])),
                Text::raw(format_line(chip.screen[10])),
                Text::raw(format_line(chip.screen[11])),
                Text::raw(format_line(chip.screen[12])),
                Text::raw(format_line(chip.screen[13])),
                Text::raw(format_line(chip.screen[14])),
                Text::raw(format_line(chip.screen[15])),
                Text::raw(format_line(chip.screen[16])),
                Text::raw(format_line(chip.screen[17])),
                Text::raw(format_line(chip.screen[18])),
                Text::raw(format_line(chip.screen[19])),
                Text::raw(format_line(chip.screen[20])),
                Text::raw(format_line(chip.screen[21])),
                Text::raw(format_line(chip.screen[22])),
                Text::raw(format_line(chip.screen[23])),
                Text::raw(format_line(chip.screen[24])),
                Text::raw(format_line(chip.screen[25])),
                Text::raw(format_line(chip.screen[26])),
                Text::raw(format_line(chip.screen[27])),
                Text::raw(format_line(chip.screen[28])),
                Text::raw(format_line(chip.screen[29])),
                Text::raw(format_line(chip.screen[30])),
                Text::raw(format_line(chip.screen[31])),
            ];

            let paragraph = Paragraph::new(text.iter())
                .block(block.clone())
                .alignment(Alignment::Left);

            f.render_widget(paragraph, chunks[0]);

            // DEBUG INFO
            let block = Block::default().title(" Debug ").borders(Borders::ALL);

            let text = [
                Text::raw("Press 'q' to exit\n"),
                Text::raw(chip.get_state()),
            ];

            let paragraph = Paragraph::new(text.iter())
                .block(block.clone())
                .alignment(Alignment::Left);

            f.render_widget(paragraph, chunks[1]);
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

fn format_line(data: [u8; 64]) -> String {
    let mut result: String = String::from("");
    for x in 0..64 {
        if data[x] == 0 {
            result = format!("{}{}", result, "░");
        } else {
            result = format!("{}{}", result, "█");
        }
    }
    format!("{}\n", result)
}
