use std::{collections::HashMap, time::Duration};

use crossterm::{
    event::{
        poll, read, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent,
        KeyEventKind, KeyModifiers, KeyboardEnhancementFlags, MouseButton, MouseEvent,
        MouseEventKind, PopKeyboardEnhancementFlags, PushKeyboardEnhancementFlags,
    },
    execute, queue, terminal,
};

use crate::engine::{Camera, Pos, ScreenPos};

/// Only LeftMouse and RightMouse actually support release events,
/// at least in WSL + Windows Terminal. Keyboard keys with just toggle
/// between pressed this frame and pressed after their first press.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Button {
    Quit,
    Up,
    Down,
    Left,
    Right,
    Jump,
    LeftMouse,
    RightMouse,
}

impl Button {
    fn from_key_event(event: &KeyEvent) -> Option<Self> {
        match event.code {
            KeyCode::Esc => Some(Self::Quit),
            KeyCode::Char(c) => match c {
                'c' => {
                    if event.modifiers == KeyModifiers::CONTROL {
                        Some(Self::Quit)
                    } else {
                        None
                    }
                }
                'w' => Some(Self::Up),
                'a' => Some(Self::Left),
                's' => Some(Self::Down),
                'd' => Some(Self::Right),
                ' ' => Some(Self::Jump),
                _ => None,
            },
            KeyCode::Up => Some(Self::Up),
            KeyCode::Down => Some(Self::Down),
            KeyCode::Left => Some(Self::Left),
            KeyCode::Right => Some(Self::Right),
            _ => None,
        }
    }

    fn from_mouse_event(event: &MouseEvent) -> Option<Self> {
        match event.kind {
            MouseEventKind::Up(mouse_button) | MouseEventKind::Down(mouse_button) => {
                match mouse_button {
                    MouseButton::Left => Some(Self::LeftMouse),
                    MouseButton::Right => Some(Self::RightMouse),
                    MouseButton::Middle => None,
                }
            }
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
    pub mouse_pos: Pos,
    state: HashMap<Button, ButtonState>,
}

impl Input {
    pub fn new() -> std::io::Result<Self> {
        terminal::enable_raw_mode()?;

        execute!(
            std::io::stdout(),
            EnableMouseCapture,
            PushKeyboardEnhancementFlags(KeyboardEnhancementFlags::all())
        )?;

        Ok(Self {
            state: HashMap::new(),
            mouse_pos: Pos::ZERO,
        })
    }

    pub fn button_state(&self, button: Button) -> ButtonState {
        *self.state.get(&button).unwrap_or(&ButtonState::Released)
    }

    pub fn pressed(&self, button: Button) -> bool {
        matches!(
            self.button_state(button),
            ButtonState::PressedThisFrame | ButtonState::Pressed
        )
    }

    pub fn pressed_this_frame(&self, button: Button) -> bool {
        matches!(self.button_state(button), ButtonState::PressedThisFrame)
    }

    pub fn released(&self, button: Button) -> bool {
        matches!(
            self.button_state(button),
            ButtonState::ReleasedThisFrame | ButtonState::Released
        )
    }

    pub fn released_this_frame(&self, button: Button) -> bool {
        matches!(self.button_state(button), ButtonState::ReleasedThisFrame)
    }

    pub fn update(&mut self, camera: &Camera) -> std::io::Result<()> {
        for button_state in self.state.values_mut() {
            match button_state {
                ButtonState::PressedThisFrame => *button_state = ButtonState::Pressed,
                ButtonState::ReleasedThisFrame => *button_state = ButtonState::Released,
                _ => (),
            }
        }

        while poll(Duration::ZERO)? {
            match read()? {
                Event::Key(key_event) => {
                    if let Some(button) = Button::from_key_event(&key_event) {
                        match key_event.kind {
                            KeyEventKind::Press => {
                                self.state.insert(button, ButtonState::PressedThisFrame);
                            }
                            KeyEventKind::Release => {
                                self.state.insert(button, ButtonState::ReleasedThisFrame);
                            }
                            _ => (),
                        }
                    }
                }
                Event::Mouse(mouse_event) => {
                    let mouse_pos =
                        ScreenPos::new(mouse_event.column.into(), mouse_event.row.into())
                            - camera.frame_pos;
                    self.mouse_pos = camera.pos + mouse_pos.into();

                    if let Some(button) = Button::from_mouse_event(&mouse_event) {
                        match mouse_event.kind {
                            MouseEventKind::Down(_) => {
                                self.state.insert(button, ButtonState::PressedThisFrame);
                            }
                            MouseEventKind::Up(_) => {
                                self.state.insert(button, ButtonState::ReleasedThisFrame);
                            }
                            _ => (),
                        }
                    }
                }
                _ => (),
            }
        }
        Ok(())
    }
}

impl Drop for Input {
    fn drop(&mut self) {
        let _ = terminal::disable_raw_mode();
        let _ = queue!(
            std::io::stdout(),
            DisableMouseCapture,
            PopKeyboardEnhancementFlags
        );
    }
}
