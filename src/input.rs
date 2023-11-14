use std::{collections::HashMap, time::Duration};

use crossterm::{
    event::{
        poll, read, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers, KeyboardEnhancementFlags,
        PushKeyboardEnhancementFlags,
    },
    execute, terminal,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Button {
    Quit,
    Up,
    Down,
    Left,
    Right,
    Jump,
}

impl Button {
    fn from(event: &KeyEvent) -> Option<Self> {
        match event.code {
            KeyCode::Esc => Some(Button::Quit),
            KeyCode::Char(c) => match c {
                'c' => {
                    if event.modifiers == KeyModifiers::CONTROL {
                        Some(Button::Quit)
                    } else {
                        None
                    }
                }
                'w' => Some(Button::Up),
                'a' => Some(Button::Left),
                's' => Some(Button::Down),
                'd' => Some(Button::Right),
                ' ' => Some(Button::Jump),
                _ => None,
            },
            KeyCode::Up => Some(Button::Up),
            KeyCode::Down => Some(Button::Down),
            KeyCode::Left => Some(Button::Left),
            KeyCode::Right => Some(Button::Right),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonState {
    PressedThisFrame,
    Pressed,
    ReleasedThisFrame,
    Released,
}

pub struct Input {
    pub state: HashMap<Button, ButtonState>,
}

impl Drop for Input {
    fn drop(&mut self) {
        let _ = terminal::disable_raw_mode();
    }
}

impl Input {
    pub fn new() -> std::io::Result<Self> {
        terminal::enable_raw_mode()?;
        execute!(
            std::io::stdout(),
            PushKeyboardEnhancementFlags(KeyboardEnhancementFlags::all())
        )?;
        Ok(Self {
            state: HashMap::new(),
        })
    }

    pub fn pressed(&self, button: Button) -> bool {
        match *self.state.get(&button).unwrap_or(&ButtonState::Released) {
            ButtonState::PressedThisFrame | ButtonState::Pressed => true,
            _ => false,
        }
    }

    pub fn pressed_this_frame(&self, button: Button) -> bool {
        match *self.state.get(&button).unwrap_or(&ButtonState::Released) {
            ButtonState::PressedThisFrame => true,
            _ => false,
        }
    }

    pub fn released(&self, button: Button) -> bool {
        match *self.state.get(&button).unwrap_or(&ButtonState::Released) {
            ButtonState::ReleasedThisFrame | ButtonState::Released => true,
            _ => false,
        }
    }

    pub fn released_this_frame(&self, button: Button) -> bool {
        match *self.state.get(&button).unwrap_or(&ButtonState::Released) {
            ButtonState::ReleasedThisFrame => true,
            _ => false,
        }
    }

    pub fn update(&mut self) -> std::io::Result<()> {
        for button_state in self.state.values_mut() {
            match button_state {
                ButtonState::PressedThisFrame => *button_state = ButtonState::Pressed,
                ButtonState::ReleasedThisFrame => *button_state = ButtonState::Released,
                _ => (),
            }
        }

        while poll(Duration::ZERO)? {
            if let Event::Key(event) = read()? {
                if let Some(button) = Button::from(&event) {
                    println!("{:?}", event.kind);
                    match self.state.get(&button).unwrap_or(&ButtonState::Released) {
                        ButtonState::Pressed => {
                            if event.kind == KeyEventKind::Release {
                                self.state.insert(button, ButtonState::ReleasedThisFrame);
                            }
                        }
                        ButtonState::Released => {
                            if event.kind == KeyEventKind::Press {
                                self.state.insert(button, ButtonState::PressedThisFrame);
                            }
                        }
                        _ => (),
                    }
                }
            }
        }
        Ok(())
    }
}
