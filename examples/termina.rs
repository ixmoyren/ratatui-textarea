use std::error::Error;
use std::io::Write as _;

use ratatui_core::terminal::Terminal;
use ratatui_termina::TerminaBackend;
use ratatui_termina::termina::escape::csi::{self, Csi};
use ratatui_termina::termina::{Event, EventReader, PlatformTerminal, Terminal as _};
use ratatui_textarea::{Input, Key, TextArea};
use ratatui_widgets::block::Block;
use ratatui_widgets::borders::Borders;

macro_rules! decset {
    ($mode:ident) => {{
        let mode = csi::DecPrivateMode::Code(csi::DecPrivateModeCode::$mode);
        Csi::Mode(csi::Mode::SetDecPrivateMode(mode))
    }};
}

macro_rules! decreset {
    ($mode:ident) => {{
        let mode = csi::DecPrivateMode::Code(csi::DecPrivateModeCode::$mode);
        Csi::Mode(csi::Mode::ResetDecPrivateMode(mode))
    }};
}

type AppTerminal = Terminal<TerminaBackend<PlatformTerminal>>;

fn main() -> Result<(), Box<dyn Error>> {
    let (mut term, events) = init_terminal()?;

    let mut textarea = TextArea::default();
    textarea.set_block(
        Block::default()
            .borders(Borders::ALL)
            .title("Termina Minimal Example"),
    );

    // The event loop
    loop {
        term.draw(|f| {
            f.render_widget(&textarea, f.area());
        })?;

        // Block until a key or mouse event arrives, leaving other events buffered.
        match events.read(|event| matches!(event, Event::Key(_) | Event::Mouse(_)))? {
            Event::Key(key) => {
                let input = Input::from(key);
                match input {
                    Input { key: Key::Esc, .. } => break,
                    input => {
                        textarea.input(input);
                    }
                }
            }
            Event::Mouse(mouse) => {
                textarea.input(mouse);
            }
            _ => {}
        }
    }

    restore_terminal(&mut term)?; // Leave the alternate screen to print the following line
    println!("Lines: {:?}", textarea.lines());
    Ok(())
}

fn init_terminal() -> Result<(AppTerminal, EventReader), Box<dyn Error>> {
    let mut output = PlatformTerminal::new()?;
    output.enter_raw_mode()?;

    let enter_alternate_screen = decset!(ClearAndEnableAlternateScreen);
    let show_cursor = decset!(ShowCursor);
    write!(output, "{enter_alternate_screen}{show_cursor}")?;
    output.flush()?;

    let events = output.event_reader();
    let backend = TerminaBackend::new(output);
    let term = Terminal::new(backend)?;
    Ok((term, events))
}

fn restore_terminal(term: &mut AppTerminal) -> Result<(), Box<dyn Error>> {
    let leave_alternate_screen = decreset!(ClearAndEnableAlternateScreen);
    let backend = term.backend_mut();
    write!(backend, "{leave_alternate_screen}")?;
    backend.flush()?;
    Ok(())
}
