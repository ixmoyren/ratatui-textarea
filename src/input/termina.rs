use super::{Input, Key};
use ratatui_termina::termina::event::{
    Event, KeyCode, KeyEvent, KeyEventKind, Modifiers, MouseEvent, MouseEventKind,
};

impl From<Event> for Input {
    /// Convert [`ratatui_termina::termina::event::Event`] into [`Input`].
    fn from(event: Event) -> Self {
        match event {
            Event::Key(key) => Self::from(key),
            Event::Mouse(mouse) => Self::from(mouse),
            _ => Self::default(),
        }
    }
}

impl From<KeyCode> for Key {
    /// Convert [`ratatui_termina::termina::event::KeyCode`] into [`Key`].
    fn from(code: KeyCode) -> Self {
        match code {
            KeyCode::Char(c) => Key::Char(c),
            KeyCode::Backspace => Key::Backspace,
            KeyCode::Enter => Key::Enter,
            KeyCode::Left => Key::Left,
            KeyCode::Right => Key::Right,
            KeyCode::Up => Key::Up,
            KeyCode::Down => Key::Down,
            KeyCode::Tab => Key::Tab,
            KeyCode::Delete => Key::Delete,
            KeyCode::Home => Key::Home,
            KeyCode::End => Key::End,
            KeyCode::PageUp => Key::PageUp,
            KeyCode::PageDown => Key::PageDown,
            KeyCode::Escape => Key::Esc,
            KeyCode::Function(x) => Key::F(x),
            _ => Key::Null,
        }
    }
}

impl From<KeyEvent> for Input {
    /// Convert [`ratatui_termina::termina::event::KeyEvent`] into [`Input`].
    fn from(key: KeyEvent) -> Self {
        if key.kind == KeyEventKind::Release {
            // On Windows or when an enhanced keyboard protocol is enabled, key release events can
            // be reported. Ignore them. (#14)
            return Self::default();
        }
        let ctrl = key.modifiers.contains(Modifiers::CONTROL);
        let alt = key.modifiers.contains(Modifiers::ALT);

        if key.code == KeyCode::BackTab {
            // `termina` does not support Shift+Tab, but reports it as BackTab.
            return Self {
                key: Key::Tab,
                shift: true,
                ctrl,
                alt,
            };
        }

        let shift = key.modifiers.contains(Modifiers::SHIFT);
        let key = Key::from(key.code);

        Self {
            key,
            ctrl,
            alt,
            shift,
        }
    }
}

impl From<MouseEventKind> for Key {
    /// Convert [`ratatui_termina::termina::event::MouseEventKind`] into [`Key`].
    fn from(kind: MouseEventKind) -> Self {
        match kind {
            MouseEventKind::ScrollDown => Key::MouseScrollDown,
            MouseEventKind::ScrollUp => Key::MouseScrollUp,
            _ => Key::Null,
        }
    }
}

impl From<MouseEvent> for Input {
    /// Convert [`ratatui_termina::termina::event::MouseEvent`] into [`Input`].
    fn from(mouse: MouseEvent) -> Self {
        let key = Key::from(mouse.kind);
        let ctrl = mouse.modifiers.contains(Modifiers::CONTROL);
        let alt = mouse.modifiers.contains(Modifiers::ALT);
        let shift = mouse.modifiers.contains(Modifiers::SHIFT);
        Self {
            key,
            ctrl,
            alt,
            shift,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::tests::input;
    use ratatui_termina::termina::event::KeyEventState;

    fn key_event(code: KeyCode, modifiers: Modifiers) -> KeyEvent {
        KeyEvent {
            code,
            modifiers,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }
    }

    fn mouse_event(kind: MouseEventKind, modifiers: Modifiers) -> MouseEvent {
        MouseEvent {
            kind,
            column: 1,
            row: 1,
            modifiers,
        }
    }

    #[test]
    fn key_to_input() {
        for (from, to) in [
            (
                key_event(KeyCode::Char('a'), Modifiers::NONE),
                input(Key::Char('a'), false, false, false),
            ),
            (
                key_event(KeyCode::Enter, Modifiers::NONE),
                input(Key::Enter, false, false, false),
            ),
            (
                key_event(KeyCode::Left, Modifiers::CONTROL),
                input(Key::Left, true, false, false),
            ),
            (
                key_event(KeyCode::Right, Modifiers::SHIFT),
                input(Key::Right, false, false, true),
            ),
            (
                key_event(KeyCode::Home, Modifiers::ALT),
                input(Key::Home, false, true, false),
            ),
            (
                key_event(
                    KeyCode::Function(1),
                    Modifiers::ALT | Modifiers::CONTROL | Modifiers::SHIFT,
                ),
                input(Key::F(1), true, true, true),
            ),
            (
                key_event(KeyCode::NumLock, Modifiers::CONTROL),
                input(Key::Null, true, false, false),
            ),
            (
                key_event(KeyCode::BackTab, Modifiers::NONE),
                input(Key::Tab, false, false, true),
            ),
        ] {
            assert_eq!(Input::from(from), to, "{:?} -> {:?}", from, to);
        }
    }

    #[test]
    fn mouse_to_input() {
        for (from, to) in [
            (
                mouse_event(MouseEventKind::ScrollDown, Modifiers::NONE),
                input(Key::MouseScrollDown, false, false, false),
            ),
            (
                mouse_event(MouseEventKind::ScrollUp, Modifiers::CONTROL),
                input(Key::MouseScrollUp, true, false, false),
            ),
            (
                mouse_event(MouseEventKind::ScrollUp, Modifiers::SHIFT),
                input(Key::MouseScrollUp, false, false, true),
            ),
            (
                mouse_event(MouseEventKind::ScrollDown, Modifiers::ALT),
                input(Key::MouseScrollDown, false, true, false),
            ),
            (
                mouse_event(
                    MouseEventKind::ScrollUp,
                    Modifiers::CONTROL | Modifiers::ALT,
                ),
                input(Key::MouseScrollUp, true, true, false),
            ),
            (
                mouse_event(MouseEventKind::Moved, Modifiers::CONTROL),
                input(Key::Null, true, false, false),
            ),
        ] {
            assert_eq!(Input::from(from), to, "{:?} -> {:?}", from, to);
        }
    }

    #[test]
    fn event_to_input() {
        for (from, to) in [
            (
                Event::Key(key_event(KeyCode::Char('a'), Modifiers::NONE)),
                input(Key::Char('a'), false, false, false),
            ),
            (
                Event::Mouse(mouse_event(MouseEventKind::ScrollDown, Modifiers::NONE)),
                input(Key::MouseScrollDown, false, false, false),
            ),
            (Event::FocusIn, input(Key::Null, false, false, false)),
        ] {
            assert_eq!(Input::from(from.clone()), to, "{:?} -> {:?}", from, to);
        }
    }

    // Regression for https://github.com/rhysd/tui-textarea/issues/14
    #[test]
    fn ignore_key_release_event() {
        let mut from = key_event(KeyCode::Char('a'), Modifiers::NONE);
        from.kind = KeyEventKind::Release;
        let to = input(Key::Null, false, false, false);
        assert_eq!(Input::from(from), to, "{:?} -> {:?}", from, to);
    }
}
