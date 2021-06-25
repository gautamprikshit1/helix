use bitflags::bitflags;

bitflags! {
    /// Represents key modifiers (shift, control, alt).
    #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
    pub struct KeyModifiers: u8 {
        const SHIFT = 0b0000_0001;
        const CONTROL = 0b0000_0010;
        const ALT = 0b0000_0100;
        const NONE = 0b0000_0000;
    }
}

#[cfg(feature = "term")]
impl From<KeyModifiers> for crossterm::event::KeyModifiers {
    fn from(key_modifiers: KeyModifiers) -> Self {
        use crossterm::event::KeyModifiers as CKeyModifiers;

        let mut result = CKeyModifiers::NONE;

        if key_modifiers & KeyModifiers::SHIFT != KeyModifiers::NONE {
            result &= CKeyModifiers::SHIFT;
        }

        if key_modifiers & KeyModifiers::CONTROL != KeyModifiers::NONE {
            result &= CKeyModifiers::CONTROL;
        }

        if key_modifiers & KeyModifiers::ALT != KeyModifiers::NONE {
            result &= CKeyModifiers::ALT;
        }

        result
    }
}

#[cfg(feature = "term")]
impl From<crossterm::event::KeyModifiers> for KeyModifiers {
    fn from(val: crossterm::event::KeyModifiers) -> Self {
        use crossterm::event::KeyModifiers as CKeyModifiers;

        let mut result = KeyModifiers::NONE;

        if val & CKeyModifiers::SHIFT != CKeyModifiers::NONE {
            result &= KeyModifiers::SHIFT;
        }

        if val & CKeyModifiers::CONTROL != CKeyModifiers::NONE {
            result &= KeyModifiers::CONTROL;
        }

        if val & CKeyModifiers::ALT != CKeyModifiers::NONE {
            result &= KeyModifiers::ALT;
        }

        result
    }
}

/// Represents a key.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum KeyCode {
    /// Backspace key.
    Backspace,
    /// Enter key.
    Enter,
    /// Left arrow key.
    Left,
    /// Right arrow key.
    Right,
    /// Up arrow key.
    Up,
    /// Down arrow key.
    Down,
    /// Home key.
    Home,
    /// End key.
    End,
    /// Page up key.
    PageUp,
    /// Page dow key.
    PageDown,
    /// Tab key.
    Tab,
    /// Shift + Tab key.
    BackTab,
    /// Delete key.
    Delete,
    /// Insert key.
    Insert,
    /// F key.
    ///
    /// `KeyCode::F(1)` represents F1 key, etc.
    F(u8),
    /// A character.
    ///
    /// `KeyCode::Char('c')` represents `c` character, etc.
    Char(char),
    /// Null.
    Null,
    /// Escape key.
    Esc,
}

#[cfg(feature = "term")]
impl From<KeyCode> for crossterm::event::KeyCode {
    fn from(key_code: KeyCode) -> Self {
        use crossterm::event::KeyCode as CKeyCode;

        match key_code {
            KeyCode::Backspace => CKeyCode::Backspace,
            KeyCode::Enter => CKeyCode::Enter,
            KeyCode::Left => CKeyCode::Left,
            KeyCode::Right => CKeyCode::Right,
            KeyCode::Up => CKeyCode::Up,
            KeyCode::Down => CKeyCode::Down,
            KeyCode::Home => CKeyCode::Home,
            KeyCode::End => CKeyCode::End,
            KeyCode::PageUp => CKeyCode::PageUp,
            KeyCode::PageDown => CKeyCode::PageDown,
            KeyCode::Tab => CKeyCode::Tab,
            KeyCode::BackTab => CKeyCode::BackTab,
            KeyCode::Delete => CKeyCode::Delete,
            KeyCode::Insert => CKeyCode::Insert,
            KeyCode::F(f_number) => CKeyCode::F(f_number),
            KeyCode::Char(character) => CKeyCode::Char(character),
            KeyCode::Null => CKeyCode::Null,
            KeyCode::Esc => CKeyCode::Esc,
        }
    }
}

#[cfg(feature = "term")]
impl From<crossterm::event::KeyCode> for KeyCode {
    fn from(val: crossterm::event::KeyCode) -> Self {
        use crossterm::event::KeyCode as CKeyCode;

        match val {
            CKeyCode::Backspace => KeyCode::Backspace,
            CKeyCode::Enter => KeyCode::Enter,
            CKeyCode::Left => KeyCode::Left,
            CKeyCode::Right => KeyCode::Right,
            CKeyCode::Up => KeyCode::Up,
            CKeyCode::Down => KeyCode::Down,
            CKeyCode::Home => KeyCode::Home,
            CKeyCode::End => KeyCode::End,
            CKeyCode::PageUp => KeyCode::PageUp,
            CKeyCode::PageDown => KeyCode::PageDown,
            CKeyCode::Tab => KeyCode::Tab,
            CKeyCode::BackTab => KeyCode::BackTab,
            CKeyCode::Delete => KeyCode::Delete,
            CKeyCode::Insert => KeyCode::Insert,
            CKeyCode::F(f_number) => KeyCode::F(f_number),
            CKeyCode::Char(character) => KeyCode::Char(character),
            CKeyCode::Null => KeyCode::Null,
            CKeyCode::Esc => KeyCode::Esc,
        }
    }
}
